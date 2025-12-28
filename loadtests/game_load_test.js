import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate, Trend } from 'k6/metrics';

// Custom metrics
const errorRate = new Rate('errors');
const responseTime = new Trend('response_time');

// Test configuration
export const options = {
  stages: [
    { duration: '30s', target: 10 },  // Ramp up to 10 users
    { duration: '1m', target: 10 },   // Stay at 10 users
    { duration: '30s', target: 50 },  // Ramp up to 50 users
    { duration: '2m', target: 50 },   // Stay at 50 users
    { duration: '30s', target: 0 },   // Ramp down to 0 users
  ],
  thresholds: {
    http_req_duration: ['p(95)<500'], // 95% of requests should be below 500ms
    errors: ['rate<0.1'],              // Error rate should be less than 10%
  },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';

export default function () {
  // Test health endpoint
  const healthRes = http.get(`${BASE_URL}/health`);
  const healthCheck = check(healthRes, {
    'health status is 200': (r) => r.status === 200,
  });
  errorRate.add(!healthCheck);

  // Test metrics endpoint
  const metricsRes = http.get(`${BASE_URL}/metrics`);
  const metricsCheck = check(metricsRes, {
    'metrics status is 200': (r) => r.status === 200,
    'metrics response contains prometheus data': (r) => r.body.includes('bevy_shaman'),
  });
  errorRate.add(!metricsCheck);
  responseTime.add(metricsRes.timings.duration);

  // Simulate realistic user behavior
  sleep(1);
}

export function handleSummary(data) {
  return {
    'loadtest-summary.json': JSON.stringify(data, null, 2),
    stdout: textSummary(data, { indent: ' ', enableColors: true }),
  };
}

function textSummary(data, options = {}) {
  const indent = options.indent || '';
  const enableColors = options.enableColors || false;

  let summary = `\n${indent}Test Summary:\n`;
  summary += `${indent}  Scenarios: ${Object.keys(data.metrics).length}\n`;
  summary += `${indent}  Duration: ${data.state.testRunDurationMs}ms\n\n`;

  for (const [name, metric] of Object.entries(data.metrics)) {
    if (metric.type === 'trend') {
      summary += `${indent}${name}:\n`;
      summary += `${indent}  avg: ${metric.values.avg.toFixed(2)}ms\n`;
      summary += `${indent}  min: ${metric.values.min.toFixed(2)}ms\n`;
      summary += `${indent}  max: ${metric.values.max.toFixed(2)}ms\n`;
      summary += `${indent}  p(95): ${metric.values['p(95)'].toFixed(2)}ms\n\n`;
    } else if (metric.type === 'rate') {
      summary += `${indent}${name}: ${(metric.values.rate * 100).toFixed(2)}%\n`;
    }
  }

  return summary;
}
