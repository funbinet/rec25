#!/usr/bin/env python3
"""
REC#25 Release Automation Script
--------------------------------
This script automates the process of publishing a new release:
1. Automatically updates the version in Cargo.toml and src/ui/theme.rs.
2. Prepends the new release notes to RELEASE_NOTES.md.
3. Commits and tags the version locally.
4. Force-pushes the code and tags to both GitHub and Codeberg.
5. Programmatically creates/publishes official releases on GitHub and Codeberg
   using the credentials stored in your Git remote URLs.

Edit the VERSION and RELEASE_NOTES constants below before running.
"""

import os
import re
import sys
import json
import subprocess
from urllib.request import Request, urlopen
from urllib.error import HTTPError

# ── CONFIGURATION ───────────────────────────────────────────────────────────

VERSION = "v0.6.1"

RELEASE_NOTES = """## Release v0.6.1

### Sizing, Alignment, and Border Refinements
- **Adaptive Sizing**: The output preview box now dynamically fits the exact terminal window size.
- **Perfect Borders & No Junk**: Integrated a centralized text padding helper that strips ANSI escape codes and expands tabs to spaces. This prevents vertical border `║` breakages and layout shifting when viewing logs.
- **Closed Headers**: Fixed the section header box rendering to close the border with a bottom-left `╚` and bottom-right `╝` corner, eliminating the incomplete junction line `╠═╣`.
"""

# ── HELPERS ─────────────────────────────────────────────────────────────────

def log(msg):
    print(f"[*] {msg}")

def error(msg):
    print(f"[-] ERROR: {msg}", file=sys.stderr)
    sys.exit(1)

def run_cmd(args, allow_fail=False):
    res = subprocess.run(args, capture_output=True, text=True)
    if res.returncode != 0 and not allow_fail:
        error(f"Command failed: {' '.join(args)}\nStdout: {res.stdout}\nStderr: {res.stderr}")
    return res

def get_remote_info(remote_name):
    """Parse username, token, domain, and repo path from a Git remote URL."""
    try:
        url = run_cmd(["git", "remote", "get-url", remote_name]).stdout.strip()
        # Regex to match https://<user>:<token>@<domain>/<owner>/<repo>.git
        match = re.search(r"https://([^:]+):([^@]+)@([^/]+)/([^/]+)/([^.]+)\.git", url)
        if match:
            return {
                "username": match.group(1),
                "token": match.group(2),
                "domain": match.group(3),
                "repo": f"{match.group(4)}/{match.group(5)}"
            }
    except Exception as e:
        log(f"Could not parse remote info for {remote_name}: {e}")
    return None

def make_api_post(url, headers, data):
    """Perform a standard JSON POST request using only built-in urllib."""
    req = Request(url, data=json.dumps(data).encode("utf-8"), headers=headers, method="POST")
    try:
        with urlopen(req) as resp:
            return resp.status, json.loads(resp.read().decode("utf-8"))
    except HTTPError as e:
        body = e.read().decode("utf-8")
        try:
            err_json = json.loads(body)
        except Exception:
            err_json = body
        return e.code, err_json
    except Exception as e:
        return 500, str(e)

# ── MAIN WORKFLOW ───────────────────────────────────────────────────────────

def main():
    if not VERSION.startswith("v"):
        error("VERSION must start with 'v' (e.g., 'v0.6.1')")

    semver = VERSION[1:] # strip 'v'
    log(f"Starting release workflow for version {VERSION} (SemVer: {semver})...")

    # 1. Update Cargo.toml version
    cargo_path = "Cargo.toml"
    if os.path.exists(cargo_path):
        with open(cargo_path, "r") as f:
            content = f.read()
        updated = re.sub(r'(version\s*=\s*")([^"]+)(")', f'\\1{semver}\\3', content, count=1)
        with open(cargo_path, "w") as f:
            f.write(updated)
        log("Updated Cargo.toml version field.")

    # 2. Update version in src/ui/theme.rs
    theme_path = "src/ui/theme.rs"
    if os.path.exists(theme_path):
        with open(theme_path, "r") as f:
            content = f.read()
        updated = re.sub(
            r'(Reconnaissance Framework\s+v)[0-9.]+',
            f'\\1{semver}',
            content
        )
        with open(theme_path, "w") as f:
            f.write(updated)
        log("Updated src/ui/theme.rs version banner.")

    # 3. Prepend to RELEASE_NOTES.md
    notes_path = "RELEASE_NOTES.md"
    new_notes_block = f"# REC#25 {VERSION} — Release Notes\n\n{RELEASE_NOTES.strip()}\n\n---\n\n"
    if os.path.exists(notes_path):
        with open(notes_path, "r") as f:
            old_content = f.read()
        # Clean up any existing notes for this same version to avoid duplication
        version_header_pattern = f"# REC#25 {VERSION} — Release Notes"
        if version_header_pattern in old_content:
            log(f"Overwriting previous release notes block for {VERSION} in RELEASE_NOTES.md.")
            # split at next version or just overwrite
            parts = old_content.split("\n\n---\n\n")
            # Filter out the matching version part
            filtered_parts = [p for p in parts if version_header_pattern not in p and p.strip()]
            old_content = "\n\n---\n\n".join(filtered_parts) + "\n\n---\n\n"
        
        with open(notes_path, "w") as f:
            f.write(new_notes_block + old_content)
    else:
        with open(notes_path, "w") as f:
            f.write(new_notes_block)
    log("Prepend release notes to RELEASE_NOTES.md completed.")

    # 4. Local git commit and tag
    log("Staging and committing files...")
    run_cmd(["git", "add", "."])
    # Commit might return non-zero if there's nothing new to commit, so we check if changes exist
    status = run_cmd(["git", "status", "--porcelain"])
    if status.stdout.strip():
        run_cmd(["git", "commit", "-m", f"Release version {VERSION}"])
        log("Committed release changes.")
    else:
        log("No changes to commit. Proceeding with tagging.")

    log(f"Tagging commit as {VERSION}...")
    run_cmd(["git", "tag", "-d", VERSION], allow_fail=True) # delete if exists locally
    run_cmd(["git", "tag", "-a", VERSION, "-m", f"Release {VERSION}"])

    # 5. Push to Git Remotes
    remotes = ["codeberg", "github"]
    remote_data = {}
    for remote in remotes:
        log(f"Pushing to remote: {remote}...")
        run_cmd(["git", "push", remote, "main", "--force"])
        run_cmd(["git", "push", remote, VERSION, "--force"])
        log(f"Successfully pushed branch and tag to {remote}.")
        
        # Get remote credentials
        info = get_remote_info(remote)
        if info:
            remote_data[remote] = info

    # 6. Call APIs to create Release Pages
    # -- GitHub Release API
    if "github" in remote_data:
        gh = remote_data["github"]
        log(f"Creating release on GitHub: {gh['repo']}...")
        url = f"https://api.github.com/repos/{gh['repo']}/releases"
        headers = {
            "Authorization": f"Bearer {gh['token']}",
            "Accept": "application/vnd.github.v3+json",
            "User-Agent": "Python-Urllib",
            "Content-Type": "application/json"
        }
        data = {
            "tag_name": VERSION,
            "name": f"REC#25 {VERSION}",
            "body": RELEASE_NOTES.strip(),
            "draft": False,
            "prerelease": False
        }
        # First check if release already exists to delete/update it
        check_req = Request(f"{url}/tags/{VERSION}", headers=headers, method="GET")
        release_id = None
        try:
            with urlopen(check_req) as resp:
                resp_data = json.loads(resp.read().decode("utf-8"))
                release_id = resp_data.get("id")
        except Exception:
            pass

        if release_id:
            log(f"GitHub release for tag {VERSION} already exists. Updating it...")
            update_req = Request(f"{url}/{release_id}", data=json.dumps(data).encode("utf-8"), headers=headers, method="PATCH")
            try:
                with urlopen(update_req) as resp:
                    log("GitHub release updated successfully!")
            except Exception as e:
                log(f"Failed to update GitHub release: {e}")
        else:
            code, resp = make_api_post(url, headers, data)
            if code in (201, 200):
                log("GitHub release created successfully!")
            else:
                log(f"Could not create GitHub release (HTTP {code}): {resp}")

    # -- Codeberg Release API
    if "codeberg" in remote_data:
        cb = remote_data["codeberg"]
        log(f"Creating release on Codeberg: {cb['repo']}...")
        url = f"https://codeberg.org/api/v1/repos/{cb['repo']}/releases"
        headers = {
            "Authorization": f"token {cb['token']}",
            "Accept": "application/json",
            "Content-Type": "application/json",
            "User-Agent": "Python-Urllib"
        }
        data = {
            "tag_name": VERSION,
            "target_commitish": "main",
            "name": f"REC#25 {VERSION}",
            "body": RELEASE_NOTES.strip(),
            "draft": False,
            "prerelease": False
        }
        # Check if release exists
        release_id = None
        try:
            with urlopen(Request(url, headers=headers, method="GET")) as resp:
                releases = json.loads(resp.read().decode("utf-8"))
                for r in releases:
                    if r.get("tag_name") == VERSION:
                        release_id = r.get("id")
                        break
        except Exception:
            pass

        if release_id:
            log(f"Codeberg release for tag {VERSION} already exists. Updating it...")
            patch_req = Request(f"{url}/{release_id}", data=json.dumps(data).encode("utf-8"), headers=headers, method="PATCH")
            try:
                with urlopen(patch_req) as resp:
                    log("Codeberg release updated successfully!")
            except Exception as e:
                log(f"Failed to update Codeberg release: {e}")
        else:
            code, resp = make_api_post(url, headers, data)
            if code in (201, 200):
                log("Codeberg release created successfully!")
            else:
                log(f"Could not create Codeberg release (HTTP {code}): {resp}")

    log("Release process completed successfully!")

if __name__ == "__main__":
    main()
