# REC#25 v0.6.0 — Release Notes

## Release v0.6.0

### What is REC#25?

**REC#25** is a fast, interactive terminal-based security reconnaissance framework written in Rust. It wraps **82 external security tools** across 5 categories into a beautifully styled, fully navigable menu-driven CLI with a pure green/black hacker aesthetic.

---

### What's New in v0.6.0

**1. Numbered Menu Items**
All menus now use a consistent, clean bracket-number convention:
- Main Menu: `[1]` Discovery, `[2]` Mapping ... `[O]` Outputs, `[*]` Settings, `[x]` Exit
- Sub-menus: Tools and modes are automatically numbered `[1]`, `[2]`, `[3]`...
- File actions: `[O]` View, `[E]` Edit, `[D]` Delete

**2. Universal `[#] Home` Button**
Every sub-menu (Tool, Mode, Outputs, File Action) now includes a `[#] Home` option. Selecting it immediately jumps back to the Main Menu without having to press Back repeatedly.

**3. Cyan Active Item Indicator**
The selected item `>` indicator is now rendered in **Cyan** to stand out clearly against the green menu borders and text — making navigation extremely intuitive.

**4. Boxed, Centered Section Headers**
The execution section header (shown when a tool runs) is now a properly enclosed, centered box instead of an open-ended line:
```
╔══════════════════════════════════════════════════════════════╗
║                 crobat :: Query All Subdomains               ║
╠══════════════════════════════════════════════════════════════╣
```

**5. Enhanced README**
The `README.md` has been significantly enhanced with detailed feature descriptions, navigation controls guide, and version information.

---

### Navigation Reference (v0.6.0+)

| Key          | Action                    |
|--------------|---------------------------|
| `j` / Down   | Move selection down       |
| `k` / Up     | Move selection up         |
| `Enter`      | Confirm selection         |
| `q` / `Esc`  | Go Back                   |
| `[#] Home`   | Jump to Main Menu         |
| `[<] Back`   | Return to previous menu   |
| `[x] Exit`   | Quit the framework        |

---

### Installation

```bash
git clone https://github.com/funbinet/rec25.git
cd rec25
chmod +x install.sh
sudo ./install.sh
```

**Requirements:** Rust stable toolchain (`rustup` — https://rustup.rs)

---

### Files in this release

| File | Description |
|------|-------------|
| `install.sh` | Automated installer — compiles and installs to `/usr/bin/rec25` |
| `README.md` | Full project documentation |
