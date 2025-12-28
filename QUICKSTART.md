# Shaman's Journey - Quick Start Guide

Get up and running in 5 minutes! ⚡

## Development (Ubuntu/Linux)

### One-Line Setup

```bash
git clone https://github.com/psikosen/dba.git && cd dba && bash scripts/setup_dev_environment.sh
```

This will install everything you need automatically.

### What Gets Installed

- ✅ System dependencies (ALSA, udev, X11, OpenGL)
- ✅ Rust toolchain
- ✅ DragonflyDB (cache)
- ✅ RabbitMQ (message queue)
- ✅ Systemd services
- ✅ .env configuration

### After Setup

```bash
# Build the game
cargo build --release

# Run the game
cargo run --release

# Check system health
./scripts/health_check.sh
```

---

## Production (Ubuntu Server)

### Quick Production Deploy

```bash
# 1. On your dev machine: Build the release binary
cargo build --release

# 2. Copy deployment files to server
scp -r deployment/ user@server:/tmp/

# 3. SSH into server and run installer
ssh user@server
cd /tmp/deployment
sudo bash install_production.sh

# 4. Copy your binary
sudo cp /path/to/bevy_shaman /opt/shaman-journey/bin/

# 5. Copy your assets
sudo cp -r /path/to/assets/* /opt/shaman-journey/assets/

# 6. Configure environment
sudo cp /opt/shaman-journey/.env.example /opt/shaman-journey/.env
sudo nano /opt/shaman-journey/.env  # Edit passwords and settings

# 7. Start the service
sudo systemctl start shaman-journey

# 8. Check status
sudo systemctl status shaman-journey
```

---

## Common Commands

### Service Management

```bash
# Start all services
sudo systemctl start dragonfly rabbitmq-server shaman-journey

# Stop all services
sudo systemctl stop shaman-journey rabbitmq-server dragonfly

# View game logs
sudo journalctl -u shaman-journey -f

# Check service status
sudo systemctl status shaman-journey
```

### Development

```bash
# Build (debug - fast compilation)
cargo build

# Build (release - optimized)
cargo build --release

# Run tests (skip audio crate)
cargo test --workspace --exclude bevy_shaman_audio

# Run benchmarks
cargo bench

# Security audit
cargo audit
```

### Helper Scripts

```bash
# Start services
./scripts/start_services.sh

# Stop services
./scripts/stop_services.sh

# Health check
./scripts/health_check.sh
```

---

## Access Points

After installation:

- **Game**: `cargo run --release` (dev) or `systemctl start shaman-journey` (prod)
- **RabbitMQ UI**: http://localhost:15672 (guest/guest)
- **Prometheus Metrics**: http://localhost:9091/metrics
- **DragonflyDB**: `redis-cli -p 6379`

---

## Troubleshooting

### Build Fails

```bash
# Install missing dependencies
sudo apt-get install -y libudev-dev libasound2-dev pkg-config build-essential
```

### Service Won't Start

```bash
# Check logs
sudo journalctl -u shaman-journey -n 50

# Verify binary exists
ls -la /opt/shaman-journey/bin/bevy_shaman

# Check permissions
sudo chown -R shaman:shaman /opt/shaman-journey
```

### Can't Connect to Services

```bash
# Check if services are running
sudo systemctl status dragonfly rabbitmq-server

# Restart services
sudo systemctl restart dragonfly rabbitmq-server
```

---

## Need More Details?

- **Full deployment guide**: [PRODUCTION_DEPLOYMENT_GUIDE.md](PRODUCTION_DEPLOYMENT_GUIDE.md)
- **Architecture overview**: [README.md](README.md)
- **Services setup**: [SERVICES_ARCHITECTURE.md](SERVICES_ARCHITECTURE.md)
- **Monitoring**: [docs/MONITORING.md](docs/MONITORING.md)

---

**That's it! You're ready to go! 🎮**
