//! Terminal theme: colours, box drawing, banner, helpers.

use crossterm::{
    event::{read, Event},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};

// ── Colour helpers ─────────────────────────────────────────────────────────

#[inline] pub fn green()      -> Color { Color::Green }
#[inline] pub fn bright_green() -> Color { Color::Rgb { r: 0, g: 255, b: 0 } }
#[inline] pub fn red()        -> Color { Color::Red }
#[inline] pub fn white()      -> Color { Color::White }
#[inline] pub fn grey()       -> Color { Color::DarkGrey }
#[inline] pub fn cyan()       -> Color { Color::Cyan }

/// Write `text` in `colour` then reset.
pub fn cprint(colour: Color, text: &str) {
    let mut out = io::stdout();
    let _ = execute!(out, SetForegroundColor(colour), Print(text), ResetColor);
}

pub fn cprintln(colour: Color, text: &str) {
    cprint(colour, text);
    println!();
}

pub fn terminal_width() -> usize {
    crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80)
}

// ── Banner ─────────────────────────────────────────────────────────────────

pub fn draw_banner() {
    let term_width = terminal_width();
    let width = term_width.saturating_sub(2).max(40);
    
    let border_colour = green();
    let name_colour   = white();

    let top    = format!("╔{}╗", "═".repeat(width));
    let bottom = format!("╚{}╝", "═".repeat(width));

    cprintln(border_colour, &top);
    cprint(border_colour, "║");
    cprint(name_colour, &format!("{:^width$}", "REC#25", width = width));
    cprintln(border_colour, "║");
    cprintln(border_colour, &bottom);
    println!();
}

// ── Generic box ────────────────────────────────────────────────────────────

/// Draw a green-bordered box with an optional title and content lines.
pub fn print_box(title: &str, lines: &[&str]) {
    let term_width = terminal_width();
    let width = term_width.saturating_sub(2).max(40);

    let border = green();
    let text   = white();

    let top_bar = format!("╔{}╗", "═".repeat(width));
    cprintln(border, &top_bar);

    if !title.is_empty() {
        cprint(border, "╠");
        cprint(text, &format!("{:^width$}", title, width = width));
        cprintln(border, "╣");
        let sep = format!("╠{}╣", "═".repeat(width));
        cprintln(border, &sep);
    }

    for l in lines {
        let inner_space = width.saturating_sub(2);
        let display_text = if l.len() > inner_space {
            format!("{}...", &l[..inner_space.saturating_sub(3)])
        } else {
            format!("{:<inner_space$}", l, inner_space = inner_space)
        };
        cprint(border, "║ ");
        cprint(text, &display_text);
        cprintln(border, " ║");
    }

    let bottom_bar = format!("╚{}╝", "═".repeat(width));
    cprintln(border, &bottom_bar);
}

// ── Status messages ────────────────────────────────────────────────────────

pub fn print_error(msg: &str) {
    println!();
    let lines: Vec<&str> = msg.lines().collect();
    
    let term_width = terminal_width();
    let width = term_width.saturating_sub(2).max(40);

    let _ = execute!(io::stdout(), SetForegroundColor(red()));
    println!("╔{}╗", "═".repeat(width));
    println!("║ {:^width$} ║", "✘  ERROR", width = width - 2);
    println!("╠{}╣", "═".repeat(width));
    for l in &lines {
        let inner_space = width.saturating_sub(2);
        let display_text = if l.len() > inner_space {
            format!("{}...", &l[..inner_space.saturating_sub(3)])
        } else {
            format!("{:<inner_space$}", l, inner_space = inner_space)
        };
        println!("║ {} ║", display_text);
    }
    println!("╚{}╝", "═".repeat(width));
    let _ = execute!(io::stdout(), ResetColor);
    println!();
}

pub fn print_success(msg: &str) {
    println!();
    cprint(green(), "  ✔  ");
    cprintln(white(), msg);
}

pub fn print_info(msg: &str) {
    cprint(cyan(), "  ℹ  ");
    cprintln(white(), msg);
}

/// Draw a thin coloured section divider with a label.
pub fn section_header(label: &str) {
    println!();
    let term_width = terminal_width();
    let width = term_width.saturating_sub(2).max(40);
    // ╔═ label ═══...
    // 1 + 1 + 1 + len + 1 + X = width => X = width - len - 4
    let right_len = width.saturating_sub(label.len() + 4);

    cprint(green(), "╔═ ");
    cprint(white(), label);
    cprintln(green(), &format!(" {}", "═".repeat(right_len)));
    println!();
}

// ── Wait-for-key ───────────────────────────────────────────────────────────

pub fn wait_key() {
    println!();
    cprint(grey(), "  Press any key to continue…");
    let _ = io::stdout().flush();
    let _ = enable_raw_mode();
    loop {
        if matches!(read(), Ok(Event::Key(_))) {
            break;
        }
    }
    let _ = disable_raw_mode();
    println!();
}
