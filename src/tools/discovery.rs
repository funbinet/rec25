//! Discovery category — 19 tools, 4 modes each.

use super::types::{Category, InputKind, Mode, OutputFormat, Tool};

pub static DISCOVERY: Category = Category {
    name: "Discovery",
    tools: &[
        // ── Subfinder ──────────────────────────────────────────────────────
        Tool {
            name: "Subfinder",
            binary: "subfinder",
            modes: &[
                Mode { name: "Passive Subdomain Enum",   cmd_template: "subfinder -d {domain} -o {output_file}",                          inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Active Subdomain Enum",    cmd_template: "subfinder -d {domain} -active -o {output_file}",                   inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Subdomain with Resolve",   cmd_template: "subfinder -d {domain} -r -o {output_file}",                        inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON Export",              cmd_template: "subfinder -d {domain} -json -o {output_file}",                     inputs: &[InputKind::Domain], output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── Amass ─────────────────────────────────────────────────────────
        Tool {
            name: "Amass",
            binary: "amass",
            modes: &[
                Mode { name: "Passive DNS Enum",         cmd_template: "amass enum -passive -d {domain} -o {output_file}",                 inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Active DNS Enum",          cmd_template: "amass enum -active -d {domain} -o {output_file}",                  inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Brute Force DNS",          cmd_template: "amass enum -brute -d {domain} -o {output_file}",                   inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Intel Whois Search",       cmd_template: "amass intel -whois -d {domain} -o {output_file}",                  inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── Assetfinder ───────────────────────────────────────────────────
        Tool {
            name: "Assetfinder",
            binary: "assetfinder",
            modes: &[
                Mode { name: "Find Related Domains",     cmd_template: "assetfinder {domain} | tee {output_file}",                         inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Subdomains Only",          cmd_template: "assetfinder --subs-only {domain} | tee {output_file}",             inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Resolve Found Subs",       cmd_template: "assetfinder --subs-only {domain} | httpx -silent | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Count Found Assets",       cmd_template: "assetfinder {domain} | tee {output_file} | wc -l",                 inputs: &[InputKind::Domain], output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
        // ── Findomain ─────────────────────────────────────────────────────
        Tool {
            name: "Findomain",
            binary: "findomain",
            modes: &[
                Mode { name: "Fast Subdomain Scan",      cmd_template: "findomain -t {domain} -u {output_file}",                           inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Subdomain Resolve",        cmd_template: "findomain -t {domain} -r -u {output_file}",                        inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Quiet Mode Scan",          cmd_template: "findomain -t {domain} -q -u {output_file}",                        inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export to CSV",            cmd_template: "findomain -t {domain} --csv -u {output_file}",                     inputs: &[InputKind::Domain], output_format: OutputFormat::Csv,   file_ext: "csv" },
            ],
        },
        // ── github-subdomains ─────────────────────────────────────────────
        Tool {
            name: "github-subdomains",
            binary: "github-subdomains",
            modes: &[
                Mode { name: "Search GitHub Tokens",     cmd_template: "github-subdomains -d {domain} -t {value} -o {output_file}",        inputs: &[InputKind::Domain, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Search Org Repos",         cmd_template: "github-subdomains -d {domain} -o {output_file}",                   inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Silent Mode Scan",         cmd_template: "github-subdomains -d {domain} -s -o {output_file}",                inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Raw Hit Export",           cmd_template: "github-subdomains -d {domain} -raw -o {output_file}",              inputs: &[InputKind::Domain], output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
        // ── crobat ────────────────────────────────────────────────────────
        Tool {
            name: "crobat",
            binary: "crobat",
            modes: &[
                Mode { name: "Query All Subdomains",     cmd_template: "crobat -s {domain} | tee {output_file}",                           inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Query TLD Domains",        cmd_template: "crobat -tld {domain} | tee {output_file}",                         inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Reverse DNS Lookup",       cmd_template: "crobat -r {ip} | tee {output_file}",                               inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Bulk Domain Query",        cmd_template: "cat {file} | crobat -s - | tee {output_file}",                     inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── theHarvester ──────────────────────────────────────────────────
        Tool {
            name: "theHarvester",
            binary: "theHarvester",
            modes: &[
                Mode { name: "Passive Email Harvest",    cmd_template: "theHarvester -d {domain} -b all -f {output_file}",                 inputs: &[InputKind::Domain], output_format: OutputFormat::Xml,   file_ext: "xml" },
                Mode { name: "Google Source Harvest",    cmd_template: "theHarvester -d {domain} -b google -l 500 -f {output_file}",       inputs: &[InputKind::Domain], output_format: OutputFormat::Xml,   file_ext: "xml" },
                Mode { name: "DNS Brute Force",          cmd_template: "theHarvester -d {domain} -c -f {output_file}",                     inputs: &[InputKind::Domain], output_format: OutputFormat::Xml,   file_ext: "xml" },
                Mode { name: "DNS Resolve Found",        cmd_template: "theHarvester -d {domain} -r -f {output_file}",                     inputs: &[InputKind::Domain], output_format: OutputFormat::Xml,   file_ext: "xml" },
            ],
        },
        // ── Shodan CLI ────────────────────────────────────────────────────
        Tool {
            name: "Shodan CLI",
            binary: "shodan",
            modes: &[
                Mode { name: "Host Info Lookup",         cmd_template: "shodan host {ip} | tee {output_file}",                             inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Network Search Query",     cmd_template: "shodan search --fields ip_str,port,org {target} | tee {output_file}", inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "SSL Cert Search",          cmd_template: "shodan search ssl.cert.subject.cn:{domain} | tee {output_file}",   inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Download Results",         cmd_template: "shodan count {target} | tee {output_file}",                        inputs: &[InputKind::Target], output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
        // ── Waybackurls ───────────────────────────────────────────────────
        Tool {
            name: "Waybackurls",
            binary: "waybackurls",
            modes: &[
                Mode { name: "Fetch Archived URLs",      cmd_template: "echo {domain} | waybackurls | tee {output_file}",                  inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Filter by Extension",      cmd_template: "echo {domain} | waybackurls | grep '\\.{value}' | tee {output_file}", inputs: &[InputKind::Domain, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Unique Paths Only",        cmd_template: "echo {domain} | waybackurls | sort -u | tee {output_file}",        inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Keyword URL Filter",       cmd_template: "echo {domain} | waybackurls | grep '{value}' | tee {output_file}", inputs: &[InputKind::Domain, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── gau ───────────────────────────────────────────────────────────
        Tool {
            name: "gau",
            binary: "gau",
            modes: &[
                Mode { name: "Fetch All URLs",           cmd_template: "echo {domain} | gau | tee {output_file}",                          inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Wayback Provider Only",    cmd_template: "echo {domain} | gau --providers wayback | tee {output_file}",      inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Blacklist Extensions",     cmd_template: "echo {domain} | gau --blacklist png,jpg,gif,svg | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON Output",              cmd_template: "echo {domain} | gau --json | tee {output_file}",                   inputs: &[InputKind::Domain], output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── certgraph ─────────────────────────────────────────────────────
        Tool {
            name: "certgraph",
            binary: "certgraph",
            modes: &[
                Mode { name: "Cert Domain Graph",        cmd_template: "certgraph -depth 1 {domain} | tee {output_file}",                  inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Follow Cert Chain",        cmd_template: "certgraph -depth 3 {domain} | tee {output_file}",                  inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Find Related Certs",       cmd_template: "certgraph -ct {domain} | tee {output_file}",                       inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export JSON Graph",        cmd_template: "certgraph -json {domain} | tee {output_file}",                     inputs: &[InputKind::Domain], output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── whois ─────────────────────────────────────────────────────────
        Tool {
            name: "whois",
            binary: "whois",
            modes: &[
                Mode { name: "Domain Whois",             cmd_template: "whois {domain} | tee {output_file}",                               inputs: &[InputKind::Domain], output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "IP Whois",                 cmd_template: "whois {ip} | tee {output_file}",                                   inputs: &[InputKind::Ip],     output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "Registrar Info Extract",   cmd_template: "whois {domain} | grep -iE 'registrar|created|expires|updated' | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Raw Whois Dump",           cmd_template: "whois -H {domain} | tee {output_file}",                            inputs: &[InputKind::Domain], output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
        // ── dig ───────────────────────────────────────────────────────────
        Tool {
            name: "dig",
            binary: "dig",
            modes: &[
                Mode { name: "A Record Lookup",          cmd_template: "dig {domain} A | tee {output_file}",                               inputs: &[InputKind::Domain], output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "MX Record Lookup",         cmd_template: "dig {domain} MX | tee {output_file}",                              inputs: &[InputKind::Domain], output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "NS Record Lookup",         cmd_template: "dig {domain} NS | tee {output_file}",                              inputs: &[InputKind::Domain], output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "Full DNS ANY Dump",        cmd_template: "dig {domain} ANY | tee {output_file}",                             inputs: &[InputKind::Domain], output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
        // ── dnsenum ───────────────────────────────────────────────────────
        Tool {
            name: "dnsenum",
            binary: "dnsenum",
            modes: &[
                Mode { name: "Zone Transfer Attempt",    cmd_template: "dnsenum --dnsserver {target} {domain} | tee {output_file}",        inputs: &[InputKind::Domain, InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "DNS Brute Force",          cmd_template: "dnsenum -f /usr/share/wordlists/dnsenum/dns.txt {domain} | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "No Reverse Lookup",        cmd_template: "dnsenum --noreverse {domain} | tee {output_file}",                 inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Google Scrape Enum",       cmd_template: "dnsenum --google {domain} | tee {output_file}",                    inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── dnsrecon ──────────────────────────────────────────────────────
        Tool {
            name: "dnsrecon",
            binary: "dnsrecon",
            modes: &[
                Mode { name: "Standard DNS Scan",        cmd_template: "dnsrecon -d {domain} | tee {output_file}",                         inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Zone Transfer Probe",      cmd_template: "dnsrecon -d {domain} -t axfr | tee {output_file}",                 inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Brute Force Subs",         cmd_template: "dnsrecon -d {domain} -t brt | tee {output_file}",                  inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Reverse IP Range",         cmd_template: "dnsrecon -r {ip} | tee {output_file}",                             inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── Fierce ────────────────────────────────────────────────────────
        Tool {
            name: "Fierce",
            binary: "fierce",
            modes: &[
                Mode { name: "DNS Recon Scan",           cmd_template: "fierce --domain {domain} | tee {output_file}",                     inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Zone Transfer Probe",      cmd_template: "fierce --domain {domain} --dns-servers {target} | tee {output_file}", inputs: &[InputKind::Domain, InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Brute Sub Lookup",         cmd_template: "fierce --domain {domain} --wordlist /usr/share/wordlists/fierce/hosts.txt | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Wide Net Scan",            cmd_template: "fierce --domain {domain} --wide | tee {output_file}",              inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── Knockpy ───────────────────────────────────────────────────────
        Tool {
            name: "Knockpy",
            binary: "knockpy",
            modes: &[
                Mode { name: "Subdomain Wordlist Scan",  cmd_template: "knockpy {domain} | tee {output_file}",                             inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Resolve All Subs",         cmd_template: "knockpy {domain} --resolve | tee {output_file}",                   inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export to CSV",            cmd_template: "knockpy {domain} --csv -o {output_file}",                          inputs: &[InputKind::Domain], output_format: OutputFormat::Csv,   file_ext: "csv" },
                Mode { name: "Verbose Recon Scan",       cmd_template: "knockpy {domain} --verbose | tee {output_file}",                   inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── puredns ───────────────────────────────────────────────────────
        Tool {
            name: "puredns",
            binary: "puredns",
            modes: &[
                Mode { name: "Mass Resolve Domains",     cmd_template: "puredns resolve {file} -r /etc/resolv.conf | tee {output_file}",   inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Brute Force Subs",         cmd_template: "puredns bruteforce /usr/share/wordlists/seclists/Discovery/DNS/subdomains-top1million-5000.txt {domain} | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Wildcard Filter",          cmd_template: "puredns resolve {file} --wildcard-tests 3 | tee {output_file}",    inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Output Valid Subs",        cmd_template: "puredns bruteforce /usr/share/wordlists/seclists/Discovery/DNS/subdomains-top1million-5000.txt {domain} -w {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── shuffledns ────────────────────────────────────────────────────
        Tool {
            name: "shuffledns",
            binary: "shuffledns",
            modes: &[
                Mode { name: "Subdomain Brute Force",    cmd_template: "shuffledns -d {domain} -w /usr/share/wordlists/seclists/Discovery/DNS/subdomains-top1million-5000.txt -r /etc/resolv.conf | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Resolve Wordlist File",    cmd_template: "shuffledns -d {domain} -list {file} -r /etc/resolv.conf | tee {output_file}", inputs: &[InputKind::Domain, InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Mass Resolve File",        cmd_template: "shuffledns -list {file} -r /etc/resolv.conf | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Wildcard Detection",       cmd_template: "shuffledns -d {domain} -sw -r /etc/resolv.conf | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
    ],
};
