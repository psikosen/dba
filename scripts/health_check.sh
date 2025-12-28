#!/bin/bash
# Health check script for Shaman's Journey services and dependencies

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[CHECK]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[⚠]${NC} $1"
}

log_error() {
    echo -e "${RED}[✗]${NC} $1"
}

ERRORS=0
WARNINGS=0

echo ""
echo "========================================"
echo " Shaman's Journey - Health Check"
echo "========================================"
echo ""

# Check Rust
log_info "Checking Rust installation..."
if command -v cargo &> /dev/null; then
    VERSION=$(rustc --version)
    log_success "Rust installed: $VERSION"
else
    log_error "Rust not found"
    ((ERRORS++))
fi

# Check system dependencies
log_info "Checking system dependencies..."

deps_ok=true
for lib in libasound2-dev libudev-dev pkg-config; do
    if dpkg -l | grep -q "$lib" 2>/dev/null; then
        :  # OK
    else
        deps_ok=false
        break
    fi
done

if $deps_ok || [ -d "/usr/include/alsa" ] && [ -d "/usr/include/libudev.h" ]; then
    log_success "System dependencies installed"
else
    log_warning "Some system dependencies may be missing"
    ((WARNINGS++))
fi

# Check DragonflyDB
log_info "Checking DragonflyDB..."
if command -v dragonfly &> /dev/null; then
    if systemctl is-active --quiet dragonfly 2>/dev/null; then
        log_success "DragonflyDB running on port 6379"

        # Test connection
        if command -v redis-cli &> /dev/null; then
            if redis-cli -p 6379 ping &>/dev/null; then
                log_success "DragonflyDB connection OK"
            else
                log_warning "DragonflyDB running but connection failed (may need auth)"
                ((WARNINGS++))
            fi
        fi
    else
        log_warning "DragonflyDB installed but not running"
        ((WARNINGS++))
    fi
else
    log_warning "DragonflyDB not installed (optional for development)"
    ((WARNINGS++))
fi

# Check RabbitMQ
log_info "Checking RabbitMQ..."
if command -v rabbitmq-server &> /dev/null; then
    if systemctl is-active --quiet rabbitmq-server 2>/dev/null; then
        log_success "RabbitMQ running on port 5672"

        # Check management plugin
        if curl -s -u guest:guest http://localhost:15672/api/overview &>/dev/null; then
            log_success "RabbitMQ Management UI accessible at http://localhost:15672"
        else
            log_warning "RabbitMQ Management UI not accessible"
            ((WARNINGS++))
        fi
    else
        log_warning "RabbitMQ installed but not running"
        ((WARNINGS++))
    fi
else
    log_warning "RabbitMQ not installed (optional for development)"
    ((WARNINGS++))
fi

# Check project structure
log_info "Checking project structure..."
cd "$(dirname "$0")/.."

if [ -f Cargo.toml ]; then
    log_success "Cargo.toml found"
else
    log_error "Cargo.toml not found - are you in the right directory?"
    ((ERRORS++))
fi

if [ -f .env ]; then
    log_success ".env file exists"
else
    log_warning ".env file not found - copy .env.example to .env"
    ((WARNINGS++))
fi

if [ -d assets ]; then
    log_success "Assets directory exists"

    # Count assets
    AUDIO_COUNT=$(find assets/audio -type f 2>/dev/null | wc -l)
    SPRITE_COUNT=$(find assets/sprites -type f 2>/dev/null | wc -l)

    if [ "$AUDIO_COUNT" -gt 0 ] || [ "$SPRITE_COUNT" -gt 0 ]; then
        log_success "Assets found: $AUDIO_COUNT audio, $SPRITE_COUNT sprites"
    else
        log_warning "No assets found (will use placeholders)"
        ((WARNINGS++))
    fi
else
    log_warning "Assets directory not found"
    ((WARNINGS++))
fi

# Check cargo build
log_info "Checking if project compiles..."
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

if cargo check --workspace --quiet 2>/dev/null; then
    log_success "Project compiles successfully"
else
    log_error "Project has compilation errors"
    ((ERRORS++))
fi

# Check disk space
log_info "Checking disk space..."
AVAILABLE=$(df . | tail -1 | awk '{print $4}')
if [ "$AVAILABLE" -gt 5000000 ]; then  # 5GB
    log_success "Sufficient disk space available"
else
    log_warning "Low disk space (less than 5GB available)"
    ((WARNINGS++))
fi

# Summary
echo ""
echo "========================================"
echo " Health Check Summary"
echo "========================================"
echo ""

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    log_success "All checks passed! System is ready."
    exit 0
elif [ $ERRORS -eq 0 ]; then
    log_warning "$WARNINGS warning(s) found. System is mostly ready."
    exit 0
else
    log_error "$ERRORS error(s) and $WARNINGS warning(s) found."
    echo ""
    echo "Please fix the errors before running the game."
    exit 1
fi
