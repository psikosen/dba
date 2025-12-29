#!/bin/bash
# Phase 1: Production Security Hardening Script
# Shaman's Journey - Complete Pre-Production Setup
#
# This script implements all Phase 1 security requirements:
# 1. Generate strong passwords and secrets
# 2. Create TLS certificates for services
# 3. Update configuration files for production
# 4. Validate environment setup
# 5. Output secrets for secure storage
#
# Usage: ./scripts/phase1_production_hardening.sh [OPTIONS]
#
# Options:
#   --sentry-dsn DSN    Sentry DSN for error tracking (optional)
#   --domain DOMAIN     Domain name for TLS certificates (default: localhost)
#   --skip-certs        Skip TLS certificate generation
#   --help              Show this help message

set -euo pipefail

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Default values
SENTRY_DSN=""
DOMAIN="localhost"
SKIP_CERTS=false
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --sentry-dsn)
            SENTRY_DSN="$2"
            shift 2
            ;;
        --domain)
            DOMAIN="$2"
            shift 2
            ;;
        --skip-certs)
            SKIP_CERTS=true
            shift
            ;;
        --help)
            cat <<EOF
Phase 1: Production Security Hardening

This script performs complete production security setup:
- Generates strong random passwords (32+ characters)
- Creates TLS certificates for encrypted communication
- Updates Docker Compose configurations
- Creates production-ready environment files
- Outputs all secrets for secure storage

Usage: $0 [OPTIONS]

Options:
  --sentry-dsn DSN    Sentry DSN for error tracking (optional)
  --domain DOMAIN     Domain name for TLS certificates (default: localhost)
  --skip-certs        Skip TLS certificate generation
  --help              Show this help message

Examples:
  # Basic setup
  $0

  # With Sentry and custom domain
  $0 --sentry-dsn "https://...@sentry.io/..." --domain "game.example.com"

  # Skip certificate generation (use existing certs)
  $0 --skip-certs

EOF
            exit 0
            ;;
        *)
            echo -e "${RED}Error: Unknown option $1${NC}"
            exit 1
            ;;
    esac
done

# Navigate to project root
cd "$PROJECT_ROOT"

echo -e "${CYAN}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                                                           ║${NC}"
echo -e "${CYAN}║  ${GREEN}Shaman's Journey - Phase 1 Production Hardening${CYAN}      ║${NC}"
echo -e "${CYAN}║                                                           ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check prerequisites
echo -e "${BLUE}[1/8] Checking prerequisites...${NC}"

if ! command -v openssl &> /dev/null; then
    echo -e "${RED}Error: openssl is required but not installed${NC}"
    echo "Install it with: apt-get install openssl (Debian/Ubuntu) or brew install openssl (macOS)"
    exit 1
fi

if ! command -v docker &> /dev/null; then
    echo -e "${YELLOW}Warning: docker not found (optional for local testing)${NC}"
fi

echo -e "${GREEN}✓ Prerequisites check passed${NC}"
echo ""

# Backup existing files
echo -e "${BLUE}[2/8] Backing up existing configuration...${NC}"

BACKUP_DIR=".backups/phase1_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"

if [ -f .env.production ]; then
    cp .env.production "$BACKUP_DIR/"
    echo -e "${GREEN}✓ Backed up .env.production${NC}"
fi

if [ -f docker-compose.yml ]; then
    cp docker-compose.yml "$BACKUP_DIR/"
    echo -e "${GREEN}✓ Backed up docker-compose.yml${NC}"
fi

echo -e "${GREEN}✓ Backups saved to $BACKUP_DIR${NC}"
echo ""

# Generate strong passwords
echo -e "${BLUE}[3/8] Generating strong random passwords...${NC}"

DRAGONFLY_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-32)
RABBITMQ_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-32)
GRAFANA_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-32)
METRICS_API_KEY=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-32)

echo -e "${GREEN}✓ Generated 4 strong passwords (32 characters each)${NC}"
echo ""

# Generate TLS certificates
if [ "$SKIP_CERTS" = false ]; then
    echo -e "${BLUE}[4/8] Generating TLS certificates...${NC}"

    mkdir -p certs

    # Generate CA certificate
    echo -e "  ${CYAN}→ Generating Certificate Authority...${NC}"
    openssl genrsa -out certs/ca-key.pem 4096 2>/dev/null
    openssl req -new -x509 -days 3650 -key certs/ca-key.pem -out certs/ca-cert.pem \
        -subj "/C=US/ST=State/L=City/O=Shaman Journey/CN=Shaman Journey CA" 2>/dev/null

    # Generate DragonflyDB certificate
    echo -e "  ${CYAN}→ Generating DragonflyDB certificate...${NC}"
    openssl genrsa -out certs/dragonfly-key.pem 2048 2>/dev/null
    openssl req -new -key certs/dragonfly-key.pem -out certs/dragonfly.csr \
        -subj "/C=US/ST=State/L=City/O=Shaman Journey/CN=dragonfly" 2>/dev/null
    openssl x509 -req -days 3650 -in certs/dragonfly.csr -CA certs/ca-cert.pem \
        -CAkey certs/ca-key.pem -CAcreateserial -out certs/dragonfly-cert.pem 2>/dev/null
    rm certs/dragonfly.csr

    # Generate RabbitMQ certificate
    echo -e "  ${CYAN}→ Generating RabbitMQ certificate...${NC}"
    openssl genrsa -out certs/rabbitmq-key.pem 2048 2>/dev/null
    openssl req -new -key certs/rabbitmq-key.pem -out certs/rabbitmq.csr \
        -subj "/C=US/ST=State/L=City/O=Shaman Journey/CN=rabbitmq" 2>/dev/null
    openssl x509 -req -days 3650 -in certs/rabbitmq.csr -CA certs/ca-cert.pem \
        -CAkey certs/ca-key.pem -CAcreateserial -out certs/rabbitmq-cert.pem 2>/dev/null
    rm certs/rabbitmq.csr

    # Set permissions
    chmod 600 certs/*.pem

    echo -e "${GREEN}✓ TLS certificates generated in ./certs/${NC}"
    echo -e "  ${GREEN}• CA: certs/ca-cert.pem${NC}"
    echo -e "  ${GREEN}• DragonflyDB: certs/dragonfly-cert.pem, certs/dragonfly-key.pem${NC}"
    echo -e "  ${GREEN}• RabbitMQ: certs/rabbitmq-cert.pem, certs/rabbitmq-key.pem${NC}"
else
    echo -e "${YELLOW}[4/8] Skipping TLS certificate generation (--skip-certs)${NC}"
fi
echo ""

# Prompt for Sentry DSN if not provided
if [ -z "$SENTRY_DSN" ]; then
    echo -e "${YELLOW}Sentry DSN for error tracking (optional - press Enter to skip):${NC}"
    read -r SENTRY_DSN
fi

# Create production environment file
echo -e "${BLUE}[5/8] Creating production environment file...${NC}"

cat > .env.production << EOF
# =======================================================
# Shaman's Journey - Production Environment Configuration
# =======================================================
#
# Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
# By: Phase 1 Production Hardening Script
#
# ⚠️  CRITICAL SECURITY NOTICE ⚠️
# - This file contains sensitive credentials
# - NEVER commit this file to version control
# - Store securely in a secrets manager (Vault, AWS Secrets Manager, etc.)
# - Rotate credentials every 90 days
# - Use different credentials for each environment (staging, production)
#
# =======================================================

# Environment
ENVIRONMENT=production

# Rust Configuration
RUST_LOG=info,bevy_shaman=warn
RUST_BACKTRACE=0

# =======================================================
# DragonflyDB Configuration (Redis-compatible cache)
# =======================================================
# Using rediss:// for TLS encryption (REQUIRED in production)
DRAGONFLY_URL=rediss://:${DRAGONFLY_PASSWORD}@dragonfly:6380
DRAGONFLY_PASSWORD=${DRAGONFLY_PASSWORD}

# Cache TTL Settings (in seconds)
LLM_CACHE_TTL=3600        # 1 hour
DUNGEON_CACHE_TTL=86400   # 24 hours
SESSION_CACHE_TTL=1800    # 30 minutes

# =======================================================
# RabbitMQ Configuration (Message Queue)
# =======================================================
# Using amqps:// for TLS encryption (REQUIRED in production)
RABBITMQ_URL=amqps://admin:${RABBITMQ_PASSWORD}@rabbitmq:5671/%2f
RABBITMQ_USER=admin
RABBITMQ_PASSWORD=${RABBITMQ_PASSWORD}

# Queue Configuration
BACKGROUND_JOBS_QUEUE=background_jobs
EVENT_BUS_EXCHANGE=game_events
OFFLINE_JOBS_QUEUE=offline_jobs

# =======================================================
# Monitoring Configuration
# =======================================================
# Prometheus
PROMETHEUS_PORT=9091
METRICS_API_KEY=${METRICS_API_KEY}

# Grafana (Admin Dashboard)
GRAFANA_ADMIN_USER=admin
GRAFANA_ADMIN_PASSWORD=${GRAFANA_PASSWORD}

# =======================================================
# Error Tracking (Sentry)
# =======================================================
SENTRY_DSN=${SENTRY_DSN}
# Leave empty to disable error tracking

# =======================================================
# Game Configuration
# =======================================================
# Asset and save file paths (use Docker volumes)
ASSETS_PATH=/app/assets
SAVES_PATH=/app/saves

# =======================================================
# LLM Configuration (Optional)
# =======================================================
# Uncomment and configure if using LLM features
# LLM_MODEL_PATH=/models/llama-model.gguf
# LLM_CONTEXT_SIZE=2048

# =======================================================
# TLS/SSL Configuration
# =======================================================
TLS_CA_CERT=/certs/ca-cert.pem
TLS_DRAGONFLY_CERT=/certs/dragonfly-cert.pem
TLS_DRAGONFLY_KEY=/certs/dragonfly-key.pem
TLS_RABBITMQ_CERT=/certs/rabbitmq-cert.pem
TLS_RABBITMQ_KEY=/certs/rabbitmq-key.pem
EOF

chmod 600 .env.production

echo -e "${GREEN}✓ Production environment file created: .env.production${NC}"
echo ""

# Create production docker-compose override
echo -e "${BLUE}[6/8] Creating production Docker Compose override...${NC}"

cat > docker-compose.production.yml << 'EOFCOMPOSE'
version: '3.8'

# Production override for docker-compose.yml
# Usage: docker-compose -f docker-compose.yml -f docker-compose.production.yml up -d

services:
  dragonfly:
    # TLS Configuration
    volumes:
      - ./certs/ca-cert.pem:/certs/ca-cert.pem:ro
      - ./certs/dragonfly-cert.pem:/certs/dragonfly-cert.pem:ro
      - ./certs/dragonfly-key.pem:/certs/dragonfly-key.pem:ro
    command: >
      dragonfly
      --requirepass ${DRAGONFLY_PASSWORD:?DRAGONFLY_PASSWORD must be set}
      --tls
      --tls_cert_file /certs/dragonfly-cert.pem
      --tls_key_file /certs/dragonfly-key.pem
      --tls_ca_cert_file /certs/ca-cert.pem
      --port 6380
      --maxmemory 512mb
      --save_schedule "*:*"
    ports:
      - "127.0.0.1:6380:6380"  # TLS port
    healthcheck:
      test: ["CMD", "redis-cli", "-a", "${DRAGONFLY_PASSWORD:?Required}", "--tls", "--cacert", "/certs/ca-cert.pem", "ping"]

  rabbitmq:
    # TLS Configuration
    volumes:
      - ./certs/ca-cert.pem:/certs/ca-cert.pem:ro
      - ./certs/rabbitmq-cert.pem:/certs/rabbitmq-cert.pem:ro
      - ./certs/rabbitmq-key.pem:/certs/rabbitmq-key.pem:ro
      - ./monitoring/rabbitmq/rabbitmq.conf:/etc/rabbitmq/rabbitmq.conf:ro
    environment:
      - RABBITMQ_DEFAULT_USER=${RABBITMQ_USER:?RABBITMQ_USER must be set}
      - RABBITMQ_DEFAULT_PASS=${RABBITMQ_PASSWORD:?RABBITMQ_PASSWORD must be set}
      - RABBITMQ_SSL_CACERTFILE=/certs/ca-cert.pem
      - RABBITMQ_SSL_CERTFILE=/certs/rabbitmq-cert.pem
      - RABBITMQ_SSL_KEYFILE=/certs/rabbitmq-key.pem
    ports:
      - "127.0.0.1:5671:5671"  # TLS port
      - "127.0.0.1:15672:15672"  # Management UI

  grafana:
    environment:
      - GF_SECURITY_ADMIN_USER=${GRAFANA_ADMIN_USER:-admin}
      - GF_SECURITY_ADMIN_PASSWORD=${GRAFANA_ADMIN_PASSWORD:?GRAFANA_ADMIN_PASSWORD must be set}
      - GF_USERS_ALLOW_SIGN_UP=false
      - GF_SERVER_ROOT_URL=http://localhost:3000
EOFCOMPOSE

echo -e "${GREEN}✓ Production Docker Compose override created: docker-compose.production.yml${NC}"
echo ""

# Create RabbitMQ configuration for TLS
echo -e "${BLUE}[7/8] Creating RabbitMQ TLS configuration...${NC}"

mkdir -p monitoring/rabbitmq

cat > monitoring/rabbitmq/rabbitmq.conf << 'EOFRABBIT'
# RabbitMQ Production Configuration with TLS

# Enable TLS
listeners.ssl.default = 5671

ssl_options.cacertfile = /certs/ca-cert.pem
ssl_options.certfile   = /certs/rabbitmq-cert.pem
ssl_options.keyfile    = /certs/rabbitmq-key.pem

ssl_options.verify     = verify_peer
ssl_options.fail_if_no_peer_cert = false

# Disable non-TLS listener in production (optional - uncomment for strict TLS)
# listeners.tcp = none

# Management plugin
management.tcp.port = 15672
management.ssl.port = 15671
management.ssl.cacertfile = /certs/ca-cert.pem
management.ssl.certfile   = /certs/rabbitmq-cert.pem
management.ssl.keyfile    = /certs/rabbitmq-key.pem

# Performance tuning
vm_memory_high_watermark.relative = 0.6
disk_free_limit.absolute = 2GB
EOFRABBIT

echo -e "${GREEN}✓ RabbitMQ TLS configuration created${NC}"
echo ""

# Output secrets summary
echo -e "${BLUE}[8/8] Generating secrets summary...${NC}"

SECRETS_FILE="PRODUCTION_SECRETS_$(date +%Y%m%d_%H%M%S).txt"

cat > "$SECRETS_FILE" << EOF
═══════════════════════════════════════════════════════════
  PRODUCTION SECRETS - STORE IN SECRETS MANAGER
═══════════════════════════════════════════════════════════

Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Environment: PRODUCTION

⚠️  CRITICAL SECURITY WARNING ⚠️
- Store these credentials in a secure secrets manager
- NEVER commit this file to version control
- DELETE this file after storing secrets
- Rotate credentials every 90 days

───────────────────────────────────────────────────────────
DRAGONFLY (REDIS) CREDENTIALS
───────────────────────────────────────────────────────────
URL:      rediss://:${DRAGONFLY_PASSWORD}@dragonfly:6380
Password: ${DRAGONFLY_PASSWORD}

───────────────────────────────────────────────────────────
RABBITMQ CREDENTIALS
───────────────────────────────────────────────────────────
URL:      amqps://admin:${RABBITMQ_PASSWORD}@rabbitmq:5671/%2f
Username: admin
Password: ${RABBITMQ_PASSWORD}

───────────────────────────────────────────────────────────
GRAFANA CREDENTIALS
───────────────────────────────────────────────────────────
URL:      http://localhost:3000
Username: admin
Password: ${GRAFANA_PASSWORD}

───────────────────────────────────────────────────────────
METRICS API KEY
───────────────────────────────────────────────────────────
API Key:  ${METRICS_API_KEY}

───────────────────────────────────────────────────────────
SENTRY (ERROR TRACKING)
───────────────────────────────────────────────────────────
DSN:      ${SENTRY_DSN:-<not configured>}

───────────────────────────────────────────────────────────
TLS CERTIFICATES LOCATION
───────────────────────────────────────────────────────────
CA Cert:           ./certs/ca-cert.pem
DragonflyDB Cert:  ./certs/dragonfly-cert.pem
DragonflyDB Key:   ./certs/dragonfly-key.pem
RabbitMQ Cert:     ./certs/rabbitmq-cert.pem
RabbitMQ Key:      ./certs/rabbitmq-key.pem

───────────────────────────────────────────────────────────
NEXT STEPS
───────────────────────────────────────────────────────────

1. IMMEDIATELY store these credentials in your secrets manager:
   □ AWS Secrets Manager: aws secretsmanager create-secret ...
   □ HashiCorp Vault: vault kv put secret/shaman-journey ...
   □ Azure Key Vault: az keyvault secret set ...
   □ Google Secret Manager: gcloud secrets create ...

2. DELETE this file after storing secrets:
   rm -f ${SECRETS_FILE}

3. Deploy to production:
   cp .env.production .env
   docker-compose -f docker-compose.yml -f docker-compose.production.yml up -d

4. Verify deployment:
   ./scripts/health_check.sh

5. Test TLS connections:
   redis-cli --tls --cacert ./certs/ca-cert.pem -h localhost -p 6380 ping

6. Schedule credential rotation (90 days):
   Add to calendar: $(date -d '+90 days' +%Y-%m-%d 2>/dev/null || date -v+90d +%Y-%m-%d 2>/dev/null || echo 'in 90 days')

───────────────────────────────────────────────────────────
KUBERNETES DEPLOYMENT (Optional)
───────────────────────────────────────────────────────────

kubectl create secret generic shaman-secrets \\
  --from-literal=dragonfly-password=${DRAGONFLY_PASSWORD} \\
  --from-literal=rabbitmq-password=${RABBITMQ_PASSWORD} \\
  --from-literal=grafana-password=${GRAFANA_PASSWORD} \\
  --from-literal=metrics-api-key=${METRICS_API_KEY} \\
  --from-literal=sentry-dsn='${SENTRY_DSN}'

═══════════════════════════════════════════════════════════
EOF

chmod 600 "$SECRETS_FILE"

echo -e "${GREEN}✓ Secrets summary created: ${SECRETS_FILE}${NC}"
echo ""

# Final summary
echo -e "${CYAN}╔═══════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                                                           ║${NC}"
echo -e "${CYAN}║            ${GREEN}✓ Phase 1 Hardening Complete!${CYAN}                  ║${NC}"
echo -e "${CYAN}║                                                           ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${GREEN}Summary of changes:${NC}"
echo -e "  ✓ Generated 4 strong passwords (32 characters each)"
if [ "$SKIP_CERTS" = false ]; then
    echo -e "  ✓ Created TLS certificates for encrypted communication"
fi
echo -e "  ✓ Created production environment file (.env.production)"
echo -e "  ✓ Created Docker Compose production override"
echo -e "  ✓ Created RabbitMQ TLS configuration"
echo -e "  ✓ Generated secrets summary for secure storage"
echo ""

echo -e "${YELLOW}⚠️  IMPORTANT NEXT STEPS:${NC}"
echo ""
echo -e "1. ${MAGENTA}IMMEDIATELY${NC} store credentials from: ${GREEN}${SECRETS_FILE}${NC}"
echo -e "   Use your organization's secrets manager (Vault, AWS, Azure, etc.)"
echo ""
echo -e "2. ${MAGENTA}DELETE${NC} the secrets file after storing:"
echo -e "   ${CYAN}rm -f ${SECRETS_FILE}${NC}"
echo ""
echo -e "3. Review the production configuration:"
echo -e "   ${CYAN}cat .env.production${NC}"
echo ""
echo -e "4. Deploy to production:"
echo -e "   ${CYAN}cp .env.production .env${NC}"
echo -e "   ${CYAN}docker-compose -f docker-compose.yml -f docker-compose.production.yml up -d${NC}"
echo ""
echo -e "5. Verify deployment health:"
echo -e "   ${CYAN}./scripts/health_check.sh${NC}"
echo ""
echo -e "6. Test TLS encryption:"
echo -e "   ${CYAN}redis-cli --tls --cacert ./certs/ca-cert.pem -h localhost -p 6380 ping${NC}"
echo ""

echo -e "${RED}⚠️  SECURITY REMINDERS:${NC}"
echo -e "  • NEVER commit .env.production or ${SECRETS_FILE} to git"
echo -e "  • NEVER share credentials via email/Slack/etc."
echo -e "  • Rotate credentials every 90 days"
echo -e "  • Monitor for security alerts in Prometheus/Grafana"
echo ""

echo -e "${GREEN}✅ Production hardening complete!${NC}"
echo -e "${GREEN}Your application is now ready for secure production deployment.${NC}"
echo ""
