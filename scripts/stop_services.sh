#!/bin/bash
# Stop all Shaman's Journey services

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

echo "Stopping Shaman's Journey services..."
echo ""

# Check if systemd is available
if command -v systemctl &> /dev/null; then
    log_info "Using systemd to stop services..."

    # Stop DragonflyDB
    if systemctl list-unit-files | grep -q dragonfly.service; then
        sudo systemctl stop dragonfly
        log_success "DragonflyDB stopped"
    else
        log_info "DragonflyDB service not found, skipping..."
    fi

    # Stop RabbitMQ
    if systemctl list-unit-files | grep -q rabbitmq-server.service; then
        sudo systemctl stop rabbitmq-server
        log_success "RabbitMQ stopped"
    else
        log_info "RabbitMQ service not found, skipping..."
    fi

    echo ""
    log_success "All services stopped!"
else
    log_error "systemd not available"
    log_info "Please stop services manually"
fi
