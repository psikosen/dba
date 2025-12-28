# Monitoring and Observability Guide

This guide covers the monitoring, observability, and performance profiling setup for Shaman's Journey.

## Table of Contents

1. [Overview](#overview)
2. [Prometheus Metrics](#prometheus-metrics)
3. [Grafana Dashboards](#grafana-dashboards)
4. [Sentry Error Tracking](#sentry-error-tracking)
5. [Performance Profiling](#performance-profiling)
6. [Load Testing](#load-testing)
7. [Staging Environment](#staging-environment)

## Overview

The monitoring stack includes:

- **Prometheus**: Metrics collection and storage
- **Grafana**: Visualization and dashboards
- **Sentry**: Error tracking and crash reporting
- **Criterion**: Performance benchmarking
- **k6**: Load testing

## Prometheus Metrics

### Available Metrics

#### Performance Metrics

- `bevy_shaman_fps`: Current frames per second
- `bevy_shaman_frame_time_seconds`: Frame rendering time (histogram)
- `bevy_shaman_entity_count`: Total entity count
- `bevy_shaman_system_count`: Active system count

#### Game Metrics

- `bevy_shaman_monsters_spawned_total`: Total monsters spawned (counter)
- `bevy_shaman_monsters_defeated_total`: Total monsters defeated (counter)
- `bevy_shaman_player_deaths_total`: Total player deaths (counter)
- `bevy_shaman_corruption_level`: Current world corruption level (gauge, 0-1)
- `bevy_shaman_purification_total`: Total purifications performed (counter)
- `bevy_shaman_combat_encounters_total`: Total combat encounters (counter)
- `bevy_shaman_boss_encounters_total`: Total boss encounters (counter)

### Accessing Metrics

Metrics are exposed on the `/metrics` endpoint (default port 9091):

```bash
curl http://localhost:9091/metrics
```

### Configuration

Edit `monitoring/prometheus/prometheus.yml` to configure scrape intervals and targets:

```yaml
scrape_configs:
  - job_name: 'shaman-journey'
    static_configs:
      - targets: ['localhost:9091']
    scrape_interval: 5s
```

## Grafana Dashboards

### Starting Grafana

```bash
# Start monitoring stack
docker-compose -f docker-compose.monitoring.yml up -d

# Access Grafana
open http://localhost:3000
```

**Default credentials:**
- Username: `admin`
- Password: `admin`

### Available Dashboards

#### 1. Performance Dashboard

Location: `monitoring/grafana/dashboards/shaman-journey-performance.json`

Visualizes:
- FPS over time
- Frame time percentiles (p95, p99)
- Entity count gauge
- Active systems gauge

#### 2. Game Metrics Dashboard

Location: `monitoring/grafana/dashboards/shaman-journey-game-metrics.json`

Visualizes:
- Monster spawn/defeat rates
- World corruption level
- Encounter distribution (combat vs boss)
- Player actions (purifications, deaths)

### Creating Custom Dashboards

1. Log into Grafana (http://localhost:3000)
2. Click "+" → "Dashboard"
3. Add panels with Prometheus queries
4. Save dashboard
5. Export JSON to `monitoring/grafana/dashboards/`

Example query:
```promql
rate(bevy_shaman_monsters_defeated_total[5m])
```

## Sentry Error Tracking

### Setup

1. Create a Sentry account at https://sentry.io
2. Create a new project for "Rust"
3. Copy your DSN

### Configuration

Set the Sentry DSN environment variable:

```bash
export SENTRY_DSN="https://your-key@sentry.io/your-project"
```

Or add to `.env` file:
```
SENTRY_DSN=https://your-key@sentry.io/your-project
ENVIRONMENT=production
```

### Features

- **Automatic panic capture**: All panics are sent to Sentry
- **Breadcrumbs**: Track user actions leading to errors
- **Context**: Attach game state to error reports
- **Release tracking**: Track errors by version

### Usage in Code

```rust
use bevy_shaman_monitoring::sentry_integration::{
    capture_message, add_breadcrumb, SentryLevel
};

// Capture a message
capture_message("Player entered corrupted zone", SentryLevel::Info);

// Add breadcrumb for context
add_breadcrumb("game_event", "Boss encounter started", SentryLevel::Info);
```

## Performance Profiling

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench --workspace --exclude bevy_shaman_audio

# Run specific benchmark
cargo bench --bench game_benchmarks

# Generate flamegraph
cargo install flamegraph
cargo flamegraph --bench game_benchmarks
```

### Benchmark Results

Results are stored in `target/criterion/`:
- HTML reports in `target/criterion/report/index.html`
- Statistical data for regression detection
- Comparison with previous runs

### CI Benchmarks

Benchmarks run automatically:
- On every push to main
- Weekly on Sundays
- Can be triggered manually

View results in GitHub Actions → Benchmarks workflow

### Profiling Tools

#### Flamegraph

```bash
cargo flamegraph --release
```

Generates `flamegraph.svg` showing CPU time distribution.

#### perf (Linux)

```bash
cargo build --release
perf record --call-graph=dwarf ./target/release/bevy_shaman
perf report
```

#### Instruments (macOS)

```bash
cargo instruments --release --bench game_benchmarks
```

## Load Testing

### Setup k6

```bash
# macOS
brew install k6

# Ubuntu/Debian
sudo apt-get install k6

# Docker
docker pull grafana/k6
```

### Running Load Tests

```bash
# Start the game server
cargo run --release

# Run load test
k6 run loadtests/game_load_test.js

# With custom configuration
BASE_URL=http://staging:8080 k6 run loadtests/game_load_test.js
```

### Test Scenarios

**Default scenario** (`game_load_test.js`):
1. Ramp up to 10 users (30s)
2. Maintain 10 users (1m)
3. Ramp up to 50 users (30s)
4. Maintain 50 users (2m)
5. Ramp down to 0 users (30s)

**Performance targets:**
- p95 response time < 500ms
- p99 response time < 1000ms
- Error rate < 10%

### Analyzing Results

k6 provides:
- Real-time console output
- JSON export for analysis
- Integration with Grafana

Generate HTML report:
```bash
k6 run --out json=results.json loadtests/game_load_test.js
```

## Staging Environment

### Starting Staging

```bash
# Start staging stack
docker-compose -f docker-compose.staging.yml up -d

# View logs
docker-compose -f docker-compose.staging.yml logs -f shaman-journey-staging

# Stop staging
docker-compose -f docker-compose.staging.yml down
```

### Configuration

Environment variables (`.env` file):
```bash
SENTRY_DSN=https://your-key@sentry.io/project
GRAFANA_ADMIN_USER=admin
GRAFANA_ADMIN_PASSWORD=changeme
ENVIRONMENT=staging
```

### Services

Staging includes:
- **Game server**: http://localhost:8080
- **Prometheus**: http://localhost:9092
- **Grafana**: http://localhost:3001
- **Metrics endpoint**: http://localhost:9091/metrics

### Smoke Tests

Run smoke tests against staging:

```bash
# Start staging
docker-compose -f docker-compose.staging.yml up -d

# Wait for services to be ready
sleep 10

# Run smoke tests
cargo test --test smoke --features staging -- --ignored
```

## Monitoring Best Practices

### 1. Metric Collection

- Keep scrape intervals reasonable (5-15s)
- Use histograms for latency metrics
- Use counters for events
- Use gauges for current values

### 2. Dashboard Design

- Group related metrics together
- Use appropriate time ranges
- Set meaningful thresholds
- Include documentation

### 3. Alerting

Configure alerts in `monitoring/alertmanager/config.yml`:

```yaml
receivers:
  - name: 'team-email'
    email_configs:
      - to: 'team@example.com'
        from: 'alerts@example.com'
```

### 4. Error Tracking

- Set appropriate environment (dev/staging/production)
- Include context with errors
- Use breadcrumbs for debugging
- Review errors regularly

### 5. Performance

- Run benchmarks before releases
- Track performance trends
- Set regression thresholds
- Profile hot paths

## Troubleshooting

### Metrics not appearing

1. Check Prometheus is scraping: http://localhost:9090/targets
2. Verify game server is exposing metrics: `curl http://localhost:9091/metrics`
3. Check Prometheus config: `monitoring/prometheus/prometheus.yml`

### Grafana dashboard empty

1. Verify Prometheus datasource is configured
2. Check time range in dashboard
3. Ensure metrics exist: http://localhost:9090/graph

### Sentry not capturing errors

1. Verify `SENTRY_DSN` environment variable is set
2. Check Sentry project settings
3. Review Sentry quota limits

### Load tests failing

1. Ensure game server is running
2. Check firewall/network settings
3. Verify `BASE_URL` is correct
4. Review k6 error output

## Further Reading

- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Tutorials](https://grafana.com/tutorials/)
- [Sentry Rust SDK](https://docs.sentry.io/platforms/rust/)
- [Criterion.rs Guide](https://bheisler.github.io/criterion.rs/book/)
- [k6 Documentation](https://k6.io/docs/)
