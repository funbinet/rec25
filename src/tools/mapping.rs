//! Mapping category — 15 tools, 8+ modes each.
//! Advanced network mapping, port scanning, service detection, and infrastructure discovery.

use super::types::{Category, InputKind, Mode, OutputFormat, Tool};

pub static MAPPING: Category = Category {
    name: "Mapping",
    tools: &[
        // ── HTTPX (Modern Web Probe) ─────────────────────────────────────
Tool {
    name: "HTTPX",
    binary: "httpx-pd",
    modes: &[
        Mode { 
            name: "Live Host Detection",       
            cmd_template: "httpx-pd -u {url} -silent -status-code -title -content-length -o \"{output_file}\"", 
            inputs: &[InputKind::Url],   
            output_format: OutputFormat::Lines, 
            file_ext: "txt" 
        },
        Mode { 
            name: "Technology Fingerprinting",    
            cmd_template: "httpx-pd -u {url} -silent -tech-detect -csp-probe -tls-probe -o \"{output_file}\"", 
            inputs: &[InputKind::Url],   
            output_format: OutputFormat::Lines, 
            file_ext: "txt" 
        },
        Mode { 
            name: "WAF_CDN_Detection",  // Removed spaces and & symbol
            cmd_template: "httpx-pd -u {url} -silent -cdn -server -o \"{output_file}\"",  // Removed -waf-detect
            inputs: &[InputKind::Url],   
            output_format: OutputFormat::Lines, 
            file_ext: "txt" 
        },
        Mode { 
            name: "Screenshot_Capture",  // Removed spaces
            cmd_template: "httpx-pd -u {url} -silent -screenshot -o \"{output_file}\"",  // Removed -screenshot-path
            inputs: &[InputKind::Url],   
            output_format: OutputFormat::Lines, 
            file_ext: "txt" 
        },
        Mode { 
            name: "JSON_Export_Full_Metadata",  // Removed spaces
            cmd_template: "httpx-pd -u {url} -silent -json -o \"{output_file}\"", 
            inputs: &[InputKind::Url],   
            output_format: OutputFormat::Json,  
            file_ext: "json" 
        },
        Mode { 
            name: "Response_Body_Hash_Check",  // Removed spaces
            cmd_template: "httpx-pd -u {url} -silent -hash -o \"{output_file}\"", 
            inputs: &[InputKind::Url],   
            output_format: OutputFormat::Lines, 
            file_ext: "txt" 
        },
        Mode { 
            name: "Custom_Port_Web_Probe",  // Removed spaces
            cmd_template: "httpx-pd -u {url} -silent -ports 80,443,8080,8443,9000,9443 -o \"{output_file}\"", 
            inputs: &[InputKind::Url],   
            output_format: OutputFormat::Lines, 
            file_ext: "txt" 
        },
        Mode { 
            name: "Bulk_Subdomain_Probe",  // Removed spaces
            cmd_template: "httpx-pd -l \"{file}\" -silent -status-code -title -tech-detect -o \"{output_file}\"",  // Quoted {file}
            inputs: &[InputKind::File],   
            output_format: OutputFormat::Lines, 
            file_ext: "txt" 
        },
    ],
},
        // ── DNSX (DNS Resolution) ────────────────────────────────────────
        Tool {
            name: "DNSX",
            binary: "dnsx",
            modes: &[
                Mode { 
                    name: "Comprehensive DNS Resolution", 
                    cmd_template: "dnsx -d {domain} -a -aaaa -cname -mx -ns -txt -ptr -silent -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "A/AAAA Record Lookup",        
                    cmd_template: "dnsx -d {domain} -a -aaaa -resp-only -silent -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "CDN Detection via DNS",       
                    cmd_template: "dnsx -d {domain} -a -cdn -cname -silent -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Wildcard Detection",          
                    cmd_template: "dnsx -d {domain} -wildcard -silent -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export with All Records", 
                    cmd_template: "dnsx -d {domain} -a -cname -mx -ns -txt -silent -json -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Reverse PTR Lookup",          
                    cmd_template: "dnsx -d {domain} -ptr -silent -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk Resolve with Rate Limit", 
                    cmd_template: "dnsx -l {file} -a -aaaa -silent -o {output_file} -rate-limit 100", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "SOA & NS Record Query",       
                    cmd_template: "dnsx -d {domain} -ns -soa -silent -o {output_file}", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Naabu (Port Scanner) ─────────────────────────────────────────
        Tool {
            name: "Naabu",
            binary: "naabu",
            modes: &[
                Mode { 
                    name: "Top 1000 Port Scan",          
                    cmd_template: "naabu -host {ip} -top-ports 1000 -silent -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Full Port Scan (1-65535)",    
                    cmd_template: "naabu -host {ip} -p - -rate 1000 -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Service Version Detection",    
                    cmd_template: "naabu -host {ip} -top-ports 100 -nmap-cli -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export with Services",    
                    cmd_template: "naabu -host {ip} -top-ports 1000 -json -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Host Discovery Mode",          
                    cmd_template: "naabu -host {ip} -top-ports 500 -host -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "CIDR Range Port Scan",         
                    cmd_template: "naabu -host {ip} -top-ports 1000 -rate 500 -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Exclude Ports Scan",           
                    cmd_template: "naabu -host {ip} -top-ports 1000 -exclude-ports 9100,9200 -o {output_file}", 
                    inputs: &[InputKind::Ip],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk IP Range Discovery",      
                    cmd_template: "naabu -list {file} -top-ports 1000 -silent -rate 200 -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Nmap (Advanced Port Scanner) ────────────────────────────────
        Tool {
            name: "Nmap",
            binary: "nmap",
            modes: &[
                Mode { 
                    name: "Quick TCP Port Scan",          
                    cmd_template: "nmap -T4 -F {ip} -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "Full TCP Port Scan",           
                    cmd_template: "nmap -sS -p- -T4 -Pn {ip} -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "Service & Version Detection",  
                    cmd_template: "nmap -sV -sC -O -T4 {ip} -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "UDP Port Scan",                
                    cmd_template: "nmap -sU -p 53,161,137,123,139,445 -T4 {ip} -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "JSON Output Export",           
                    cmd_template: "nmap -sS -p- -T4 -Pn {ip} -oN {output_file} --json", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Script Scan (Default)",        
                    cmd_template: "nmap -sC -sV -T4 {ip} -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "Vulnerability Script Scan",    
                    cmd_template: "nmap --script vuln -T4 {ip} -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "Bulk Host Discovery",          
                    cmd_template: "nmap -sn {ip}/24 -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
            ],
        },
        // ── ASNMap (ASN Discovery) ───────────────────────────────────────
        Tool {
            name: "ASNMap",
            binary: "asnmap",
            modes: &[
                Mode { 
                    name: "Domain ASN Mapping",           
                    cmd_template: "asnmap -d {domain} -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "ASN to CIDR Ranges",           
                    cmd_template: "asnmap -d {domain} -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Organization ASN Map",         
                    cmd_template: "asnmap -org '{value}' -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "IP to ASN Mapping",            
                    cmd_template: "asnmap -ip {ip} -o {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export with All Data",    
                    cmd_template: "asnmap -d {domain} -json -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk Domain ASN Lookup",       
                    cmd_template: "asnmap -l {file} -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "ASN to IPv4 Range",            
                    cmd_template: "asnmap -asn {value} -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Verbose ASN Info",             
                    cmd_template: "asnmap -d {domain} -v -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── TLSX (SSL/TLS Analysis) ──────────────────────────────────────
        Tool {
            name: "TLSX",
            binary: "tlsx",
            modes: &[
                Mode { 
                    name: "Comprehensive TLS Analysis",   
                    cmd_template: "tlsx -u {domain} -silent -o {output_file} -c", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "SSL Certificate Details",      
                    cmd_template: "tlsx -u {domain} -silent -o {output_file} -c -cname -cert", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Cipher Suite Analysis",        
                    cmd_template: "tlsx -u {domain} -silent -o {output_file} -c -ciphers", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Vulnerability Detection",      
                    cmd_template: "tlsx -u {domain} -silent -o {output_file} -c -p", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Certificate Transparency Log", 
                    cmd_template: "tlsx -u {domain} -silent -json -o {output_file} -c -ct", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "JSON Export Full Details",     
                    cmd_template: "tlsx -u {domain} -silent -json -o {output_file} -c -cname -cert", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk Host TLS Enumeration",    
                    cmd_template: "tlsx -l {file} -silent -o {output_file} -c -tls-ciphers -cname", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "TLS Version Check",            
                    cmd_template: "tlsx -u {domain} -silent -o {output_file} -c -versions", 
                    inputs: &[InputKind::Domain],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── CDNCheck (CDN Detection) ─────────────────────────────────────
        Tool {
            name: "CDNCheck",
            binary: "cdncheck",
            modes: &[
                Mode { 
                    name: "CDN Detection & Mapping",     
                    cmd_template: "cdncheck -l {file} -silent -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "IP CDN Classification",        
                    cmd_template: "cdncheck -i {ip} -silent -o {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "CDN Provider Identification", 
                    cmd_template: "cdncheck -l {file} -silent -json -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk Domain CDN Analysis",     
                    cmd_template: "cdncheck -l {file} -silent -provider -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "CDN-Only IP Extraction",       
                    cmd_template: "cdncheck -l {file} -silent -cdn-only -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Non-CDN IP Discovery",         
                    cmd_template: "cdncheck -l {file} -silent -non-cdn-only -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Domain CDN Check",             
                    cmd_template: "cdncheck -d {domain} -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk IP CDN Validation",       
                    cmd_template: "cdncheck -l {file} -silent -o {output_file} -provider", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── MapCIDR (CIDR Mapping) ───────────────────────────────────────
        Tool {
            name: "MapCIDR",
            binary: "mapcidr",
            modes: &[
                Mode { 
                    name: "CIDR to IP Range",             
                    cmd_template: "mapcidr -cidr {ip}/24 -o {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk CIDR Expansion",          
                    cmd_template: "mapcidr -cidr {file} -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "ASN CIDR Expansion",           
                    cmd_template: "mapcidr -asn {value} -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "IP Range to CIDR",             
                    cmd_template: "mapcidr -ip {ip} -o {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "CIDR Split into /24s",         
                    cmd_template: "mapcidr -cidr {ip}/16 -split 24 -o {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export CIDR Ranges",      
                    cmd_template: "mapcidr -cidr {ip}/24 -json -o {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Aggregate CIDR Ranges",        
                    cmd_template: "mapcidr -cidr {file} -aggregate -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Count IPs in CIDR",            
                    cmd_template: "mapcidr -cidr {ip}/24 -count -o {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Cloudlist (Cloud Discovery) ──────────────────────────────────
        Tool {
            name: "Cloudlist",
            binary: "cloudlist",
            modes: &[
                Mode { 
                    name: "Multi-Cloud Asset Discovery", 
                    cmd_template: "cloudlist -provider all -profile {value} -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "AWS Resource Enumeration",     
                    cmd_template: "cloudlist -provider aws -profile {value} -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Azure Infrastructure Discovery",
                    cmd_template: "cloudlist -provider azure -profile {value} -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "GCP Resource Collection",     
                    cmd_template: "cloudlist -provider gcp -profile {value} -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export with Cloud Metadata",
                    cmd_template: "cloudlist -provider all -profile {value} -silent -json -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Multi-Provider Asset Export",  
                    cmd_template: "cloudlist -provider aws,azure,gcp -profile {value} -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "DigitalOcean Discovery",       
                    cmd_template: "cloudlist -provider digitalocean -profile {value} -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Verbose Cloud Asset Output",   
                    cmd_template: "cloudlist -provider all -profile {value} -v -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Uncover (OSINT Discovery) ────────────────────────────────────
        Tool {
            name: "Uncover",
            binary: "uncover",
            modes: &[
                Mode { 
                    name: "Multi-Engine OSINT Discovery", 
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
                    name: "Certificate & SSL Discovery",  
                    cmd_template: "uncover -q {domain} -e certspotter,censys -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk Query OSINT Scanning",    
                    cmd_template: "uncover -q {value} -e shodan,censys,fofa -silent -o {output_file}", 
                    inputs: &[InputKind::FreeText], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export with Providers",   
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
                Mode { 
                    name: "Shodan-IDB Specific Query",    
                    cmd_template: "uncover -q {domain} -e shodan-idb -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "All Engines Discovery",        
                    cmd_template: "uncover -q {domain} -e all -silent -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── AlterX (Permutation Scanner) ─────────────────────────────────
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
                    name: "Enriched Pattern Generation",  
                    cmd_template: "alterx -d {domain} -silent -enrich -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Custom Wordlist Permutation",  
                    cmd_template: "alterx -d {domain} -silent -w /usr/share/seclists/Discovery/DNS/dns-Jhaddix.txt -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "URL Path Permutation",         
                    cmd_template: "alterx -d {domain}/ -silent -path -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export with Patterns",    
                    cmd_template: "alterx -d {domain} -silent -json -o {output_file}", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk Domain Permutation",      
                    cmd_template: "alterx -list {file} -silent -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Permutation with Extensions",  
                    cmd_template: "alterx -d {domain} -silent -o {output_file} -ext txt,html,php", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Aggressive Permutation Scan",  
                    cmd_template: "alterx -d {domain} -silent -enrich -o {output_file} -fuzz", 
                    inputs: &[InputKind::Domain], 
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── WhatWeb (Web Fingerprinting) ──────────────────────────────────
        Tool {
            name: "WhatWeb",
            binary: "whatweb",
            modes: &[
                Mode { 
                    name: "Aggressive Web Fingerprint",  
                    cmd_template: "whatweb -a 3 {url} -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Quiet Fingerprint Mode",      
                    cmd_template: "whatweb -q {url} -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Enumerate All Plugins",       
                    cmd_template: "whatweb -a 3 --info-plugins {url} -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export Results",         
                    cmd_template: "whatweb -a 3 --log-json={output_file} {url}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk URL File Scan",          
                    cmd_template: "whatweb -i {file} -a 3 -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Verbose Output Mode",         
                    cmd_template: "whatweb -a 3 -v {url} -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Custom Headers Scan",         
                    cmd_template: "whatweb -a 3 --header 'User-Agent: Mozilla/5.0' {url} -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "XML Export Results",          
                    cmd_template: "whatweb -a 3 --log-xml={output_file} {url}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Xml,   
                    file_ext: "xml" 
                },
            ],
        },
        // ── Wappalyzer (Technology Detection) ─────────────────────────────
        Tool {
            name: "Wappalyzer",
            binary: "wappalyzer-cli",
            modes: &[
                Mode { 
                    name: "Technology Fingerprint",      
                    cmd_template: "wappalyzer-cli {url} > {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Batch URL File Scan",         
                    cmd_template: "wappalyzer-cli -i {file} > {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "JSON Technology Output",      
                    cmd_template: "wappalyzer-cli --json {url} > {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Export CSV Results",          
                    cmd_template: "wappalyzer-cli --csv {url} > {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Csv,   
                    file_ext: "csv" 
                },
                Mode { 
                    name: "Pretty Print Output",         
                    cmd_template: "wappalyzer-cli --pretty {url} > {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk Domain Tech Discovery",  
                    cmd_template: "wappalyzer-cli -i {file} --json > {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Custom User Agent Scan",      
                    cmd_template: "wappalyzer-cli --user-agent 'Mozilla/5.0' {url} > {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "TLS/SSL Tech Detection",      
                    cmd_template: "wappalyzer-cli --tls {url} > {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
            ],
        },
        // ── Masscan (Mass Port Scanner) ──────────────────────────────────
        Tool {
            name: "Masscan",
            binary: "masscan",
            modes: &[
                Mode { 
                    name: "Fast Full TCP Scan",          
                    cmd_template: "masscan {ip} -p1-65535 --rate 10000 -oJ {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Top 1000 Ports Only",         
                    cmd_template: "masscan {ip} --top-ports 1000 --rate 10000 -oJ {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Banner Grab Mode",            
                    cmd_template: "masscan {ip} -p80,443,8080,8443 --banners --rate 1000 -oJ {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "XML Output Export",           
                    cmd_template: "masscan {ip} -p1-1000 --rate 10000 -oX {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Xml,   
                    file_ext: "xml" 
                },
                Mode { 
                    name: "CIDR Range Scan",             
                    cmd_template: "masscan {ip}/24 -p1-65535 --rate 10000 -oJ {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "CSV Output Export",           
                    cmd_template: "masscan {ip} -p1-1000 --rate 10000 -oJ {output_file} | jq -r '.[] | [.ip,.port] | @csv' > {output_file}.csv", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Csv,   
                    file_ext: "csv" 
                },
                Mode { 
                    name: "Exclude Ports Scan",          
                    cmd_template: "masscan {ip} -p1-65535 --rate 10000 --exclude 9100,9200 -oJ {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk IP Masscan",             
                    cmd_template: "masscan -iL {file} -p1-1000 --rate 10000 -oJ {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
            ],
        },
        // ── RustScan (Fast Port Scanner) ──────────────────────────────────
        Tool {
            name: "RustScan",
            binary: "rustscan",
            modes: &[
                Mode { 
                    name: "Fast Port Scan",              
                    cmd_template: "rustscan -a {ip} -- -sV -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "Script Scan (Default)",       
                    cmd_template: "rustscan -a {ip} -- -sC -sV -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "Service Version Detection",   
                    cmd_template: "rustscan -a {ip} -p1-1000 -- -sV -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "Batch Mode Scan",             
                    cmd_template: "rustscan -a {ip} -b 500 -- -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "JSON Output Export",          
                    cmd_template: "rustscan -a {ip} --json -o {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Custom Port Range Scan",      
                    cmd_template: "rustscan -a {ip} -p 80,443,8080,8443 -- -sV -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "UDP Scan Mode",               
                    cmd_template: "rustscan -a {ip} -u -- -sU -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
                Mode { 
                    name: "Aggressive Scan Mode",        
                    cmd_template: "rustscan -a {ip} -A -- -sV -sC -oN {output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Nmap,  
                    file_ext: "nmap" 
                },
            ],
        },
        // ── ZGrab2 (Banner Grabbing) ──────────────────────────────────────
        Tool {
            name: "ZGrab2",
            binary: "zgrab2",
            modes: &[
                Mode { 
                    name: "HTTP Banner Grab",            
                    cmd_template: "zgrab2 http --target {ip}:80 --output-file={output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "HTTPS Banner Grab",           
                    cmd_template: "zgrab2 http --target {ip}:443 --use-https --output-file={output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "SSH Banner Grab",             
                    cmd_template: "zgrab2 ssh --target {ip}:22 --output-file={output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "SMTP Banner Grab",            
                    cmd_template: "zgrab2 smtp --target {ip}:25 --output-file={output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "FTP Banner Grab",             
                    cmd_template: "zgrab2 ftp --target {ip}:21 --output-file={output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Custom Module Grab",          
                    cmd_template: "zgrab2 {value} --target {ip} --output-file={output_file}", 
                    inputs: &[InputKind::FreeText, InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk IP Banner Grab",         
                    cmd_template: "zgrab2 http --input-file={file} --output-file={output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "TLS Banner Grab",             
                    cmd_template: "zgrab2 tls --target {ip}:443 --output-file={output_file}", 
                    inputs: &[InputKind::Ip],     
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
            ],
        },
    ],
};