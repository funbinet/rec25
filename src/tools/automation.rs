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
                Mode { name: "Passive Recon Scan",       cmd_template: "bbot -t {domain} -p passive -o {output_file}",                     inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Full Recon Scan",          cmd_template: "bbot -t {domain} -p all -o {output_file}",                         inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Subdomain Enum Only",      cmd_template: "bbot -t {domain} -m subdomain-enum -o {output_file}",              inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON Output Scan",         cmd_template: "bbot -t {domain} --json -o {output_file}",                         inputs: &[InputKind::Domain], output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── reconftw ──────────────────────────────────────────────────────
        Tool {
            name: "reconftw",
            binary: "reconftw.sh",
            modes: &[
                Mode { name: "Full Recon All",           cmd_template: "reconftw.sh -d {domain} -A | tee {output_file}",                   inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Passive Recon Only",       cmd_template: "reconftw.sh -d {domain} -p | tee {output_file}",                   inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Web Recon Mode",           cmd_template: "reconftw.sh -d {domain} -w | tee {output_file}",                   inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Subdomain Recon",          cmd_template: "reconftw.sh -d {domain} -s | tee {output_file}",                   inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── osmedeus ──────────────────────────────────────────────────────
        Tool {
            name: "osmedeus",
            binary: "osmedeus",
            modes: &[
                Mode { name: "Start Default Workflow",   cmd_template: "osmedeus scan -t {domain} | tee {output_file}",                    inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Custom Workflow File",     cmd_template: "osmedeus scan -f {file} -t {domain} | tee {output_file}",          inputs: &[InputKind::Domain, InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Scan with Report",         cmd_template: "osmedeus scan -t {domain} --report | tee {output_file}",           inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Standard Profile Scan",   cmd_template: "osmedeus scan -t {domain} --profile standard | tee {output_file}", inputs: &[InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── sn1per ────────────────────────────────────────────────────────
        Tool {
            name: "sn1per",
            binary: "sniper",
            modes: &[
                Mode { name: "Normal Recon Scan",        cmd_template: "sniper -t {target} | tee {output_file}",                           inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Full Recon Mode",          cmd_template: "sniper -t {target} -m normal | tee {output_file}",                 inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "OSINT Recon Scan",         cmd_template: "sniper -t {target} -m osint | tee {output_file}",                  inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Web App Scan",             cmd_template: "sniper -t {target} -m webportals | tee {output_file}",             inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── unfurl ────────────────────────────────────────────────────────
        Tool {
            name: "unfurl",
            binary: "unfurl",
            modes: &[
                Mode { name: "Extract Domains",          cmd_template: "cat {file} | unfurl domains | tee {output_file}",                  inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Extract Keys/Params",      cmd_template: "cat {file} | unfurl keys | tee {output_file}",                     inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Format URL Fields",        cmd_template: "cat {file} | unfurl format {value} | tee {output_file}",           inputs: &[InputKind::File, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Parse from URL File",      cmd_template: "cat {file} | unfurl -u | tee {output_file}",                       inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── anew ──────────────────────────────────────────────────────────
        Tool {
            name: "anew",
            binary: "anew",
            modes: &[
                Mode { name: "Deduplicate to File",      cmd_template: "cat {file} | anew {output_file}",                                  inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Append Unique Lines",      cmd_template: "cat {file} | anew -d {target} | tee {output_file}",               inputs: &[InputKind::File, InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Compare Two Files",        cmd_template: "cat {file} | anew {target} | tee {output_file}",                   inputs: &[InputKind::File, InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Sort and Unique",          cmd_template: "sort -u {file} | anew {output_file}",                              inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── gf ────────────────────────────────────────────────────────────
        Tool {
            name: "gf",
            binary: "gf",
            modes: &[
                Mode { name: "XSS Pattern Match",        cmd_template: "gf xss {file} | tee {output_file}",                                inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "SQLi Pattern Match",       cmd_template: "gf sqli {file} | tee {output_file}",                               inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "SSRF Pattern Match",       cmd_template: "gf ssrf {file} | tee {output_file}",                               inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "LFI Pattern Match",        cmd_template: "gf lfi {file} | tee {output_file}",                                inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── uro ───────────────────────────────────────────────────────────
        Tool {
            name: "uro",
            binary: "uro",
            modes: &[
                Mode { name: "Filter Unique URLs",       cmd_template: "cat {file} | uro | tee {output_file}",                             inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Remove Parameters",        cmd_template: "cat {file} | uro -b | tee {output_file}",                          inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Deduplicate by Pattern",   cmd_template: "cat {file} | uro -d {value} | tee {output_file}",                 inputs: &[InputKind::File, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Whitelist Filter",         cmd_template: "cat {file} | uro -w {value} | tee {output_file}",                 inputs: &[InputKind::File, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── jq ────────────────────────────────────────────────────────────
        Tool {
            name: "jq",
            binary: "jq",
            modes: &[
                Mode { name: "Filter JSON Query",        cmd_template: "jq '{value}' {file} | tee {output_file}",                         inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Json, file_ext: "json" },
                Mode { name: "Extract JSON Field",       cmd_template: "jq '.{value}' {file} | tee {output_file}",                        inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Json, file_ext: "json" },
                Mode { name: "Format Pretty Print",      cmd_template: "jq '.' {file} | tee {output_file}",                               inputs: &[InputKind::File],   output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Convert to CSV",           cmd_template: "jq -r '.[] | [.{value}] | @csv' {file} | tee {output_file}",     inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Csv, file_ext: "csv" },
            ],
        },
        // ── yq ────────────────────────────────────────────────────────────
        Tool {
            name: "yq",
            binary: "yq",
            modes: &[
                Mode { name: "Parse YAML File",          cmd_template: "yq e '.' {file} | tee {output_file}",                              inputs: &[InputKind::File],   output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "Convert to JSON",          cmd_template: "yq e -o=json '.' {file} | tee {output_file}",                      inputs: &[InputKind::File],   output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Extract YAML Field",       cmd_template: "yq e '.{value}' {file} | tee {output_file}",                      inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "Pretty Format Output",     cmd_template: "yq e -P '.' {file} | tee {output_file}",                           inputs: &[InputKind::File],   output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
        // ── ripgrep ───────────────────────────────────────────────────────
        Tool {
            name: "ripgrep",
            binary: "rg",
            modes: &[
                Mode { name: "Search Pattern in File",   cmd_template: "rg '{value}' {file} | tee {output_file}",                         inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Search by Filetype",       cmd_template: "rg '{value}' --type {target} | tee {output_file}",                inputs: &[InputKind::FreeText, InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Recursive Dir Search",     cmd_template: "rg -r '{value}' {file} | tee {output_file}",                      inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Output Only Matches",      cmd_template: "rg -o '{value}' {file} | tee {output_file}",                      inputs: &[InputKind::FreeText, InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── tmux ──────────────────────────────────────────────────────────
        Tool {
            name: "tmux",
            binary: "tmux",
            modes: &[
                Mode { name: "New Named Session",        cmd_template: "tmux new-session -d -s {value} && echo 'Session {value} created' | tee {output_file}", inputs: &[InputKind::FreeText], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "List All Sessions",        cmd_template: "tmux list-sessions | tee {output_file}",                           inputs: &[],                  output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Kill Named Session",       cmd_template: "tmux kill-session -t {value} && echo 'Killed {value}' | tee {output_file}", inputs: &[InputKind::FreeText], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "Attach Existing Session",  cmd_template: "tmux attach-session -t {value}",                                  inputs: &[InputKind::FreeText], output_format: OutputFormat::Raw, file_ext: "txt" },
            ],
        },
        // ── curl ──────────────────────────────────────────────────────────
        Tool {
            name: "curl",
            binary: "curl",
            modes: &[
                Mode { name: "GET Request",              cmd_template: "curl -sv {url} | tee {output_file}",                               inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "POST Request",             cmd_template: "curl -sv -X POST -d '{value}' {url} | tee {output_file}",         inputs: &[InputKind::Url, InputKind::FreeText], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "Follow Redirects",         cmd_template: "curl -sL {url} | tee {output_file}",                               inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "Save Response Body",       cmd_template: "curl -s -o {output_file} {url}",                                   inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
        // ── wget ──────────────────────────────────────────────────────────
        Tool {
            name: "wget",
            binary: "wget",
            modes: &[
                Mode { name: "Download File",            cmd_template: "wget -q {url} -O {output_file}",                                   inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "bin" },
                Mode { name: "Mirror Site",              cmd_template: "wget -q -m -r -np {url} -P {output_file}",                         inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "Quiet Stealth Download",   cmd_template: "wget -q --no-check-certificate -O {output_file} {url}",            inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "Recursive Fetch Depth",    cmd_template: "wget -q -r -l 2 -np {url} -P {output_file}",                       inputs: &[InputKind::Url],    output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
    ],
};
