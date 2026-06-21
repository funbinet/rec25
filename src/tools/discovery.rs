//! Advanced Discovery & Attack Surface — 15 tools, 6 modes each.
//! Modern toolchain for comprehensive attack surface enumeration with verified commands.

use super::types::{Category, InputKind, Mode, OutputFormat, Tool};

pub static DISCOVERY: Category = Category {
    name: "Discovery",
    tools: &[
        // ── Subfinder (Verified) ─────────────────────────────────────────
        Tool {
            name: "Subfinder",
            binary: "subfinder",
            modes: &[
                Mode { 
                    name: "Passive Subdomain Enum",   
                    cmd_template: "subfinder -d {domain} -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "All Sources Enum",    
                    cmd_template: "subfinder -d {domain} -all -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Recursive Enum",   
                    cmd_template: "subfinder -d {domain} -recursive -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Specific Sources",   
                    cmd_template: "subfinder -d {domain} -sources shodan,censys,virustotal -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",              
                    cmd_template: "subfinder -d {domain} -json -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk Domain Scan",   
                    cmd_template: "subfinder -dL {file} -silent -o {output_file}", 
                    inputs: &[InputKind::File], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Amass (Verified - No -o flag) ──────────────────────────────
        Tool {
            name: "Amass",
            binary: "amass",
            modes: &[
                Mode { 
                    name: "Passive Enum",         
                    cmd_template: "amass enum -passive -d {domain} -nocolor > {output_file} 2>&1", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Active Enum",          
                    cmd_template: "amass enum -active -d {domain} -nocolor > {output_file} 2>&1", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Brute Force",          
                    cmd_template: "amass enum -brute -d {domain} -nocolor > {output_file} 2>&1", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Whois Intel",           
                    cmd_template: "amass intel -whois -d {domain} -nocolor > {output_file} 2>&1", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",              
                    cmd_template: "amass enum -d {domain} -json {output_file} -nocolor 2>&1", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk Domain Enum",           
                    cmd_template: "amass enum -df {file} -nocolor > {output_file} 2>&1", 
                    inputs: &[InputKind::File], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Findomain (Verified) ──────────────────────────────────────
        Tool {
            name: "Findomain",
            binary: "findomain",
            modes: &[
                Mode { 
                    name: "Basic Scan",      
                    cmd_template: "findomain -t {domain} -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Resolve Subdomains",        
                    cmd_template: "findomain -t {domain} -r -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Quiet Mode",          
                    cmd_template: "findomain -t {domain} -q -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Aggressive Scan",           
                    cmd_template: "findomain -t {domain} -a -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "CSV Output",            
                    cmd_template: "findomain -t {domain} --csv -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Csv,   
                    file_ext: "csv" 
                },
                Mode { 
                    name: "Bulk Domain Scan",           
                    cmd_template: "findomain -f {file} -o {output_file}", 
                    inputs: &[InputKind::File], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── DNSX (Verified) ─────────────────────────────────────────────
        Tool {
            name: "DNSX",
            binary: "dnsx",
            modes: &[
                Mode { 
                    name: "DNS Resolution",      
                    cmd_template: "dnsx -d {domain} -a -aaaa -cname -mx -ns -txt -ptr -silent -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "A/AAAA Lookup",        
                    cmd_template: "dnsx -d {domain} -a -aaaa -resp-only -silent -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "CDN Detection",         
                    cmd_template: "dnsx -d {domain} -a -cdn -cname -silent -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Wildcard Probe",          
                    cmd_template: "dnsx -d {domain} -wildcard -silent -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",           
                    cmd_template: "dnsx -d {domain} -a -cname -mx -ns -txt -silent -json -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk Resolve",           
                    cmd_template: "dnsx -l {file} -a -aaaa -silent -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── HTTPX (Verified) ──────────────────────────────────────────────
        Tool {
            name: "HTTPX",
            binary: "httpx",
            modes: &[
                Mode { 
                    name: "Web Detection",      
                    cmd_template: "httpx -u {url} -silent -status-code -title -tech-detect -content-length -o {output_file}", 
                    inputs: &[InputKind::Url],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Tech Stack Analysis",        
                    cmd_template: "httpx -u {url} -silent -tech-detect -csp-probe -tls-probe -hash -o {output_file}", 
                    inputs: &[InputKind::Url],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "WAF Detection",         
                    cmd_template: "httpx -u {url} -silent -waf-detect -cdn -server -o {output_file}", 
                    inputs: &[InputKind::Url],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Screenshot Capture",          
                    cmd_template: "httpx -u {url} -silent -screenshot -screenshot-path ./screenshots/ -o {output_file}", 
                    inputs: &[InputKind::Url],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",           
                    cmd_template: "httpx -u {url} -silent -json -o {output_file}", 
                    inputs: &[InputKind::Url],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Port Scan",           
                    cmd_template: "httpx -u {url} -silent -ports 80,443,8080,8443 -o {output_file}", 
                    inputs: &[InputKind::Url],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Naabu (Verified) ──────────────────────────────────────────────
        Tool {
            name: "Naabu",
            binary: "naabu",
            modes: &[
                Mode { 
                    name: "Top 1000 Ports",      
                    cmd_template: "naabu -host {ip} -top-ports 1000 -silent -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Full Port Scan",        
                    cmd_template: "naabu -host {ip} -p - -rate 1000 -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Service Detection",         
                    cmd_template: "naabu -host {ip} -top-ports 100 -nmap-cli -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",          
                    cmd_template: "naabu -host {ip} -top-ports 1000 -json -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Host Discovery",           
                    cmd_template: "naabu -host {ip} -top-ports 500 -host -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk IP Scan",           
                    cmd_template: "naabu -list {file} -top-ports 1000 -rate 500 -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── ASNMap (Verified) ─────────────────────────────────────────────
        Tool {
            name: "ASNMap",
            binary: "asnmap",
            modes: &[
                Mode { 
                    name: "Domain ASN Map",      
                    cmd_template: "asnmap -d {domain} -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Domain CIDR",        
                    cmd_template: "asnmap -d {domain} -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Org ASN Map",         
                    cmd_template: "asnmap -org '{value}' -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "IP to ASN",          
                    cmd_template: "asnmap -ip {ip} -o {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",           
                    cmd_template: "asnmap -d {domain} -json -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk ASN Lookup",           
                    cmd_template: "asnmap -l {file} -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Katana (Verified) ──────────────────────────────────────────────
        Tool {
            name: "Katana",
            binary: "katana",
            modes: &[
                Mode { 
                    name: "Web Crawl",      
                    cmd_template: "katana -u {domain} -silent -o {output_file} -d 3", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Deep Crawl",        
                    cmd_template: "katana -u {domain} -silent -o {output_file} -d 5 -jc", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Headless Crawl",         
                    cmd_template: "katana -u {domain} -headless -silent -o {output_file} -d 3", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Subdomain Crawl",          
                    cmd_template: "katana -u {domain} -silent -o {output_file} -d 3 -sf", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",           
                    cmd_template: "katana -u {domain} -silent -json -o {output_file} -d 3", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk Domain Crawl",           
                    cmd_template: "katana -list {file} -silent -o {output_file} -d 3 -c 50", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Gau (Verified - no -subs flag) ──────────────────────────────
        Tool {
            name: "Gau",
            binary: "gau",
            modes: &[
                Mode { 
                    name: "Multi-Provider URL Discovery",      
                    cmd_template: "gau {domain} --providers wayback,commoncrawl,otx,urlscan --o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Filtered URLs",        
                    cmd_template: "gau {domain} --providers wayback,commoncrawl,otx,urlscan --o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Blacklist Filter",         
                    cmd_template: "gau {domain} --blacklist png,jpg,gif,svg,ico,css,js,woff,ttf,mp3,mp4 --o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",          
                    cmd_template: "gau {domain} --json --o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Parameter Discovery",           
                    cmd_template: "gau {domain} --o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk Domain URL Collection",           
                    cmd_template: "gau --list {file} --providers wayback,commoncrawl,otx,urlscan --o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Waybackurls (Verified) ──────────────────────────────────────
        Tool {
            name: "Waybackurls",
            binary: "waybackurls",
            modes: &[
                Mode { 
                    name: "Archive URL Extraction",      
                    cmd_template: "echo {domain} | waybackurls | sort -u > {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Content Filtering",        
                    cmd_template: "echo {domain} | waybackurls | grep -Ev '\\.(png|jpg|gif|svg|ico|css|js|woff|ttf)$' | sort -u > {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Param Discovery",         
                    cmd_template: "echo {domain} | waybackurls | grep '?' | sort -u > {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk Domain Scrape",          
                    cmd_template: "cat {file} | waybackurls | sort -u > {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",           
                    cmd_template: "echo {domain} | waybackurls --json > {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Unique Paths Only",           
                    cmd_template: "echo {domain} | waybackurls | sort -u > {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Uncover (Verified) ──────────────────────────────────────────────
        Tool {
            name: "Uncover",
            binary: "uncover",
            modes: &[
                Mode { 
                    name: "OSINT Discovery",      
                    cmd_template: "uncover -q {domain} -e shodan,censys,fofa -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "IP & Host Discovery",        
                    cmd_template: "uncover -q {domain} -e shodan,censys -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Cert Discovery",         
                    cmd_template: "uncover -q {domain} -e certspotter,censys -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk Query",          
                    cmd_template: "uncover -q {value} -e shodan,censys,fofa -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",           
                    cmd_template: "uncover -q {domain} -e shodan,censys,fofa -silent -json -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "IP Range Discovery",           
                    cmd_template: "uncover -q {ip}/24 -e shodan,censys -silent -o {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── TLSX (Verified) ──────────────────────────────────────────────
        Tool {
            name: "TLSX",
            binary: "tlsx",
            modes: &[
                Mode { 
                    name: "TLS Analysis",      
                    cmd_template: "tlsx -u {domain} -silent -o {output_file} -c", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "SSL Certificate",        
                    cmd_template: "tlsx -u {domain} -silent -o {output_file} -c -cname -cert", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Cipher Analysis",         
                    cmd_template: "tlsx -u {domain} -silent -o {output_file} -c -ciphers", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Vuln Detection",          
                    cmd_template: "tlsx -u {domain} -silent -o {output_file} -c -p", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",           
                    cmd_template: "tlsx -u {domain} -silent -json -o {output_file} -c", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk TLS Scan",           
                    cmd_template: "tlsx -l {file} -silent -o {output_file} -c -cname", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── AlterX (Verified) ──────────────────────────────────────────────
        Tool {
            name: "AlterX",
            binary: "alterx",
            modes: &[
                Mode { 
                    name: "Subdomain Permutation",      
                    cmd_template: "alterx -d {domain} -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Enriched Permutation",        
                    cmd_template: "alterx -d {domain} -silent -enrich -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Custom Wordlist",         
                    cmd_template: "alterx -d {domain} -silent -w /usr/share/seclists/Discovery/DNS/dns-Jhaddix.txt -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Path Permutation",          
                    cmd_template: "alterx -d {domain}/ -silent -path -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",           
                    cmd_template: "alterx -d {domain} -silent -json -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk Domain",           
                    cmd_template: "alterx -list {file} -silent -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── CDNCheck (Verified) ──────────────────────────────────────────────
        Tool {
            name: "CDNCheck",
            binary: "cdncheck",
            modes: &[
                Mode { 
                    name: "CDN Detection",      
                    cmd_template: "cdncheck -l {file} -silent -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "IP CDN Check",        
                    cmd_template: "cdncheck -i {ip} -silent -o {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Provider Identification",         
                    cmd_template: "cdncheck -l {file} -silent -json -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "CDN-Only Extract",          
                    cmd_template: "cdncheck -l {file} -silent -cdn-only -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Non-CDN Extract",           
                    cmd_template: "cdncheck -l {file} -silent -non-cdn-only -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk CDN Check",           
                    cmd_template: "cdncheck -l {file} -silent -provider -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Cloudlist (Verified) ──────────────────────────────────────────────
        Tool {
            name: "Cloudlist",
            binary: "cloudlist",
            modes: &[
                Mode { 
                    name: "Multi-Cloud Discovery",      
                    cmd_template: "cloudlist -provider all -profile {value} -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "AWS Discovery",        
                    cmd_template: "cloudlist -provider aws -profile {value} -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Azure Discovery",         
                    cmd_template: "cloudlist -provider azure -profile {value} -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "GCP Discovery",          
                    cmd_template: "cloudlist -provider gcp -profile {value} -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output",           
                    cmd_template: "cloudlist -provider all -profile {value} -silent -json -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Multi-Provider Export",           
                    cmd_template: "cloudlist -provider aws,azure,gcp -profile {value} -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
    ],
};