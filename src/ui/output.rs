//! Formatted output display: preview box, line-numbered list, summary.

use crate::parser::ParsedOutput;
use crate::tools::types::OutputFormat;
use crate::ui::theme::{cprint, cprintln, green, grey, white, bright_green};

/// Display everything after a command completes: preview, summary, saved path.
pub fn display_result(
    parsed:       &ParsedOutput,
    _fmt:         OutputFormat,
    saved_path:   &str,
    preview_lines: usize,
) {
    println!();
    display_preview(parsed, preview_lines);
    display_summary(parsed);
    display_saved(saved_path);
}

/// Show the first `max` lines in a numbered, green-bordered preview box.
pub fn display_preview(parsed: &ParsedOutput, max: usize) {
    let lines: Vec<&str> = parsed.display
        .lines()
        .take(max)
        .collect();

    if lines.is_empty() {
        cprintln(grey(), "  (no output to preview)");
        return;
    }

    let total_displayed = lines.len();
    let total_lines     = parsed.lines.len();

    // Box header.
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0).max(50);
    let pad   = width + 4;

    cprintln(green(), &format!("╔{}╗", "═".repeat(pad)));
    cprint(green(), "║  ");
    cprint(white(), &format!("{:<width$}", "Output Preview", width = width));
    cprintln(green(), "  ║");
    cprintln(green(), &format!("╠{}╣", "═".repeat(pad)));

    for (i, line) in lines.iter().enumerate() {
        let num = format!("{:>3}. ", i + 1);
        cprint(green(), "║  ");
        cprint(grey(), &num);
        cprint(bright_green(), &format!("{:<w$}", line, w = width.saturating_sub(5)));
        cprintln(green(), "  ║");
    }

    if total_lines > total_displayed {
        cprint(green(), "║  ");
        cprint(grey(), &format!("… {} more line{} not shown", total_lines - total_displayed,
            if total_lines - total_displayed == 1 { "" } else { "s" }));
        let remaining_width = width.saturating_sub(40);
        cprint(grey(), &" ".repeat(remaining_width));
        cprintln(green(), "  ║");
    }

    cprintln(green(), &format!("╚{}╝", "═".repeat(pad)));
}

/// Print the one-line summary line.
pub fn display_summary(parsed: &ParsedOutput) {
    println!();
    cprint(green(), "  ◆  ");
    cprintln(white(), &parsed.summary);
}

/// Print the "Saved to:" path.
pub fn display_saved(path: &str) {
    cprint(grey(), "  💾  Saved → ");
    cprintln(bright_green(), path);
    println!();
}
