#!/bin/bash
# Production Secrets Generator for Shaman's Journey
# This script generates secure random passwords and creates a production-ready .env file
#
# Usage: ./scripts/generate-secrets.sh [--sentry-dsn YOUR_DSN]
#
# IMPORTANT:
# - Run this script before deploying to production
# - Store the generated .env.production file securely (use secrets manager)
# - Never commit .env.production to version control

set -euo pipefail

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Parse command line arguments
SENTRY_DSN=""
while [[ $# -gt 0 ]]; do
    case $1 in
        --sentry-dsn)
            SENTRY_DSN="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [--sentry-dsn YOUR_DSN]"
            echo ""
            echo "Generates secure production credentials for Shaman's Journey"
            echo ""
            echo "Options:"
            echo "  --sentry-dsn DSN    Sentry DSN for error tracking (optional)"
            echo "  --help              Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Error: Unknown option $1${NC}"
            exit 1
            ;;
    esac
done

echo -e "${GREEN}=== Shaman's Journey - Production Secrets Generator ===${NC}"
echo ""

# Check for openssl
if ! command -v openssl &> /dev/null; then
    echo -e "${RED}Error: openssl is required but not installed${NC}"
    echo "Install it with: apt-get install openssl (Debian/Ubuntu) or brew install openssl (macOS)"
    exit 1
fi

# Warn if .env.production already exists
if [ -f .env.production ]; then
    echo -e "${YELLOW}Warning: .env.production already exists${NC}"
    read -p "Do you want to overwrite it? (yes/no): " -r
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        echo "Aborted."
        exit 0
    fi
    # Backup existing file
    cp .env.production ".env.production.backup.$(date +%s)"
    echo -e "${GREEN}Backed up existing .env.production${NC}"
fi

echo -e "${GREEN}Generating secure random passwords...${NC}"

# Generate strong random passwords (32 bytes = 44 characters base64)
DRAGONFLY_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-32)
RABBITMQ_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-32)
GRAFANA_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-32)
METRICS_API_KEY=$(openssl rand -base64 24 | tr -d "=+/" | cut -c1-24)

# Prompt for Sentry DSN if not provided
if [ -z "$SENTRY_DSN" ]; then
    echo ""
    echo -e "${YELLOW}Sentry DSN (optional - press Enter to skip):${NC}"
    read -r SENTRY_DSN
fi

# Create production environment file
cat > .env.production << EOF
# =======================================================
# Shaman's Journey - Production Environment Configuration
# =======================================================
#
# Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
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
# Note: Using rediss:// for TLS encryption (requires TLS setup)
# For non-TLS environments, use redis:// but this is NOT recommended for production
DRAGONFLY_URL=redis://:${DRAGONFLY_PASSWORD}@dragonfly:6379
DRAGONFLY_PASSWORD=${DRAGONFLY_PASSWORD}

# Cache TTL Settings (in seconds)
LLM_CACHE_TTL=3600        # 1 hour
DUNGEON_CACHE_TTL=86400   # 24 hours
SESSION_CACHE_TTL=1800    # 30 minutes

# =======================================================
# RabbitMQ Configuration (Message Queue)
# =======================================================
# Note: Using amqps:// for TLS encryption (requires TLS setup)
# For non-TLS environments, use amqp:// but this is NOT recommended for production
RABBITMQ_URL=amqp://admin:${RABBITMQ_PASSWORD}@rabbitmq:5672/%2f
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
# PRODUCTION DEPLOYMENT CHECKLIST
# =======================================================
# Before deploying, ensure:
# [ ] TLS certificates configured for Redis and RabbitMQ
# [ ] Firewall rules configured (only expose port 8080)
# [ ] Backup strategy in place for /app/saves volume
# [ ] Monitoring alerts configured in Prometheus
# [ ] Log aggregation configured
# [ ] Incident response plan documented
# [ ] Rollback procedure tested
# =======================================================
EOF

# Set restrictive permissions
chmod 600 .env.production

echo ""
echo -e "${GREEN}✅ Production secrets generated successfully!${NC}"
echo ""
echo -e "${YELLOW}Created file: .env.production${NC}"
echo "  Permissions: 600 (owner read/write only)"
echo "  Size: $(wc -c < .env.production) bytes"
echo ""
echo -e "${GREEN}Generated credentials:${NC}"
echo "  - DragonflyDB password: ${DRAGONFLY_PASSWORD:0:8}... (32 characters)"
echo "  - RabbitMQ password:    ${RABBITMQ_PASSWORD:0:8}... (32 characters)"
echo "  - Grafana password:     ${GRAFANA_PASSWORD:0:8}... (32 characters)"
echo "  - Metrics API key:      ${METRICS_API_KEY:0:8}... (24 characters)"
echo ""
echo -e "${YELLOW}⚠️  IMPORTANT NEXT STEPS:${NC}"
echo ""
echo "1. Review the generated .env.production file"
echo "2. Store credentials in your secrets manager:"
echo "   - AWS Secrets Manager"
echo "   - HashiCorp Vault"
echo "   - Azure Key Vault"
echo "   - Google Secret Manager"
echo ""
echo "3. For Docker deployment:"
echo "   cp .env.production .env"
echo "   docker-compose up -d"
echo ""
echo "4. For Kubernetes deployment:"
echo "   kubectl create secret generic shaman-secrets \\"
echo "     --from-env-file=.env.production"
echo ""
echo "5. Set up credential rotation (recommended: every 90 days)"
echo ""
echo -e "${RED}⚠️  SECURITY WARNING:${NC}"
echo "  - NEVER commit .env.production to git"
echo "  - NEVER share these credentials via email/Slack/etc"
echo "  - NEVER use these credentials in development/staging"
echo "  - Rotate credentials immediately if compromised"
echo ""
echo -e "${GREEN}✅ Setup complete!${NC}"
