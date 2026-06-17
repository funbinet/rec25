# REC#25 – Reconnaissance Framework

**REC#25** is a highly interactive, fast, and beautifully styled terminal-based security reconnaissance framework written in Rust. It wraps **82 external security tools** covering 5 core categories, standardizes their outputs, and saves all results to a structured workspace.

## 🚀 Features

- **Blazing Fast CLI:** Built in Rust `v1.96.0`, utilizing `crossterm` and `dialoguer` for a highly interactive and green-on-black "hacker" aesthetic.
- **Zero-Allocation Registry:** Tool configuration is heavily optimized using statically allocated definitions (`&'static`).
- **5 Core Categories:**
  - 📡 Discovery (19 tools)
  - 🗺️ Mapping (23 tools)
  - 🕷️ Crawling (14 tools)
  - 🔍 Analysis (13 tools)
  - 🤖 Automation (14 tools)
- **4 Modes per Tool:** Each tool supports exactly 4 distinct execution modes (e.g., standard scan, JSON export, brute-force, reverse lookups).
- **Intelligent Parsers:** Outputs from tools are intercepted and parsed natively in Rust (supporting JSON, CSV, Nmap greppable, Lines, and Raw formats), generating clean summaries and styled previews.
- **Unified Workspace:** Output and logs are automatically dumped into `/opt/rec25/`.
- **Integrated Outputs Manager:** Browse, preview, edit (`nano`), and delete generated output files directly from the CLI.
- **Interactive Configuration:** Save global settings (timeouts, API keys) via an integrated interactive menu into a central TOML configuration.
- **Asynchronous Execution:** Background thread processing with a dynamic braille spinner to prevent CLI lockups during long tool runs.

## 📦 Installation

To install REC#25, you can use the provided bash installer script. The installer will compile the binary in release mode using `cargo`, place it in your `PATH`, and setup `/opt/rec25`.

```bash
git clone https://github.com/funbinet/rec25.git
cd rec25
chmod +x install.sh
sudo ./install.sh
```

*(Note: Cargo/Rust `stable` must be installed on your system).*

## 💡 Usage

Run the tool from anywhere in your terminal:

```bash
rec25
```

Navigate through the dynamic menus utilizing the Arrow Keys or Vim keys (j/k), press Enter to select an item.

### Top-Level Menu Options
1. **Category Selection:** Browse the 5 core recon categories to select tools and execution modes.
2. **Outputs:** Browse your generated output files in `/opt/rec25/output`. Select a file to view a line-numbered preview and optionally open it for full editing in `nano`, or delete it.
3. **Settings:** Interactively configure your global timeouts, preview display length, output directories, and API keys (Shodan, GitHub, Censys).

## 🛠️ Tool Registry Breakdown

* **Discovery:** Subfinder, Amass, Assetfinder, Findomain, github-subdomains, crobat, theHarvester, Shodan CLI, Waybackurls, gau, certgraph, whois, dig, dnsenum, dnsrecon, Fierce, Knockpy, puredns, shuffledns.
* **Mapping:** httpx, httprobe, Naabu, Nmap, dnsx, masscan, RustScan, zmap, zgrab2, WhatWeb, Wappalyzer CLI, traceroute, mtr, netcat, socat, onesixtyone, snmpwalk, ldapsearch, rpcclient, enum4linux, enum4linux-ng, smbmap, crackmapexec.
* **Crawling:** Katana, hakrawler, photon, crawlergo, xnLinkFinder, ParamSpider, Arjun, getJS, SecretFinder, subjack, ffuf, dirsearch, feroxbuster, Gobuster.
* **Analysis:** nuclei, dalfox, sqlmap, nikto, wafw00f, testssl.sh, sslscan, tcpdump, tshark, trufflehog, gitleaks, EyeWitness, aquatone.
* **Automation:** bbot, reconftw, osmedeus, sn1per, unfurl, anew, gf, uro, jq, yq, ripgrep, tmux, curl, wget.

## 🤝 Contribution

Feel free to fork the repository and submit pull requests. Due to the static nature of the tool registry (`src/tools/*`), adding new tools requires updating the arrays in the respective category file.

## 📄 License

MIT License.
