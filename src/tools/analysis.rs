//! Analysis category — 13 tools, 4 modes each.

use super::types::{Category, InputKind, Mode, OutputFormat, Tool};

pub static ANALYSIS: Category = Category {
    name: "Analysis",
    tools: &[
        // ── nuclei ────────────────────────────────────────────────────────
        Tool {
            name: "nuclei",
            binary: "nuclei",
            modes: &[
                Mode { name: "Template Scan URL",        cmd_template: "nuclei -u {url} -o {output_file}",                                 inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Severity Filter Scan",     cmd_template: "nuclei -u {url} -severity critical,high -o {output_file}",         inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Tech Detection Scan",      cmd_template: "nuclei -u {url} -tags tech -o {output_file}",                      inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON Export Scan",         cmd_template: "nuclei -u {url} -json -o {output_file}",                           inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── dalfox ────────────────────────────────────────────────────────
        Tool {
            name: "dalfox",
            binary: "dalfox",
            modes: &[
                Mode { name: "Blind XSS Scan",           cmd_template: "dalfox url {url} --blind {target} -o {output_file}",              inputs: &[InputKind::Url, InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Param XSS Scan",           cmd_template: "dalfox url {url} -o {output_file}",                               inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "DOM XSS Scan",             cmd_template: "dalfox url {url} --skip-dom-check=false -o {output_file}",        inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON Output Scan",         cmd_template: "dalfox url {url} --format json -o {output_file}",                 inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── sqlmap ────────────────────────────────────────────────────────
        Tool {
            name: "sqlmap",
            binary: "sqlmap",
            modes: &[
                Mode { name: "Basic SQLi Detect",        cmd_template: "sqlmap -u '{url}' --batch | tee {output_file}",                   inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Enumerate Databases",      cmd_template: "sqlmap -u '{url}' --dbs --batch | tee {output_file}",             inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Dump Table Data",          cmd_template: "sqlmap -u '{url}' -D {target} --tables --batch | tee {output_file}", inputs: &[InputKind::Url, InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "OS Shell Attempt",         cmd_template: "sqlmap -u '{url}' --os-shell --batch | tee {output_file}",        inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── nikto ─────────────────────────────────────────────────────────
        Tool {
            name: "nikto",
            binary: "nikto",
            modes: &[
                Mode { name: "Full Web Scan",            cmd_template: "nikto -h {url} -o {output_file}",                                 inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "SSL/TLS Scan",             cmd_template: "nikto -h {url} -ssl -o {output_file}",                            inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "CGI Directory Scan",       cmd_template: "nikto -h {url} -Cgidirs all -o {output_file}",                    inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export XML Report",        cmd_template: "nikto -h {url} -Format xml -o {output_file}",                     inputs: &[InputKind::Url],    output_format: OutputFormat::Xml,   file_ext: "xml" },
            ],
        },
        // ── wafw00f ───────────────────────────────────────────────────────
        Tool {
            name: "wafw00f",
            binary: "wafw00f",
            modes: &[
                Mode { name: "WAF Detection",            cmd_template: "wafw00f {url} | tee {output_file}",                                inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Verbose WAF Detect",       cmd_template: "wafw00f -v {url} | tee {output_file}",                             inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Batch URL Detect",         cmd_template: "wafw00f -i {file} | tee {output_file}",                            inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON WAF Export",          cmd_template: "wafw00f -o json {url} | tee {output_file}",                        inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── testssl.sh ────────────────────────────────────────────────────
        Tool {
            name: "testssl.sh",
            binary: "testssl.sh",
            modes: &[
                Mode { name: "Full TLS Scan",            cmd_template: "testssl.sh {url} | tee {output_file}",                             inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Cipher Suite Check",       cmd_template: "testssl.sh --cipher-per-proto {url} | tee {output_file}",          inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Certificate Info",         cmd_template: "testssl.sh --certinfo {url} | tee {output_file}",                  inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Vulnerability Check",      cmd_template: "testssl.sh --vuln {url} | tee {output_file}",                      inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── sslscan ───────────────────────────────────────────────────────
        Tool {
            name: "sslscan",
            binary: "sslscan",
            modes: &[
                Mode { name: "Full SSL/TLS Scan",        cmd_template: "sslscan {target} | tee {output_file}",                             inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Show Certificate",         cmd_template: "sslscan --show-certificate {target} | tee {output_file}",          inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "No Failed Ciphers",        cmd_template: "sslscan --no-failed {target} | tee {output_file}",                 inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export XML Report",        cmd_template: "sslscan --xml {output_file} {target}",                             inputs: &[InputKind::Target], output_format: OutputFormat::Xml,   file_ext: "xml" },
            ],
        },
        // ── tcpdump ───────────────────────────────────────────────────────
        Tool {
            name: "tcpdump",
            binary: "tcpdump",
            modes: &[
                Mode { name: "Capture on Port",          cmd_template: "tcpdump -i any port {ports} -w {output_file}.pcap",               inputs: &[InputKind::Ports],  output_format: OutputFormat::Raw,   file_ext: "pcap" },
                Mode { name: "Capture HTTP Traffic",     cmd_template: "tcpdump -i any port 80 or port 443 -c 100 -w {output_file}.pcap", inputs: &[],                  output_format: OutputFormat::Raw,   file_ext: "pcap" },
                Mode { name: "Capture DNS Traffic",      cmd_template: "tcpdump -i any port 53 -c 100 -w {output_file}.pcap",             inputs: &[],                  output_format: OutputFormat::Raw,   file_ext: "pcap" },
                Mode { name: "Save PCAP Capture",        cmd_template: "tcpdump -i any -c 100 -w {output_file}.pcap {target}",            inputs: &[InputKind::Target], output_format: OutputFormat::Raw,   file_ext: "pcap" },
            ],
        },
        // ── tshark ────────────────────────────────────────────────────────
        Tool {
            name: "tshark",
            binary: "tshark",
            modes: &[
                Mode { name: "Decode PCAP File",         cmd_template: "tshark -r {file} | tee {output_file}",                             inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Filter by Protocol",       cmd_template: "tshark -r {file} -Y '{value}' | tee {output_file}",               inputs: &[InputKind::File, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export as JSON",           cmd_template: "tshark -r {file} -T json | tee {output_file}",                     inputs: &[InputKind::File],   output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "HTTP Stream Extract",      cmd_template: "tshark -r {file} -Y http -T fields -e http.host -e http.request.uri | tee {output_file}", inputs: &[InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── trufflehog ────────────────────────────────────────────────────
        Tool {
            name: "trufflehog",
            binary: "trufflehog",
            modes: &[
                Mode { name: "Scan Git Repo",            cmd_template: "trufflehog git {url} | tee {output_file}",                         inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Scan Filesystem Path",     cmd_template: "trufflehog filesystem {file} | tee {output_file}",                 inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Scan S3 Bucket",           cmd_template: "trufflehog s3 --bucket={target} | tee {output_file}",              inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON Output Scan",         cmd_template: "trufflehog git {url} --json | tee {output_file}",                  inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── gitleaks ──────────────────────────────────────────────────────
        Tool {
            name: "gitleaks",
            binary: "gitleaks",
            modes: &[
                Mode { name: "Scan Repo Secrets",        cmd_template: "gitleaks detect --source {url} -r {output_file}",                  inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Detect in Directory",      cmd_template: "gitleaks detect --source {file} -r {output_file}",                 inputs: &[InputKind::File],   output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Scan Staged Files",        cmd_template: "gitleaks protect --staged -r {output_file}",                       inputs: &[],                  output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "No-Git Dir Scan",          cmd_template: "gitleaks detect --source {file} --no-git -r {output_file}",        inputs: &[InputKind::File],   output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── EyeWitness ────────────────────────────────────────────────────
        Tool {
            name: "EyeWitness",
            binary: "eyewitness",
            modes: &[
                Mode { name: "Screenshot URL File",      cmd_template: "eyewitness --web -f {file} -d {output_file}",                      inputs: &[InputKind::File],   output_format: OutputFormat::Raw,   file_ext: "html" },
                Mode { name: "Single URL Report",        cmd_template: "eyewitness --web --single {url} -d {output_file}",                 inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "html" },
                Mode { name: "RDP Screenshot",           cmd_template: "eyewitness --rdp -f {file} -d {output_file}",                      inputs: &[InputKind::File],   output_format: OutputFormat::Raw,   file_ext: "html" },
                Mode { name: "HTML Report Export",       cmd_template: "eyewitness --web -f {file} --report-filename {output_file}",       inputs: &[InputKind::File],   output_format: OutputFormat::Raw,   file_ext: "html" },
            ],
        },
        // ── aquatone ──────────────────────────────────────────────────────
        Tool {
            name: "aquatone",
            binary: "aquatone",
            modes: &[
                Mode { name: "Screenshot Domains",       cmd_template: "cat {file} | aquatone -out {output_file}",                         inputs: &[InputKind::File],   output_format: OutputFormat::Raw,   file_ext: "html" },
                Mode { name: "Parse Hosts File",         cmd_template: "cat {file} | aquatone -scan-timeout 300 -out {output_file}",       inputs: &[InputKind::File],   output_format: OutputFormat::Raw,   file_ext: "html" },
                Mode { name: "Large Ports Full Scan",    cmd_template: "cat {file} | aquatone -ports large -out {output_file}",            inputs: &[InputKind::File],   output_format: OutputFormat::Raw,   file_ext: "html" },
                Mode { name: "JSON Summary Export",      cmd_template: "cat {file} | aquatone -json-summary -out {output_file}",           inputs: &[InputKind::File],   output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
    ],
};
