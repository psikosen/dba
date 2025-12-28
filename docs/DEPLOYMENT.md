# Production Deployment Runbook

**Version:** 1.0
**Last Updated:** December 28, 2025
**Maintained by:** DevOps Team

---

## Quick Reference

| Environment | URL | Port | Health Check |
|-------------|-----|------|--------------|
| Development | localhost | 8080 | `docker ps` |
| Staging | localhost | 8080 | http://localhost:8080/health |
| Production | TBD | 8080 | http://prod-url/health |

**Emergency Contacts:**
- DevOps Lead: [contact info]
- On-Call Engineer: [contact info]
- Security Team: [contact info]

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Pre-Deployment Checklist](#pre-deployment-checklist)
3. [Deployment Procedure](#deployment-procedure)
4. [Post-Deployment Verification](#post-deployment-verification)
5. [Rollback Procedure](#rollback-procedure)
6. [Monitoring & Alerting](#monitoring--alerting)
7. [Troubleshooting](#troubleshooting)
8. [Emergency Procedures](#emergency-procedures)

---

## Prerequisites

### System Requirements

**Minimum:**
- CPU: 2 cores
- RAM: 2GB
- Disk: 10GB free space
- OS: Linux (Ubuntu 22.04+, Debian 11+, RHEL 8+)

**Recommended:**
- CPU: 4 cores
- RAM: 4GB
- Disk: 20GB free space (SSD preferred)
- OS: Ubuntu 22.04 LTS

### Required Software

```bash
# Verify installations
docker --version      # Required: 24.0+
docker-compose --version  # Required: 2.20+
git --version         # Required: 2.30+
```

### Required Access

- [ ] SSH access to production server
- [ ] Docker registry access (GitHub Container Registry)
- [ ] Secrets access (Sentry DSN, Grafana password)
- [ ] Monitoring access (Grafana, Prometheus)
- [ ] Incident management system access

### Network Requirements

**Inbound Ports:**
- 8080: Application (HTTP)
- 9091: Metrics (Prometheus)

**Outbound Ports:**
- 443: Docker registry, Sentry, package repositories
- 80: Package repositories (HTTP)

---

## Pre-Deployment Checklist

### Code Verification

```bash
# 1. Ensure on correct branch
git branch --show-current
# Should show: main or release/vX.X.X

# 2. Pull latest changes
git pull origin main

# 3. Verify no uncommitted changes
git status
# Should show: "working tree clean"

# 4. Check CI status
# Visit: https://github.com/psikosen/dba/actions
# All checks must be green ✅

# 5. Run tests locally
cargo test --workspace --exclude bevy_shaman_audio --release
```

### Build Verification

```bash
# 1. Build release binary
cargo build --release --workspace --exclude bevy_shaman_audio

# 2. Verify binary created
ls -lh target/release/bevy_shaman

# 3. Run smoke test
./target/release/bevy_shaman --version
```

### Docker Verification

```bash
# 1. Build Docker image
docker build -t shaman-journey:$(git rev-parse --short HEAD) .

# 2. Verify image created
docker images | grep shaman-journey

# 3. Test run locally
docker run --rm shaman-journey:$(git rev-parse --short HEAD) --version

# 4. Tag for registry
docker tag shaman-journey:$(git rev-parse --short HEAD) \
  ghcr.io/psikosen/dba:latest
```

### Security Verification

```bash
# 1. Security audit
cargo audit

# 2. Dependency check
cargo deny check

# 3. Scan Docker image (optional)
docker scan ghcr.io/psikosen/dba:latest
```

### Secrets Verification

```bash
# 1. Verify secrets exist
# Check .env file (DO NOT print values)
test -f .env && echo "✅ .env exists" || echo "❌ .env missing"

# 2. Verify required secrets are set
grep -q "SENTRY_DSN=" .env && echo "✅ SENTRY_DSN set" || echo "❌ SENTRY_DSN missing"
grep -q "ENVIRONMENT=production" .env && echo "✅ ENVIRONMENT set" || echo "❌ ENVIRONMENT incorrect"
```

### Backup Verification

```bash
# 1. Backup current production data
rsync -avz /app/saves/ /backups/saves-$(date +%Y%m%d-%H%M%S)/

# 2. Verify backup
ls -lh /backups/saves-$(date +%Y%m%d-%H%M%S)/

# 3. Test backup restore (dry-run)
rsync -avz --dry-run /backups/saves-latest/ /app/saves/
```

### Notification

```bash
# Notify team of deployment
# Post in team channel:
# "🚀 Starting production deployment of shaman-journey v1.0.0
#  ETA: 15 minutes
#  Expected downtime: None (rolling update)
#  Rollback plan: Ready
#  Monitor: https://grafana.example.com/dashboard"
```

---

## Deployment Procedure

### Method 1: Docker Compose (Recommended for Single Server)

#### Step 1: Prepare Environment

```bash
# SSH to production server
ssh production-server

# Navigate to application directory
cd /opt/shaman-journey

# Ensure we're on correct branch
git fetch origin
git checkout main
git pull origin main
```

#### Step 2: Pull Latest Images

```bash
# Login to GitHub Container Registry
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin

# Pull latest image
docker-compose pull shaman-journey
```

#### Step 3: Deploy with Rolling Update

```bash
# Create new container before stopping old one (zero downtime)
docker-compose up -d --no-deps --scale shaman-journey=2 shaman-journey

# Wait for new container health check
sleep 30

# Verify new container is healthy
docker-compose ps

# Scale down old container
docker-compose up -d --no-deps --scale shaman-journey=1 shaman-journey

# Verify single healthy container
docker-compose ps
```

#### Step 4: Cleanup

```bash
# Remove old containers
docker-compose down --remove-orphans

# Restart with single instance
docker-compose up -d

# Remove dangling images
docker image prune -f
```

### Method 2: Docker Swarm (For Multi-Server Deployments)

```bash
# Initialize swarm (first time only)
docker swarm init

# Create secrets (first time only)
echo "$SENTRY_DSN" | docker secret create sentry_dsn -
echo "$GRAFANA_PASSWORD" | docker secret create grafana_admin_password -

# Deploy stack
docker stack deploy -c docker-compose.yml shaman-journey

# Verify services
docker stack services shaman-journey

# Check service logs
docker service logs -f shaman-journey_shaman-journey
```

### Method 3: Manual Binary Deployment

```bash
# Build release binary
cargo build --release --workspace --exclude bevy_shaman_audio

# Stop current service
sudo systemctl stop shaman-journey

# Backup current binary
sudo cp /usr/local/bin/bevy_shaman /usr/local/bin/bevy_shaman.backup

# Install new binary
sudo cp target/release/bevy_shaman /usr/local/bin/

# Set permissions
sudo chmod +x /usr/local/bin/bevy_shaman
sudo chown shaman:shaman /usr/local/bin/bevy_shaman

# Start service
sudo systemctl start shaman-journey

# Verify service running
sudo systemctl status shaman-journey
```

---

## Post-Deployment Verification

### Immediate Checks (0-5 minutes)

#### 1. Container Status
```bash
# Check containers are running
docker-compose ps

# Expected output:
# NAME                  STATUS          PORTS
# shaman-journey        Up 2 minutes    0.0.0.0:8080->8080/tcp
```

#### 2. Health Check
```bash
# Check container health
docker ps --filter "name=shaman-journey"

# Look for: (healthy) in STATUS column

# Manual health check
docker exec shaman-journey pgrep -x bevy_shaman
# Should return PID number
```

#### 3. Log Check
```bash
# Check for errors in logs (last 50 lines)
docker-compose logs --tail=50 shaman-journey

# Look for:
# ❌ "ERROR", "FATAL", "panic"
# ✅ "Starting", "Initialized", "Listening"
```

#### 4. Metrics Endpoint
```bash
# Verify Prometheus metrics
curl http://localhost:9091/metrics | head -20

# Expected: Prometheus metrics format
# # HELP bevy_shaman_fps Current FPS
# # TYPE bevy_shaman_fps gauge
# bevy_shaman_fps 60.0
```

### Short-term Checks (5-15 minutes)

#### 5. Sentry Error Tracking
```bash
# Visit Sentry dashboard
# URL: https://sentry.io/organizations/your-org/projects/shaman-journey/

# Check for:
# ❌ New errors or spikes
# ✅ Normal baseline error rate
```

#### 6. Grafana Metrics
```bash
# Visit Grafana dashboard
# URL: http://production-server:3000

# Check dashboards:
# - Performance: FPS, frame time
# - Game Metrics: Entity count, encounters
# - System: CPU, memory usage

# Look for:
# ✅ FPS > 60
# ✅ Memory < 2GB
# ✅ No error rate spikes
```

#### 7. Resource Usage
```bash
# Check CPU usage
docker stats shaman-journey --no-stream

# Expected:
# CPU: < 50% under normal load
# MEM: < 1GB under normal load
```

### Extended Checks (15-60 minutes)

#### 8. Smoke Tests
```bash
# Run automated smoke tests
cargo test --test smoke --features production -- --ignored

# Expected: All tests pass ✅
```

#### 9. User Acceptance
```bash
# Manual testing checklist:
# [ ] Game launches
# [ ] Player can move
# [ ] Combat works
# [ ] Items can be used
# [ ] Inventory opens
# [ ] Save/Load works
# [ ] No graphical glitches
```

#### 10. Performance Baseline
```bash
# Compare current metrics to baseline
# Grafana: Compare last hour to previous deployment

# Metrics to check:
# - Average FPS (should be similar)
# - P95 frame time (should not increase)
# - Memory usage (should be stable)
# - Entity count (should be normal)
```

---

## Rollback Procedure

### When to Rollback

Rollback immediately if:
- ❌ Critical bugs affecting all users
- ❌ Data corruption or save game loss
- ❌ Security vulnerability discovered
- ❌ Service won't start or crashes repeatedly
- ❌ Performance degradation > 50%

Consider rollback if:
- ⚠️ Error rate increase > 100%
- ⚠️ User complaints increase
- ⚠️ Non-critical but widespread bug

### Automated Rollback (Docker Compose)

```bash
# Step 1: Stop current containers
docker-compose down

# Step 2: Pull previous image version
# (Tag with specific version, e.g., v1.0.0)
docker pull ghcr.io/psikosen/dba:v1.0.0

# Step 3: Update docker-compose.yml
sed -i 's/ghcr.io\/psikosen\/dba:latest/ghcr.io\/psikosen\/dba:v1.0.0/' docker-compose.yml

# Step 4: Start with previous version
docker-compose up -d

# Step 5: Verify rollback successful
docker-compose ps
docker-compose logs --tail=50
```

### Manual Rollback (Binary)

```bash
# Step 1: Stop service
sudo systemctl stop shaman-journey

# Step 2: Restore backup binary
sudo cp /usr/local/bin/bevy_shaman.backup /usr/local/bin/bevy_shaman

# Step 3: Restore save data (if corrupted)
rsync -avz /backups/saves-latest/ /app/saves/

# Step 4: Start service
sudo systemctl start shaman-journey

# Step 5: Verify service
sudo systemctl status shaman-journey
```

### Post-Rollback Actions

```bash
# 1. Notify team
echo "⚠️  ROLLBACK COMPLETED
Previous version: v1.0.0 restored
Reason: [describe reason]
Status: Service healthy
Next steps: Root cause analysis"

# 2. Preserve logs from failed deployment
docker logs shaman-journey > /var/log/shaman-journey-failed-$(date +%Y%m%d-%H%M%S).log

# 3. Create incident report
# Document:
# - What went wrong
# - Why it wasn't caught in testing
# - Rollback timeline
# - Prevention measures
```

---

## Monitoring & Alerting

### Real-Time Monitoring

#### Grafana Dashboards

**Access:** http://production-server:3000

**Key Dashboards:**
1. **Performance Dashboard**
   - FPS (target: > 60)
   - Frame time P95 (target: < 16ms)
   - Entity count

2. **Game Metrics Dashboard**
   - Monster spawn rate
   - Corruption level
   - Player deaths
   - Encounters

3. **System Dashboard**
   - CPU usage (alert: > 80%)
   - Memory usage (alert: > 90%)
   - Disk usage

#### Prometheus Queries

```promql
# Check current FPS
bevy_shaman_fps

# Check error rate (last 5 minutes)
rate(bevy_shaman_errors_total[5m])

# Check P95 frame time
histogram_quantile(0.95, bevy_shaman_frame_time_seconds)

# Check entity count
bevy_shaman_entity_count
```

### Alert Configuration

**Critical Alerts** (page on-call):
- Service down for > 5 minutes
- Error rate > 10% for > 5 minutes
- Memory usage > 95%
- Disk usage > 90%

**Warning Alerts** (notify team):
- FPS drops below 30 for > 10 minutes
- Error rate > 5% for > 10 minutes
- Memory usage > 80%
- Unusual entity count spike

### Log Monitoring

```bash
# Follow application logs
docker-compose logs -f --tail=100 shaman-journey

# Search for errors
docker-compose logs shaman-journey | grep -i error

# Search for panics
docker-compose logs shaman-journey | grep -i panic

# Filter by log level
docker-compose logs shaman-journey | grep "ERROR\|FATAL"
```

---

## Troubleshooting

### Container Won't Start

**Symptom:** `docker-compose ps` shows container as "Exited" or "Restarting"

**Diagnosis:**
```bash
# Check logs for errors
docker-compose logs shaman-journey

# Check container exit code
docker inspect shaman-journey | grep "ExitCode"
```

**Common Causes:**
1. **Missing environment variables**
   - Fix: Check `.env` file exists and has required vars
   ```bash
   docker-compose config  # Validates env vars
   ```

2. **Port already in use**
   - Check: `sudo lsof -i :8080`
   - Fix: Stop conflicting service or change port

3. **Permission issues**
   - Check: Volume mount permissions
   ```bash
   ls -la /app/saves
   sudo chown -R 1000:1000 /app/saves
   ```

### High Memory Usage

**Symptom:** Memory usage > 2GB and climbing

**Diagnosis:**
```bash
# Check current usage
docker stats shaman-journey --no-stream

# Check for memory leaks in metrics
curl http://localhost:9091/metrics | grep memory
```

**Mitigation:**
```bash
# Restart container to reclaim memory
docker-compose restart shaman-journey

# Set memory limit
# In docker-compose.yml:
# mem_limit: 2g
```

### Performance Degradation

**Symptom:** FPS drops below 60, high frame times

**Diagnosis:**
```bash
# Check system resources
top
htop

# Check entity count (too many entities?)
curl http://localhost:9091/metrics | grep entity_count

# Check for CPU throttling
dmesg | grep -i throttl
```

**Fixes:**
1. **Reduce entity count** (game logic issue)
2. **Increase CPU allocation** (resource constraint)
3. **Check for background processes** competing for CPU

### Connection Issues

**Symptom:** Cannot connect to application

**Diagnosis:**
```bash
# Check container is running
docker ps | grep shaman-journey

# Check port binding
sudo netstat -tlnp | grep 8080

# Check firewall
sudo ufw status
sudo iptables -L
```

**Fixes:**
```bash
# Restart container
docker-compose restart shaman-journey

# Check Docker network
docker network inspect shaman-journey_default

# Verify health check
docker exec shaman-journey pgrep bevy_shaman
```

---

## Emergency Procedures

### Complete Service Outage

**Severity: Critical**

**Immediate Actions (0-5 min):**
1. ✅ Confirm outage (check monitoring, health checks)
2. ✅ Page on-call engineer
3. ✅ Post status update: "Investigating service outage"
4. ✅ Check container status: `docker-compose ps`
5. ✅ Check logs: `docker-compose logs --tail=200`

**Diagnosis (5-15 min):**
```bash
# Check if it's a deployment issue
git log -1  # Recent changes?

# Check system resources
free -h
df -h
top

# Check Docker daemon
sudo systemctl status docker

# Check network connectivity
ping 8.8.8.8
```

**Recovery (15-30 min):**
```bash
# Option 1: Restart service
docker-compose restart shaman-journey

# Option 2: Rollback deployment
# Follow rollback procedure above

# Option 3: Restore from backup
# Follow disaster recovery procedure
```

### Data Corruption

**Severity: High**

**Immediate Actions:**
1. ✅ Stop writes to prevent further corruption
   ```bash
   docker-compose stop shaman-journey
   ```
2. ✅ Backup current state (even if corrupted)
   ```bash
   rsync -avz /app/saves/ /backups/corrupted-$(date +%Y%m%d-%H%M%S)/
   ```
3. ✅ Identify scope of corruption
   ```bash
   # Check save files
   find /app/saves -type f -exec file {} \;
   ```

**Recovery:**
```bash
# Restore from last known good backup
rsync -avz /backups/saves-[timestamp]/ /app/saves/

# Verify integrity
# (add validation logic)

# Restart service
docker-compose start shaman-journey
```

### Security Incident

**Severity: Critical**

**Immediate Actions:**
1. ✅ Isolate affected systems
2. ✅ Rotate all secrets immediately
3. ✅ Review access logs
4. ✅ Notify security team
5. ✅ Follow incident response plan

**Do NOT:**
- ❌ Destroy evidence (preserve logs)
- ❌ Panic and make hasty decisions
- ❌ Fail to notify appropriate parties

---

## Deployment Schedule

### Recommended Schedule

**Production Deployments:**
- **When:** Tuesday or Wednesday, 10 AM - 2 PM (local time)
- **Why:** Mid-week allows rollback time before weekend
- **Avoid:** Fridays (risk of weekend incident), Mondays (post-weekend)

**Emergency Deployments:**
- **When:** As needed, any time
- **Requires:** On-call engineer approval
- **Process:** Expedited but still follow checklist

### Maintenance Windows

**Planned Maintenance:**
- **Schedule:** First Sunday of month, 2 AM - 4 AM
- **Notification:** 1 week advance notice
- **Use for:** Major updates, infrastructure changes

---

## Appendix

### Deployment Checklist (Printable)

```
PRE-DEPLOYMENT
[ ] Git status clean
[ ] CI passing
[ ] Tests passing locally
[ ] Docker image built
[ ] Secrets verified
[ ] Backup created
[ ] Team notified

DEPLOYMENT
[ ] Pull latest code/image
[ ] Rolling update executed
[ ] New container healthy
[ ] Old container stopped
[ ] Logs show no errors

POST-DEPLOYMENT
[ ] Health check passing
[ ] Metrics normal
[ ] Sentry clean
[ ] Smoke tests passed
[ ] Performance baseline met
[ ] Team notified of completion

ROLLBACK (IF NEEDED)
[ ] Issue identified
[ ] Rollback decision made
[ ] Previous version restored
[ ] Service healthy
[ ] Incident report created
```

### Useful Commands

```bash
# Quick health check
docker ps && curl -s http://localhost:9091/metrics | head

# Full system status
docker-compose ps && docker stats --no-stream

# Tail all logs
docker-compose logs -f --tail=100

# Emergency restart
docker-compose restart shaman-journey

# Emergency rollback
docker-compose down && git checkout HEAD~1 && docker-compose up -d
```

---

**Document Maintenance:**
- Review quarterly
- Update after each deployment incident
- Incorporate lessons learned

**Last Reviewed:** December 28, 2025
**Next Review:** March 28, 2026
