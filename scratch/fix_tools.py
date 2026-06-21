import os
import re

def process_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    # HTTPX
    # Replace -l {file} with -u {url} and InputKind::File with InputKind::Url for modes that don't have "Bulk"
    def repl_httpx(m):
        mode_str = m.group(0)
        if "Bulk" not in mode_str:
            mode_str = mode_str.replace("-l {file}", "-u {url}")
            mode_str = mode_str.replace("InputKind::File", "InputKind::Url")
        return mode_str
    
    # We find blocks starting with `Mode {` and ending with `},` that have `httpx` inside.
    # Actually, simpler: replace line by line or use regex
    # It's safer to just iterate through modes.
    
    # Let's just fix specific tools manually via regex.
    # httpx and httpx-pd:
    content = re.sub(r'Mode\s*\{[^}]*httpx-pd[^}]*\}', repl_httpx, content)
    content = re.sub(r'Mode\s*\{[^}]*httpx\s+-l[^}]*\}', repl_httpx, content)
    
    # dnsx: change -l {file} to -d {domain} and InputKind::File to InputKind::Domain
    def repl_dnsx(m):
        mode_str = m.group(0)
        if "Bulk" not in mode_str:
            mode_str = mode_str.replace("-l {file}", "-d {domain}")
            mode_str = mode_str.replace("InputKind::File", "InputKind::Domain")
        return mode_str
    content = re.sub(r'Mode\s*\{[^}]*dnsx[^}]*\}', repl_dnsx, content)
    
    # naabu: change -list {file} to -host {ip} and InputKind::File to InputKind::Ip
    def repl_naabu(m):
        mode_str = m.group(0)
        if "Bulk" not in mode_str:
            mode_str = mode_str.replace("-list {file}", "-host {ip}")
            mode_str = mode_str.replace("InputKind::File", "InputKind::Ip")
        return mode_str
    content = re.sub(r'Mode\s*\{[^}]*naabu[^}]*\}', repl_naabu, content)

    # tlsx: change -l {file} to -u {domain} and InputKind::File to InputKind::Domain
    def repl_tlsx(m):
        mode_str = m.group(0)
        if "Bulk" not in mode_str:
            mode_str = mode_str.replace("-l {file}", "-u {domain}")
            mode_str = mode_str.replace("InputKind::File", "InputKind::Domain")
        return mode_str
    content = re.sub(r'Mode\s*\{[^}]*tlsx[^}]*\}', repl_tlsx, content)

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Updated {filepath}")

if __name__ == "__main__":
    for root, dirs, files in os.walk("/home/boule/REC#25/src/tools"):
        for file in files:
            if file.endswith(".rs"):
                process_file(os.path.join(root, file))
