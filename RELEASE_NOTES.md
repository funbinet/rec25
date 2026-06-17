# REC#25 v0.3.0 — Release Notes

## Release v0.3.0

### What is REC#25?

**REC#25** is a fast, interactive terminal-based security reconnaissance framework written in Rust. It wraps **82 external security tools** across 5 categories into a beautifully styled, fully navigable menu-driven CLI — no config files needed to get started.

---

### Changes in v0.3.0

**UI Overhaul — Full-Width Symmetrical Box System**
- All menu boxes and info boxes now span the full terminal width (detected at runtime).
- Fixed critical alignment bug: all characters — including multi-byte Unicode — are now measured by their *display column width* (using `unicode-width`), so the right border always closes perfectly.
- Removed the spurious divider line that appeared between categories in the main menu.
- Replaced non-rendering emoji icons with clean ASCII-safe menu labels (`[*]`, `[F]`, `[=]`, `[X]`).
- Banner updated: shows REC#25 title, version, and a tip bar — all centred and full-width.

**Navigation — Nested Menu Back-tracking**
- After a tool execution completes, you are returned to the **mode selection** of the same tool — not the main menu. This allows you to re-run with different input, pick a different mode, or go back step by step.
- Pressing `q` or `Esc` in any menu moves up one level (mode → tool → category → main).

**Colour Theme Restored**
- Restored the original green/aqua colour palette throughout (borders, titles, info messages, banner).
- Blue has been removed; all UI elements now use consistent `#00DC64` green and `#00C8C8` aqua.

---

### Installation

```bash
git clone https://github.com/funbinet/rec25.git
cd rec25
chmod +x install.sh
sudo ./install.sh
```

**Requirements:** Rust stable toolchain (`rustup` — [https://rustup.rs](https://rustup.rs))

#### Manual Installation (if install.sh fails)
```bash
source $HOME/.cargo/env
cargo build --release
sudo cp target/release/rec25 /usr/bin/rec25
sudo mkdir -p /opt/rec25/{output,logs,config}
sudo chown -R $USER:$USER /opt/rec25
```

---

### Running

```bash
rec25
```

Navigate with **Arrow keys** or **j/k**, confirm with **Enter**, go back with **q** or **Esc**.

---

### Files in this release

| File | Description |
|------|-------------|
| `install.sh` | Automated installer — compiles and installs to `/usr/bin/rec25` |
| `README.md` | Full project documentation |
| `Cargo.toml` | Rust project manifest |
| `src/` | Full source code |
