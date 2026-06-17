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

/// Pad or truncate `s` to exactly `target_cols` terminal display columns.
fn pad_to(s: &str, target_cols: usize) -> String {
    let w = UnicodeWidthStr::width(s);
    if w >= target_cols {
        // Truncate with ellipsis
        let mut out = String::new();
        let mut cols = 0usize;
        for c in s.chars() {
            let cw = unicode_width::UnicodeWidthChar::width(c).unwrap_or(1);
            if cols + cw > target_cols.saturating_sub(1) {
                out.push('.');
                break;
            }
            out.push(c);
            cols += cw;
        }
        // Pad any remaining space
        let cur_w = UnicodeWidthStr::width(out.as_str());
        if cur_w < target_cols {
            out.push_str(&" ".repeat(target_cols - cur_w));
        }
        out
    } else {
        format!("{}{}", s, " ".repeat(target_cols - w))
    }
}

// ── Core boxed menu ────────────────────────────────────────────────────────

/// Draw a full-terminal-width box menu with a centred title and navigate with
/// arrow/j/k keys.  Returns `Some(index)` on Enter, `None` on Esc/q.
fn run_boxed_menu(title: &str, items: &[String]) -> Result<Option<usize>> {
    let term_cols = terminal_width();

    // Total box width = term_cols (fills the whole terminal row)
    // Box chars consume 2 cols (left ║ + right ║), so inner = term_cols - 2
    let inner = term_cols.saturating_sub(2).max(40);

    let top = format!("╔{}╗", "═".repeat(inner));
    let mid = format!("╠{}╣", "═".repeat(inner));
    let bot = format!("╚{}╝", "═".repeat(inner));

    // Centred title: ║ + <spaces> + title + <spaces> + ║
    // The spaces fill `inner` columns total.
    let title_dw = UnicodeWidthStr::width(title);
    let (lpad, rpad) = if title_dw < inner {
        let total = inner - title_dw;
        (total / 2, total - total / 2)
    } else {
        (0, 0)
    };
    let title_row = format!("║{}{}{}║", " ".repeat(lpad), pad_to(title, title_dw.min(inner)), " ".repeat(rpad));

    // Item text columns:
    // Each row = ║(1) + space(1) + marker(1) + space(1) + TEXT + space(1) + ║(1)
    // = 6 fixed cols + TEXT cols = inner + 2
    // => TEXT cols = inner + 2 - 6 = inner - 4
    let text_cols = inner.saturating_sub(4);

    // ── Raw mode ───────────────────────────────────────────────────────────
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    let _ = execute!(stdout, cursor::Hide);

    let mut selected  = 0usize;
    let max_visible   = 18usize;
    let mut prev_rows = 0u16;

    let result = loop {
        let visible = items.len().min(max_visible);
        let start   = if selected >= max_visible { selected - max_visible + 1 } else { 0 };
        let end     = start + visible;

        // Total rows this frame: top(1) + title(1) + mid(1) + items + bot(1)
        let frame_rows = (3 + visible + 1) as u16;

        // Move cursor up to overwrite previous frame
        if prev_rows > 0 {
            let _ = queue!(stdout,
                cursor::MoveUp(prev_rows),
                cursor::MoveToColumn(0)
            );
        }
        prev_rows = frame_rows;

        // ── Draw header ────────────────────────────────────────────────────
        let _ = queue!(stdout,
            SetForegroundColor(Color::Green),
            Print(&top),  Print("\r\n"),
            Print(&title_row), Print("\r\n"),
            Print(&mid),  Print("\r\n")
        );

        // ── Draw items ─────────────────────────────────────────────────────
        for i in start..end {
            let item_text = pad_to(&items[i], text_cols);

            if i == selected {
                // Active: bright green + bold, marker ">"
                let _ = queue!(stdout,
                    SetForegroundColor(Color::Rgb { r: 0, g: 220, b: 100 }),
                    SetAttribute(Attribute::Bold),
                    Print("║ > "),
                    Print(&item_text),
                    Print(" ║"),
                    SetAttribute(Attribute::Reset),
                    Print("\r\n")
                );
            } else {
                // Inactive: dim border, white text
                let _ = queue!(stdout,
                    SetForegroundColor(Color::Rgb { r: 0, g: 220, b: 100 }),
                    Print("║ "),
                    SetForegroundColor(Color::Reset),
                    Print("  "),
                    SetForegroundColor(Color::White),
                    Print(&item_text),
                    SetForegroundColor(Color::Rgb { r: 0, g: 220, b: 100 }),
                    Print(" ║\r\n")
                );
            }
        }

        // ── Draw footer — always closes the box ────────────────────────────
        let _ = queue!(stdout,
            SetForegroundColor(Color::Rgb { r: 0, g: 220, b: 100 }),
            Print(&bot), Print("\r\n"),
            ResetColor
        );
        let _ = stdout.flush();

        // ── Key handling ───────────────────────────────────────────────────
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up   | KeyCode::Char('k') => {
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
    // All ASCII — guaranteed to display correctly in any terminal.
    let items: Vec<String> = vec![
        " [1] Discovery".into(),
        " [2] Mapping".into(),
        " [3] Crawling".into(),
        " [4] Analysis".into(),
        " [5] Automation".into(),
        " [O] Outputs".into(),
        " [S] Settings".into(),
        " [Q] Exit".into(),
    ];

    match run_boxed_menu("  R E C # 2 5   -   M A I N   M E N U  ", &items)? {
        Some(0) => Ok(TopMenuChoice::Category(0)),
        Some(1) => Ok(TopMenuChoice::Category(1)),
        Some(2) => Ok(TopMenuChoice::Category(2)),
        Some(3) => Ok(TopMenuChoice::Category(3)),
        Some(4) => Ok(TopMenuChoice::Category(4)),
        Some(5) => Ok(TopMenuChoice::Outputs),
        Some(6) => Ok(TopMenuChoice::Settings),
        _       => Ok(TopMenuChoice::Exit),
    }
}

// ── Tool menu ──────────────────────────────────────────────────────────────

pub fn tool_menu(cat: &'static Category) -> Result<Option<&'static Tool>> {
    println!();
    let title = format!("  {}  -  Select Tool  ", cat.name);
    let mut items: Vec<String> = cat.tools.iter().enumerate()
        .map(|(i, t)| format!(" [{:>2}] {}", i + 1, t.name))
        .collect();
    items.push(" [<] Back".to_string());

    match run_boxed_menu(&title, &items)? {
        Some(i) if i < cat.tools.len() => Ok(Some(&cat.tools[i])),
        _ => Ok(None),
    }
}

// ── Mode menu ──────────────────────────────────────────────────────────────

pub fn mode_menu(tool: &'static Tool) -> Result<Option<&'static Mode>> {
    println!();
    let title = format!("  {}  -  Select Mode  ", tool.name);
    let mut items: Vec<String> = tool.modes.iter().enumerate()
        .map(|(i, m)| format!(" [{}] {}", i + 1, m.name))
        .collect();
    items.push(" [<] Back".to_string());

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
            .enumerate()
            .map(|(i, (name, size, age))| {
                format!(" [{:>2}] {:<45} {:>8}  {}", i + 1, name, size, age)
            })
            .collect();
        items.push(" [<] Back".to_string());

        match run_boxed_menu("  Outputs  -  Browse & Manage  ", &items)? {
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
    let Ok(d) = std::time::SystemTime::now().duration_since(mtime) else { return "now".into(); };
    let s = d.as_secs();
    if s < 60 { "now".into() }
    else if s < 3600   { format!("{}m ago", s / 60) }
    else if s < 86400  { format!("{}h ago", s / 3600) }
    else               { format!("{}d ago", s / 86400) }
}

fn file_action_menu(path: &str) -> Result<()> {
    println!();
    let fname = std::path::Path::new(path)
        .file_name().unwrap_or_default()
        .to_string_lossy().into_owned();

    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|_| "[binary or unreadable file]".into());
    let preview: Vec<&str> = content.lines().take(20).collect();
    print_box(&format!("Preview - {}", fname), &preview);
    println!();

    let actions = vec![
        " [E] Open in nano (edit)".to_string(),
        " [D] Delete file".to_string(),
        " [<] Back".to_string(),
    ];

    match run_boxed_menu("  File Action  ", &actions)? {
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
        new.api_keys.shodan        = Input::with_theme(&theme).with_prompt("Shodan API key").default(config.api_keys.shodan.clone()).allow_empty(true).interact_text()?;
        new.api_keys.github        = Input::with_theme(&theme).with_prompt("GitHub token").default(config.api_keys.github.clone()).allow_empty(true).interact_text()?;
        new.api_keys.censys_id     = Input::with_theme(&theme).with_prompt("Censys ID").default(config.api_keys.censys_id.clone()).allow_empty(true).interact_text()?;
        new.api_keys.censys_secret = Input::with_theme(&theme).with_prompt("Censys secret").default(config.api_keys.censys_secret.clone()).allow_empty(true).interact_text()?;
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
