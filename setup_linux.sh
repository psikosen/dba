#!/bin/bash
# Linux Build Setup Script for Shaman's Journey
# This script installs all necessary dependencies and enables audio support

set -e  # Exit on error

echo "================================================"
echo "Shaman's Journey - Linux Build Setup"
echo "================================================"
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect package manager
if command -v apt-get &> /dev/null; then
    PKG_MANAGER="apt"
elif command -v dnf &> /dev/null; then
    PKG_MANAGER="dnf"
elif command -v pacman &> /dev/null; then
    PKG_MANAGER="pacman"
elif command -v zypper &> /dev/null; then
    PKG_MANAGER="zypper"
else
    echo -e "${RED}Error: Could not detect package manager${NC}"
    echo "Please install dependencies manually:"
    echo "  - ALSA development libraries (libasound2-dev or alsa-lib-devel)"
    echo "  - udev development libraries (libudev-dev or systemd-devel)"
    echo "  - pkg-config"
    exit 1
fi

echo -e "${GREEN}Detected package manager: $PKG_MANAGER${NC}"
echo ""

# Function to install packages based on package manager
install_deps() {
    echo -e "${YELLOW}Installing build dependencies...${NC}"

    case $PKG_MANAGER in
        apt)
            echo "Using apt (Debian/Ubuntu)..."
            sudo apt-get update
            sudo apt-get install -y \
                libasound2-dev \
                libudev-dev \
                pkg-config \
                build-essential \
                libx11-dev \
                libxi-dev \
                libgl1-mesa-dev \
                libglu1-mesa-dev \
                libxcursor-dev \
                libxinerama-dev \
                libxrandr-dev
            ;;
        dnf)
            echo "Using dnf (Fedora/RHEL)..."
            sudo dnf install -y \
                alsa-lib-devel \
                systemd-devel \
                pkgconfig \
                gcc \
                gcc-c++ \
                libX11-devel \
                libXi-devel \
                mesa-libGL-devel \
                mesa-libGLU-devel \
                libXcursor-devel \
                libXinerama-devel \
                libXrandr-devel
            ;;
        pacman)
            echo "Using pacman (Arch Linux)..."
            sudo pacman -Syu --noconfirm \
                alsa-lib \
                systemd \
                pkgconf \
                base-devel \
                libx11 \
                libxi \
                mesa \
                libxcursor \
                libxinerama \
                libxrandr
            ;;
        zypper)
            echo "Using zypper (openSUSE)..."
            sudo zypper install -y \
                alsa-devel \
                systemd-devel \
                pkg-config \
                gcc \
                gcc-c++ \
                libX11-devel \
                libXi-devel \
                Mesa-libGL-devel \
                libXcursor-devel \
                libXinerama-devel \
                libXrandr-devel
            ;;
    esac

    echo -e "${GREEN}✓ Dependencies installed successfully${NC}"
}

# Install Rust if not already installed
install_rust() {
    if ! command -v cargo &> /dev/null; then
        echo -e "${YELLOW}Rust not found. Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
        echo -e "${GREEN}✓ Rust installed successfully${NC}"
    else
        echo -e "${GREEN}✓ Rust is already installed${NC}"
        rustc --version
    fi
}

# Enable audio plugin in main.rs
enable_audio() {
    echo -e "${YELLOW}Enabling AudioPlugin in main.rs...${NC}"

    MAIN_RS="crates/bevy_shaman/src/main.rs"

    if [ -f "$MAIN_RS" ]; then
        # Check if AudioPlugin is already uncommented
        if grep -q "^[[:space:]]*app\.add_plugins(bevy_shaman_audio::AudioPlugin)" "$MAIN_RS"; then
            echo -e "${GREEN}✓ AudioPlugin is already enabled${NC}"
        else
            # Uncomment the AudioPlugin line
            sed -i 's|^[[:space:]]*// app\.add_plugins(bevy_shaman_audio::AudioPlugin);|    app.add_plugins(bevy_shaman_audio::AudioPlugin);|' "$MAIN_RS"
            echo -e "${GREEN}✓ AudioPlugin enabled in main.rs${NC}"
        fi
    else
        echo -e "${RED}Error: Could not find $MAIN_RS${NC}"
        exit 1
    fi
}

# Create assets directory structure
setup_assets() {
    echo -e "${YELLOW}Setting up assets directory...${NC}"

    mkdir -p assets/audio
    mkdir -p assets/sprites

    echo -e "${GREEN}✓ Assets directories created${NC}"
    echo ""
    echo -e "${YELLOW}NOTE: Audio files not included!${NC}"
    echo "To add music, place .ogg files in assets/audio/ with these names:"
    echo "  - dawn_hymn.ogg (90 BPM - Calm)"
    echo "  - war_chant.ogg (140 BPM - Aggressive)"
    echo "  - purification_rite.ogg (80 BPM - Purifying)"
    echo ""
    echo "Sound effects (optional):"
    echo "  - hit.ogg"
    echo "  - menu_click.ogg"
    echo "  - typewriter_beep.ogg"
    echo "  - level_up.ogg"
    echo "  - quest_complete.ogg"
}

# Main installation flow
main() {
    echo "This script will install the following:"
    echo "  1. Build dependencies (ALSA, udev, pkg-config, etc.)"
    echo "  2. Rust toolchain (if not installed)"
    echo "  3. Enable AudioPlugin in the game"
    echo "  4. Create assets directory structure"
    echo ""
    read -p "Continue? (y/n) " -n 1 -r
    echo ""

    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Setup cancelled."
        exit 0
    fi

    echo ""
    install_deps
    echo ""
    install_rust
    echo ""
    enable_audio
    echo ""
    setup_assets
    echo ""

    echo -e "${GREEN}================================================${NC}"
    echo -e "${GREEN}Setup complete!${NC}"
    echo -e "${GREEN}================================================${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Add .ogg audio files to assets/audio/ (see above for filenames)"
    echo "  2. Build the game: cargo build --release"
    echo "  3. Run the game: cargo run --release"
    echo ""
    echo "For VPN deployment:"
    echo "  - Copy the entire project directory to your VPN server"
    echo "  - Run this script on the server"
    echo "  - Build and run as shown above"
    echo ""
}

# Run main function
main
