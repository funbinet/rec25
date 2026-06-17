#!/usr/bin/env bash
# REC#25 Installer

set -e

echo -e "\033[32m╔═══════════════════════════════════════════════════════════════╗\033[0m"
echo -e "\033[32m║  \033[1;32mREC#25\033[0m \033[37mInstaller\033[0m                                             \033[32m║\033[0m"
echo -e "\033[32m╚═══════════════════════════════════════════════════════════════╝\033[0m"
echo ""

# 1. Require root or prompt for sudo
if [ "$EUID" -ne 0 ]; then
    echo -e "\033[36mℹ\033[0m Requesting administrative privileges..."
    exec sudo "$0" "$@"
fi

# 2. Check for Cargo/Rust
if ! command -v cargo &> /dev/null; then
    echo -e "\033[31m✘ ERROR: Cargo (Rust) is not installed or not in PATH.\033[0m"
    echo "Please install Rust via rustup (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh) and try again."
    exit 1
fi

echo -e "\033[32m✔\033[0m Found Cargo/Rust."

# 3. Compile project
echo -e "\033[36mℹ\033[0m Compiling REC#25 (release mode)..."
# We run cargo build as the user who invoked sudo (SUDO_USER) if possible,
# otherwise it pollutes target/ with root-owned files.
if [ -n "$SUDO_USER" ]; then
    sudo -u "$SUDO_USER" cargo build --release
else
    cargo build --release
fi

if [ ! -f "target/release/rec25" ]; then
    echo -e "\033[31m✘ ERROR: Build failed. Cannot find target/release/rec25.\033[0m"
    exit 1
fi

# 4. Install binary to /usr/bin/
echo -e "\033[36mℹ\033[0m Installing binary to /usr/bin/rec25..."
cp target/release/rec25 /usr/bin/rec25
chmod +x /usr/bin/rec25

# 5. Create working directories
echo -e "\033[36mℹ\033[0m Creating workspace at /opt/rec25/..."
mkdir -p /opt/rec25/output
mkdir -p /opt/rec25/logs
mkdir -p /opt/rec25/config

# If invoked via sudo, chown the /opt/rec25 directory to the normal user
# so they don't need root to run the tool and write outputs.
if [ -n "$SUDO_USER" ]; then
    chown -R "$SUDO_USER:$SUDO_USER" /opt/rec25
fi

echo ""
echo -e "\033[32m✔ Installation Complete!\033[0m"
echo -e "You can now run \033[1;32mrec25\033[0m from anywhere."
