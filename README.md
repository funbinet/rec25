# REC#25 – Reconnaissance Framework

**REC#25** is a highly interactive, fast, and beautifully styled terminal-based security reconnaissance framework written in Rust. It wraps **82 external security tools** covering 5 core categories, standardizes their outputs, and saves all results to a structured workspace. 

Version `0.6.0` introduces a strict green/black hacker aesthetic, a custom full-screen output viewer, perfectly centered UI boxes, and a fully unified `Home/Back/Exit` navigation system.

## Features

- **Blazing Fast CLI:** Built in Rust utilizing `crossterm` and `dialoguer` for a highly interactive and pure green-on-black terminal experience.
- **Zero-Allocation Registry:** Tool configuration is heavily optimized using statically allocated definitions (`&'static`).
- **5 Core Categories:**
  - Discovery (19 tools)
  - Mapping (23 tools)
  - Crawling (14 tools)
  - Analysis (13 tools)
  - Automation (14 tools)
- **4 Modes per Tool:** Each tool supports exactly 4 distinct execution modes (e.g., standard scan, JSON export, brute-force, reverse lookups).
- **Intelligent Parsers:** Outputs from tools are intercepted and parsed natively in Rust (supporting JSON, CSV, Nmap greppable, Lines, and Raw formats).
- **Unified Workspace:** Output and logs are automatically dumped into `/opt/rec25/`.
- **Integrated Outputs Manager & Viewer:** Browse and delete generated output files. View files in a custom full-screen scrollable output viewer with line numbering, or open them in `nano` for editing.
- **Interactive Configuration:** Save global settings (timeouts, API keys) via an integrated interactive menu into a central TOML configuration.
- **Unified Navigation:** Navigate effortlessly with explicitly numbered options and standard `[#] Home`, `[<] Back`, and `[x] Exit` buttons on every menu.

## Installation

To install REC#25, you can use the provided bash installer script. The installer will compile the binary in release mode using `cargo`, place it in your `PATH`, and setup `/opt/rec25`.

```bash
git clone https://github.com/funbinet/rec25.git
cd rec25
chmod +x install.sh
sudo ./install.sh
```

*(Note: Cargo/Rust `stable` must be installed on your system).*

## Usage

Run the tool from anywhere in your terminal:

```bash
rec25
```

Navigate through the dynamic menus utilizing the **Arrow Keys** or **Vim keys (j/k)**, and press **Enter** to select an item.

### Navigation Controls
- `j` / `Down Arrow`: Move down
- `k` / `Up Arrow`: Move up
- `Enter`: Select option
- `q` / `Esc`: Go back

### Top-Level Menu Options
1. **[1-5] Category Selection:** Browse the 5 core recon categories to select tools and execution modes.
2. **[O] Outputs:** Browse your generated output files in `/opt/rec25/output`. View them in the custom scrollable viewer, or edit/delete them.
3. **[*] Settings:** Interactively configure your global timeouts, preview display length, output directories, and API keys (Shodan, GitHub, Censys).
4. **[x] Exit:** Close the framework safely.

## Tool Registry Breakdown

* **Discovery:** Subfinder, Amass, Assetfinder, Findomain, github-subdomains, crobat, theHarvester, Shodan CLI, Waybackurls, gau, certgraph, whois, dig, dnsenum, dnsrecon, Fierce, Knockpy, puredns, shuffledns.
* **Mapping:** httpx, httprobe, Naabu, Nmap, dnsx, masscan, RustScan, zmap, zgrab2, WhatWeb, Wappalyzer CLI, traceroute, mtr, netcat, socat, onesixtyone, snmpwalk, ldapsearch, rpcclient, enum4linux, enum4linux-ng, smbmap, crackmapexec.
* **Crawling:** Katana, hakrawler, photon, crawlergo, xnLinkFinder, ParamSpider, Arjun, getJS, SecretFinder, subjack, ffuf, dirsearch, feroxbuster, Gobuster.
* **Analysis:** nuclei, dalfox, sqlmap, nikto, wafw00f, testssl.sh, sslscan, tcpdump, tshark, trufflehog, gitleaks, EyeWitness, aquatone.
* **Automation:** bbot, reconftw, osmedeus, sn1per, unfurl, anew, gf, uro, jq, yq, ripgrep, tmux, curl, wget.

## Contribution

Feel free to fork the repository and submit pull requests. Due to the static nature of the tool registry (`src/tools/*`), adding new tools requires updating the arrays in the respective category file.

## License

MIT License.
