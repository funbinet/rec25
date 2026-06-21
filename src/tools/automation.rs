//! Automation category — 14 tools, 4 modes each.

use super::types::{Category, InputKind, Mode, OutputFormat, Tool};

pub static AUTOMATION: Category = Category {
    name: "Automation",
    tools: &[
        // ── bbot ──────────────────────────────────────────────────────────
        Tool {
            name: "bbot",
            binary: "bbot",
            modes: &[
                Mode { name: "Passive Recon Scan",       cmd_template: "bbot -t {domain} 2>&1 | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Full Recon Scan",          cmd_template: "bbot -t {domain} -m all 2>&1 | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Subdomain Enum Only",      cmd_template: "bbot -t {domain} -m subdomaincenter 2>&1 | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON Output Scan",         cmd_template: "bbot -t {domain} -o json 2>&1 | tee {output_file}.json", inputs: &[InputKind::Domain], output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── reconftw ──────────────────────────────────────────────────────
        Tool {
            name: "reconftw",
            binary: "reconftw",
            modes: &[
                Mode { name: "Full Recon All",           cmd_template: "reconftw -d {domain} --all 2>&1 | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Passive Recon Only",       cmd_template: "reconftw -d {domain} --passive 2>&1 | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Web Recon Mode",           cmd_template: "reconftw -d {domain} --web 2>&1 | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Subdomain Recon",          cmd_template: "reconftw -d {domain} --subdomain 2>&1 | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── osmedeus ──────────────────────────────────────────────────────
        Tool {
            name: "osmedeus",
            binary: "osmedeus",
            modes: &[
                Mode { name: "Start Default Workflow",   cmd_template: "osmedeus -t {domain} 2>&1 | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Custom Workflow File",     cmd_template: "osmedeus -t {domain} -f {file} 2>&1 | tee {output_file}", inputs: &[InputKind::Domain, InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Scan with Report",         cmd_template: "osmedeus -t {domain} --report 2>&1 | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Standard Profile Scan",    cmd_template: "osmedeus -t {domain} -p standard 2>&1 | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── sn1per ────────────────────────────────────────────────────────
        Tool {
            name: "sn1per",
            binary: "sn1per",
            modes: &[
                Mode { name: "Normal Recon Scan",        cmd_template: "sn1per -t {target} 2>&1 | tee {output_file}", inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Full Recon Mode",          cmd_template: "sn1per -t {target} -m normal 2>&1 | tee {output_file}", inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "OSINT Recon Scan",         cmd_template: "sn1per -t {target} -m osint 2>&1 | tee {output_file}", inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Web App Scan",             cmd_template: "sn1per -t {target} -m webportals 2>&1 | tee {output_file}", inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── unfurl ────────────────────────────────────────────────────────
        Tool {
            name: "unfurl",
            binary: "unfurl",
            modes: &[
                Mode { name: "Extract Domains",          cmd_template: "cat {file} | unfurl domains 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Extract Keys/Params",      cmd_template: "cat {file} | unfurl keys 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Extract Paths",            cmd_template: "cat {file} | unfurl paths 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Extract All URLs",         cmd_template: "cat {file} | unfurl urls 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── anew ──────────────────────────────────────────────────────────
        Tool {
            name: "anew",
            binary: "anew",
            modes: &[
                Mode { name: "Deduplicate to File",      cmd_template: "cat {file} | anew {output_file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Append Unique Lines",      cmd_template: "cat {file} | anew {output_file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Deduplicate from Stdin",   cmd_template: "cat {file} | anew {output_file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Sort and Unique",          cmd_template: "sort -u {file} | anew {output_file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── gf ────────────────────────────────────────────────────────────
        Tool {
            name: "gf",
            binary: "gf",
            modes: &[
                Mode { name: "XSS Pattern Match",        cmd_template: "gf xss {file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "SQLi Pattern Match",       cmd_template: "gf sqli {file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "SSRF Pattern Match",       cmd_template: "gf ssrf {file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "LFI Pattern Match",        cmd_template: "gf lfi {file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── uro ───────────────────────────────────────────────────────────
        Tool {
            name: "uro",
            binary: "uro",
            modes: &[
                Mode { name: "Filter Unique URLs",       cmd_template: "cat {file} | uro 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Remove Parameters",        cmd_template: "cat {file} | uro --base 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Deduplicate by Pattern",   cmd_template: "cat {file} | uro --dedupe 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Whitelist Filter",         cmd_template: "cat {file} | uro --whitelist {value} 2>&1 | tee {output_file}", inputs: &[InputKind::File, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── jq ────────────────────────────────────────────────────────────
        Tool {
            name: "jq",
            binary: "jq",
            modes: &[
                Mode { name: "Filter JSON Query",        cmd_template: "jq '{value}' {file} 2>&1 | tee {output_file}", inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Json, file_ext: "json" },
                Mode { name: "Extract JSON Field",       cmd_template: "jq '.{value}' {file} 2>&1 | tee {output_file}", inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Json, file_ext: "json" },
                Mode { name: "Format Pretty Print",      cmd_template: "jq '.' {file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Convert to CSV",           cmd_template: "jq -r '.[] | [.{value}] | @csv' {file} 2>&1 | tee {output_file}", inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Csv, file_ext: "csv" },
            ],
        },
        // ── yq ────────────────────────────────────────────────────────────
        Tool {
            name: "yq",
            binary: "yq",
            modes: &[
                Mode { name: "Parse YAML File",          cmd_template: "yq e '.' {file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "Convert to JSON",          cmd_template: "yq e -o=json '.' {file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Extract YAML Field",       cmd_template: "yq e '.{value}' {file} 2>&1 | tee {output_file}", inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "Pretty Format Output",     cmd_template: "yq e -P '.' {file} 2>&1 | tee {output_file}", inputs: &[InputKind::File],   output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
        // ── ripgrep ───────────────────────────────────────────────────────
        Tool {
            name: "ripgrep",
            binary: "rg",
            modes: &[
                Mode { name: "Search Pattern in File",   cmd_template: "rg '{value}' {file} 2>&1 | tee {output_file}", inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Search by Filetype",       cmd_template: "rg '{value}' --type {target} 2>&1 | tee {output_file}", inputs: &[InputKind::FreeText, InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Recursive Dir Search",     cmd_template: "rg -r '{value}' {file} 2>&1 | tee {output_file}", inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Output Only Matches",      cmd_template: "rg -o '{value}' {file} 2>&1 | tee {output_file}", inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── tmux ──────────────────────────────────────────────────────────
        Tool {
            name: "tmux",
            binary: "tmux",
            modes: &[
                Mode { name: "New Named Session",        cmd_template: "tmux new-session -d -s {value} && echo 'Session {value} created' > {output_file} 2>&1", inputs: &[InputKind::FreeText, InputKind::FreeText], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "List All Sessions",        cmd_template: "tmux list-sessions 2>&1 | tee {output_file}", inputs: &[],                  output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Kill Named Session",       cmd_template: "tmux kill-session -t {value} && echo 'Session {value} killed' > {output_file} 2>&1", inputs: &[InputKind::FreeText, InputKind::FreeText], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "Attach Existing Session",  cmd_template: "tmux attach-session -t {value} 2>&1 | tee {output_file}", inputs: &[InputKind::FreeText], output_format: OutputFormat::Raw, file_ext: "txt" },
            ],
        },
        // ── curl ──────────────────────────────────────────────────────────
        Tool {
            name: "curl",
            binary: "curl",
            modes: &[
                Mode { name: "GET Request",              cmd_template: "curl -sv {url} 2>&1 | tee {output_file}", inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "POST Request",             cmd_template: "curl -sv -X POST -d '{value}' {url} 2>&1 | tee {output_file}", inputs: &[InputKind::FreeText, InputKind::Url], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "Follow Redirects",         cmd_template: "curl -sL {url} 2>&1 | tee {output_file}", inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "Save Response Body",       cmd_template: "curl -s -o {output_file} {url} 2>&1 | tee {output_file}", inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
        // ── wget ──────────────────────────────────────────────────────────
        Tool {
            name: "wget",
            binary: "wget",
            modes: &[
                Mode { name: "Download File",            cmd_template: "wget -q {url} -O {output_file} 2>&1 | tee {output_file}", inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "bin" },
                Mode { name: "Mirror Site",              cmd_template: "wget -q -m -r -np {url} -P {output_file} 2>&1 | tee {output_file}", inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "Quiet Stealth Download",   cmd_template: "wget -q --no-check-certificate -O {output_file} {url} 2>&1 | tee {output_file}", inputs: &[InputKind::Url], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "Recursive Fetch Depth",    cmd_template: "wget -q -r -l 5 -np {url} -P {output_file} 2>&1 | tee {output_file}", inputs: &[InputKind::Url], output_format: OutputFormat::Raw, file_ext: "txt" },
            ],
        },
    ],
};