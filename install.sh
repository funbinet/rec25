#!/usr/bin/env bash
# REC#25 Installer v0.6.0

set -e

echo -e "\033[32m╔═══════════════════════════════════════════════════════════════╗\033[0m"
echo -e "\033[32m║  \033[1;32mREC#25\033[0m \033[37mInstaller  v0.6.0\033[0m                                       \033[32m║\033[0m"
echo -e "\033[32m╚═══════════════════════════════════════════════════════════════╝\033[0m"
echo ""

# 1. Require root or prompt for sudo
if [ "$EUID" -ne 0 ]; then
    echo -e "\033[36m[i]\033[0m Requesting administrative privileges..."
    exec sudo "$0" "$@"
fi

# 2. Find cargo — look in the real user's home even when running as root
find_cargo() {
    # If invoked via sudo, check that user's .cargo/bin first
    if [ -n "$SUDO_USER" ]; then
        local user_home
        user_home=$(getent passwd "$SUDO_USER" | cut -d: -f6)
        local cargo_bin="$user_home/.cargo/bin"
        if [ -f "$cargo_bin/cargo" ]; then
            export PATH="$cargo_bin:$PATH"
            echo -e "\033[32m[OK]\033[0m Found cargo at $cargo_bin"
            return 0
        fi
    fi
    # Fall back to checking current PATH
    if command -v cargo &> /dev/null; then
        echo -e "\033[32m[OK]\033[0m Found cargo in PATH"
        return 0
    fi
    return 1
}

if ! find_cargo; then
    echo -e "\033[31m[x] ERROR: Cargo (Rust) is not installed or not in PATH.\033[0m"
    echo "Please install Rust via rustup:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "Then open a new terminal and run this installer again."
    exit 1
fi

# 3. Compile project as the invoking user (avoids root-owned target/ files)
echo -e "\033[36m[i]\033[0m Compiling REC#25 in release mode..."
if [ -n "$SUDO_USER" ]; then
    USER_HOME=$(getent passwd "$SUDO_USER" | cut -d: -f6)
    sudo -u "$SUDO_USER" env PATH="$USER_HOME/.cargo/bin:$PATH" cargo build --release
else
    cargo build --release
fi

if [ ! -f "target/release/rec25" ]; then
    echo -e "\033[31m[x] ERROR: Build failed — cannot find target/release/rec25.\033[0m"
    exit 1
fi

# 4. Install binary
echo -e "\033[36m[i]\033[0m Installing binary to /usr/bin/rec25..."
cp -f target/release/rec25 /usr/bin/rec25
chmod +x /usr/bin/rec25

# 5. Create workspace directories
echo -e "\033[36m[i]\033[0m Creating workspace at /opt/rec25/..."
mkdir -p /opt/rec25/output /opt/rec25/logs /opt/rec25/config

if [ -n "$SUDO_USER" ]; then
    chown -R "$SUDO_USER:$SUDO_USER" /opt/rec25
fi

echo ""
echo -e "\033[32m[OK] Installation Complete!\033[0m"
echo -e "Run \033[1;32mrec25\033[0m from anywhere to launch the framework."
