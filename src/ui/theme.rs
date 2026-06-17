//! Terminal theme: colours, box drawing, banner, helpers.

use crossterm::{
    cursor,
    event::{read, Event},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
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

// ── Clear / cursor ─────────────────────────────────────────────────────────

pub fn clear_screen() {
    let _ = execute!(
        io::stdout(),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    );
}

// ── Banner ─────────────────────────────────────────────────────────────────

pub fn draw_banner() {
    let border_colour = green();
    let art_colour    = bright_green();
    let name_colour   = white();

    let top    = "╔═══════════════════════════════════════════════════════════════╗";
    let bottom = "╚═══════════════════════════════════════════════════════════════╝";
    let blank  = "║                                                               ║";

    let art = [
        "║  ██████╗ ███████╗ ██████╗  ██╗  ██╗██████╗ ███████╗       ║",
        "║  ██╔══██╗██╔════╝██╔════╝  ██║  ██║╚════██╗██╔════╝       ║",
        "║  ██████╔╝█████╗  ██║       ███████║ █████╔╝███████╗       ║",
        "║  ██╔══██╗██╔══╝  ██║       ╚════██║╚═══██╗╚════██║       ║",
        "║  ██║  ██║███████╗╚██████╗       ██║██████╔╝███████║       ║",
        "║  ╚═╝  ╚═╝╚══════╝ ╚═════╝      ╚═╝╚═════╝ ╚══════╝       ║",
    ];

    let name_line = "║               ◆  REC#25 ◆  Reconnaissance Framework  ◆      ║";
    let ver_line  = "║                           v0.1.0                              ║";

    cprintln(border_colour, top);
    cprintln(border_colour, blank);
    for l in &art {
        cprint(border_colour, "");
        cprintln(art_colour, l);
    }
    cprintln(border_colour, blank);
    cprint(border_colour, "");
    cprintln(name_colour, name_line);
    cprint(border_colour, "");
    cprintln(grey(), ver_line);
    cprintln(border_colour, blank);
    cprintln(border_colour, bottom);
    println!();
}

// ── Generic box ────────────────────────────────────────────────────────────

/// Draw a green-bordered box with an optional title and content lines.
pub fn print_box(title: &str, lines: &[&str]) {
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0).max(title.len() + 4).max(40);
    let pad   = width + 4; // account for "║  " and "  ║"

    let border = green();
    let text   = white();

    let top_bar = format!("╔{}╗", "═".repeat(pad));
    cprintln(border, &top_bar);

    if !title.is_empty() {
        cprint(border, "╠");
        cprint(text, &format!("  {:<width$}  ", title, width = width));
        cprintln(border, "║");
        let sep = format!("╠{}╣", "═".repeat(pad));
        cprintln(border, &sep);
    }

    for l in lines {
        cprint(border, "║  ");
        cprint(text, &format!("{:<width$}", l, width = width));
        cprintln(border, "  ║");
    }

    let bottom_bar = format!("╚{}╝", "═".repeat(pad));
    cprintln(border, &bottom_bar);
}

// ── Status messages ────────────────────────────────────────────────────────

pub fn print_error(msg: &str) {
    println!();
    let lines: Vec<&str> = msg.lines().collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0).max(40);

    let _ = execute!(io::stdout(), SetForegroundColor(red()));
    println!("╔{}╗", "═".repeat(width + 4));
    println!("║  {:<width$}  ║", "✘  ERROR", width = width);
    println!("╠{}╣", "═".repeat(width + 4));
    for l in &lines {
        println!("║  {:<width$}  ║", l, width = width);
    }
    println!("╚{}╝", "═".repeat(width + 4));
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

pub fn print_dim(msg: &str) {
    cprintln(grey(), msg);
}

/// Draw a thin coloured section divider with a label.
pub fn section_header(label: &str) {
    println!();
    cprint(green(), "  ╔═ ");
    cprint(white(), label);
    cprintln(green(), " ═══════════════════════════");
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
