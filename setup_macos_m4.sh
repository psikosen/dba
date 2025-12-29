#!/bin/bash
# macOS M4 (Apple Silicon) Setup Script for Shaman's Journey
# Installs dependencies and sets up the development environment
# Skips already installed components

set -e  # Exit on error

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
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

log_skip() {
    echo -e "${CYAN}[SKIP]${NC} $1"
}

# Banner
echo -e "${GREEN}"
cat << "EOF"
╔═══════════════════════════════════════════════════════════╗
║     Shaman's Journey - macOS M4 Setup Script             ║
║                                                           ║
║  Optimized for Apple Silicon M4 chip                     ║
║                                                           ║
║  This script will install:                               ║
║  • Homebrew package manager (if not installed)           ║
║  • Rust toolchain (if not installed)                     ║
║  • System dependencies (SDL2, pkg-config, etc.)          ║
║  • Optional: DragonflyDB and RabbitMQ                    ║
║  • Game assets directories                               ║
║                                                           ║
║  Already installed components will be skipped.           ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"

# Check if running on macOS
check_macos() {
    if [[ "$OSTYPE" != "darwin"* ]]; then
        log_error "This script is for macOS only. Detected OS: $OSTYPE"
        exit 1
    fi

    log_success "Running on macOS"

    # Check for Apple Silicon
    ARCH=$(uname -m)
    if [[ "$ARCH" == "arm64" ]]; then
        log_success "Detected Apple Silicon (M-series chip)"
    else
        log_warning "Not running on Apple Silicon (detected: $ARCH)"
        log_warning "This script is optimized for M4 chip but will continue anyway"
    fi
}

# Install Homebrew
install_homebrew() {
    if command -v brew &> /dev/null; then
        log_skip "Homebrew is already installed: $(brew --version | head -n1)"
        return 0
    fi

    log_info "Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

    # Add Homebrew to PATH for Apple Silicon
    if [[ "$ARCH" == "arm64" ]]; then
        echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
        eval "$(/opt/homebrew/bin/brew shellenv)"
    fi

    log_success "Homebrew installed successfully"
}

# Update Homebrew
update_homebrew() {
    log_info "Updating Homebrew..."
    brew update
    log_success "Homebrew updated"
}

# Install Rust
install_rust() {
    if command -v cargo &> /dev/null; then
        RUST_VERSION=$(rustc --version)
        log_skip "Rust is already installed: $RUST_VERSION"

        # Check if we should update
        log_info "To update Rust, run: rustup update"
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

# Install system dependencies via Homebrew
install_system_deps() {
    log_info "Checking system dependencies..."

    local deps_to_install=()
    local deps=(
        "pkg-config"
        "sdl2"
        "cmake"
        "openssl"
    )

    for dep in "${deps[@]}"; do
        if brew list "$dep" &>/dev/null; then
            log_skip "$dep is already installed"
        else
            deps_to_install+=("$dep")
        fi
    done

    if [ ${#deps_to_install[@]} -eq 0 ]; then
        log_success "All system dependencies are already installed"
        return 0
    fi

    log_info "Installing missing dependencies: ${deps_to_install[*]}"
    brew install "${deps_to_install[@]}"

    log_success "System dependencies installed"
}

# Install optional services
install_optional_services() {
    log_info "Checking optional services (DragonflyDB and RabbitMQ)..."
    echo ""
    echo "These services are optional but provide enhanced functionality:"
    echo "  • DragonflyDB - Redis-compatible cache for dungeon generation"
    echo "  • RabbitMQ - Message queue for async processing"
    echo ""

    read -p "Install optional services? (y/n) " -n 1 -r
    echo ""

    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        log_skip "Skipping optional services"
        return 0
    fi

    # Install DragonflyDB
    if brew list dragonfly &>/dev/null; then
        log_skip "DragonflyDB is already installed"
    else
        log_info "Installing DragonflyDB..."
        brew install dragonfly || {
            log_warning "Failed to install DragonflyDB via Homebrew"
            log_info "You can install it via Docker: docker run -d -p 6379:6379 docker.dragonflydb.io/dragonflydb/dragonfly"
        }
    fi

    # Install RabbitMQ
    if brew list rabbitmq &>/dev/null; then
        log_skip "RabbitMQ is already installed"
    else
        log_info "Installing RabbitMQ..."
        brew install rabbitmq || {
            log_warning "Failed to install RabbitMQ"
        }
    fi

    log_success "Optional services installation complete"
}

# Start services
start_services() {
    log_info "Starting services..."

    # Start DragonflyDB
    if brew list dragonfly &>/dev/null; then
        if brew services list | grep dragonfly | grep started &>/dev/null; then
            log_skip "DragonflyDB is already running"
        else
            log_info "Starting DragonflyDB..."
            brew services start dragonfly
            log_success "DragonflyDB started on port 6379"
        fi
    fi

    # Start RabbitMQ
    if brew list rabbitmq &>/dev/null; then
        if brew services list | grep rabbitmq | grep started &>/dev/null; then
            log_skip "RabbitMQ is already running"
        else
            log_info "Starting RabbitMQ..."
            brew services start rabbitmq
            log_success "RabbitMQ started on port 5672"
            log_info "RabbitMQ Management UI: http://localhost:15672 (guest/guest)"
        fi
    fi
}

# Create directory structure
create_directories() {
    log_info "Creating directory structure..."

    cd "$(dirname "$0")"

    local dirs=(
        "assets/audio"
        "assets/sprites"
        "assets/fonts"
        "assets/portraits/characters"
        "assets/portraits/npcs"
        "assets/portraits/bosses"
        "saves"
        "logs"
        "data/cache"
    )

    for dir in "${dirs[@]}"; do
        if [ -d "$dir" ]; then
            log_skip "Directory already exists: $dir"
        else
            mkdir -p "$dir"
            log_info "Created directory: $dir"
        fi
    done

    log_success "Directory structure ready"
}

# Generate .env file
generate_env_file() {
    cd "$(dirname "$0")"

    if [ -f .env ]; then
        log_skip ".env file already exists"
        return 0
    fi

    log_info "Generating .env configuration file..."

    # Generate random passwords
    DRAGONFLY_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-25)
    GRAFANA_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-25)

    cat > .env <<EOF
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
# Development Tools
# ============================================
DEBUG_UI=false
ENABLE_PROFILING=false
EOF

    log_success "Environment file created: .env"
    log_info "Generated passwords stored in .env file"
}

# Verify Rust installation and cargo
verify_rust() {
    log_info "Verifying Rust installation..."

    # Source cargo env if not in PATH
    if ! command -v cargo &> /dev/null; then
        if [ -f "$HOME/.cargo/env" ]; then
            source "$HOME/.cargo/env"
        fi
    fi

    if command -v cargo &> /dev/null; then
        log_success "✓ Rust: $(rustc --version)"
        log_success "✓ Cargo: $(cargo --version)"
    else
        log_error "✗ Rust/Cargo not found in PATH"
        log_error "Please restart your terminal or run: source ~/.cargo/env"
        return 1
    fi
}

# Run cargo check
run_cargo_check() {
    log_info "Running cargo check to verify dependencies..."

    cd "$(dirname "$0")"

    # Source cargo env
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi

    # Load environment variables
    if [ -f .env ]; then
        set -a
        source .env
        set +a
    fi

    log_info "This may take a few minutes on first run..."

    if cargo check --workspace; then
        log_success "Cargo check passed! All dependencies resolved."
    else
        log_warning "Cargo check completed with warnings (this is normal)"
    fi
}

# Run cargo build
run_cargo_build() {
    log_info "Building the project in release mode..."

    cd "$(dirname "$0")"

    # Source cargo env
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi

    # Load environment variables
    if [ -f .env ]; then
        set -a
        source .env
        set +a
    fi

    log_info "This will take several minutes on first build..."
    echo ""

    if cargo build --release; then
        log_success "Build successful!"
        return 0
    else
        log_error "Build failed. Please check the error messages above."
        return 1
    fi
}

# Run the game
run_game() {
    log_info "Running the game..."

    cd "$(dirname "$0")"

    # Source cargo env
    if [ -f "$HOME/.cargo/env" ]; then
        source "$HOME/.cargo/env"
    fi

    # Load environment variables
    if [ -f .env ]; then
        set -a
        source .env
        set +a
    fi

    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║              Starting Shaman's Journey...                 ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
    echo ""

    cargo run --release
}

# Print summary
print_summary() {
    echo ""
    echo -e "${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║                    Setup Complete! 🎉                     ║${NC}"
    echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}Installation Summary:${NC}"
    echo ""

    # Rust
    if command -v cargo &> /dev/null; then
        echo -e "  ${GREEN}✓${NC} Rust: $(rustc --version)"
    fi

    # Homebrew
    if command -v brew &> /dev/null; then
        echo -e "  ${GREEN}✓${NC} Homebrew: $(brew --version | head -n1)"
    fi

    # DragonflyDB
    if brew list dragonfly &>/dev/null; then
        if brew services list | grep dragonfly | grep started &>/dev/null; then
            echo -e "  ${GREEN}✓${NC} DragonflyDB: Running on port 6379"
        else
            echo -e "  ${YELLOW}⚠${NC} DragonflyDB: Installed but not running"
        fi
    else
        echo -e "  ${CYAN}○${NC} DragonflyDB: Not installed (optional)"
    fi

    # RabbitMQ
    if brew list rabbitmq &>/dev/null; then
        if brew services list | grep rabbitmq | grep started &>/dev/null; then
            echo -e "  ${GREEN}✓${NC} RabbitMQ: Running on port 5672"
            echo -e "      Management UI: http://localhost:15672"
        else
            echo -e "  ${YELLOW}⚠${NC} RabbitMQ: Installed but not running"
        fi
    else
        echo -e "  ${CYAN}○${NC} RabbitMQ: Not installed (optional)"
    fi

    echo ""
    echo -e "${BLUE}Useful Commands:${NC}"
    echo ""
    echo "  • Run the game:"
    echo -e "    ${YELLOW}cargo run --release${NC}"
    echo ""
    echo "  • Run in development mode (faster compile):"
    echo -e "    ${YELLOW}cargo run${NC}"
    echo ""
    echo "  • Run tests (excluding audio):"
    echo -e "    ${YELLOW}cargo test --workspace --exclude bevy_shaman_audio${NC}"
    echo ""
    echo "  • Check code without building:"
    echo -e "    ${YELLOW}cargo check${NC}"
    echo ""
    echo "  • Start services:"
    echo -e "    ${YELLOW}brew services start dragonfly${NC}"
    echo -e "    ${YELLOW}brew services start rabbitmq${NC}"
    echo ""
    echo "  • Stop services:"
    echo -e "    ${YELLOW}brew services stop dragonfly${NC}"
    echo -e "    ${YELLOW}brew services stop rabbitmq${NC}"
    echo ""
    echo -e "${GREEN}Happy gaming! 🎮${NC}"
    echo ""
}

# Main installation flow
main() {
    log_info "Starting macOS M4 setup..."
    echo ""

    check_macos
    echo ""

    install_homebrew
    echo ""

    update_homebrew
    echo ""

    install_rust
    echo ""

    install_system_deps
    echo ""

    install_optional_services
    echo ""

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        start_services
        echo ""
    fi

    create_directories
    echo ""

    generate_env_file
    echo ""

    verify_rust
    echo ""

    run_cargo_check
    echo ""

    print_summary

    # Ask if user wants to build and run now
    echo ""
    read -p "Build and run the game now? (y/n) " -n 1 -r
    echo ""

    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo ""
        run_cargo_build
        echo ""

        if [ $? -eq 0 ]; then
            read -p "Build successful! Run the game? (y/n) " -n 1 -r
            echo ""

            if [[ $REPLY =~ ^[Yy]$ ]]; then
                run_game
            else
                log_info "You can run the game later with: cargo run --release"
            fi
        fi
    else
        log_info "Setup complete! Build the game with: cargo build --release"
        log_info "Then run it with: cargo run --release"
    fi
}

# Run main function
main
