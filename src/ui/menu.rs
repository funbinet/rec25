//! Interactive menus: top-level, tool, mode, outputs, settings.

use anyhow::Result;
use dialoguer::{Input, Select, Confirm};

use crate::config::Config;
use crate::tools::types::{Category, Mode, Tool};
use crate::ui::theme::{
    clear_screen, cprintln, draw_banner, grey, print_box,
    print_info, print_success, section_header, wait_key,
};
use crate::input::hacker_theme;

// ── Top-level choice ───────────────────────────────────────────────────────

#[derive(Debug)]
pub enum TopMenuChoice {
    Category(usize),
    Outputs,
    Settings,
    Exit,
}

/// Show the root menu and return the user's choice.
pub fn top_menu() -> Result<TopMenuChoice> {
    clear_screen();
    draw_banner();

    let categories = ["Discovery", "Mapping", "Crawling", "Analysis", "Automation"];
    let mut items: Vec<String> = categories.iter().map(|s| format!("  📡  {}", s)).collect();
    // Spacer + utility items.
    items.push("  ──────────────────────────────".to_string());
    items.push("  📂  Outputs".to_string());
    items.push("  ⚙   Settings".to_string());
    items.push("  🚪  Exit".to_string());

    let idx = Select::with_theme(&hacker_theme())
        .with_prompt("  Select category")
        .items(&items)
        .default(0)
        .interact_opt()?;

    let choice = match idx {
        Some(i) if i < 5                  => TopMenuChoice::Category(i),
        Some(i) if i == 5                 => {
            // Spacer selected — re-show menu.
            return top_menu();
        }
        Some(i) if i == 6                 => TopMenuChoice::Outputs,
        Some(i) if i == 7                 => TopMenuChoice::Settings,
        Some(8) | None                    => TopMenuChoice::Exit,
        _                                 => TopMenuChoice::Exit,
    };

    Ok(choice)
}

// ── Tool menu ──────────────────────────────────────────────────────────────

/// Show the tool list for a category.  Returns `None` when the user goes back.
pub fn tool_menu(cat: &'static Category) -> Result<Option<&'static Tool>> {
    clear_screen();
    draw_banner();
    section_header(&format!("{} — Select Tool", cat.name));

    let mut items: Vec<String> = cat
        .tools
        .iter()
        .map(|t| format!("  {}", t.name))
        .collect();
    items.push("  ← Back".to_string());

    let idx = Select::with_theme(&hacker_theme())
        .with_prompt("  Tool")
        .items(&items)
        .default(0)
        .interact_opt()?;

    match idx {
        Some(i) if i < cat.tools.len() => Ok(Some(&cat.tools[i])),
        _                               => Ok(None),
    }
}

// ── Mode menu ──────────────────────────────────────────────────────────────

/// Show the mode list for a tool.  Returns `None` when the user goes back.
pub fn mode_menu(tool: &'static Tool) -> Result<Option<&'static Mode>> {
    clear_screen();
    draw_banner();
    section_header(&format!("{} — Select Mode", tool.name));

    let mut items: Vec<String> = tool
        .modes
        .iter()
        .map(|m| format!("  {}", m.name))
        .collect();
    items.push("  ← Back".to_string());

    let idx = Select::with_theme(&hacker_theme())
        .with_prompt("  Mode")
        .items(&items)
        .default(0)
        .interact_opt()?;

    match idx {
        Some(i) if i < tool.modes.len() => Ok(Some(&tool.modes[i])),
        _                                => Ok(None),
    }
}

// ── Outputs menu ───────────────────────────────────────────────────────────

/// Browse, preview, and manage output files.
pub fn outputs_menu(config: &Config) -> Result<()> {
    loop {
        clear_screen();
        draw_banner();
        section_header("Outputs — Browse & Manage");

        // Collect files from output dir, sorted newest first.
        let files = collect_output_files(config.output_dir());

        if files.is_empty() {
            print_info("No output files found yet.");
            cprintln(grey(), "  Run a tool first to generate output.");
            wait_key();
            return Ok(());
        }

        let mut items: Vec<String> = files
            .iter()
            .map(|(name, size, age)| format!("  {:50} {:>8}  {}", name, size, age))
            .collect();
        items.push("  ← Back".to_string());

        let idx = Select::with_theme(&hacker_theme())
            .with_prompt("  Select file")
            .items(&items)
            .default(0)
            .interact_opt()?;

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

/// Collect (filename, human-size, age-str) tuples sorted newest-first.
fn collect_output_files(dir: &str) -> Vec<(String, String, String)> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return vec![];
    };

    let mut files: Vec<(String, u64, std::time::SystemTime)> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let meta = e.metadata().ok()?;
            if !meta.is_file() { return None; }
            let name  = e.file_name().to_string_lossy().into_owned();
            let size  = meta.len();
            let mtime = meta.modified().ok()?;
            Some((name, size, mtime))
        })
        .collect();

    files.sort_by(|a, b| b.2.cmp(&a.2));

    files
        .into_iter()
        .map(|(name, size, mtime)| {
            let size_str = human_size(size);
            let age_str  = human_age(mtime);
            (name, size_str, age_str)
        })
        .collect()
}

fn human_size(bytes: u64) -> String {
    if bytes < 1024 { format!("{} B", bytes) }
    else if bytes < 1024 * 1024 { format!("{:.1} KB", bytes as f64 / 1024.0) }
    else { format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0)) }
}

fn human_age(mtime: std::time::SystemTime) -> String {
    let now = std::time::SystemTime::now();
    let Ok(dur) = now.duration_since(mtime) else { return "just now".to_string(); };
    let secs = dur.as_secs();
    if secs < 60 { "just now".to_string() }
    else if secs < 3600 { format!("{}m ago", secs / 60) }
    else if secs < 86400 { format!("{}h ago", secs / 3600) }
    else { format!("{}d ago", secs / 86400) }
}

/// Sub-menu for a single output file: preview → open/delete/back.
fn file_action_menu(path: &str) -> Result<()> {
    clear_screen();
    draw_banner();

    let fname   = std::path::Path::new(path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    // Preview first 20 lines.
    let content = std::fs::read_to_string(path)
        .unwrap_or_else(|_| "[binary / unreadable file]".to_string());
    let preview: Vec<&str> = content.lines().take(20).collect();
    print_box(&format!("Preview — {}", fname), &preview);

    let actions = [
        "  📝  Open in nano (edit)",
        "  🗑   Delete file",
        "  ← Back",
    ];

    let idx = Select::with_theme(&hacker_theme())
        .with_prompt("  Action")
        .items(&actions)
        .default(2)
        .interact_opt()?;

    match idx {
        Some(0) => {
            // Spawn nano — blocks until the user quits nano; returns to REC#25.
            let _ = std::process::Command::new("nano").arg(path).status();
        }
        Some(1) => {
            let confirm = Confirm::with_theme(&hacker_theme())
                .with_prompt(format!("  Delete '{}'?", fname))
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

/// Show and optionally edit configuration.  Returns `Some(new_config)` if saved.
pub fn settings_menu(config: &Config) -> Result<Option<Config>> {
    clear_screen();
    draw_banner();
    section_header("Settings");

    // Display current values.
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

    let theme = hacker_theme();
    let mut new = config.clone();

    // ── Editable fields ────────────────────────────────────────────────────

    new.general.timeout_secs = Input::with_theme(&theme)
        .with_prompt("  Timeout (seconds)")
        .default(config.general.timeout_secs)
        .interact_text()?;

    new.general.preview_lines = Input::with_theme(&theme)
        .with_prompt("  Preview lines")
        .default(config.general.preview_lines)
        .interact_text()?;

    new.general.output_dir = Input::with_theme(&theme)
        .with_prompt("  Output directory")
        .default(config.general.output_dir.clone())
        .interact_text()?;

    let edit_keys = Confirm::with_theme(&theme)
        .with_prompt("  Edit API keys?")
        .default(false)
        .interact()?;

    if edit_keys {
        new.api_keys.shodan = Input::with_theme(&theme)
            .with_prompt("  Shodan API key")
            .default(config.api_keys.shodan.clone())
            .allow_empty(true)
            .interact_text()?;

        new.api_keys.github = Input::with_theme(&theme)
            .with_prompt("  GitHub token")
            .default(config.api_keys.github.clone())
            .allow_empty(true)
            .interact_text()?;

        new.api_keys.censys_id = Input::with_theme(&theme)
            .with_prompt("  Censys ID")
            .default(config.api_keys.censys_id.clone())
            .allow_empty(true)
            .interact_text()?;

        new.api_keys.censys_secret = Input::with_theme(&theme)
            .with_prompt("  Censys secret")
            .default(config.api_keys.censys_secret.clone())
            .allow_empty(true)
            .interact_text()?;
    }

    let save = Confirm::with_theme(&theme)
        .with_prompt("  Save settings?")
        .default(true)
        .interact()?;

    if save {
        new.save()?;
        print_success("Settings saved to /opt/rec25/config/config.toml");
        wait_key();
        Ok(Some(new))
    } else {
        Ok(None)
    }
}
