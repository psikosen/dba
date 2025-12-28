# GitHub Actions CI/CD Workflows

This directory contains GitHub Actions workflows for continuous integration, security scanning, and container image building.

## Workflows

### 1. CI Workflow (`ci.yml`)

**Triggers:**
- Push to `main` or `master` branches
- Pull requests targeting `main` or `master`

**Jobs:**
- **Test Suite**: Runs all unit tests and integration tests
- **Rustfmt**: Checks code formatting
- **Clippy**: Runs Rust linter for code quality
- **Build Check**: Verifies debug and release builds succeed

**Features:**
- Caches Cargo dependencies for faster builds
- Installs required system dependencies
- Excludes `bevy_shaman_audio` crate (requires ALSA on CI)

### 2. Security Workflow (`security.yml`)

**Triggers:**
- Push to `main` or `master` branches
- Pull requests targeting `main` or `master`
- Scheduled daily at 00:00 UTC

**Jobs:**
- **Security Audit**: Uses `cargo-audit` to check for known vulnerabilities
- **Dependency Review**: Reviews dependency changes in PRs (GitHub native)
- **Cargo Deny**: Checks licenses, bans, and sources using `cargo-deny`

**Configuration:**
- `deny.toml` in repository root configures `cargo-deny`
- Fails on moderate+ severity vulnerabilities
- Allows MIT, Apache-2.0, BSD, and other permissive licenses

### 3. Docker Workflow (`docker.yml`)

**Triggers:**
- Push to `main` or `master` branches
- Pull requests targeting `main` or `master`
- Tags matching `v*.*.*` pattern

**Jobs:**
- **Build and Push**: Builds Docker images and publishes to GitHub Container Registry

**Features:**
- Multi-stage builds using existing `Dockerfile`
- Publishes to `ghcr.io/psikosen/dba`
- Generates artifact attestations for supply chain security
- Uses GitHub Actions cache for layer caching
- Tags images with:
  - Branch name for branch pushes
  - PR number for pull requests
  - Semantic version tags (e.g., `v1.2.3`, `v1.2`, `v1`)
  - Git SHA with branch prefix
  - `latest` tag for default branch

## Container Registry Setup

### GitHub Container Registry (GHCR)

The Docker images are automatically published to GitHub Container Registry:

**Image URL:**
```
ghcr.io/psikosen/dba:latest
```

**Available Tags:**
- `latest` - Latest build from main branch
- `main` - Latest build from main branch
- `v1.2.3` - Semantic version tags (when creating releases)
- `main-<sha>` - Specific commit builds

### Pulling Images

**Public Repository:**
```bash
docker pull ghcr.io/psikosen/dba:latest
```

**Private Repository:**
```bash
# Login with GitHub Personal Access Token
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin

# Pull the image
docker pull ghcr.io/psikosen/dba:latest
```

### Running the Container

```bash
# Run the latest image
docker run -it ghcr.io/psikosen/dba:latest

# Run with volume mounts for saves
docker run -it -v $(pwd)/saves:/app/saves ghcr.io/psikosen/dba:latest

# Run specific version
docker run -it ghcr.io/psikosen/dba:v1.0.0
```

## Using the Workflows

### For Contributors

**Pull Requests:**
1. Create a PR targeting `main` or `master`
2. All CI checks must pass before merging
3. Security audit must pass (no known vulnerabilities)
4. Docker build must succeed (tests multi-stage build)

**Viewing Results:**
- Check the "Actions" tab in GitHub
- Status badges appear in PR checks
- Click on failed checks to see detailed logs

### For Maintainers

**Creating Releases:**
1. Update version in `Cargo.toml`
2. Commit changes
3. Create and push a git tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
4. Docker workflow automatically builds and tags the image

**Manual Workflow Runs:**
- Go to Actions tab
- Select workflow
- Click "Run workflow" button
- Choose branch and trigger

## Secrets and Permissions

### Required Secrets

**None!** All workflows use `GITHUB_TOKEN` which is automatically provided.

### Required Permissions

The Docker workflow requires these permissions (already configured):
- `contents: read` - Read repository contents
- `packages: write` - Push to GitHub Container Registry
- `id-token: write` - Generate artifact attestations

## Troubleshooting

### Build Failures

**Dependency caching issues:**
```bash
# Clear cache by updating Cargo.lock
cargo update
git commit -am "Update dependencies"
```

**System dependency issues:**
- Check that all required libraries are in `ci.yml` install step
- Match dependencies with those in `Dockerfile`

### Security Audit Failures

**Known vulnerability found:**
1. Check `cargo audit` output for details
2. Update affected dependencies: `cargo update -p <package>`
3. If no fix available, add to `deny.toml` ignore list with justification

**License issues:**
1. Review `cargo deny check` output
2. Update `deny.toml` to allow new license if appropriate
3. Or replace dependency with compatible alternative

### Docker Build Failures

**Image too large:**
- Review `Dockerfile` for optimization opportunities
- Ensure multi-stage build is properly configured
- Check that dependencies are cached correctly

**Push failed:**
- Verify repository package permissions
- Check that workflow has `packages: write` permission

## Performance Optimization

### Caching Strategy

The CI workflow uses three cache levels:
1. **Cargo Registry** - Downloaded crates
2. **Cargo Index** - Crates.io index
3. **Build Artifacts** - Compiled dependencies

Cache is keyed on `Cargo.lock` hash for consistency.

### Parallel Execution

Jobs run in parallel when possible:
- Test, fmt, clippy, and build run concurrently
- Security jobs (audit, deny, review) run in parallel
- Docker build runs independently

### Reducing CI Time

**Strategies:**
1. Use `--workspace --exclude` to skip unnecessary crates
2. Keep `Cargo.lock` committed for reproducible builds
3. Update dependencies regularly to prevent large cache invalidations
4. Use `matrix` strategy for multi-platform builds (future enhancement)

## Future Enhancements

### Planned Improvements

**Week 1 (Current):**
- ✅ Automated testing on PR
- ✅ Security scanning with cargo-audit
- ✅ Docker image builds
- ✅ Container registry setup (GHCR)

**Month 1:**
- [ ] Staging environment deployment
- [ ] Automated smoke tests
- [ ] Integration test suite expansion

**Quarter 1:**
- [ ] Performance profiling in CI
- [ ] Benchmark tracking over time
- [ ] Load testing automation
- [ ] Multi-platform Docker builds (ARM64)

### Additional Workflow Ideas

**Code Coverage:**
```yaml
- name: Generate coverage
  run: cargo tarpaulin --out Xml
- name: Upload to Codecov
  uses: codecov/codecov-action@v3
```

**Deployment:**
```yaml
- name: Deploy to staging
  if: github.ref == 'refs/heads/main'
  run: ./deploy.sh staging
```

**Release Automation:**
```yaml
- name: Create GitHub Release
  uses: softprops/action-gh-release@v1
  with:
    files: target/release/bevy_shaman
```

## Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/actions)
- [Cargo Audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit)
- [Cargo Deny](https://github.com/EmbarkStudios/cargo-deny)
- [GitHub Container Registry](https://docs.github.com/packages/working-with-a-github-packages-registry/working-with-the-container-registry)
