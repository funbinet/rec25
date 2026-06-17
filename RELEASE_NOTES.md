# REC#25 v0.5.0 — Release Notes

## Release v0.5.0

### What is REC#25?

**REC#25** is a fast, interactive terminal-based security reconnaissance framework written in Rust. It wraps **82 external security tools** across 5 categories into a beautifully styled, fully navigable menu-driven CLI — no config files needed to get started.

---

### Changes in v0.5.0

**UI Enhancements & Fixes**
- **Strict Hacker Theme**: Eliminated all white text for a pure, immersive green/black terminal experience.
- **Scrollable Output Viewer**: Replaced the static 20-line preview box with a fully interactive, full-screen scrollable output viewer with a distinct Cyan (Aqua) theme.
- **Unified Navigation**: Added explicitly selectable `Back` and `Exit` buttons to all menus (no longer required to use keyboard shortcuts).
- **Zero Emojis**: Removed all emojis and square brackets from menus for maximum compatibility and a cleaner look.
- **Install Script Fix**: Resolved `Cargo Rust is not installed` error by explicitly sourcing `$HOME/.cargo/env` before running checks.

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
