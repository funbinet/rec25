# REC#25 Code Update, Build, and Release Guide

This document lists all the steps and commands required to rebuild the project, update the local installation on your system, and publish new releases.

---

## 1. Verifying and Rebuilding Local Changes

Whenever you modify the REC#25 codebase:

### Check compilation
Ensure that the code is free of syntax or compile errors:
```bash
cargo check
```

### Build for Release
Compile an optimized production binary:
```bash
cargo build --release
```

### Run Dev Build (No Installation)
Run the freshly compiled version directly from the project directory without overriding the system installation:
```bash
cargo run
```

---

## 2. Updating the Local System Installation

To update the system-wide tool `/usr/bin/rec25` with your latest changes:

Run the automated installer script:
```bash
sudo ./install.sh
```

### What `install.sh` does:
1. Verifies that the Rust cargo toolchain is in your path.
2. Builds the project in release mode as the invoking user (preventing root ownership issues in `target/`).
3. Installs the optimized binary to `/usr/bin/rec25` and makes it executable.
4. Initializes the work directories (`/opt/rec25/output`, `/opt/rec25/logs`, `/opt/rec25/config`) and sets ownership to the regular user.

---

## 3. Pushing Changes and Publishing Releases

We have created an automated release script [release.py](file:///home/boule/REC%2325/release.py) in the root of the project to manage Git commits, tagging, pushing, and publishing release pages to both Codeberg and GitHub.

### Step-by-step release process:

#### Step 1: Open [release.py](file:///home/boule/REC%2325/release.py) and update variables
Open the script and edit the `VERSION` and `RELEASE_NOTES` constants at the top:
```python
VERSION = "v0.6.1"

RELEASE_NOTES = """## Release v0.6.1

### Sizing, Alignment, and Border Refinements
- **Adaptive Sizing**: The output preview box now dynamically fits the exact terminal window size.
...
"""
```

#### Step 2: Execute the release script
Run the script from the project root:
```bash
./release.py
```

### What `release.py` automates for you:
1. **Version Updates**: Finds and updates the version in `Cargo.toml` and the subtitle version banner in `src/ui/theme.rs`.
2. **Changelog Maintenance**: Prepends the new release notes to [RELEASE_NOTES.md](file:///home/boule/REC%2325/RELEASE_NOTES.md) (removing any duplicates for the same version if you are re-running it).
3. **Commit & Tag**: Runs `git add .`, commits the release, and tags the commit with the specified version.
4. **Push to Remotes**: Force-pushes the branch and tag to both configured remotes (`codeberg` and `github`).
5. **Programmatic Releases**: Extracts the access tokens directly from your Git remote configurations and sends REST API calls to both GitHub and Codeberg to publish official release pages featuring your release notes.
