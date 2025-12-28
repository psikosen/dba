#!/bin/bash
# Production Installation Script for Shaman's Journey
# This script sets up the game for production deployment on Ubuntu Server

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m'

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

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    log_error "Please run this script as root or with sudo"
    exit 1
fi

INSTALL_DIR="/opt/shaman-journey"
SERVICE_USER="shaman"
SERVICE_GROUP="shaman"

echo -e "${GREEN}"
cat << "EOF"
╔═══════════════════════════════════════════════════════════╗
║    Shaman's Journey - Production Installation Script     ║
╚═══════════════════════════════════════════════════════════╝
EOF
echo -e "${NC}"

log_info "This script will:"
echo "  • Create service user and group"
echo "  • Install system dependencies"
echo "  • Install DragonflyDB and RabbitMQ"
echo "  • Create directory structure at $INSTALL_DIR"
echo "  • Install systemd service files"
echo "  • Configure security and resource limits"
echo ""
read -p "Continue? (y/n) " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    log_warning "Installation cancelled."
    exit 0
fi

# Create service user
log_info "Creating service user and group..."
if ! getent group "$SERVICE_GROUP" > /dev/null; then
    groupadd --system "$SERVICE_GROUP"
    log_success "Created group: $SERVICE_GROUP"
fi

if ! getent passwd "$SERVICE_USER" > /dev/null; then
    useradd --system --gid "$SERVICE_GROUP" --home-dir "$INSTALL_DIR" \
        --shell /bin/false --comment "Shaman's Journey Service User" "$SERVICE_USER"
    log_success "Created user: $SERVICE_USER"
fi

# Install system dependencies
log_info "Installing system dependencies..."
apt-get update -qq
apt-get install -y \
    curl wget gnupg2 ca-certificates lsb-release \
    build-essential pkg-config \
    libasound2-dev libudev-dev \
    libx11-dev libxi-dev libgl1-mesa-dev libglu1-mesa-dev \
    libxcursor-dev libxinerama-dev libxrandr-dev \
    redis-tools

log_success "System dependencies installed"

# Install DragonflyDB
log_info "Installing DragonflyDB..."
ARCH=$(uname -m)
if [ "$ARCH" = "x86_64" ] || [ "$ARCH" = "aarch64" ]; then
    DRAGONFLY_VERSION="v1.14.0"
    DRAGONFLY_URL="https://dragonflydb.gateway.scarf.sh/$DRAGONFLY_VERSION/dragonfly-$ARCH.tar.gz"

    cd /tmp
    wget -q "$DRAGONFLY_URL" -O dragonfly.tar.gz
    tar -xzf dragonfly.tar.gz
    mv dragonfly-$ARCH /usr/local/bin/dragonfly
    chmod +x /usr/local/bin/dragonfly
    rm -f dragonfly.tar.gz

    # Create dragonfly user and directory
    if ! getent passwd dragonfly > /dev/null; then
        useradd --system --shell /bin/false dragonfly
    fi
    mkdir -p /var/lib/dragonfly
    chown dragonfly:dragonfly /var/lib/dragonfly

    log_success "DragonflyDB installed"
else
    log_warning "DragonflyDB not available for $ARCH architecture"
fi

# Install RabbitMQ
log_info "Installing RabbitMQ..."
apt-get install -y erlang-base erlang-asn1 erlang-crypto erlang-eldap \
    erlang-inets erlang-mnesia erlang-os-mon erlang-parsetools \
    erlang-public-key erlang-runtime-tools erlang-snmp erlang-ssl \
    erlang-syntax-tools erlang-tftp erlang-tools erlang-xmerl

curl -fsSL https://github.com/rabbitmq/signing-keys/releases/download/2.0/rabbitmq-release-signing-key.asc | apt-key add -
tee /etc/apt/sources.list.d/rabbitmq.list <<EOFRABBIT
deb https://dl.bintray.com/rabbitmq-erlang/debian $(lsb_release -sc) erlang
deb https://dl.bintray.com/rabbitmq/debian $(lsb_release -sc) main
EOFRABBIT

apt-get update -qq
apt-get install -y rabbitmq-server

# Enable RabbitMQ management
rabbitmq-plugins enable rabbitmq_management

log_success "RabbitMQ installed"

# Create directory structure
log_info "Creating directory structure..."
mkdir -p "$INSTALL_DIR"/{bin,assets,saves,logs,data/cache}
chown -R "$SERVICE_USER:$SERVICE_GROUP" "$INSTALL_DIR"

log_success "Directory structure created"

# Install systemd service files
log_info "Installing systemd service files..."

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [ -f "$SCRIPT_DIR/systemd/dragonfly.service" ]; then
    cp "$SCRIPT_DIR/systemd/dragonfly.service" /etc/systemd/system/
    log_success "DragonflyDB service file installed"
fi

if [ -f "$SCRIPT_DIR/systemd/shaman-journey.service" ]; then
    cp "$SCRIPT_DIR/systemd/shaman-journey.service" /etc/systemd/system/
    log_success "Shaman's Journey service file installed"
fi

# Reload systemd
systemctl daemon-reload

# Enable services
log_info "Enabling services..."
systemctl enable dragonfly
systemctl enable rabbitmq-server
systemctl enable shaman-journey

log_success "Services enabled"

# Start dependency services
log_info "Starting dependency services..."
systemctl start dragonfly
systemctl start rabbitmq-server

log_success "Dependency services started"

# Create example .env file
log_info "Creating .env template..."
cat > "$INSTALL_DIR/.env.example" <<EOFENV
# Shaman's Journey - Production Environment Configuration
ENVIRONMENT=production
RUST_LOG=info
RUST_BACKTRACE=1

# Sentry
SENTRY_DSN=

# Monitoring
PROMETHEUS_PORT=9091

# Grafana
GRAFANA_ADMIN_USER=admin
GRAFANA_ADMIN_PASSWORD=changeme

# DragonflyDB
DRAGONFLY_URL=redis://:changeme@localhost:6379
DRAGONFLY_PASSWORD=changeme

# RabbitMQ
RABBITMQ_URL=amqp://shaman:changeme@localhost:5672/%2f
RABBITMQ_USER=shaman
RABBITMQ_PASSWORD=changeme

# Paths
ASSETS_PATH=$INSTALL_DIR/assets
SAVES_PATH=$INSTALL_DIR/saves
EOFENV

chown "$SERVICE_USER:$SERVICE_GROUP" "$INSTALL_DIR/.env.example"

log_success ".env template created"

# Print completion message
echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║              Production Installation Complete!            ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Next Steps:${NC}"
echo ""
echo "1. Copy your game binary to:"
echo "   ${YELLOW}$INSTALL_DIR/bin/bevy_shaman${NC}"
echo ""
echo "2. Copy your assets to:"
echo "   ${YELLOW}$INSTALL_DIR/assets/${NC}"
echo ""
echo "3. Configure environment:"
echo "   ${YELLOW}cp $INSTALL_DIR/.env.example $INSTALL_DIR/.env${NC}"
echo "   ${YELLOW}nano $INSTALL_DIR/.env${NC}"
echo ""
echo "4. Set proper ownership:"
echo "   ${YELLOW}chown -R $SERVICE_USER:$SERVICE_GROUP $INSTALL_DIR${NC}"
echo ""
echo "5. Start the service:"
echo "   ${YELLOW}systemctl start shaman-journey${NC}"
echo ""
echo "6. Check status:"
echo "   ${YELLOW}systemctl status shaman-journey${NC}"
echo ""
echo "7. View logs:"
echo "   ${YELLOW}journalctl -u shaman-journey -f${NC}"
echo ""
echo -e "${BLUE}Service Management:${NC}"
echo ""
echo "• RabbitMQ Management UI: ${YELLOW}http://localhost:15672${NC}"
echo "• Default credentials: guest/guest"
echo ""
echo -e "${GREEN}Installation complete!${NC}"
echo ""
