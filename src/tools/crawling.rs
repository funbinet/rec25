//! Crawling category — 9 tools, 7+ modes each.
//! Modern web crawling and attack surface discovery with verified commands.

use super::types::{Category, InputKind, Mode, OutputFormat, Tool};

pub static CRAWLING: Category = Category {
    name: "Crawling",
    tools: &[
        // ── Katana (Modern Web Crawler) ─────────────────────────────────
        Tool {
            name: "Katana",
            binary: "katana",
            modes: &[
                Mode { 
                    name: "Standard Web Crawl",       
                    cmd_template: "katana -u {url} -silent -o {output_file} -d 3 -jc -fx -ef png,jpg,gif,svg,css", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Deep JavaScript Crawl",    
                    cmd_template: "katana -u {url} -silent -o {output_file} -d 5 -jc -fx -ef png,jpg,gif,svg,css -fs fqdn,rdn", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Headless Browser Crawl",   
                    cmd_template: "katana -u {url} -headless -silent -o {output_file} -d 3 -jc -fx", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Subdomain Discovery Crawl", 
                    cmd_template: "katana -u {url} -silent -o {output_file} -d 3 -jc -fx -sf", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Form Extraction Crawl",    
                    cmd_template: "katana -u {url} -silent -o {output_file} -d 3 -form-extraction -fx -ef png,jpg,gif", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export Crawl",        
                    cmd_template: "katana -u {url} -silent -json -o {output_file} -d 3 -jc", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk Domain Crawl",        
                    cmd_template: "katana -list {file} -silent -o {output_file} -d 3 -jc -fx -c 50", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Advanced Scope Crawl",     
                    cmd_template: "katana -u {url} -silent -o {output_file} -d 4 -jc -fx -ef png,jpg,gif -fs rdn -c 100", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Gau (URL Discovery) ──────────────────────────────────────────
        Tool {
            name: "Gau",
            binary: "gau",
            modes: &[
                Mode { 
                    name: "Multi-Provider URL Discovery",      
                    cmd_template: "gau {url} --providers wayback,commoncrawl,otx,urlscan --o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Subdomain URL Collection",        
                    cmd_template: "gau {url} --providers wayback,commoncrawl,otx,urlscan --subs --o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Blacklist Extension Filter",         
                    cmd_template: "gau {url} --blacklist png,jpg,gif,svg,ico,css,js,woff,ttf,mp3,mp4 --o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export with Metadata",          
                    cmd_template: "gau {url} --json --o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Parameter-Only Discovery",           
                    cmd_template: "gau {url} --o {output_file} && grep -E '.*\\?.*=.*' {output_file} > {output_file}.params", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Date-Range URL Fetch",           
                    cmd_template: "gau {url} --from {value} --to {value} --o {output_file}", 
                    inputs: &[InputKind::Url, InputKind::FreeText, InputKind::FreeText],    
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
                Mode { 
                    name: "Threaded High-Speed Crawl",           
                    cmd_template: "gau {url} --threads 10 --o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Waybackurls (Archive URL Extraction) ────────────────────────
        Tool {
            name: "Waybackurls",
            binary: "waybackurls",
            modes: &[
                Mode { 
                    name: "Archive URL Extraction",      
                    cmd_template: "echo {url} | waybackurls | sort -u > {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Advanced Content Filtering",        
                    cmd_template: "echo {url} | waybackurls | grep -Ev '\\.(png|jpg|gif|svg|ico|css|js|woff|ttf|mp3|mp4)$' | sort -u > {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Parameterized URL Discovery",         
                    cmd_template: "echo {url} | waybackurls | grep -E '.*\\?.*=.*' | sort -u > {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Date-Range Archive Extraction",          
                    cmd_template: "echo {url} | waybackurls -from {value} -to {value} | sort -u > {output_file}", 
                    inputs: &[InputKind::Url, InputKind::FreeText, InputKind::FreeText],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Output with Timestamps",           
                    cmd_template: "echo {url} | waybackurls --json > {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk Domain Wayback Scraping",           
                    cmd_template: "cat {file} | waybackurls | sort -u > {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Extension-Specific Filter",           
                    cmd_template: "echo {url} | waybackurls | grep '\\.{value}' | sort -u > {output_file}", 
                    inputs: &[InputKind::Url, InputKind::FreeText],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Subdomain URL Extraction",           
                    cmd_template: "echo {url} | waybackurls -subs | sort -u > {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── XnLinkFinder (Link Discovery) ──────────────────────────────
        Tool {
            name: "XnLinkFinder",
            binary: "xnLinkFinder",
            modes: &[
                Mode { 
                    name: "Extract Page Links",       
                    cmd_template: "xnLinkFinder -i {url} -o {output_file} -sf {domain}", 
                    inputs: &[InputKind::Url, InputKind::Domain],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Recursive Link Crawl",     
                    cmd_template: "xnLinkFinder -i {url} -d 3 -o {output_file} -sf {domain}", 
                    inputs: &[InputKind::Url, InputKind::Domain],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JavaScript Link Extraction", 
                    cmd_template: "xnLinkFinder -i {url} -js -o {output_file} -sf {domain}", 
                    inputs: &[InputKind::Url, InputKind::Domain],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "In-Scope Only Links",      
                    cmd_template: "xnLinkFinder -i {url} -sp {domain} -o {output_file}", 
                    inputs: &[InputKind::Url, InputKind::Domain],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export Links",        
                    cmd_template: "xnLinkFinder -i {url} -o {output_file} -json -sf {domain}", 
                    inputs: &[InputKind::Url, InputKind::Domain],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Bulk URL Link Extraction", 
                    cmd_template: "xnLinkFinder -i {file} -o {output_file}", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Param Extraction Mode",    
                    cmd_template: "xnLinkFinder -i {url} -op -o {output_file} -sf {domain}", 
                    inputs: &[InputKind::Url, InputKind::Domain],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Exclude Extensions Filter", 
                    cmd_template: "xnLinkFinder -i {url} -x png,jpg,gif,css,js -o {output_file} -sf {domain}", 
                    inputs: &[InputKind::Url, InputKind::Domain],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── Arjun (Parameter Discovery) ──────────────────────────────────
        Tool {
            name: "Arjun",
            binary: "arjun",
            modes: &[
                Mode { 
                    name: "GET Parameter Discovery",   
                    cmd_template: "arjun -u {url} -m GET -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "POST Parameter Discovery",  
                    cmd_template: "arjun -u {url} -m POST -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "JSON Body Parameter Fuzz",  
                    cmd_template: "arjun -u {url} -m JSON -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Custom Wordlist Fuzz",     
                    cmd_template: "arjun -u {url} -w {file} -o {output_file}", 
                    inputs: &[InputKind::Url, InputKind::File],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "High-Speed Multi-Thread",  
                    cmd_template: "arjun -u {url} -t 30 -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Include All HTTP Methods",  
                    cmd_template: "arjun -u {url} -m GET,POST,PUT,DELETE -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Custom Headers Injection",  
                    cmd_template: "arjun -u {url} -H 'X-Forwarded-For:127.0.0.1' -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Silent Mode with Timeout",  
                    cmd_template: "arjun -u {url} -s -t 10 -o {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
            ],
        },
        // ── FFUF (Fuzzing) ──────────────────────────────────────────────
        Tool {
            name: "FFUF",
            binary: "ffuf",
            modes: &[
                Mode { 
                    name: "Directory Bruteforce",      
                    cmd_template: "ffuf -w /usr/share/seclists/Discovery/Web-Content/directory-list-2.3-medium.txt -u {url}/FUZZ -mc 200,301,302,403 -o {output_file} -of json", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "VHost Discovery",           
                    cmd_template: "ffuf -w /usr/share/seclists/Discovery/DNS/subdomains-top1million-5000.txt -H 'Host: FUZZ.{domain}' -u {url} -mc 200 -o {output_file} -of json", 
                    inputs: &[InputKind::Domain, InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "GET Parameter Fuzzing",     
                    cmd_template: "ffuf -w /usr/share/seclists/Discovery/Web-Content/burp-parameter-names.txt -u '{url}?FUZZ=test' -mc 200,301,302 -o {output_file} -of json", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "POST Parameter Fuzzing",    
                    cmd_template: "ffuf -w /usr/share/seclists/Discovery/Web-Content/burp-parameter-names.txt -u {url} -X POST -d 'FUZZ=test' -mc 200,301,302 -o {output_file} -of json", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Extension Fuzzing",         
                    cmd_template: "ffuf -w /usr/share/seclists/Discovery/Web-Content/web-extensions.txt -u {url}/indexFUZZ -mc 200,301,302 -o {output_file} -of json", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Recursive Directory Scan",  
                    cmd_template: "ffuf -w /usr/share/seclists/Discovery/Web-Content/directory-list-2.3-medium.txt -u {url}/FUZZ -recursion -recursion-depth 3 -o {output_file} -of json", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Custom Headers Fuzzing",    
                    cmd_template: "ffuf -w /usr/share/seclists/Discovery/Web-Content/directory-list-2.3-medium.txt -u {url}/FUZZ -H 'Authorization: Bearer token' -mc 200 -o {output_file} -of json", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Rate-Limited Safe Scan",    
                    cmd_template: "ffuf -w /usr/share/seclists/Discovery/Web-Content/directory-list-2.3-medium.txt -u {url}/FUZZ -t 50 -p 0.5 -mc 200,301,302 -o {output_file} -of json", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
            ],
        },
        // ── Feroxbuster (Directory Bruteforce) ──────────────────────────
        Tool {
            name: "Feroxbuster",
            binary: "feroxbuster",
            modes: &[
                Mode { 
                    name: "Recursive Directory Bust",  
                    cmd_template: "feroxbuster -u {url} -o {output_file} -n -a", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Filter Status Codes",       
                    cmd_template: "feroxbuster -u {url} --filter-status 404,403 -o {output_file} -n", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Append File Extensions",    
                    cmd_template: "feroxbuster -u {url} -x php,html,js,txt,json,xml -o {output_file} -n", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Custom Wordlist Scan",      
                    cmd_template: "feroxbuster -u {url} -w {file} -o {output_file} -n", 
                    inputs: &[InputKind::Url, InputKind::File],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "High-Threaded Scan",        
                    cmd_template: "feroxbuster -u {url} -t 100 -o {output_file} -n -a", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export Results",       
                    cmd_template: "feroxbuster -u {url} -o {output_file} --json -n", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Proxy-Enabled Scan",        
                    cmd_template: "feroxbuster -u {url} -x php,html -o {output_file} --proxy http://127.0.0.1:8080 -n", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Silent Mode with Depth",    
                    cmd_template: "feroxbuster -u {url} -d 3 -s -o {output_file} -n", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
        // ── TruffleHog (Secret Scanning) ──────────────────────────────────
        Tool {
            name: "TruffleHog",
            binary: "trufflehog",
            modes: &[
                Mode { 
                    name: "Git Repository Secret Scan", 
                    cmd_template: "trufflehog git {url} --json | tee {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "File System Secret Scan",   
                    cmd_template: "trufflehog filesystem {path} --json | tee {output_file}", 
                    inputs: &[InputKind::FreeText],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Entropy-Based Secret Scan", 
                    cmd_template: "trufflehog git {url} --entropy --json | tee {output_file}", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "GitHub Organization Scan",  
                    cmd_template: "trufflehog github --org {value} --json | tee {output_file}", 
                    inputs: &[InputKind::FreeText],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "GitHub Repo with Token",    
                    cmd_template: "trufflehog github --repo {url} --token {value} --json | tee {output_file}", 
                    inputs: &[InputKind::Url, InputKind::FreeText],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "S3 Bucket Secret Scan",     
                    cmd_template: "trufflehog s3 --bucket={value} --json | tee {output_file}", 
                    inputs: &[InputKind::FreeText],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Custom Regex Secret Scan",  
                    cmd_template: "trufflehog git {url} --regex {value} --json | tee {output_file}", 
                    inputs: &[InputKind::Url, InputKind::FreeText],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Docker Image Secret Scan",  
                    cmd_template: "trufflehog docker --image={value} --json | tee {output_file}", 
                    inputs: &[InputKind::FreeText],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
            ],
        },
        // ── Nuclei (Vulnerability Scanning) ──────────────────────────────
        Tool {
            name: "Nuclei",
            binary: "nuclei",
            modes: &[
                Mode { 
                    name: "All Templates Scan",        
                    cmd_template: "nuclei -u {url} -o {output_file} -t ~/nuclei-templates/ -severity low,medium,high,critical", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Technology-Specific Scan",  
                    cmd_template: "nuclei -u {url} -o {output_file} -t ~/nuclei-templates/technologies/", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Critical Vulnerabilities",  
                    cmd_template: "nuclei -u {url} -o {output_file} -severity critical -t ~/nuclei-templates/", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "JSON Export Results",       
                    cmd_template: "nuclei -u {url} -o {output_file} -json -t ~/nuclei-templates/", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Json,  
                    file_ext: "json" 
                },
                Mode { 
                    name: "Custom Template Scan",      
                    cmd_template: "nuclei -u {url} -o {output_file} -t {value} -severity low,medium,high", 
                    inputs: &[InputKind::Url, InputKind::FreeText],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Rate-Limited Safe Scan",    
                    cmd_template: "nuclei -u {url} -o {output_file} -rate-limit 50 -t ~/nuclei-templates/", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Bulk URL File Scan",        
                    cmd_template: "nuclei -l {file} -o {output_file} -t ~/nuclei-templates/ -severity medium,high,critical", 
                    inputs: &[InputKind::File],   
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
                Mode { 
                    name: "Verbose Debug Mode Scan",   
                    cmd_template: "nuclei -u {url} -o {output_file} -v -t ~/nuclei-templates/", 
                    inputs: &[InputKind::Url],    
                    output_format: OutputFormat::Lines, 
                    file_ext: "txt" 
                },
            ],
        },
    ],
};