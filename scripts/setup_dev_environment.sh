#!/bin/bash
# Comprehensive Development Environment Setup for Shaman's Journey
# Sets up everything needed to run the game locally on Ubuntu/Linux (no Docker required)
# Supports: Ubuntu 20.04+, Debian 11+, Fedora 35+, Arch Linux
#
# Usage: ./setup_dev_environment.sh [-y|--yes]
#   -y, --yes    Non-interactive mode, auto-confirm all prompts

set -e  # Exit on error

# Parse command line arguments
AUTO_CONFIRM=false
while [[ $# -gt 0 ]]; do
    case $1 in
        -y|--yes)
            AUTO_CONFIRM=true
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Banner
echo -e "${GREEN}"
cat << "EOF"
╔═══════════════════════════════════════════════════════════╗
║     Shaman's Journey - Development Environment Setup     ║
║                                                           ║
║  This script will install:                               ║
║  • System dependencies (ALSA, udev, X11, OpenGL)         ║
║  • Rust toolchain (latest stable)                        ║
║  • DragonflyDB (Redis-compatible cache)                  ║
║  • RabbitMQ (Message queue)                              ║
║  • Game assets directories                               ║
║  • Environment configuration                             ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    log_error "Please do not run this script as root. Use your regular user account."
    log_info "The script will ask for sudo password when needed."
    exit 1
fi

# Detect OS and package manager
detect_system() {
    log_info "Detecting system..."

    if [ -f /etc/os-release ]; then
        . /etc/os-release
        OS=$NAME
        VERSION=$VERSION_ID
        log_success "Detected: $OS $VERSION"
    else
        log_error "Cannot detect OS. /etc/os-release not found."
        exit 1
    fi

    if command -v apt-get &> /dev/null; then
        PKG_MANAGER="apt"
    elif command -v dnf &> /dev/null; then
        PKG_MANAGER="dnf"
    elif command -v pacman &> /dev/null; then
        PKG_MANAGER="pacman"
    elif command -v zypper &> /dev/null; then
        PKG_MANAGER="zypper"
    else
        log_error "Could not detect package manager"
        exit 1
    fi

    log_success "Package manager: $PKG_MANAGER"
}

# Install system dependencies
install_system_deps() {
    log_info "Installing system dependencies..."

    case $PKG_MANAGER in
        apt)
            sudo apt-get update -qq
            sudo apt-get install -y \
                curl wget gnupg2 ca-certificates lsb-release \
                build-essential pkg-config \
                libasound2-dev libudev-dev \
                libx11-dev libxi-dev libgl1-mesa-dev libglu1-mesa-dev \
                libxcursor-dev libxinerama-dev libxrandr-dev \
                git vim tmux htop \
                redis-tools || true  # redis-cli for testing
            ;;
        dnf)
            sudo dnf install -y \
                curl wget gnupg2 ca-certificates \
                gcc gcc-c++ pkgconfig \
                alsa-lib-devel systemd-devel \
                libX11-devel libXi-devel mesa-libGL-devel mesa-libGLU-devel \
                libXcursor-devel libXinerama-devel libXrandr-devel \
                git vim tmux htop \
                redis || true
            ;;
        pacman)
            sudo pacman -Syu --noconfirm \
                curl wget gnupg ca-certificates \
                base-devel pkgconf \
                alsa-lib systemd \
                libx11 libxi mesa \
                libxcursor libxinerama libxrandr \
                git vim tmux htop \
                redis || true
            ;;
        zypper)
            sudo zypper install -y \
                curl wget gpg2 ca-certificates \
                gcc gcc-c++ pkg-config \
                alsa-devel systemd-devel \
                libX11-devel libXi-devel Mesa-libGL-devel \
                libXcursor-devel libXinerama-devel libXrandr-devel \
                git vim tmux htop \
                redis || true
            ;;
    esac

    log_success "System dependencies installed"
}

# Install Rust
install_rust() {
    if command -v cargo &> /dev/null; then
        log_success "Rust is already installed: $(rustc --version)"
        return 0
    fi

    log_info "Installing Rust toolchain..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable

    # Source cargo env
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi

    log_success "Rust installed: $(rustc --version)"
}

# Install DragonflyDB
install_dragonfly() {
    log_info "Checking for DragonflyDB..."

    if command -v dragonfly &> /dev/null; then
        log_success "DragonflyDB is already installed"
        return 0
    fi

    log_info "Installing DragonflyDB..."

    # Detect architecture
    ARCH=$(uname -m)
    if [ "$ARCH" != "x86_64" ] && [ "$ARCH" != "aarch64" ]; then
        log_warning "DragonflyDB binary not available for $ARCH. Skipping..."
        log_info "You can run it via Docker: docker run -d -p 6379:6379 docker.dragonflydb.io/dragonflydb/dragonfly"
        return 0
    fi

    # Download and install DragonflyDB
    DRAGONFLY_VERSION="v1.14.0"
    DRAGONFLY_URL="https://dragonflydb.gateway.scarf.sh/$DRAGONFLY_VERSION/dragonfly-$ARCH.tar.gz"

    log_info "Downloading DragonflyDB $DRAGONFLY_VERSION..."
    cd /tmp
    wget -q "$DRAGONFLY_URL" -O dragonfly.tar.gz || {
        log_warning "Failed to download DragonflyDB. You can install via Docker instead."
        return 0
    }

    tar -xzf dragonfly.tar.gz
    sudo mv dragonfly-$ARCH /usr/local/bin/dragonfly
    sudo chmod +x /usr/local/bin/dragonfly
    rm -f dragonfly.tar.gz

    log_success "DragonflyDB installed: $(dragonfly --version 2>&1 | head -n1 || echo 'installed')"
}

# Install RabbitMQ
install_rabbitmq() {
    log_info "Checking for RabbitMQ..."

    if command -v rabbitmq-server &> /dev/null; then
        log_success "RabbitMQ is already installed"
        return 0
    fi

    log_info "Installing RabbitMQ..."

    case $PKG_MANAGER in
        apt)
            # Install Erlang first
            sudo apt-get install -y erlang-base erlang-asn1 erlang-crypto erlang-eldap \
                erlang-inets erlang-mnesia erlang-os-mon erlang-parsetools \
                erlang-public-key erlang-runtime-tools erlang-snmp erlang-ssl \
                erlang-syntax-tools erlang-tftp erlang-tools erlang-xmerl

            # Install RabbitMQ
            curl -fsSL https://github.com/rabbitmq/signing-keys/releases/download/2.0/rabbitmq-release-signing-key.asc | sudo apt-key add -
            sudo tee /etc/apt/sources.list.d/rabbitmq.list <<EOF
deb https://dl.bintray.com/rabbitmq-erlang/debian $(lsb_release -sc) erlang
deb https://dl.bintray.com/rabbitmq/debian $(lsb_release -sc) main
EOF
            sudo apt-get update -qq
            sudo apt-get install -y rabbitmq-server
            ;;
        dnf)
            sudo dnf install -y erlang rabbitmq-server
            ;;
        pacman)
            sudo pacman -S --noconfirm erlang rabbitmq
            ;;
        zypper)
            sudo zypper install -y erlang rabbitmq-server
            ;;
    esac

    log_success "RabbitMQ installed"
}

# Setup systemd services
setup_services() {
    log_info "Setting up systemd services..."

    # DragonflyDB service
    if command -v dragonfly &> /dev/null; then
        sudo tee /etc/systemd/system/dragonfly.service > /dev/null <<EOF
[Unit]
Description=DragonflyDB Server
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME
ExecStart=/usr/local/bin/dragonfly --logtostderr --port 6379 --maxmemory 512mb --dbfilename dragonfly.db --dir /var/lib/dragonfly
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
EOF

        sudo mkdir -p /var/lib/dragonfly
        sudo chown $USER:$USER /var/lib/dragonfly

        sudo systemctl daemon-reload
        sudo systemctl enable dragonfly
        sudo systemctl restart dragonfly

        log_success "DragonflyDB service configured and started"
    fi

    # Enable RabbitMQ
    if command -v rabbitmq-server &> /dev/null; then
        sudo systemctl enable rabbitmq-server
        sudo systemctl restart rabbitmq-server

        # Enable management plugin
        sleep 3
        sudo rabbitmq-plugins enable rabbitmq_management || true

        log_success "RabbitMQ service configured and started"
    fi
}

# Create directory structure
create_directories() {
    log_info "Creating directory structure..."

    cd "$(dirname "$0")/.."  # Go to project root

    mkdir -p assets/{audio,sprites,fonts,portraits/{characters,npcs,bosses}}
    mkdir -p saves
    mkdir -p logs
    mkdir -p data/cache

    log_success "Directory structure created"
}

# Generate .env file
generate_env_file() {
    log_info "Generating .env configuration file..."

    cd "$(dirname "$0")/.."  # Go to project root

    if [ -f .env ]; then
        log_warning ".env file already exists. Creating .env.new instead."
        ENV_FILE=".env.new"
    else
        ENV_FILE=".env"
    fi

    # Generate random passwords
    DRAGONFLY_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-25)
    GRAFANA_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-25)

    cat > "$ENV_FILE" <<EOF
# Shaman's Journey - Environment Configuration
# Generated on $(date)

# ============================================
# Environment
# ============================================
ENVIRONMENT=development

# ============================================
# Logging Configuration
# ============================================
RUST_LOG=info
RUST_BACKTRACE=1

# ============================================
# Sentry Configuration (Error Tracking)
# ============================================
# Get your DSN from https://sentry.io/settings/projects/
# Format: https://<key>@o<org-id>.ingest.sentry.io/<project-id>
SENTRY_DSN=

# ============================================
# Monitoring Configuration
# ============================================
PROMETHEUS_PORT=9091

# ============================================
# Grafana Configuration
# ============================================
GRAFANA_ADMIN_USER=admin
GRAFANA_ADMIN_PASSWORD=$GRAFANA_PASSWORD

# ============================================
# DragonflyDB Configuration
# ============================================
DRAGONFLY_URL=redis://:$DRAGONFLY_PASSWORD@localhost:6379
DRAGONFLY_PASSWORD=$DRAGONFLY_PASSWORD

# Cache TTL settings (in seconds)
LLM_CACHE_TTL=3600          # 1 hour
DUNGEON_CACHE_TTL=86400     # 24 hours
SESSION_CACHE_TTL=1800      # 30 minutes

# ============================================
# RabbitMQ Configuration
# ============================================
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f
RABBITMQ_USER=guest
RABBITMQ_PASSWORD=guest

# ============================================
# Game Configuration
# ============================================
ASSETS_PATH=./assets
SAVES_PATH=./saves

# ============================================
# LLM Configuration (Optional)
# ============================================
# Path to LLM model file (for dialogue generation)
# LLM_MODEL_PATH=/path/to/model.gguf
# LLM_CONTEXT_SIZE=2048

# ============================================
# Development Tools
# ============================================
DEBUG_UI=false
ENABLE_PROFILING=false
EOF

    log_success "Environment file created: $ENV_FILE"

    if [ "$ENV_FILE" = ".env.new" ]; then
        log_warning "Please review .env.new and merge with your existing .env file"
    else
        log_info "DragonflyDB password: $DRAGONFLY_PASSWORD"
        log_info "Grafana password: $GRAFANA_PASSWORD"
        log_warning "Please save these passwords securely!"
    fi
}

# Verify installation
verify_installation() {
    log_info "Verifying installation..."

    local all_ok=true

    # Check Rust
    if command -v cargo &> /dev/null; then
        log_success "✓ Rust: $(rustc --version)"
    else
        log_error "✗ Rust not found"
        all_ok=false
    fi

    # Check DragonflyDB
    if systemctl is-active --quiet dragonfly 2>/dev/null; then
        log_success "✓ DragonflyDB: running on port 6379"
    elif command -v dragonfly &> /dev/null; then
        log_warning "⚠ DragonflyDB: installed but not running"
    else
        log_warning "⚠ DragonflyDB: not installed (optional)"
    fi

    # Check RabbitMQ
    if systemctl is-active --quiet rabbitmq-server 2>/dev/null; then
        log_success "✓ RabbitMQ: running on port 5672"
        log_info "  Management UI: http://localhost:15672 (guest/guest)"
    elif command -v rabbitmq-server &> /dev/null; then
        log_warning "⚠ RabbitMQ: installed but not running"
    else
        log_warning "⚠ RabbitMQ: not installed (optional)"
    fi

    # Test DragonflyDB connection
    if command -v redis-cli &> /dev/null && systemctl is-active --quiet dragonfly 2>/dev/null; then
        if redis-cli -p 6379 ping &>/dev/null; then
            log_success "✓ DragonflyDB connection: OK"
        else
            log_warning "⚠ DragonflyDB connection: failed (may need password)"
        fi
    fi

    # Check project structure
    cd "$(dirname "$0")/.."
    if [ -f Cargo.toml ]; then
        log_success "✓ Project structure: OK"
    else
        log_error "✗ Project structure: Cargo.toml not found"
        all_ok=false
    fi

    if $all_ok; then
        log_success "Verification complete!"
    else
        log_warning "Verification complete with warnings"
    fi
}

# Build the project
build_project() {
    log_info "Building the project..."

    cd "$(dirname "$0")/.."

    # Load environment
    if [ -f .env ]; then
        set -a
        source .env
        set +a
    fi

    # Source cargo env
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi

    log_info "Running cargo check..."
    cargo check --workspace || {
        log_warning "cargo check failed. This is expected if you haven't resolved all dependencies."
        return 0
    }

    log_success "Build check complete"
}

# Print next steps
print_next_steps() {
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                    Setup Complete! 🎉                     ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}Next Steps:${NC}"
    echo ""
    echo "1. Review your .env file and update any required values"
    echo "   ${YELLOW}nano .env${NC}"
    echo ""
    echo "2. Build the game:"
    echo "   ${YELLOW}cargo build --release${NC}"
    echo ""
    echo "3. Run the game:"
    echo "   ${YELLOW}cargo run --release${NC}"
    echo ""
    echo "4. Run tests (excluding audio):"
    echo "   ${YELLOW}cargo test --workspace --exclude bevy_shaman_audio${NC}"
    echo ""
    echo -e "${BLUE}Service Management:${NC}"
    echo ""
    echo "• Check DragonflyDB status:"
    echo "  ${YELLOW}systemctl status dragonfly${NC}"
    echo ""
    echo "• Check RabbitMQ status:"
    echo "  ${YELLOW}systemctl status rabbitmq-server${NC}"
    echo ""
    echo "• RabbitMQ Management UI:"
    echo "  ${YELLOW}http://localhost:15672${NC} (guest/guest)"
    echo ""
    echo "• Test DragonflyDB connection:"
    echo "  ${YELLOW}redis-cli -p 6379 ping${NC}"
    echo ""
    echo -e "${BLUE}Helpful Scripts:${NC}"
    echo ""
    echo "• Start all services:"
    echo "  ${YELLOW}./scripts/start_services.sh${NC}"
    echo ""
    echo "• Stop all services:"
    echo "  ${YELLOW}./scripts/stop_services.sh${NC}"
    echo ""
    echo "• Health check:"
    echo "  ${YELLOW}./scripts/health_check.sh${NC}"
    echo ""
    echo -e "${GREEN}Happy coding! 🎮${NC}"
    echo ""
}

# Main installation flow
main() {
    echo ""
    log_info "Starting installation..."
    echo ""

    detect_system
    echo ""

    log_info "This will install system packages and configure services."

    if [ "$AUTO_CONFIRM" = true ]; then
        log_info "Auto-confirming (running with -y flag)..."
        REPLY="y"
    else
        read -p "Continue? (y/n) " -n 1 -r
        echo ""
    fi

    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_warning "Installation cancelled."
        exit 0
    fi

    echo ""
    install_system_deps
    echo ""
    install_rust
    echo ""
    install_dragonfly
    echo ""
    install_rabbitmq
    echo ""
    setup_services
    echo ""
    create_directories
    echo ""
    generate_env_file
    echo ""
    verify_installation
    echo ""
    build_project
    echo ""
    print_next_steps
}

# Run main function
main
