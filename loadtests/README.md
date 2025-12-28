# Load Testing for Shaman's Journey

This directory contains load tests for the game server using k6.

## Prerequisites

Install k6:

```bash
# macOS
brew install k6

# Ubuntu/Debian
sudo gpg -k
sudo gpg --no-default-keyring --keyring /usr/share/keyrings/k6-archive-keyring.gpg --keyserver hkp://keyserver.ubuntu.com:80 --recv-keys C5AD17C747E3415A3642D57D77C6C491D6AC1D69
echo "deb [signed-by=/usr/share/keyrings/k6-archive-keyring.gpg] https://dl.k6.io/deb stable main" | sudo tee /etc/apt/sources.list.d/k6.list
sudo apt-get update
sudo apt-get install k6

# Using Docker
docker pull grafana/k6
```

## Running Load Tests

### Local Testing

1. Start the game server:
```bash
cargo run --release
```

2. Run the load test:
```bash
k6 run loadtests/game_load_test.js
```

### Using Docker

```bash
docker run --rm -i grafana/k6 run - <loadtests/game_load_test.js
```

### Custom Configuration

Set environment variables to customize the test:

```bash
BASE_URL=http://staging.example.com:8080 k6 run loadtests/game_load_test.js
```

## Test Scenarios

### game_load_test.js

Simulates realistic user load with:
- Ramp up from 0 to 10 users over 30s
- Maintain 10 concurrent users for 1 minute
- Ramp up to 50 users over 30s
- Maintain 50 concurrent users for 2 minutes
- Ramp down to 0 users over 30s

**Thresholds:**
- 95% of requests should complete in < 500ms
- Error rate should be < 10%

## Analyzing Results

### Real-time Monitoring

Use Grafana to monitor load test results in real-time:

1. Start monitoring stack: `docker-compose -f docker-compose.monitoring.yml up -d`
2. Run load test: `k6 run loadtests/game_load_test.js`
3. View metrics at: http://localhost:3000

### Output Formats

Generate HTML report:
```bash
k6 run --out json=results.json loadtests/game_load_test.js
k6 report results.json --export results.html
```

## Performance Targets

Based on game requirements:

- **Response Time**: p95 < 500ms, p99 < 1000ms
- **Throughput**: 100 requests/second minimum
- **Error Rate**: < 1% under normal load
- **Concurrent Users**: 50+ without degradation

## Continuous Load Testing

Load tests can be run:
- Manually before releases
- Automatically on staging environment
- Scheduled weekly via CI/CD

See `.github/workflows/loadtest.yml` for automated testing configuration.
