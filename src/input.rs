//! Input prompts with validation for each InputKind.

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

use crate::config::Config;
use crate::tools::types::{InputKind, Mode};

// ── Compiled regexes (lazy) ────────────────────────────────────────────────

fn domain_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)^([a-z0-9]([a-z0-9\-]{0,61}[a-z0-9])?\.)+[a-z]{2,}$").unwrap())
}

fn ip_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^\d{1,3}(\.\d{1,3}){3}(/\d{1,2})?$").unwrap())
}

fn ports_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^[\d,\-]+$").unwrap())
}

// ── Public API ─────────────────────────────────────────────────────────────

/// Collect every input required by `mode`, returning a map of
/// placeholder → value.  Handles the special "recent files" UI for File inputs.
pub fn collect_inputs(mode: &Mode, config: &Config) -> Result<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();

    for kind in mode.inputs {
        let value = match kind {
            InputKind::File => prompt_file(config)?,
            other           => prompt_text(*other)?,
        };
        // For duplicate kinds the last value wins (uncommon in practice).
        map.insert(kind.placeholder().to_string(), value);
    }

    Ok(map)
}

// ── Internal helpers ───────────────────────────────────────────────────────

/// Prompt for a text value, validating per-kind.
fn prompt_text(kind: InputKind) -> Result<String> {
    let theme = hacker_theme();
    let label = kind.label();

    let value: String = match kind {
        InputKind::Domain => Input::with_theme(&theme)
            .with_prompt(label)
            .validate_with(|s: &String| {
                if domain_re().is_match(s.trim()) { Ok(()) }
                else { Err(format!("'{}' does not look like a valid domain.", s)) }
            })
            .interact_text()?,

        InputKind::Ip => Input::with_theme(&theme)
            .with_prompt(label)
            .validate_with(|s: &String| {
                if ip_re().is_match(s.trim()) { Ok(()) }
                else { Err(format!("'{}' does not look like a valid IP or CIDR.", s)) }
            })
            .interact_text()?,

        InputKind::Url => Input::with_theme(&theme)
            .with_prompt(label)
            .validate_with(|s: &String| {
                let t = s.trim();
                if t.starts_with("http://") || t.starts_with("https://") { Ok(()) }
                else { Err("URL must begin with http:// or https://".to_string()) }
            })
            .interact_text()?,

        InputKind::Ports => Input::with_theme(&theme)
            .with_prompt(label)
            .validate_with(|s: &String| {
                if ports_re().is_match(s.trim()) { Ok(()) }
                else { Err("Ports must be digits, commas, or dashes (e.g. 80,443,8000-9000)".to_string()) }
            })
            .interact_text()?,

        _ => Input::with_theme(&theme)
            .with_prompt(label)
            .validate_with(|s: &String| {
                if s.trim().is_empty() { Err("Input cannot be empty.".to_string()) }
                else { Ok(()) }
            })
            .interact_text()?,
    };

    Ok(value.trim().to_string())
}

/// Prompt for a file path, showing recent output files as quick-picks.
fn prompt_file(config: &Config) -> Result<String> {
    let theme = hacker_theme();

    // Collect recent files from the output dir, newest first.
    let mut recent: Vec<(String, std::time::SystemTime)> = std::fs::read_dir(config.output_dir())
        .unwrap_or_else(|_| {
            // If the dir doesn't exist yet just return an empty iterator stub.
            std::fs::read_dir(".").unwrap()
        })
        .filter_map(|e| {
            let e = e.ok()?;
            let meta = e.metadata().ok()?;
            if !meta.is_file() { return None; }
            Some((e.path().to_string_lossy().into_owned(), meta.modified().ok()?))
        })
        .collect();
    recent.sort_by(|a, b| b.1.cmp(&a.1));
    recent.truncate(10);

    // Build menu: recent files + manual entry option.
    let manual_label = "[ Enter path manually ]";
    let mut options: Vec<String> = recent.iter().map(|(p, _)| p.clone()).collect();
    options.push(manual_label.to_string());

    let idx = Select::with_theme(&theme)
        .with_prompt("Select input file")
        .items(&options)
        .default(options.len() - 1)   // default = manual
        .interact_opt()?
        .unwrap_or(options.len() - 1);

    if idx == options.len() - 1 {
        // Manual path with existence validation.
        let path: String = Input::with_theme(&theme)
            .with_prompt("File path")
            .validate_with(|s: &String| {
                if std::path::Path::new(s.trim()).exists() { Ok(()) }
                else { Err(format!("File not found: '{}'", s)) }
            })
            .interact_text()?;
        Ok(path.trim().to_string())
    } else {
        Ok(recent[idx].0.clone())
    }
}

/// A green-on-black dialoguer theme.
pub fn hacker_theme() -> ColorfulTheme {
    use dialoguer::console::Style;
    ColorfulTheme {
        active_item_style:   Style::new().green().bold(),
        active_item_prefix:  dialoguer::console::style("║ ❯".to_string()).green(),
        inactive_item_prefix: dialoguer::console::style("║  ".to_string()).green(),
        values_style:        Style::new().green(),
        prompt_style:        Style::new().white().bold(),
        prompt_prefix:       dialoguer::console::style("?".to_string()).green(),
        error_style:         Style::new().red().bold(),
        error_prefix:        dialoguer::console::style("✘".to_string()).red().bold(),
        success_prefix:      dialoguer::console::style("✔".to_string()).green(),
        ..ColorfulTheme::default()
    }
}
