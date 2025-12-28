# Shaman's Journey - Production Deployment Guide

**Last Updated**: December 28, 2025
**Version**: 1.0.0

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Development Setup](#development-setup)
4. [Production Deployment](#production-deployment)
5. [Service Management](#service-management)
6. [Monitoring & Logging](#monitoring--logging)
7. [Troubleshooting](#troubleshooting)
8. [Security Best Practices](#security-best-practices)

---

## Overview

This guide covers the complete deployment process for Shaman's Journey, from local development to production deployment on Ubuntu/Linux servers.

### Architecture

The game consists of:
- **Game Binary**: Rust/Bevy application
- **DragonflyDB**: Redis-compatible cache for LLM responses and dungeon seeds
- **RabbitMQ**: Message queue for async processing and events
- **Monitoring**: Prometheus + Grafana + Sentry

---

## Prerequisites

### Minimum Requirements

**Development:**
- Ubuntu 20.04+ / Debian 11+ / Fedora 35+ / Arch Linux
- 4GB RAM
- 5GB free disk space
- Rust 1.75+ (installed automatically)

**Production:**
- Ubuntu Server 20.04+ LTS
- 8GB RAM (recommended)
- 10GB free disk space
- Rust 1.75+
- systemd

### System Dependencies

The following system libraries are required:

**Ubuntu/Debian:**
```bash
libasound2-dev libudev-dev pkg-config build-essential
libx11-dev libxi-dev libgl1-mesa-dev libglu1-mesa-dev
libxcursor-dev libxinerama-dev libxrandr-dev
```

**Fedora/RHEL:**
```bash
alsa-lib-devel systemd-devel pkgconfig gcc gcc-c++
libX11-devel libXi-devel mesa-libGL-devel mesa-libGLU-devel
libXcursor-devel libXinerama-devel libXrandr-devel
```

**Arch Linux:**
```bash
alsa-lib systemd pkgconf base-devel
libx11 libxi mesa libxcursor libxinerama libxrandr
```

---

## Development Setup

### Quick Start (Automated)

Use our comprehensive setup script that installs everything:

```bash
# Clone the repository
git clone https://github.com/psikosen/dba.git
cd dba

# Run the automated setup script
bash scripts/setup_dev_environment.sh
```

This script will:
1. ✅ Install system dependencies
2. ✅ Install Rust toolchain
3. ✅ Install DragonflyDB
4. ✅ Install RabbitMQ
5. ✅ Setup systemd services
6. ✅ Create directory structure
7. ✅ Generate .env configuration
8. ✅ Verify installation

### Manual Setup

If you prefer manual installation:

#### 1. Install System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y \
    libasound2-dev libudev-dev pkg-config build-essential \
    libx11-dev libxi-dev libgl1-mesa-dev libglu1-mesa-dev \
    libxcursor-dev libxinerama-dev libxrandr-dev
```

#### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

#### 3. Install DragonflyDB

```bash
# Download and install
wget https://dragonflydb.gateway.scarf.sh/v1.14.0/dragonfly-x86_64.tar.gz
tar -xzf dragonfly-x86_64.tar.gz
sudo mv dragonfly-x86_64 /usr/local/bin/dragonfly
sudo chmod +x /usr/local/bin/dragonfly
```

#### 4. Install RabbitMQ

**Ubuntu/Debian:**
```bash
sudo apt-get install -y erlang-base erlang-asn1 erlang-crypto \
    erlang-eldap erlang-inets erlang-mnesia erlang-os-mon \
    erlang-parsetools erlang-public-key erlang-runtime-tools \
    erlang-snmp erlang-ssl erlang-syntax-tools erlang-tftp \
    erlang-tools erlang-xmerl rabbitmq-server

sudo systemctl enable rabbitmq-server
sudo systemctl start rabbitmq-server
sudo rabbitmq-plugins enable rabbitmq_management
```

#### 5. Create .env File

```bash
cp .env.example .env
nano .env  # Edit configuration
```

#### 6. Build the Project

```bash
# Debug build (fast compilation)
cargo build

# Release build (optimized)
cargo build --release
```

### Running Locally

```bash
# Start services
./scripts/start_services.sh

# Run the game (debug)
cargo run

# Run the game (release)
cargo run --release

# Stop services
./scripts/stop_services.sh
```

### Health Check

Verify your installation:

```bash
./scripts/health_check.sh
```

---

## Production Deployment

### Automated Production Installation

For Ubuntu Server:

```bash
# Copy the deployment directory to the server
scp -r deployment/ user@server:/tmp/

# SSH into the server
ssh user@server

# Run the production installation script
cd /tmp/deployment
sudo bash install_production.sh
```

This script will:
1. ✅ Create service user and group
2. ✅ Install all dependencies
3. ✅ Install DragonflyDB and RabbitMQ
4. ✅ Setup systemd service files
5. ✅ Create directory structure at `/opt/shaman-journey`
6. ✅ Configure security hardening

### Manual Production Deployment

#### 1. Prepare the Binary

On your development machine:

```bash
# Build optimized release binary
cargo build --release

# Binary will be at: target/release/bevy_shaman
```

#### 2. Setup Production Server

**Create service user:**
```bash
sudo useradd --system --shell /bin/false shaman
sudo mkdir -p /opt/shaman-journey/{bin,assets,saves,logs,data/cache}
sudo chown -R shaman:shaman /opt/shaman-journey
```

**Install systemd services:**
```bash
sudo cp deployment/systemd/dragonfly.service /etc/systemd/system/
sudo cp deployment/systemd/shaman-journey.service /etc/systemd/system/
sudo systemctl daemon-reload
```

#### 3. Deploy the Application

```bash
# Copy binary to server
scp target/release/bevy_shaman user@server:/tmp/

# On server: Move to installation directory
sudo mv /tmp/bevy_shaman /opt/shaman-journey/bin/
sudo chmod +x /opt/shaman-journey/bin/bevy_shaman
sudo chown shaman:shaman /opt/shaman-journey/bin/bevy_shaman

# Copy assets
scp -r assets/* user@server:/tmp/assets/
sudo mv /tmp/assets/* /opt/shaman-journey/assets/
sudo chown -R shaman:shaman /opt/shaman-journey/assets
```

#### 4. Configure Environment

```bash
# Create .env file
sudo nano /opt/shaman-journey/.env
```

Example production `.env`:
```env
ENVIRONMENT=production
RUST_LOG=info
RUST_BACKTRACE=1

# Sentry (get from https://sentry.io)
SENTRY_DSN=https://your-key@o123456.ingest.sentry.io/789

# Monitoring
PROMETHEUS_PORT=9091

# DragonflyDB
DRAGONFLY_URL=redis://:your-strong-password@localhost:6379
DRAGONFLY_PASSWORD=your-strong-password

# RabbitMQ
RABBITMQ_URL=amqp://shaman:your-password@localhost:5672/%2f
RABBITMQ_USER=shaman
RABBITMQ_PASSWORD=your-password

# Paths
ASSETS_PATH=/opt/shaman-journey/assets
SAVES_PATH=/opt/shaman-journey/saves
```

**Secure the .env file:**
```bash
sudo chown shaman:shaman /opt/shaman-journey/.env
sudo chmod 600 /opt/shaman-journey/.env
```

#### 5. Start Services

```bash
# Enable services to start on boot
sudo systemctl enable dragonfly
sudo systemctl enable rabbitmq-server
sudo systemctl enable shaman-journey

# Start services
sudo systemctl start dragonfly
sudo systemctl start rabbitmq-server
sudo systemctl start shaman-journey
```

#### 6. Verify Deployment

```bash
# Check service status
sudo systemctl status shaman-journey

# View logs
sudo journalctl -u shaman-journey -f

# Check if services are listening
sudo netstat -tlnp | grep -E '6379|5672|9091'
```

---

## Service Management

### Starting Services

```bash
# All services
sudo systemctl start dragonfly rabbitmq-server shaman-journey

# Individual services
sudo systemctl start shaman-journey
```

### Stopping Services

```bash
# All services
sudo systemctl stop shaman-journey rabbitmq-server dragonfly

# Individual service
sudo systemctl stop shaman-journey
```

### Restarting Services

```bash
sudo systemctl restart shaman-journey
```

### Checking Status

```bash
# All services
sudo systemctl status dragonfly rabbitmq-server shaman-journey

# Individual service
sudo systemctl status shaman-journey
```

### Viewing Logs

```bash
# Live log tail
sudo journalctl -u shaman-journey -f

# Last 100 lines
sudo journalctl -u shaman-journey -n 100

# Logs since boot
sudo journalctl -u shaman-journey -b

# Logs for specific date
sudo journalctl -u shaman-journey --since "2025-12-28"
```

### Enabling/Disabling Auto-start

```bash
# Enable auto-start on boot
sudo systemctl enable shaman-journey

# Disable auto-start
sudo systemctl disable shaman-journey
```

---

## Monitoring & Logging

### Prometheus Metrics

The game exposes metrics on port 9091 (configurable via `PROMETHEUS_PORT`):

```bash
# View metrics
curl http://localhost:9091/metrics
```

**Available metrics:**
- `game_fps` - Frames per second
- `game_frame_time_seconds` - Frame processing time
- `game_entities_total` - Entity count
- `game_monsters_total` - Monster count
- `game_corruption_level` - Corruption level

### Sentry Error Tracking

Configure Sentry for error tracking:

```env
SENTRY_DSN=https://your-key@o123456.ingest.sentry.io/789
```

Errors and panics will be automatically reported with:
- Stack traces
- Environment context
- User context
- Breadcrumbs

### RabbitMQ Management

Access the RabbitMQ management UI:

```
URL: http://your-server:15672
Default credentials: guest/guest
```

**Security Note**: Change default credentials in production!

```bash
sudo rabbitmqctl add_user shaman your-strong-password
sudo rabbitmqctl set_permissions -p / shaman ".*" ".*" ".*"
sudo rabbitmqctl set_user_tags shaman administrator
sudo rabbitmqctl delete_user guest
```

### DragonflyDB Monitoring

```bash
# Connect with redis-cli
redis-cli -p 6379 -a your-password

# Get server info
INFO

# Monitor commands
MONITOR

# Get stats
INFO stats
```

---

## Troubleshooting

### Service Won't Start

**Check logs:**
```bash
sudo journalctl -u shaman-journey -n 50 --no-pager
```

**Common issues:**

1. **Missing dependencies:**
   ```bash
   sudo apt-get install -y libasound2 libudev1
   ```

2. **Permission denied:**
   ```bash
   sudo chown -R shaman:shaman /opt/shaman-journey
   sudo chmod +x /opt/shaman-journey/bin/bevy_shaman
   ```

3. **Port already in use:**
   ```bash
   sudo netstat -tlnp | grep 9091
   sudo kill -9 <PID>
   ```

### DragonflyDB Connection Issues

```bash
# Test connection
redis-cli -p 6379 -a your-password ping

# Should return: PONG

# Check if running
sudo systemctl status dragonfly

# View logs
sudo journalctl -u dragonfly -n 50
```

### RabbitMQ Connection Issues

```bash
# Check status
sudo systemctl status rabbitmq-server

# Restart
sudo systemctl restart rabbitmq-server

# Check diagnostics
sudo rabbitmq-diagnostics status

# List queues
sudo rabbitmqctl list_queues
```

### Build Failures

**Missing system dependencies:**
```bash
# Ubuntu/Debian
sudo apt-get install -y libudev-dev libasound2-dev pkg-config build-essential

# Check what's missing
cargo build 2>&1 | grep "not found"
```

**Audio crate issues:**
```bash
# Skip audio crate in tests
cargo test --workspace --exclude bevy_shaman_audio
```

---

## Security Best Practices

### 1. Firewall Configuration

```bash
# Allow only necessary ports
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 9091/tcp  # Prometheus (optional, can be internal only)
sudo ufw enable
```

### 2. Secure Credentials

- ✅ Use strong passwords (32+ characters)
- ✅ Store `.env` with 600 permissions
- ✅ Never commit `.env` to git
- ✅ Rotate credentials regularly

### 3. Service Hardening

Our systemd services include:
- `NoNewPrivileges=true` - Prevents privilege escalation
- `ProtectSystem=strict` - Read-only filesystem
- `PrivateTmp=true` - Isolated /tmp
- `MemoryMax=2G` - Resource limits
- `RestrictNamespaces=true` - Namespace isolation

### 4. Network Security

- ✅ Bind DragonflyDB to localhost only
- ✅ Use strong passwords for RabbitMQ
- ✅ Disable guest user in production
- ✅ Use SSL/TLS for external connections

### 5. Updates & Patching

```bash
# Update Rust
rustup update

# Update dependencies
cargo update

# Security audit
cargo audit

# Check for vulnerabilities
cargo deny check
```

---

## Additional Resources

- **README.md**: Project overview and architecture
- **SERVICES_ARCHITECTURE.md**: DragonflyDB and RabbitMQ integration
- **docs/MONITORING.md**: Detailed monitoring setup
- **docs/DEPLOYMENT.md**: Advanced deployment scenarios
- **docs/PRODUCTION_READINESS_SUMMARY.md**: Production readiness assessment

---

## Support

For issues and questions:
- GitHub Issues: https://github.com/psikosen/dba/issues
- Documentation: `docs/` directory

---

**Happy deploying! 🎮**
