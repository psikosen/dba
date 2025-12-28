#!/bin/bash
# Start all Shaman's Journey services
# Supports both systemd (production) and manual startup (development)

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

echo "Starting Shaman's Journey services..."
echo ""

# Check if systemd is available
if command -v systemctl &> /dev/null; then
    log_info "Using systemd to start services..."

    # Start DragonflyDB
    if systemctl list-unit-files | grep -q dragonfly.service; then
        sudo systemctl start dragonfly
        log_success "DragonflyDB started"
    else
        log_info "DragonflyDB service not found, skipping..."
    fi

    # Start RabbitMQ
    if systemctl list-unit-files | grep -q rabbitmq-server.service; then
        sudo systemctl start rabbitmq-server
        log_success "RabbitMQ started"
    else
        log_info "RabbitMQ service not found, skipping..."
    fi

    echo ""
    log_success "All services started!"
    echo ""
    log_info "Check status with: systemctl status dragonfly rabbitmq-server"
else
    log_error "systemd not available"
    log_info "Please start services manually or use Docker Compose"
fi
