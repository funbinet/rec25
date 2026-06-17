//! Crawling category — 14 tools, 4 modes each.

use super::types::{Category, InputKind, Mode, OutputFormat, Tool};

pub static CRAWLING: Category = Category {
    name: "Crawling",
    tools: &[
        // ── Katana ────────────────────────────────────────────────────────
        Tool {
            name: "Katana",
            binary: "katana",
            modes: &[
                Mode { name: "Standard Web Crawl",       cmd_template: "katana -u {url} -o {output_file}",                                 inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JS Crawl Mode",            cmd_template: "katana -u {url} -js-crawl -o {output_file}",                       inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Form Extraction Crawl",    cmd_template: "katana -u {url} -form-extraction -o {output_file}",                inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON Output Crawl",        cmd_template: "katana -u {url} -json -o {output_file}",                           inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── hakrawler ─────────────────────────────────────────────────────
        Tool {
            name: "hakrawler",
            binary: "hakrawler",
            modes: &[
                Mode { name: "Quick URL Crawl",          cmd_template: "echo {url} | hakrawler | tee {output_file}",                       inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JS Link Extraction",       cmd_template: "echo {url} | hakrawler -js | tee {output_file}",                   inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Subdomain Crawl",          cmd_template: "echo {url} | hakrawler -subs | tee {output_file}",                 inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Sitemaps Crawl",           cmd_template: "echo {url} | hakrawler -sitemap | tee {output_file}",              inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── photon ────────────────────────────────────────────────────────
        Tool {
            name: "photon",
            binary: "photon",
            modes: &[
                Mode { name: "Full Site Crawl",          cmd_template: "python3 photon.py -u {url} -o {output_file}",                      inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Extract Secrets Crawl",    cmd_template: "python3 photon.py -u {url} --keys -o {output_file}",               inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export CSV Crawl",         cmd_template: "python3 photon.py -u {url} --export=csv -o {output_file}",         inputs: &[InputKind::Url],    output_format: OutputFormat::Csv,   file_ext: "csv" },
                Mode { name: "Crawl JS Files",           cmd_template: "python3 photon.py -u {url} --export=json -o {output_file}",        inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── crawlergo ─────────────────────────────────────────────────────
        Tool {
            name: "crawlergo",
            binary: "crawlergo",
            modes: &[
                Mode { name: "Headless Browser Crawl",   cmd_template: "crawlergo -c /usr/bin/chromium -o json {url} | tee {output_file}", inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Form Submit Crawl",        cmd_template: "crawlergo -c /usr/bin/chromium --fuzz-with-param {url} | tee {output_file}", inputs: &[InputKind::Url], output_format: OutputFormat::Json, file_ext: "json" },
                Mode { name: "AJAX Crawl Mode",          cmd_template: "crawlergo -c /usr/bin/chromium -t 30 {url} | tee {output_file}",   inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Request Log Export",       cmd_template: "crawlergo -c /usr/bin/chromium --request-log {output_file} {url}", inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── xnLinkFinder ──────────────────────────────────────────────────
        Tool {
            name: "xnLinkFinder",
            binary: "xnLinkFinder",
            modes: &[
                Mode { name: "Extract Page Links",       cmd_template: "python3 xnLinkFinder.py -i {url} -o {output_file}",               inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Recursive Link Crawl",     cmd_template: "python3 xnLinkFinder.py -i {url} -d 3 -o {output_file}",          inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Filter In-Scope Links",    cmd_template: "python3 xnLinkFinder.py -i {url} -sp {domain} -o {output_file}",  inputs: &[InputKind::Url, InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export Scoped URLs",       cmd_template: "python3 xnLinkFinder.py -i {url} -sf {domain} -o {output_file}",  inputs: &[InputKind::Url, InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── ParamSpider ───────────────────────────────────────────────────
        Tool {
            name: "ParamSpider",
            binary: "paramspider",
            modes: &[
                Mode { name: "Mine URL Params",          cmd_template: "python3 paramspider.py -d {domain} -o {output_file}",              inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Filter by Extension",      cmd_template: "python3 paramspider.py -d {domain} -e php,asp,aspx -o {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Exclude Static Assets",    cmd_template: "python3 paramspider.py -d {domain} -s -o {output_file}",           inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Level Deep Mine",          cmd_template: "python3 paramspider.py -d {domain} -l {value} -o {output_file}",   inputs: &[InputKind::Domain, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── Arjun ─────────────────────────────────────────────────────────
        Tool {
            name: "Arjun",
            binary: "arjun",
            modes: &[
                Mode { name: "GET Param Fuzz",           cmd_template: "arjun -u {url} -m GET -oJ {output_file}",                          inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "POST Param Fuzz",          cmd_template: "arjun -u {url} -m POST -oJ {output_file}",                         inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "JSON Param Fuzz",          cmd_template: "arjun -u {url} -m JSON -oJ {output_file}",                         inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Multithread Param Fuzz",   cmd_template: "arjun -u {url} -t 20 -oJ {output_file}",                          inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── getJS ─────────────────────────────────────────────────────────
        Tool {
            name: "getJS",
            binary: "getJS",
            modes: &[
                Mode { name: "Extract JS Files",         cmd_template: "getJS --url {url} | tee {output_file}",                            inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Complete JS URLs",         cmd_template: "getJS --url {url} --complete | tee {output_file}",                 inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JS from URL File",         cmd_template: "getJS --input {file} | tee {output_file}",                         inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Resolve JS Links",         cmd_template: "getJS --url {url} --resolve | tee {output_file}",                  inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── SecretFinder ──────────────────────────────────────────────────
        Tool {
            name: "SecretFinder",
            binary: "SecretFinder",
            modes: &[
                Mode { name: "Scan JS for Secrets",      cmd_template: "python3 SecretFinder.py -i {url} -o {output_file}",               inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Custom Regex Match",       cmd_template: "python3 SecretFinder.py -i {url} -e -r '{value}' -o {output_file}", inputs: &[InputKind::Url, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Live URL Scan",            cmd_template: "python3 SecretFinder.py -i {url} -o cli | tee {output_file}",      inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export HTML Report",       cmd_template: "python3 SecretFinder.py -i {url} -o {output_file}",               inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "html" },
            ],
        },
        // ── subjack ───────────────────────────────────────────────────────
        Tool {
            name: "subjack",
            binary: "subjack",
            modes: &[
                Mode { name: "Subdomain Takeover Scan",  cmd_template: "subjack -w {file} -t 100 -o {output_file}",                        inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "SSL Fingerprint Subs",     cmd_template: "subjack -w {file} -t 100 -ssl -o {output_file}",                   inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Verify CNAME Takeover",    cmd_template: "subjack -w {file} -c /usr/share/subjack/fingerprints.json -o {output_file}", inputs: &[InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON Takeover Export",     cmd_template: "subjack -w {file} -t 100 -o {output_file} -json",                  inputs: &[InputKind::File],   output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── ffuf ──────────────────────────────────────────────────────────
        Tool {
            name: "ffuf",
            binary: "ffuf",
            modes: &[
                Mode { name: "Dir Brute Force",          cmd_template: "ffuf -w /usr/share/wordlists/seclists/Discovery/Web-Content/directory-list-2.3-medium.txt -u {url}/FUZZ -o {output_file} -of json", inputs: &[InputKind::Url], output_format: OutputFormat::Json, file_ext: "json" },
                Mode { name: "VHOST Fuzz",               cmd_template: "ffuf -w /usr/share/wordlists/seclists/Discovery/DNS/subdomains-top1million-5000.txt -H 'Host: FUZZ.{domain}' -u {url} -o {output_file} -of json", inputs: &[InputKind::Url, InputKind::Domain], output_format: OutputFormat::Json, file_ext: "json" },
                Mode { name: "GET Param Fuzz",           cmd_template: "ffuf -w /usr/share/wordlists/seclists/Discovery/Web-Content/burp-parameter-names.txt -u '{url}?FUZZ=test' -o {output_file} -of json", inputs: &[InputKind::Url], output_format: OutputFormat::Json, file_ext: "json" },
                Mode { name: "POST Param Fuzz",          cmd_template: "ffuf -w /usr/share/wordlists/seclists/Discovery/Web-Content/burp-parameter-names.txt -u {url} -X POST -d 'FUZZ=test' -o {output_file} -of json", inputs: &[InputKind::Url], output_format: OutputFormat::Json, file_ext: "json" },
            ],
        },
        // ── dirsearch ─────────────────────────────────────────────────────
        Tool {
            name: "dirsearch",
            binary: "dirsearch",
            modes: &[
                Mode { name: "Fast Dir Scan",            cmd_template: "dirsearch -u {url} -o {output_file}",                              inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Recursive Dir Scan",       cmd_template: "dirsearch -u {url} --recursion -o {output_file}",                  inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Crawl and Scan",           cmd_template: "dirsearch -u {url} --crawl -o {output_file}",                      inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export JSON Results",      cmd_template: "dirsearch -u {url} --format=json -o {output_file}",               inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── feroxbuster ───────────────────────────────────────────────────
        Tool {
            name: "feroxbuster",
            binary: "feroxbuster",
            modes: &[
                Mode { name: "Recursive Bust",           cmd_template: "feroxbuster -u {url} -o {output_file}",                            inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Filter Status Codes",      cmd_template: "feroxbuster -u {url} --filter-status 404 -o {output_file}",        inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Append Extensions",        cmd_template: "feroxbuster -u {url} -x php,html,js,txt -o {output_file}",         inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Custom Wordlist Bust",     cmd_template: "feroxbuster -u {url} -w {file} -o {output_file}",                  inputs: &[InputKind::Url, InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── Gobuster ──────────────────────────────────────────────────────
        Tool {
            name: "Gobuster",
            binary: "gobuster",
            modes: &[
                Mode { name: "Dir Mode Scan",            cmd_template: "gobuster dir -u {url} -w /usr/share/wordlists/seclists/Discovery/Web-Content/directory-list-2.3-medium.txt -o {output_file}", inputs: &[InputKind::Url], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "DNS Mode Scan",            cmd_template: "gobuster dns -d {domain} -w /usr/share/wordlists/seclists/Discovery/DNS/subdomains-top1million-5000.txt -o {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "VHOST Mode Scan",          cmd_template: "gobuster vhost -u {url} -w /usr/share/wordlists/seclists/Discovery/DNS/subdomains-top1million-5000.txt -o {output_file}", inputs: &[InputKind::Url], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "S3 Bucket Scan",           cmd_template: "gobuster s3 -w /usr/share/wordlists/seclists/Discovery/Web-Content/s3buckets.txt -o {output_file}",                       inputs: &[],               output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
    ],
};
