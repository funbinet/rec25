//! Mapping category — 23 tools, 4 modes each.

use super::types::{Category, InputKind, Mode, OutputFormat, Tool};

pub static MAPPING: Category = Category {
    name: "Mapping",
    tools: &[
        // ── httpx ─────────────────────────────────────────────────────────
        Tool {
            name: "httpx",
            binary: "httpx",
            modes: &[
                Mode { name: "Probe Live Hosts",         cmd_template: "httpx -l {file} -o {output_file}",                                 inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Detect Web Tech",          cmd_template: "httpx -l {file} -tech-detect -o {output_file}",                    inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Status Code Filter",       cmd_template: "httpx -l {file} -mc 200,301,302,403 -o {output_file}",             inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON Response Export",     cmd_template: "httpx -l {file} -json -o {output_file}",                           inputs: &[InputKind::File],   output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── httprobe ──────────────────────────────────────────────────────
        Tool {
            name: "httprobe",
            binary: "httprobe",
            modes: &[
                Mode { name: "Probe HTTP/HTTPS",         cmd_template: "cat {file} | httprobe | tee {output_file}",                        inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Concurrency Probe",        cmd_template: "cat {file} | httprobe -c 50 | tee {output_file}",                  inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Custom Ports Probe",       cmd_template: "cat {file} | httprobe -p https:8443 -p http:8080 | tee {output_file}", inputs: &[InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Skip Default Ports",       cmd_template: "cat {file} | httprobe -s | tee {output_file}",                     inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── Naabu ─────────────────────────────────────────────────────────
        Tool {
            name: "Naabu",
            binary: "naabu",
            modes: &[
                Mode { name: "Top 100 Port Scan",        cmd_template: "naabu -l {file} -top-ports 100 -o {output_file}",                  inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Full Port Scan",           cmd_template: "naabu -host {ip} -p - -o {output_file}",                           inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Service Version Detect",   cmd_template: "naabu -host {ip} -top-ports 1000 -o {output_file}",                inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "JSON Export",              cmd_template: "naabu -host {ip} -json -o {output_file}",                          inputs: &[InputKind::Ip],     output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── Nmap ──────────────────────────────────────────────────────────
        Tool {
            name: "Nmap",
            binary: "nmap",
            modes: &[
                Mode { name: "Quick Port Scan",          cmd_template: "nmap -T4 -F {ip} | tee {output_file}",                             inputs: &[InputKind::Ip],     output_format: OutputFormat::Nmap,  file_ext: "txt" },
                Mode { name: "Full TCP Scan",            cmd_template: "nmap -sS -p- -T4 {ip} -oN {output_file}",                          inputs: &[InputKind::Ip],     output_format: OutputFormat::Nmap,  file_ext: "nmap" },
                Mode { name: "Service Detection",        cmd_template: "nmap -sV -sC {ip} -oN {output_file}",                              inputs: &[InputKind::Ip],     output_format: OutputFormat::Nmap,  file_ext: "nmap" },
                Mode { name: "UDP Port Scan",            cmd_template: "nmap -sU -p 53,161,137,123 {ip} | tee {output_file}",              inputs: &[InputKind::Ip],     output_format: OutputFormat::Nmap,  file_ext: "txt" },
            ],
        },
        // ── dnsx ──────────────────────────────────────────────────────────
        Tool {
            name: "dnsx",
            binary: "dnsx",
            modes: &[
                Mode { name: "Resolve Domains",          cmd_template: "dnsx -l {file} -o {output_file}",                                  inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "PTR Reverse Lookup",       cmd_template: "dnsx -l {file} -ptr -o {output_file}",                             inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "CNAME Chain Resolve",      cmd_template: "dnsx -l {file} -cname -o {output_file}",                           inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Bulk A/RESP Resolve",      cmd_template: "dnsx -l {file} -a -resp -o {output_file}",                         inputs: &[InputKind::File],   output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── masscan ───────────────────────────────────────────────────────
        Tool {
            name: "masscan",
            binary: "masscan",
            modes: &[
                Mode { name: "Fast Full TCP Scan",       cmd_template: "masscan {ip} -p 1-65535 --rate 10000 -oL {output_file}",           inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Top Ports Only",           cmd_template: "masscan {ip} --top-ports 1000 --rate 1000 -oL {output_file}",      inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Banner Grab Scan",         cmd_template: "masscan {ip} -p 80,443,8080 --banners --rate 100 -oL {output_file}", inputs: &[InputKind::Ip],   output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export XML Results",       cmd_template: "masscan {ip} -p 1-1000 --rate 1000 -oX {output_file}",             inputs: &[InputKind::Ip],     output_format: OutputFormat::Xml,   file_ext: "xml" },
            ],
        },
        // ── RustScan ──────────────────────────────────────────────────────
        Tool {
            name: "RustScan",
            binary: "rustscan",
            modes: &[
                Mode { name: "Fast Port Scan",           cmd_template: "rustscan -a {ip} -- -sV | tee {output_file}",                      inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Script Scan",              cmd_template: "rustscan -a {ip} -- -sC -sV | tee {output_file}",                  inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Service Version Scan",     cmd_template: "rustscan -a {ip} -p 1-1000 -- -sV | tee {output_file}",            inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Batch Mode Scan",          cmd_template: "rustscan -a {ip} -b 500 | tee {output_file}",                      inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── zmap ──────────────────────────────────────────────────────────
        Tool {
            name: "zmap",
            binary: "zmap",
            modes: &[
                Mode { name: "ICMP Host Scan",           cmd_template: "zmap -M icmp_echoscan -t 60 {ip} | tee {output_file}",             inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "TCP SYN Scan",             cmd_template: "zmap -p {ports} -t 60 {ip} | tee {output_file}",                   inputs: &[InputKind::Ip, InputKind::Ports], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "UDP Port Scan",            cmd_template: "zmap -M udp -p {ports} {ip} | tee {output_file}",                  inputs: &[InputKind::Ip, InputKind::Ports], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "CSV Output Scan",          cmd_template: "zmap -p {ports} {ip} -o {output_file} -O csv",                     inputs: &[InputKind::Ip, InputKind::Ports], output_format: OutputFormat::Csv, file_ext: "csv" },
            ],
        },
        // ── zgrab2 ────────────────────────────────────────────────────────
        Tool {
            name: "zgrab2",
            binary: "zgrab2",
            modes: &[
                Mode { name: "HTTP Banner Grab",         cmd_template: "echo {ip} | zgrab2 http --port 80 | tee {output_file}",             inputs: &[InputKind::Ip],     output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "HTTPS Banner Grab",        cmd_template: "echo {ip} | zgrab2 http --port 443 --use-https | tee {output_file}", inputs: &[InputKind::Ip],    output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "SSH Banner Grab",          cmd_template: "echo {ip} | zgrab2 ssh --port 22 | tee {output_file}",              inputs: &[InputKind::Ip],     output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Custom Module Grab",       cmd_template: "echo {ip} | zgrab2 {value} | tee {output_file}",                   inputs: &[InputKind::Ip, InputKind::FreeText], output_format: OutputFormat::Json, file_ext: "json" },
            ],
        },
        // ── WhatWeb ───────────────────────────────────────────────────────
        Tool {
            name: "WhatWeb",
            binary: "whatweb",
            modes: &[
                Mode { name: "Aggressive Web Ident",     cmd_template: "whatweb -a 3 {url} | tee {output_file}",                           inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Quiet Fingerprint",        cmd_template: "whatweb -q {url} | tee {output_file}",                             inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Enumerate Plugins",        cmd_template: "whatweb -a 3 --info-plugins {url} | tee {output_file}",            inputs: &[InputKind::Url],    output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export JSON Results",      cmd_template: "whatweb -a 3 --log-json {output_file} {url}",                      inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
            ],
        },
        // ── Wappalyzer CLI ────────────────────────────────────────────────
        Tool {
            name: "Wappalyzer CLI",
            binary: "wappalyzer",
            modes: &[
                Mode { name: "Tech Fingerprint URL",     cmd_template: "wappalyzer {url} | tee {output_file}",                             inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Batch URL File Scan",      cmd_template: "wappalyzer -i {file} | tee {output_file}",                         inputs: &[InputKind::File],   output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "JSON Technology Output",   cmd_template: "wappalyzer --json {url} | tee {output_file}",                      inputs: &[InputKind::Url],    output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "Export CSV Results",       cmd_template: "wappalyzer --csv {url} | tee {output_file}",                       inputs: &[InputKind::Url],    output_format: OutputFormat::Csv,   file_ext: "csv" },
            ],
        },
        // ── traceroute ────────────────────────────────────────────────────
        Tool {
            name: "traceroute",
            binary: "traceroute",
            modes: &[
                Mode { name: "ICMP Trace Route",         cmd_template: "traceroute -I {target} | tee {output_file}",                       inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "TCP Trace Route",          cmd_template: "traceroute -T {target} | tee {output_file}",                       inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "UDP Trace Route",          cmd_template: "traceroute -U {target} | tee {output_file}",                       inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Max Hop Trace",            cmd_template: "traceroute -m 30 {target} | tee {output_file}",                    inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── mtr ───────────────────────────────────────────────────────────
        Tool {
            name: "mtr",
            binary: "mtr",
            modes: &[
                Mode { name: "Report Mode Trace",        cmd_template: "mtr -r -n -c 10 {target} | tee {output_file}",                     inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Live Network Trace",       cmd_template: "mtr --report {target} | tee {output_file}",                        inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "TCP Port Trace",           cmd_template: "mtr -T -P {ports} {target} | tee {output_file}",                   inputs: &[InputKind::Target, InputKind::Ports], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Wide Column Output",       cmd_template: "mtr -w {target} | tee {output_file}",                              inputs: &[InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── netcat ────────────────────────────────────────────────────────
        Tool {
            name: "netcat",
            binary: "nc",
            modes: &[
                Mode { name: "Port Banner Grab",         cmd_template: "nc -w 3 -nv {ip} {ports} | tee {output_file}",                     inputs: &[InputKind::Ip, InputKind::Ports], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "Listen Mode",              cmd_template: "nc -lvnp {ports} | tee {output_file}",                             inputs: &[InputKind::Ports],  output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "HTTP GET Request",         cmd_template: "echo -e 'GET / HTTP/1.0\\n\\n' | nc {ip} 80 | tee {output_file}",  inputs: &[InputKind::Ip],     output_format: OutputFormat::Raw,   file_ext: "txt" },
                Mode { name: "UDP Port Probe",           cmd_template: "nc -u -w 3 {ip} {ports} | tee {output_file}",                      inputs: &[InputKind::Ip, InputKind::Ports], output_format: OutputFormat::Raw, file_ext: "txt" },
            ],
        },
        // ── socat ─────────────────────────────────────────────────────────
        Tool {
            name: "socat",
            binary: "socat",
            modes: &[
                Mode { name: "TCP Port Relay",           cmd_template: "socat - TCP:{ip}:{ports} | tee {output_file}",                     inputs: &[InputKind::Ip, InputKind::Ports], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "UDP Port Relay",           cmd_template: "socat - UDP:{ip}:{ports} | tee {output_file}",                     inputs: &[InputKind::Ip, InputKind::Ports], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "SSL Wrap Probe",           cmd_template: "socat - OPENSSL:{ip}:{ports},verify=0 | tee {output_file}",        inputs: &[InputKind::Ip, InputKind::Ports], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "TCP Listen Pipe",          cmd_template: "socat TCP-LISTEN:{ports},reuseaddr - | tee {output_file}",         inputs: &[InputKind::Ports],  output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
        // ── onesixtyone ───────────────────────────────────────────────────
        Tool {
            name: "onesixtyone",
            binary: "onesixtyone",
            modes: &[
                Mode { name: "SNMP Community Scan",      cmd_template: "onesixtyone -c /usr/share/seclists/Discovery/SNMP/common-snmp-community-strings.txt {ip} | tee {output_file}", inputs: &[InputKind::Ip], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Bulk Host SNMP Scan",      cmd_template: "onesixtyone -c /usr/share/seclists/Discovery/SNMP/common-snmp-community-strings.txt -i {file} | tee {output_file}", inputs: &[InputKind::File], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "SNMP v2c Community",       cmd_template: "onesixtyone {ip} public | tee {output_file}",                      inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Verbose SNMP Probe",       cmd_template: "onesixtyone -c /usr/share/seclists/Discovery/SNMP/common-snmp-community-strings.txt -d {ip} | tee {output_file}", inputs: &[InputKind::Ip], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── snmpwalk ──────────────────────────────────────────────────────
        Tool {
            name: "snmpwalk",
            binary: "snmpwalk",
            modes: &[
                Mode { name: "Full OID Walk",            cmd_template: "snmpwalk -c public -v2c {ip} | tee {output_file}",                 inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Interface Info Walk",      cmd_template: "snmpwalk -c public -v2c {ip} IF-MIB::ifDescr | tee {output_file}", inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "System Info Walk",         cmd_template: "snmpwalk -c public -v2c {ip} system | tee {output_file}",          inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Export Full Walk",         cmd_template: "snmpwalk -c public -v2c {ip} > {output_file}",                     inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── ldapsearch ────────────────────────────────────────────────────
        Tool {
            name: "ldapsearch",
            binary: "ldapsearch",
            modes: &[
                Mode { name: "Anonymous LDAP Bind",      cmd_template: "ldapsearch -x -H ldap://{ip} -b 'dc={domain},dc=com' | tee {output_file}", inputs: &[InputKind::Ip, InputKind::Domain], output_format: OutputFormat::Raw, file_ext: "txt" },
                Mode { name: "User Enum LDAP",           cmd_template: "ldapsearch -x -H ldap://{ip} -b 'dc={domain},dc=com' '(objectClass=person)' | tee {output_file}", inputs: &[InputKind::Ip, InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Group Enum LDAP",          cmd_template: "ldapsearch -x -H ldap://{ip} -b 'dc={domain},dc=com' '(objectClass=group)' | tee {output_file}", inputs: &[InputKind::Ip, InputKind::Domain], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Full Base Dump",           cmd_template: "ldapsearch -x -H ldap://{ip} -b '' -s base | tee {output_file}",  inputs: &[InputKind::Ip],     output_format: OutputFormat::Raw,   file_ext: "txt" },
            ],
        },
        // ── rpcclient ─────────────────────────────────────────────────────
        Tool {
            name: "rpcclient",
            binary: "rpcclient",
            modes: &[
                Mode { name: "Null Session Enum",        cmd_template: "rpcclient -U '' -N {ip} -c 'srvinfo' | tee {output_file}",         inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "User List Enum",           cmd_template: "rpcclient -U '' -N {ip} -c 'enumdomusers' | tee {output_file}",    inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Share Enum",               cmd_template: "rpcclient -U '' -N {ip} -c 'netshareenum' | tee {output_file}",   inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "SID Lookup",               cmd_template: "rpcclient -U '' -N {ip} -c 'lookupnames {target}' | tee {output_file}", inputs: &[InputKind::Ip, InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── enum4linux ────────────────────────────────────────────────────
        Tool {
            name: "enum4linux",
            binary: "enum4linux",
            modes: &[
                Mode { name: "Full Enum Scan",           cmd_template: "enum4linux -a {ip} | tee {output_file}",                           inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "User Enum",                cmd_template: "enum4linux -U {ip} | tee {output_file}",                           inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Share Enum",               cmd_template: "enum4linux -S {ip} | tee {output_file}",                          inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Password Policy",          cmd_template: "enum4linux -P {ip} | tee {output_file}",                           inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── enum4linux-ng ─────────────────────────────────────────────────
        Tool {
            name: "enum4linux-ng",
            binary: "enum4linux-ng",
            modes: &[
                Mode { name: "Full Scan JSON Export",    cmd_template: "enum4linux-ng {ip} -A -oJ {output_file}",                          inputs: &[InputKind::Ip],     output_format: OutputFormat::Json,  file_ext: "json" },
                Mode { name: "User Enum",                cmd_template: "enum4linux-ng {ip} -U | tee {output_file}",                        inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Group Enum",               cmd_template: "enum4linux-ng {ip} -G | tee {output_file}",                        inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "RID Brute Force",          cmd_template: "enum4linux-ng {ip} -R | tee {output_file}",                        inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
        // ── smbmap ────────────────────────────────────────────────────────
        Tool {
            name: "smbmap",
            binary: "smbmap",
            modes: &[
                Mode { name: "List SMB Shares",          cmd_template: "smbmap -H {ip} | tee {output_file}",                               inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Check Permissions",        cmd_template: "smbmap -H {ip} -u {target} | tee {output_file}",                   inputs: &[InputKind::Ip, InputKind::Target], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Spider All Shares",        cmd_template: "smbmap -H {ip} -R | tee {output_file}",                            inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Download Remote File",     cmd_template: "smbmap -H {ip} --download '{value}' | tee {output_file}",          inputs: &[InputKind::Ip, InputKind::FreeText], output_format: OutputFormat::Raw, file_ext: "txt" },
            ],
        },
        // ── crackmapexec ──────────────────────────────────────────────────
        Tool {
            name: "crackmapexec",
            binary: "crackmapexec",
            modes: &[
                Mode { name: "SMB Host Enum",            cmd_template: "crackmapexec smb {ip} | tee {output_file}",                        inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Password Spray SMB",       cmd_template: "crackmapexec smb {ip} -u {target} -p {value} | tee {output_file}", inputs: &[InputKind::Ip, InputKind::Target, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Share Enum SMB",           cmd_template: "crackmapexec smb {ip} --shares | tee {output_file}",               inputs: &[InputKind::Ip],     output_format: OutputFormat::Lines, file_ext: "txt" },
                Mode { name: "Execute Remote Command",   cmd_template: "crackmapexec smb {ip} -u {target} -p {value} -x 'whoami' | tee {output_file}", inputs: &[InputKind::Ip, InputKind::Target, InputKind::FreeText], output_format: OutputFormat::Lines, file_ext: "txt" },
            ],
        },
    ],
};
