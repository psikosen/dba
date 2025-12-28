# Secrets Management Guide

This document outlines how secrets and sensitive configuration are managed in Shaman's Journey.

## Table of Contents

1. [Overview](#overview)
2. [What Are Secrets?](#what-are-secrets)
3. [Local Development](#local-development)
4. [Staging Environment](#staging-environment)
5. [Production Environment](#production-environment)
6. [Docker Secrets](#docker-secrets)
7. [Secret Rotation](#secret-rotation)
8. [Access Control](#access-control)
9. [Emergency Procedures](#emergency-procedures)

---

## Overview

**Golden Rules:**
- ✅ **NEVER** commit secrets to version control
- ✅ **ALWAYS** use environment variables for secrets
- ✅ **ALWAYS** use `.env.example` to document required variables
- ✅ **ALWAYS** add new secret files to `.gitignore`

**What We Protect:**
- Sentry DSN (error tracking)
- Grafana admin credentials
- API keys (future: LLM API keys, cloud services)
- Encryption keys (future)
- Database credentials (future)

---

## What Are Secrets?

Secrets are sensitive pieces of information that should not be publicly exposed:

### Current Secrets
1. **Sentry DSN** (`SENTRY_DSN`)
   - Purpose: Error tracking and crash reporting
   - Format: `https://<key>@o<org-id>.ingest.sentry.io/<project-id>`
   - Sensitivity: Medium (can be rate-limited, but prefer to keep private)

2. **Grafana Credentials** (`GRAFANA_ADMIN_USER`, `GRAFANA_ADMIN_PASSWORD`)
   - Purpose: Grafana dashboard access
   - Format: Username/password strings
   - Sensitivity: High (admin access to monitoring)

### Future Secrets (Planned)
3. **LLM API Keys**
   - For cloud-based LLM services
   - Sensitivity: High (metered usage, costs money)

4. **Encryption Keys**
   - For save game encryption
   - Sensitivity: Critical (data integrity)

### Not Secrets (Safe to Commit)
- Port numbers (unless non-standard)
- Service names
- Feature flags (boolean configs)
- Log levels
- Non-sensitive paths

---

## Local Development

### Setup

1. **Copy the example environment file:**
   ```bash
   cp .env.example .env
   ```

2. **Fill in your development values:**
   ```bash
   # .env (local development)
   SENTRY_DSN=https://your-dev-key@sentry.io/your-dev-project
   ENVIRONMENT=development
   RUST_LOG=debug
   RUST_BACKTRACE=1
   GRAFANA_ADMIN_USER=admin
   GRAFANA_ADMIN_PASSWORD=local_dev_password
   ```

3. **Verify .env is gitignored:**
   ```bash
   git status
   # Should NOT show .env as untracked
   ```

### Development Sentry Project

**Recommendation:** Use a separate Sentry project for development
- Prevents dev errors from polluting production metrics
- Allows different alert thresholds
- Free tier should be sufficient

**Setup:**
1. Go to https://sentry.io
2. Create new project: "shaman-journey-dev"
3. Copy DSN to `.env`

### Local Grafana Credentials

For local development, default credentials are acceptable:
```bash
GRAFANA_ADMIN_USER=admin
GRAFANA_ADMIN_PASSWORD=admin
```

**Important:** Change these for staging/production!

---

## Staging Environment

### Environment File

Staging uses a separate `.env.staging` file:

```bash
# .env.staging (committed to version control)
ENVIRONMENT=staging
RUST_LOG=info,bevy_shaman=debug
PROMETHEUS_PORT=9091

# Secrets are loaded separately
# SENTRY_DSN: Loaded from secret store
# GRAFANA_ADMIN_PASSWORD: Loaded from secret store
```

### Loading Secrets

**Option 1: Docker Secrets** (Recommended)
```yaml
# docker-compose.staging.yml
services:
  shaman-journey-staging:
    secrets:
      - sentry_dsn
      - grafana_password

secrets:
  sentry_dsn:
    file: ./secrets/sentry_dsn.txt
  grafana_password:
    file: ./secrets/grafana_password.txt
```

**Option 2: Environment File Override**
```bash
# Load base config + secrets
docker-compose -f docker-compose.staging.yml \
  --env-file .env.staging \
  --env-file .env.staging.secrets \
  up -d
```

Where `.env.staging.secrets` is **NOT** committed:
```bash
# .env.staging.secrets (gitignored!)
SENTRY_DSN=https://staging-key@sentry.io/staging-project
GRAFANA_ADMIN_PASSWORD=strong_staging_password_here
```

---

## Production Environment

### Secret Storage Options

#### Option 1: Cloud Secret Manager (Recommended for Cloud Deployment)

**AWS Secrets Manager:**
```bash
# Store secret
aws secretsmanager create-secret \
  --name shaman-journey/prod/sentry-dsn \
  --secret-string "https://prod-key@sentry.io/prod-project"

# Retrieve in container entrypoint
export SENTRY_DSN=$(aws secretsmanager get-secret-value \
  --secret-id shaman-journey/prod/sentry-dsn \
  --query SecretString \
  --output text)
```

**Google Secret Manager:**
```bash
# Store secret
gcloud secrets create sentry-dsn \
  --data-file=- <<< "https://prod-key@sentry.io/prod-project"

# Retrieve in container
export SENTRY_DSN=$(gcloud secrets versions access latest \
  --secret=sentry-dsn)
```

#### Option 2: Docker Secrets (Recommended for Self-Hosted)

```yaml
# docker-compose.prod.yml
version: '3.8'
services:
  shaman-journey:
    secrets:
      - sentry_dsn
      - grafana_admin_password
    environment:
      - SENTRY_DSN_FILE=/run/secrets/sentry_dsn
      - GRAFANA_PASSWORD_FILE=/run/secrets/grafana_admin_password

secrets:
  sentry_dsn:
    external: true
  grafana_admin_password:
    external: true
```

Create secrets:
```bash
# Create secret from stdin
echo "https://prod-key@sentry.io/prod" | \
  docker secret create sentry_dsn -

# Create from file
docker secret create grafana_admin_password secrets/grafana_pass.txt
```

Update application to read from files:
```rust
pub fn load_sentry_dsn() -> Option<String> {
    // Try file first (Docker secrets)
    if let Ok(path) = env::var("SENTRY_DSN_FILE") {
        std::fs::read_to_string(path).ok()
    } else {
        // Fall back to env var
        env::var("SENTRY_DSN").ok()
    }
}
```

#### Option 3: HashiCorp Vault (Enterprise)

For larger deployments with many secrets:
```bash
# Store secret
vault kv put secret/shaman-journey/prod \
  sentry_dsn="https://prod-key@sentry.io/prod" \
  grafana_password="strong_password"

# Retrieve
vault kv get -field=sentry_dsn secret/shaman-journey/prod
```

### Production Checklist

Before deploying to production:

- [ ] Separate Sentry project created for production
- [ ] Strong Grafana password generated (20+ chars, random)
- [ ] Secrets stored in secret manager (not .env files)
- [ ] `.env` files excluded from deployment artifacts
- [ ] Secret access auditing enabled
- [ ] Backup of secret values stored securely offline
- [ ] Only authorized personnel have secret access
- [ ] Secrets rotation schedule documented

---

## Docker Secrets

### When to Use

Docker Secrets are ideal for:
- Self-hosted deployments
- Docker Swarm or Kubernetes
- Non-cloud environments

### Creating Secrets

```bash
# From stdin (recommended - no file on disk)
echo "my-secret-value" | docker secret create my_secret -

# From file (remove file after!)
docker secret create my_secret ./secret.txt
rm ./secret.txt

# List secrets (values are NOT shown)
docker secret ls

# Remove secret
docker secret rm my_secret
```

### Using in Compose

```yaml
services:
  app:
    secrets:
      - source: db_password
        target: /run/secrets/db_password
        mode: 0400  # Read-only for owner

secrets:
  db_password:
    external: true  # Created outside compose
```

### Reading in Application

```rust
use std::fs;

pub fn load_secret(name: &str) -> Result<String, std::io::Error> {
    let path = format!("/run/secrets/{}", name);
    fs::read_to_string(path)?.trim()
}

// Usage
let db_password = load_secret("db_password")?;
```

---

## Secret Rotation

Secrets should be rotated regularly to minimize exposure risk.

### Rotation Schedule

| Secret | Frequency | Reason |
|--------|-----------|--------|
| Sentry DSN | Yearly or on breach | Low sensitivity, infrequent rotation |
| Grafana Admin | Quarterly | Admin access, regular rotation |
| API Keys | Monthly | External services, higher risk |
| Encryption Keys | Never (migration) | Data encryption, requires migration |

### Rotation Procedure

#### 1. Generate New Secret
```bash
# Example: New Grafana password
NEW_PASSWORD=$(openssl rand -base64 32)
```

#### 2. Update Secret Store
```bash
# Docker Swarm
docker secret create grafana_admin_password_v2 - <<< "$NEW_PASSWORD"

# Update service to use new secret
docker service update \
  --secret-rm grafana_admin_password \
  --secret-add grafana_admin_password_v2 \
  shaman-journey

# AWS Secrets Manager
aws secretsmanager update-secret \
  --secret-id shaman-journey/prod/grafana-password \
  --secret-string "$NEW_PASSWORD"
```

#### 3. Deploy Updated Application
```bash
# Rolling update to pick up new secret
docker-compose up -d --no-deps --build shaman-journey
```

#### 4. Verify New Secret Works
```bash
# Test Grafana login with new password
curl -u admin:$NEW_PASSWORD http://localhost:3000/api/health
```

#### 5. Remove Old Secret
```bash
# After confirming new secret works
docker secret rm grafana_admin_password
```

#### 6. Document Rotation
```bash
# Add to rotation log
echo "$(date): Grafana admin password rotated" >> docs/secret_rotation_log.txt
```

### Emergency Rotation

If a secret is compromised:

1. **Immediately rotate** - Don't wait for scheduled rotation
2. **Audit access logs** - Who accessed what, when
3. **Review recent changes** - Look for unauthorized activity
4. **Notify team** - Inform all relevant personnel
5. **Post-mortem** - Document how secret was exposed, prevent recurrence

---

## Access Control

### Who Needs Access?

#### Development Secrets
- **Access**: All developers
- **Scope**: Development Sentry project, local Grafana
- **Storage**: Individual `.env` files (not shared)

#### Staging Secrets
- **Access**: Developers, QA, DevOps
- **Scope**: Staging Sentry project, staging Grafana
- **Storage**: Shared secret store (encrypted)

#### Production Secrets
- **Access**: DevOps, SRE only (minimal access)
- **Scope**: Production services
- **Storage**: Production secret manager with audit logs

### Access Request Process

1. **Request**: Developer submits access request ticket
2. **Justification**: Explain why access is needed
3. **Approval**: Manager or tech lead approves
4. **Grant**: DevOps grants time-limited access (e.g., 7 days)
5. **Audit**: Access logged and reviewed monthly

### Revoking Access

When team member leaves or changes role:
- [ ] Revoke secret manager access
- [ ] Remove from shared password manager
- [ ] Rotate any secrets they had access to
- [ ] Audit their access logs

---

## Emergency Procedures

### Secret Exposed in Git

**If secret is committed to version control:**

1. **Assume Compromised**: Treat secret as public immediately
2. **Rotate Immediately**: Generate and deploy new secret
3. **Remove from Git History**:
   ```bash
   # WARNING: Rewrites history, coordinate with team
   git filter-branch --force --index-filter \
     "git rm --cached --ignore-unmatch .env" \
     --prune-empty --tag-name-filter cat -- --all

   # Force push (dangerous!)
   git push origin --force --all
   ```
4. **Invalidate Old Secret**: Revoke/delete old secret from service
5. **Post-Mortem**: Document how it happened, update process

### Secret Exposed in Logs

**If secret appears in application logs:**

1. **Stop Logging**: Immediately disable affected logging
2. **Purge Logs**: Delete log files containing secret
3. **Rotate Secret**: Generate and deploy new secret
4. **Fix Code**: Remove secret from log statements
5. **Audit**: Check if logs were sent to monitoring (Sentry, etc.)

### Secret Exposed Publicly

**If secret is posted online (forum, GitHub issue, etc.):**

1. **Delete Post**: Remove immediately (contact support if needed)
2. **Screenshot**: Capture evidence before deletion
3. **Rotate Secret**: Generate and deploy new secret
4. **Monitor**: Watch for unauthorized usage
5. **Notify**: Inform security team and affected service provider

---

## Best Practices

### DO:
- ✅ Use `.env.example` to document all required variables
- ✅ Store secrets in environment variables or secret managers
- ✅ Use different secrets for dev/staging/prod
- ✅ Rotate secrets regularly
- ✅ Audit secret access
- ✅ Use strong, random secrets (20+ characters)
- ✅ Encrypt secrets at rest
- ✅ Use separate Sentry projects per environment

### DON'T:
- ❌ Commit secrets to version control
- ❌ Hardcode secrets in source code
- ❌ Share secrets via email or chat
- ❌ Reuse secrets across environments
- ❌ Log secrets in application logs
- ❌ Use weak passwords (e.g., "admin", "password123")
- ❌ Store secrets in issue trackers or wikis
- ❌ Share production secrets with entire team

---

## Tools & Resources

### Secret Generators
```bash
# Strong random password
openssl rand -base64 32

# UUID-based secret
uuidgen

# Hex random bytes
openssl rand -hex 32
```

### Secret Scanners
```bash
# Install gitleaks (scans for secrets in git)
brew install gitleaks

# Scan repository
gitleaks detect --source . --verbose

# Install trufflehog
pip install truffleHog

# Scan repository history
trufflehog --regex --entropy=True https://github.com/user/repo
```

### Environment File Encryption
```bash
# Encrypt .env file for backup
gpg --symmetric --cipher-algo AES256 .env
# Creates .env.gpg (safe to store in encrypted backup)

# Decrypt
gpg .env.gpg
```

---

## Checklist for New Secrets

When adding a new secret to the application:

- [ ] Add to `.env.example` with empty value and documentation
- [ ] Add to `docs/SECRETS_MANAGEMENT.md` (this file)
- [ ] Add to `.gitignore` (if file-based)
- [ ] Document rotation schedule
- [ ] Document access requirements
- [ ] Create secret in all environments (dev, staging, prod)
- [ ] Test loading secret in application
- [ ] Update deployment documentation
- [ ] Notify team of new secret requirement

---

## Compliance & Auditing

### Audit Log

Maintain a log of secret operations:
```
Date       | Operation | Secret           | User      | Environment
-----------|-----------|------------------|-----------|------------
2025-12-28 | Created   | sentry_dsn       | admin     | production
2025-12-28 | Rotated   | grafana_password | devops    | production
2025-12-28 | Accessed  | api_key          | developer | development
```

### Compliance Requirements

If subject to compliance (SOC2, HIPAA, etc.):
- Encrypt secrets at rest
- Enable secret access logging
- Implement least-privilege access
- Regular access reviews (quarterly)
- Secret rotation evidence
- Breach notification procedures

---

## Contact & Support

**Questions about secrets management?**
- Primary Contact: DevOps Team
- Emergency Contact: Security Team
- Documentation: This file + `.env.example`

**Report a secret exposure:**
1. Immediately notify Security Team
2. Follow emergency procedures above
3. File incident report

---

**Last Updated**: December 28, 2025
**Review Schedule**: Quarterly
**Next Review**: March 28, 2026
