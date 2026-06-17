//! Interactive menus: top-level, tool, mode, outputs, settings.

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal,
};
use dialoguer::{Confirm, Input};
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

use crate::config::Config;
use crate::tools::types::{Category, Mode, Tool};
use crate::ui::theme::{
    cprintln, grey, print_box, print_info, print_success, section_header, terminal_width, wait_key,
};
use crate::input::hacker_theme;

// ── Display-width-aware padding ────────────────────────────────────────────

/// Pad `s` to `target_cols` display columns using spaces.
/// If `s` is already wider, it is truncated with "…".
fn pad_to(s: &str, target_cols: usize) -> String {
    let w = UnicodeWidthStr::width(s);
    if w >= target_cols {
        // truncate
        let mut out = String::new();
        let mut cols = 0;
        for c in s.chars() {
            let cw = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
            if cols + cw + 1 > target_cols { // leave room for ellipsis
                out.push('…');
                break;
            }
            out.push(c);
            cols += cw;
        }
        out
    } else {
        let mut out = s.to_string();
        out.push_str(&" ".repeat(target_cols - w));
        out
    }
}

// ── Core boxed menu (draws its own box via crossterm queues) ───────────────

fn run_boxed_menu(title: &str, items: &[String]) -> Result<Option<usize>> {
    let term_cols = terminal_width();
    // inner_cols = what fits between the left and right border chars
    let inner_cols = term_cols.saturating_sub(2).max(40);

    // Pre-compute box lines.
    let top   = format!("╔{}╗", "═".repeat(inner_cols));
    let mid   = format!("╠{}╣", "═".repeat(inner_cols));
    let bot   = format!("╚{}╝", "═".repeat(inner_cols));

    // Title centred inside inner_cols using display width.
    let title_dw = UnicodeWidthStr::width(title);
    let title_line = if title_dw <= inner_cols {
        let total_pad = inner_cols - title_dw;
        let lpad = total_pad / 2;
        let rpad = total_pad - lpad;
        format!("║{}{}{}║", " ".repeat(lpad), title, " ".repeat(rpad))
    } else {
        format!("║{}║", pad_to(title, inner_cols))
    };

    // Each item uses 4 cols for the prefix ("║ > " or "║   ") and 1 for the trailing "║".
    // So text_cols = inner_cols - 4 - 1 = inner_cols - 5
    let text_cols = inner_cols.saturating_sub(5);

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    let _ = execute!(stdout, cursor::Hide);

    let mut selected = 0usize;
    let max_display = 18usize;
    let mut prev_lines_drawn: u16 = 0;

    let result = loop {
        let visible_count = items.len().min(max_display);
        // scroll window
        let start = if selected >= max_display { selected - max_display + 1 } else { 0 };
        let end   = start + visible_count;

        // lines: top + title + mid + items + bot
        let lines_to_draw = (3 + visible_count + 1) as u16;

        // Move cursor back to top of previously drawn block
        if prev_lines_drawn > 0 {
            let _ = queue!(stdout,
                cursor::MoveUp(prev_lines_drawn),
                cursor::MoveToColumn(0)
            );
        }
        prev_lines_drawn = lines_to_draw;

        // Box top + centred title + mid separator
        let _ = queue!(stdout,
            SetForegroundColor(Color::Green),
            Print(&top), Print("\r\n"),
            Print(&title_line), Print("\r\n"),
            Print(&mid), Print("\r\n")
        );

        // Items
        for i in start..end {
            let item = &items[i];
            let is_selected = i == selected;

            // Pad the item text to exactly text_cols display cols
            let padded = pad_to(item, text_cols);

            if is_selected {
                let _ = queue!(stdout,
                    SetForegroundColor(Color::Green),
                    SetAttribute(Attribute::Bold),
                    Print("║ > "),
                    Print(&padded),
                    Print(" ║"),
                    SetAttribute(Attribute::Reset),
                    Print("\r\n")
                );
            } else {
                let _ = queue!(stdout,
                    SetForegroundColor(Color::Green),
                    Print("║   "),
                    SetForegroundColor(Color::White),
                    Print(&padded),
                    SetForegroundColor(Color::Green),
                    Print(" ║\r\n")
                );
            }
        }

        // Box bottom — always drawn, always closes the box
        let _ = queue!(stdout,
            SetForegroundColor(Color::Green),
            Print(&bot), Print("\r\n"),
            ResetColor
        );
        let _ = stdout.flush();

        // Key handling
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        if selected > 0 { selected -= 1; }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if selected + 1 < items.len() { selected += 1; }
                    }
                    KeyCode::Enter => { break Some(selected); }
                    KeyCode::Esc | KeyCode::Char('q') => { break None; }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        let _ = execute!(stdout, cursor::Show, ResetColor);
                        let _ = terminal::disable_raw_mode();
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        }
    };

    let _ = execute!(stdout, cursor::Show, ResetColor);
    let _ = terminal::disable_raw_mode();
    Ok(result)
}

// ── Top-level menu ─────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum TopMenuChoice {
    Category(usize),
    Outputs,
    Settings,
    Exit,
}

pub fn top_menu() -> Result<TopMenuChoice> {
    println!();
    // Plain text labels — no emoji that may render as boxes.
    let items: Vec<String> = vec![
        "[*] Discovery".into(),
        "[*] Mapping".into(),
        "[*] Crawling".into(),
        "[*] Analysis".into(),
        "[*] Automation".into(),
        "[F] Outputs".into(),
        "[=] Settings".into(),
        "[X] Exit".into(),
    ];

    let idx = run_boxed_menu("   R E C # 2 5   —   M A I N   M E N U", &items)?;

    let choice = match idx {
        Some(0) => TopMenuChoice::Category(0),
        Some(1) => TopMenuChoice::Category(1),
        Some(2) => TopMenuChoice::Category(2),
        Some(3) => TopMenuChoice::Category(3),
        Some(4) => TopMenuChoice::Category(4),
        Some(5) => TopMenuChoice::Outputs,
        Some(6) => TopMenuChoice::Settings,
        Some(7) | None => TopMenuChoice::Exit,
        _ => TopMenuChoice::Exit,
    };

    Ok(choice)
}

// ── Tool menu ──────────────────────────────────────────────────────────────

pub fn tool_menu(cat: &'static Category) -> Result<Option<&'static Tool>> {
    println!();
    let title = format!("{} — Select Tool", cat.name);
    let mut items: Vec<String> = cat.tools.iter().map(|t| t.name.to_string()).collect();
    items.push("<-- Back".to_string());

    match run_boxed_menu(&title, &items)? {
        Some(i) if i < cat.tools.len() => Ok(Some(&cat.tools[i])),
        _ => Ok(None),
    }
}

// ── Mode menu ──────────────────────────────────────────────────────────────

pub fn mode_menu(tool: &'static Tool) -> Result<Option<&'static Mode>> {
    println!();
    let title = format!("{} — Select Mode", tool.name);
    let mut items: Vec<String> = tool.modes.iter().map(|m| m.name.to_string()).collect();
    items.push("<-- Back".to_string());

    match run_boxed_menu(&title, &items)? {
        Some(i) if i < tool.modes.len() => Ok(Some(&tool.modes[i])),
        _ => Ok(None),
    }
}

// ── Outputs menu ───────────────────────────────────────────────────────────

pub fn outputs_menu(config: &Config) -> Result<()> {
    loop {
        println!();
        let files = collect_output_files(config.output_dir());

        if files.is_empty() {
            print_info("No output files found yet.");
            cprintln(grey(), "  Run a tool first to generate output.");
            wait_key();
            return Ok(());
        }

        let mut items: Vec<String> = files
            .iter()
            .map(|(name, size, age)| format!("{:45} {:>8}  {}", name, size, age))
            .collect();
        items.push("<-- Back".to_string());

        match run_boxed_menu("Outputs — Browse & Manage", &items)? {
            Some(i) if i < files.len() => {
                let path = format!("{}/{}", config.output_dir(), files[i].0);
                file_action_menu(&path)?;
            }
            _ => break,
        }
    }
    Ok(())
}

fn collect_output_files(dir: &str) -> Vec<(String, String, String)> {
    let Ok(entries) = std::fs::read_dir(dir) else { return vec![]; };
    let mut files: Vec<(String, u64, std::time::SystemTime)> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let meta = e.metadata().ok()?;
            if !meta.is_file() { return None; }
            Some((e.file_name().to_string_lossy().into_owned(), meta.len(), meta.modified().ok()?))
        }).collect();
    files.sort_by(|a, b| b.2.cmp(&a.2));
    files.into_iter().map(|(n, s, m)| (n, human_size(s), human_age(m))).collect()
}

fn human_size(b: u64) -> String {
    if b < 1024 { format!("{} B", b) }
    else if b < 1_048_576 { format!("{:.1} KB", b as f64 / 1024.0) }
    else { format!("{:.1} MB", b as f64 / 1_048_576.0) }
}

fn human_age(mtime: std::time::SystemTime) -> String {
    let Ok(d) = std::time::SystemTime::now().duration_since(mtime) else { return "now".to_string(); };
    let s = d.as_secs();
    if s < 60 { "now".into() }
    else if s < 3600 { format!("{}m ago", s / 60) }
    else if s < 86400 { format!("{}h ago", s / 3600) }
    else { format!("{}d ago", s / 86400) }
}

fn file_action_menu(path: &str) -> Result<()> {
    println!();
    let fname = std::path::Path::new(path)
        .file_name().unwrap_or_default()
        .to_string_lossy().into_owned();

    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|_| "[binary / unreadable file]".into());
    let preview: Vec<&str> = content.lines().take(20).collect();
    print_box(&format!("Preview — {}", fname), &preview);
    println!();

    let actions = vec![
        "[E] Open in nano (edit)".to_string(),
        "[D] Delete file".to_string(),
        "<-- Back".to_string(),
    ];

    match run_boxed_menu("File Action", &actions)? {
        Some(0) => { let _ = std::process::Command::new("nano").arg(path).status(); }
        Some(1) => {
            if Confirm::with_theme(&hacker_theme())
                .with_prompt(format!("Delete '{}'?", fname))
                .default(false).interact()?
            {
                std::fs::remove_file(path)?;
                print_success(&format!("Deleted: {}", fname));
                wait_key();
            }
        }
        _ => {}
    }
    Ok(())
}

// ── Settings menu ──────────────────────────────────────────────────────────

pub fn settings_menu(config: &Config) -> Result<Option<Config>> {
    println!();
    section_header("Settings");

    let api_shodan = if config.api_keys.shodan.is_empty()    { "not set" } else { "set [OK]" };
    let api_github = if config.api_keys.github.is_empty()    { "not set" } else { "set [OK]" };
    let api_censys = if config.api_keys.censys_id.is_empty() { "not set" } else { "set [OK]" };

    let current = [
        format!("Output dir    : {}", config.general.output_dir),
        format!("Log dir       : {}", config.general.log_dir),
        format!("Timeout       : {}s",  config.general.timeout_secs),
        format!("Preview lines : {}",   config.general.preview_lines),
        format!("Shodan API    : {}",   api_shodan),
        format!("GitHub token  : {}",   api_github),
        format!("Censys        : {}",   api_censys),
    ];
    let refs: Vec<&str> = current.iter().map(String::as_str).collect();
    print_box("Current Settings", &refs);
    println!();

    let theme = hacker_theme();
    let mut new = config.clone();

    new.general.timeout_secs  = Input::with_theme(&theme).with_prompt("Timeout (seconds)").default(config.general.timeout_secs).interact_text()?;
    new.general.preview_lines = Input::with_theme(&theme).with_prompt("Preview lines").default(config.general.preview_lines).interact_text()?;
    new.general.output_dir    = Input::with_theme(&theme).with_prompt("Output directory").default(config.general.output_dir.clone()).interact_text()?;

    if Confirm::with_theme(&theme).with_prompt("Edit API keys?").default(false).interact()? {
        new.api_keys.shodan         = Input::with_theme(&theme).with_prompt("Shodan API key").default(config.api_keys.shodan.clone()).allow_empty(true).interact_text()?;
        new.api_keys.github         = Input::with_theme(&theme).with_prompt("GitHub token").default(config.api_keys.github.clone()).allow_empty(true).interact_text()?;
        new.api_keys.censys_id      = Input::with_theme(&theme).with_prompt("Censys ID").default(config.api_keys.censys_id.clone()).allow_empty(true).interact_text()?;
        new.api_keys.censys_secret  = Input::with_theme(&theme).with_prompt("Censys secret").default(config.api_keys.censys_secret.clone()).allow_empty(true).interact_text()?;
    }

    if Confirm::with_theme(&theme).with_prompt("Save settings?").default(true).interact()? {
        new.save()?;
        print_success("Settings saved to /opt/rec25/config/config.toml");
        wait_key();
        Ok(Some(new))
    } else {
        Ok(None)
    }
}
