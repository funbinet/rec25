//! Terminal theme: colours, box drawing, banner, helpers.

use crossterm::{
    event::{read, Event},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

// ── Colour helpers ─────────────────────────────────────────────────────────

#[inline] pub fn green()  -> Color { Color::Rgb { r: 0,   g: 220, b: 100 } }
#[inline] pub fn bright_green() -> Color { Color::Rgb { r: 0, g: 255, b: 80 } }
#[inline] pub fn aqua()   -> Color { Color::Rgb { r: 0,   g: 200, b: 200 } }
#[inline] pub fn red()    -> Color { Color::Red }
#[inline] pub fn white()  -> Color { Color::Rgb { r: 0, g: 255, b: 80 } } // Replaced with bright green for pure green/black look
#[inline] pub fn grey()   -> Color { Color::DarkGrey }
#[inline] pub fn yellow() -> Color { Color::Rgb { r: 230, g: 200, b: 0   } }

/// Write `text` in `colour` then reset (no newline).
pub fn cprint(colour: Color, text: &str) {
    let mut out = io::stdout();
    let _ = execute!(out, SetForegroundColor(colour), Print(text), ResetColor);
}

/// Write `text` in `colour` then reset with a newline.
pub fn cprintln(colour: Color, text: &str) {
    cprint(colour, text);
    println!();
}

/// Return the current terminal column width (fallback: 80).
pub fn terminal_width() -> usize {
    crossterm::terminal::size().map(|(w, _)| w as usize).unwrap_or(80)
}

// ── Banner ─────────────────────────────────────────────────────────────────

pub fn draw_banner() {
    let inner = terminal_width().saturating_sub(2).max(40);

    let top    = format!("╔{}╗", "═".repeat(inner));
    let bottom = format!("╚{}╝", "═".repeat(inner));

    // Title centred
    let title = "REC#25";
    let title_w = UnicodeWidthStr::width(title);
    let pad = inner.saturating_sub(title_w);
    let lpad = pad / 2;
    let rpad = pad - lpad;

    // Subtitle centred
    let sub = "Reconnaissance Framework  v0.5.0";
    let sub_w = UnicodeWidthStr::width(sub);
    let pad2 = inner.saturating_sub(sub_w);
    let lpad2 = pad2 / 2;
    let rpad2 = pad2 - lpad2;

    let mid = format!("╠{}╣", "═".repeat(inner));

    cprintln(green(),  &top);
    cprint(green(),    "║");
    cprint(bright_green(), &format!("{}{}{}", " ".repeat(lpad), title, " ".repeat(rpad)));
    cprintln(green(),  "║");
    cprint(green(),    "║");
    cprint(aqua(),     &format!("{}{}{}", " ".repeat(lpad2), sub, " ".repeat(rpad2)));
    cprintln(green(),  "║");
    cprintln(green(),  &mid);

    // Tip bar
    let tip = "  Arrow keys / j,k to navigate   Enter to select   q to go back  ";
    let tip_w = UnicodeWidthStr::width(tip);
    let pad3 = inner.saturating_sub(tip_w);
    let lpad3 = pad3 / 2;
    let rpad3 = pad3 - lpad3;
    cprint(green(),  "║");
    cprint(grey(),   &format!("{}{}{}", " ".repeat(lpad3), tip, " ".repeat(rpad3)));
    cprintln(green(), "║");
    cprintln(green(), &bottom);
    println!();
}

// ── Generic info/result box ────────────────────────────────────────────────

/// Full-width green box with an optional centred title.
pub fn print_box(title: &str, lines: &[&str]) {
    let inner = terminal_width().saturating_sub(2).max(40);

    let top = format!("╔{}╗", "═".repeat(inner));
    let bot = format!("╚{}╝", "═".repeat(inner));

    cprintln(green(), &top);

    if !title.is_empty() {
        let tw = UnicodeWidthStr::width(title);
        let pad = inner.saturating_sub(tw);
        let lpad = pad / 2;
        let rpad = pad - lpad;
        let mid = format!("╠{}╣", "═".repeat(inner));
        cprint(green(), "║");
        cprint(aqua(),  &format!("{}{}{}", " ".repeat(lpad), title, " ".repeat(rpad)));
        cprintln(green(), "║");
        cprintln(green(), &mid);
    }

    // text_cols: inner - 2 (for "║ " and " ║")
    let text_cols = inner.saturating_sub(2);
    for l in lines {
        let lw = UnicodeWidthStr::width(*l);
        let padding = if lw < text_cols { text_cols - lw } else { 0 };
        cprint(green(), "║ ");
        cprint(white(), l);
        cprint(white(), &" ".repeat(padding));
        cprintln(green(), " ║");
    }

    cprintln(green(), &bot);
}

// ── Status helpers ─────────────────────────────────────────────────────────

pub fn print_error(msg: &str) {
    println!();
    let inner = terminal_width().saturating_sub(2).max(40);
    let text_cols = inner.saturating_sub(2);
    let top = format!("╔{}╗", "═".repeat(inner));
    let mid = format!("╠{}╣", "═".repeat(inner));
    let bot = format!("╚{}╝", "═".repeat(inner));

    let _ = execute!(io::stdout(), SetForegroundColor(red()));
    println!("{}", top);
    let hdr = " ERROR ";
    let hw = UnicodeWidthStr::width(hdr);
    let hpad = inner.saturating_sub(hw);
    println!("║{}{}{}║", " ".repeat(hpad / 2), hdr, " ".repeat(hpad - hpad / 2));
    println!("{}", mid);
    for l in msg.lines() {
        let lw = UnicodeWidthStr::width(l);
        let pad = if lw < text_cols { text_cols - lw } else { 0 };
        println!("║ {}{} ║", l, " ".repeat(pad));
    }
    println!("{}", bot);
    let _ = execute!(io::stdout(), ResetColor);
    println!();
}

pub fn print_success(msg: &str) {
    println!();
    cprint(green(), "  [OK]  ");
    cprintln(white(), msg);
}

pub fn print_info(msg: &str) {
    cprint(aqua(), "  [i]  ");
    cprintln(white(), msg);
}

pub fn print_warn(msg: &str) {
    cprint(yellow(), "  [!]  ");
    cprintln(white(), msg);
}

/// Coloured section header spanning the terminal width.
pub fn section_header(label: &str) {
    println!();
    let inner = terminal_width().saturating_sub(2).max(40);
    let lw = UnicodeWidthStr::width(label) + 4; // "= " + label + " ="
    let right = inner.saturating_sub(lw + 2);

    cprint(green(), "╔═ ");
    cprint(aqua(),  label);
    cprintln(green(), &format!(" {}", "═".repeat(right + 1)));
    println!();
}

// ── Wait-for-key ───────────────────────────────────────────────────────────

pub fn wait_key() {
    println!();
    cprint(grey(), "  Press any key to continue…");
    let _ = io::stdout().flush();
    let _ = enable_raw_mode();
    loop {
        if matches!(read(), Ok(Event::Key(_))) { break; }
    }
    let _ = disable_raw_mode();
    println!();
}
