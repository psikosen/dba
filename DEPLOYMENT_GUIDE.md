# Deployment Guide - Shaman's Journey

This guide covers deployment options for Shaman's Journey in production environments.

## Quick Start

### Docker Deployment (Recommended)

The easiest way to deploy Shaman's Journey is using Docker:

```bash
# Build the Docker image
docker build -t shaman-journey:latest .

# Run the container
docker run -d \
  --name shaman-journey \
  -v $(pwd)/saves:/app/saves \
  shaman-journey:latest
```

### Docker Compose (Easiest)

For production deployments with proper configuration:

```bash
# Start the application
docker-compose up -d

# View logs
docker-compose logs -f

# Stop the application
docker-compose down
```

## Deployment Options

### 1. Docker Container

**Advantages:**
- Consistent environment across all platforms
- All dependencies bundled
- Easy scaling and updates
- Isolated from host system

**Build Arguments:**
```bash
# Production build
docker build \
  --tag shaman-journey:v0.1.0 \
  --file Dockerfile \
  .

# Development build (with source)
docker build \
  --tag shaman-journey:dev \
  --target builder \
  .
```

**Run Options:**
```bash
# Basic run
docker run -d shaman-journey:latest

# With persistent saves
docker run -d \
  -v ./saves:/app/saves \
  shaman-journey:latest

# With custom assets
docker run -d \
  -v ./saves:/app/saves \
  -v ./assets:/app/assets:ro \
  shaman-journey:latest

# With resource limits
docker run -d \
  --memory=2g \
  --cpus=2 \
  -v ./saves:/app/saves \
  shaman-journey:latest
```

### 2. Native Binary Deployment

**Prerequisites:**
```bash
# Install system dependencies (Debian/Ubuntu)
sudo apt-get install -y \
    libasound2 \
    libudev1 \
    libx11-6 \
    libxi6 \
    libgl1 \
    libxcursor1 \
    libxinerama1 \
    libxrandr2 \
    libxfixes3
```

**Build:**
```bash
# Release build with all optimizations
cargo build --release --workspace --exclude bevy_shaman_audio

# Binary location
./target/release/bevy_shaman
```

**Deploy:**
```bash
# Copy binary to production server
scp target/release/bevy_shaman user@server:/opt/shaman-journey/

# Create systemd service (optional)
sudo cat > /etc/systemd/system/shaman-journey.service <<EOF
[Unit]
Description=Shaman's Journey Game Server
After=network.target

[Service]
Type=simple
User=shaman
WorkingDirectory=/opt/shaman-journey
ExecStart=/opt/shaman-journey/bevy_shaman
Restart=on-failure
RestartSec=10s

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl enable shaman-journey
sudo systemctl start shaman-journey
```

### 3. Container Orchestration (Kubernetes)

**Example Deployment:**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: shaman-journey
spec:
  replicas: 3
  selector:
    matchLabels:
      app: shaman-journey
  template:
    metadata:
      labels:
        app: shaman-journey
    spec:
      containers:
      - name: shaman-journey
        image: shaman-journey:latest
        resources:
          limits:
            memory: "2Gi"
            cpu: "2"
          requests:
            memory: "512Mi"
            cpu: "500m"
        volumeMounts:
        - name: saves
          mountPath: /app/saves
      volumes:
      - name: saves
        persistentVolumeClaim:
          claimName: shaman-saves-pvc
```

## Configuration

### Environment Variables

```bash
# Logging level (error, warn, info, debug, trace)
RUST_LOG=info

# Enable backtraces for debugging
RUST_BACKTRACE=1

# Custom save directory (if not using default)
SAVE_DIR=/custom/path/to/saves
```

### Volume Mounts

| Path | Purpose | Required | Mode |
|------|---------|----------|------|
| `/app/saves` | Game save files | Yes | RW |
| `/app/assets` | Game assets (sprites, audio) | Yes | RO |
| `/tmp` | Temporary files | No | RW |

## Performance Tuning

### Release Build Optimizations

The project includes optimized release profiles in `Cargo.toml`:

```toml
[profile.release]
opt-level = 3           # Maximum optimization
lto = "thin"            # Link-time optimization
codegen-units = 1       # Better optimization, slower compile
strip = true            # Remove debug symbols
panic = "abort"         # Smaller binary
overflow-checks = true  # Safety checks
```

**Expected binary size:** ~50-80 MB (after strip)

### Runtime Performance

**Resource Usage:**
- Memory: ~500 MB baseline, up to 2 GB with large worlds
- CPU: 1-2 cores recommended, can utilize more for parallel systems
- Disk: ~100 MB for binary + assets, minimal for saves

**Optimization Tips:**
1. Enable release mode for production
2. Limit FPS if running headless
3. Use SSD for faster world generation
4. Allocate sufficient memory for large dungeons

## Monitoring & Observability

### Health Checks

Docker includes a built-in health check:
```bash
# Check if process is running
pgrep -x bevy_shaman

# Container health status
docker inspect --format='{{.State.Health.Status}}' shaman-journey
```

### Logging

**View logs:**
```bash
# Docker logs
docker logs -f shaman-journey

# Docker Compose logs
docker-compose logs -f

# System logs (if using systemd)
journalctl -u shaman-journey -f
```

**Log Levels:**
- `RUST_LOG=error` - Only errors
- `RUST_LOG=warn` - Warnings and errors
- `RUST_LOG=info` - General information (recommended)
- `RUST_LOG=debug` - Detailed debugging
- `RUST_LOG=trace` - Very verbose

### Metrics (Future)

Planned monitoring integration:
- Prometheus metrics for game events
- Grafana dashboards for visualization
- Alert rules for error rates

## Security

### Best Practices

1. **Run as non-root user:**
   - Docker image uses `shaman` user (UID 1000)
   - Systemd service should use dedicated user

2. **Read-only filesystem:**
   - Docker Compose enables read-only root
   - Only `/app/saves` and `/tmp` are writable

3. **Resource limits:**
   - Set memory and CPU limits
   - Prevent resource exhaustion

4. **Network isolation:**
   - No network ports exposed by default
   - Use internal networks for multi-container setups

5. **Secret management:**
   - No secrets in images or compose files
   - Use environment variables or secret managers

### Security Scanning

```bash
# Scan Docker image for vulnerabilities
docker scan shaman-journey:latest

# Audit Rust dependencies
cargo audit

# Check for outdated dependencies
cargo outdated
```

## Backup & Recovery

### Save File Backups

**Automated backup:**
```bash
# Backup script
#!/bin/bash
BACKUP_DIR=/backups/shaman-journey
DATE=$(date +%Y%m%d-%H%M%S)

# Copy saves
cp -r /app/saves $BACKUP_DIR/saves-$DATE

# Cleanup old backups (keep last 30 days)
find $BACKUP_DIR -mtime +30 -delete
```

**Restore:**
```bash
# Stop container
docker-compose down

# Restore saves
cp -r /backups/shaman-journey/saves-20251228-120000/* ./saves/

# Start container
docker-compose up -d
```

## Troubleshooting

### Common Issues

**1. Container won't start:**
```bash
# Check logs
docker logs shaman-journey

# Verify dependencies
docker run -it shaman-journey:latest ldd /app/bevy_shaman
```

**2. Permission errors:**
```bash
# Fix save directory permissions
sudo chown -R 1000:1000 ./saves

# Or run container as root (not recommended)
docker run --user root shaman-journey:latest
```

**3. High memory usage:**
```bash
# Check resource usage
docker stats shaman-journey

# Set memory limit
docker run --memory=1g shaman-journey:latest
```

**4. Build failures:**
```bash
# Clear build cache
docker builder prune

# Rebuild without cache
docker build --no-cache -t shaman-journey:latest .
```

## Scaling

### Horizontal Scaling

For multiple instances:
```yaml
# docker-compose.yml
services:
  shaman-journey:
    deploy:
      replicas: 3
    # ... rest of config
```

### Load Balancing

Use a load balancer (nginx, HAProxy, etc.) to distribute traffic:
```nginx
upstream shaman_backend {
    server shaman-1:8080;
    server shaman-2:8080;
    server shaman-3:8080;
}

server {
    listen 80;
    location / {
        proxy_pass http://shaman_backend;
    }
}
```

## Updates & Rollbacks

### Update Process

```bash
# Pull latest code
git pull origin main

# Build new image
docker build -t shaman-journey:v0.2.0 .

# Tag as latest
docker tag shaman-journey:v0.2.0 shaman-journey:latest

# Update running container
docker-compose up -d --no-deps --build shaman-journey
```

### Rollback

```bash
# Use previous image version
docker tag shaman-journey:v0.1.0 shaman-journey:latest

# Restart with old version
docker-compose up -d
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build and Deploy

on:
  push:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Build Docker image
        run: docker build -t shaman-journey:${{ github.sha }} .

      - name: Push to registry
        run: |
          echo ${{ secrets.DOCKER_PASSWORD }} | docker login -u ${{ secrets.DOCKER_USERNAME }} --password-stdin
          docker push shaman-journey:${{ github.sha }}

      - name: Deploy
        run: |
          # Deploy to production
          ssh user@server 'docker pull shaman-journey:${{ github.sha }} && docker-compose up -d'
```

## Cost Optimization

### Resource Allocation

**Minimum requirements:**
- 1 CPU core
- 512 MB RAM
- 500 MB disk

**Recommended production:**
- 2 CPU cores
- 2 GB RAM
- 5 GB disk (with room for saves)

### Multi-stage Builds

The Dockerfile uses multi-stage builds to minimize image size:
- Builder image: ~2 GB
- Runtime image: ~300 MB

## Support & Maintenance

### Regular Tasks

- **Daily:** Check logs for errors
- **Weekly:** Review resource usage
- **Monthly:** Update dependencies, run security scans
- **Quarterly:** Performance benchmarking, capacity planning

### Getting Help

- Issues: https://github.com/psikosen/dba/issues
- Documentation: `/docs` directory
- Production readiness: `PRODUCTION_READINESS_REVIEW_2025-12-28.md`

---

**Last updated:** December 28, 2025
**Version:** 0.1.0
