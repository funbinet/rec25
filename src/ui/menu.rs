//! Interactive menus: top-level, tool, mode, outputs, settings.

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    execute, queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal,
};
use dialoguer::{Confirm, Input};
use std::io::{self, Write};

use crate::config::Config;
use crate::tools::types::{Category, Mode, Tool};
use crate::ui::theme::{
    cprintln, green, grey, print_box, print_info, print_success, section_header, terminal_width, wait_key,
};
use crate::input::hacker_theme;

fn run_boxed_menu(title: &str, items: &[String]) -> Result<Option<usize>> {
    let term_width = terminal_width();
    let width = term_width.saturating_sub(2).max(40);

    let top = format!("╔{}╗", "═".repeat(width));
    let title_line = format!("║{:^width$}║", title, width = width);
    let mid = format!("╠{}╣", "═".repeat(width));
    let bot = format!("╚{}╝", "═".repeat(width));

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    let _ = execute!(stdout, cursor::Hide);

    let mut selected = 0;
    let max_display = 15;
    let mut first_draw = true;

    let result = loop {
        let display_count = items.len().min(max_display);
        let lines_to_draw = display_count as u16 + 4; 

        if !first_draw {
            let _ = queue!(stdout, cursor::MoveUp(lines_to_draw), cursor::MoveToColumn(0));
        }
        first_draw = false;

        let _ = queue!(stdout, 
            SetForegroundColor(Color::Green), Print(&top), Print("\r\n"),
            Print(&title_line), Print("\r\n"),
            Print(&mid), Print("\r\n")
        );

        let mut start_idx = 0;
        if selected >= max_display {
            start_idx = selected - max_display + 1;
        }
        let end_idx = start_idx + display_count;

        for i in start_idx..end_idx {
            let item = &items[i];
            let prefix = if i == selected { "║ ❯ " } else { "║   " };
            let is_active = i == selected;

            let text_space = width.saturating_sub(4);
            let display_text = if item.len() > text_space {
                format!("{}...", &item[..text_space.saturating_sub(3)])
            } else {
                format!("{:<text_space$}", item, text_space = text_space)
            };

            if is_active {
                let _ = queue!(stdout, 
                    SetForegroundColor(Color::Green),
                    SetAttribute(Attribute::Bold),
                    Print(prefix),
                    Print(&display_text),
                    SetAttribute(Attribute::Reset),
                    SetForegroundColor(Color::Green),
                    Print("║\r\n")
                );
            } else {
                let _ = queue!(stdout, 
                    SetForegroundColor(Color::Green),
                    Print(prefix),
                    Print(&display_text),
                    Print("║\r\n")
                );
            }
        }

        let _ = queue!(stdout, Print(&bot), Print("\r\n"));
        let _ = stdout.flush();

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        selected = selected.saturating_sub(1);
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if selected < items.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        break Some(selected);
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break None;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(event::KeyModifiers::CONTROL) => {
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

// ── Top-level choice ───────────────────────────────────────────────────────

#[derive(Debug)]
pub enum TopMenuChoice {
    Category(usize),
    Outputs,
    Settings,
    Exit,
}

pub fn top_menu() -> Result<TopMenuChoice> {
    println!();
    let categories = ["Discovery", "Mapping", "Crawling", "Analysis", "Automation"];
    let mut items: Vec<String> = categories.iter().map(|s| format!("📡 {}", s)).collect();
    items.push("──────────────────────────────".to_string());
    items.push("📂 Outputs".to_string());
    items.push("⚙  Settings".to_string());
    items.push("🚪 Exit".to_string());

    let idx = run_boxed_menu("MAIN MENU", &items)?;

    let choice = match idx {
        Some(i) if i < 5                  => TopMenuChoice::Category(i),
        Some(i) if i == 5                 => { return top_menu(); }
        Some(i) if i == 6                 => TopMenuChoice::Outputs,
        Some(i) if i == 7                 => TopMenuChoice::Settings,
        Some(8) | None                    => TopMenuChoice::Exit,
        _                                 => TopMenuChoice::Exit,
    };

    Ok(choice)
}

// ── Tool menu ──────────────────────────────────────────────────────────────

pub fn tool_menu(cat: &'static Category) -> Result<Option<&'static Tool>> {
    println!();
    let title = format!("{} — Select Tool", cat.name);
    let mut items: Vec<String> = cat.tools.iter().map(|t| t.name.to_string()).collect();
    items.push("← Back".to_string());

    let idx = run_boxed_menu(&title, &items)?;

    match idx {
        Some(i) if i < cat.tools.len() => Ok(Some(&cat.tools[i])),
        _                               => Ok(None),
    }
}

// ── Mode menu ──────────────────────────────────────────────────────────────

pub fn mode_menu(tool: &'static Tool) -> Result<Option<&'static Mode>> {
    println!();
    let title = format!("{} — Select Mode", tool.name);
    let mut items: Vec<String> = tool.modes.iter().map(|m| m.name.to_string()).collect();
    items.push("← Back".to_string());

    let idx = run_boxed_menu(&title, &items)?;

    match idx {
        Some(i) if i < tool.modes.len() => Ok(Some(&tool.modes[i])),
        _                                => Ok(None),
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
            .map(|(name, size, age)| format!("{:40} {:>8}  {}", name, size, age))
            .collect();
        items.push("← Back".to_string());

        let idx = run_boxed_menu("Outputs — Browse & Manage", &items)?;

        match idx {
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

fn human_size(bytes: u64) -> String {
    if bytes < 1024 { format!("{} B", bytes) }
    else if bytes < 1024 * 1024 { format!("{:.1} KB", bytes as f64 / 1024.0) }
    else { format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0)) }
}

fn human_age(mtime: std::time::SystemTime) -> String {
    let Ok(dur) = std::time::SystemTime::now().duration_since(mtime) else { return "just now".to_string(); };
    let secs = dur.as_secs();
    if secs < 60 { "just now".to_string() }
    else if secs < 3600 { format!("{}m ago", secs / 60) }
    else if secs < 86400 { format!("{}h ago", secs / 3600) }
    else { format!("{}d ago", secs / 86400) }
}

fn file_action_menu(path: &str) -> Result<()> {
    println!();
    let fname = std::path::Path::new(path).file_name().unwrap_or_default().to_string_lossy().into_owned();

    let content = std::fs::read_to_string(path).unwrap_or_else(|_| "[binary / unreadable file]".to_string());
    let preview: Vec<&str> = content.lines().take(20).collect();
    print_box(&format!("Preview — {}", fname), &preview);

    println!();
    let actions = vec![
        "📝 Open in nano (edit)".to_string(),
        "🗑  Delete file".to_string(),
        "← Back".to_string(),
    ];

    let idx = run_boxed_menu("Action", &actions)?;

    match idx {
        Some(0) => { let _ = std::process::Command::new("nano").arg(path).status(); }
        Some(1) => {
            let confirm = Confirm::with_theme(&hacker_theme())
                .with_prompt(format!("Delete '{}'?", fname))
                .default(false)
                .interact()?;
            if confirm {
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

    let api_shodan  = if config.api_keys.shodan.is_empty()  { "not set" } else { "set ✔" };
    let api_github  = if config.api_keys.github.is_empty()  { "not set" } else { "set ✔" };
    let api_censys  = if config.api_keys.censys_id.is_empty() { "not set" } else { "set ✔" };

    let current = [
        format!("Output dir    : {}", config.general.output_dir),
        format!("Log dir       : {}", config.general.log_dir),
        format!("Timeout       : {}s", config.general.timeout_secs),
        format!("Preview lines : {}", config.general.preview_lines),
        format!("Shodan API    : {}", api_shodan),
        format!("GitHub token  : {}", api_github),
        format!("Censys        : {}", api_censys),
    ];
    let refs: Vec<&str> = current.iter().map(String::as_str).collect();
    print_box("Current Settings", &refs);
    println!();

    let theme = hacker_theme();
    let mut new = config.clone();

    new.general.timeout_secs = Input::with_theme(&theme).with_prompt("Timeout (seconds)").default(config.general.timeout_secs).interact_text()?;
    new.general.preview_lines = Input::with_theme(&theme).with_prompt("Preview lines").default(config.general.preview_lines).interact_text()?;
    new.general.output_dir = Input::with_theme(&theme).with_prompt("Output directory").default(config.general.output_dir.clone()).interact_text()?;

    if Confirm::with_theme(&theme).with_prompt("Edit API keys?").default(false).interact()? {
        new.api_keys.shodan = Input::with_theme(&theme).with_prompt("Shodan API key").default(config.api_keys.shodan.clone()).allow_empty(true).interact_text()?;
        new.api_keys.github = Input::with_theme(&theme).with_prompt("GitHub token").default(config.api_keys.github.clone()).allow_empty(true).interact_text()?;
        new.api_keys.censys_id = Input::with_theme(&theme).with_prompt("Censys ID").default(config.api_keys.censys_id.clone()).allow_empty(true).interact_text()?;
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
