use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::{info, warn};

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation - requests flow through
    Closed,
    /// Service is failing - reject requests immediately
    Open,
    /// Testing if service recovered - allow one request
    HalfOpen,
}

/// Circuit breaker implementation to prevent cascading failures
///
/// This pattern protects your application from wasting resources on failing services.
///
/// # States
/// - **Closed**: Normal operation, all requests go through
/// - **Open**: Service is broken, reject requests immediately (fail fast)
/// - **HalfOpen**: Testing recovery, allow one request to check if service is back
///
/// # Example
/// ```rust
/// let breaker = CircuitBreaker::new(5, Duration::from_secs(60));
///
/// // Use with service calls
/// if breaker.is_open().await {
///     return Err(anyhow!("Service unavailable"));
/// }
///
/// match some_service_call().await {
///     Ok(result) => {
///         breaker.record_success().await;
///         Ok(result)
///     }
///     Err(e) => {
///         breaker.record_failure().await;
///         Err(e)
///     }
/// }
/// ```
#[derive(Clone)]
pub struct CircuitBreaker {
    /// Number of consecutive failures before opening circuit
    failure_threshold: u32,

    /// Current failure count
    failure_count: Arc<AtomicU32>,

    /// Number of consecutive successes needed to close circuit from half-open
    success_threshold: u32,

    /// Current success count (only used in half-open state)
    consecutive_successes: Arc<AtomicU32>,

    /// How long to wait before testing service (transition from open to half-open)
    timeout_duration: Duration,

    /// When the last failure occurred
    last_failure_time: Arc<Mutex<Option<Instant>>>,

    /// Current state of the circuit
    state: Arc<Mutex<CircuitState>>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    ///
    /// # Arguments
    /// * `failure_threshold` - Number of failures before opening circuit (typically 3-10)
    /// * `timeout_duration` - How long to wait before testing recovery (typically 30-120 seconds)
    pub fn new(failure_threshold: u32, timeout_duration: Duration) -> Self {
        Self {
            failure_threshold,
            failure_count: Arc::new(AtomicU32::new(0)),
            success_threshold: 3, // Need 3 consecutive successes to fully recover
            consecutive_successes: Arc::new(AtomicU32::new(0)),
            timeout_duration,
            last_failure_time: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(CircuitState::Closed)),
        }
    }

    /// Check if circuit breaker is open (blocking requests)
    ///
    /// Returns `true` if requests should be rejected immediately
    /// Returns `false` if requests should be allowed
    pub async fn is_open(&self) -> bool {
        let mut state = self.state.lock().await;

        match *state {
            CircuitState::Closed => false, // Allow requests

            CircuitState::Open => {
                // Check if timeout expired - time to test recovery
                let last_failure = self.last_failure_time.lock().await;
                if let Some(time) = *last_failure {
                    if time.elapsed() >= self.timeout_duration {
                        info!(
                            "Circuit breaker entering HALF-OPEN state - testing service recovery after {}s",
                            self.timeout_duration.as_secs()
                        );
                        *state = CircuitState::HalfOpen;
                        return false; // Allow one test request
                    }
                }
                true // Still in open state, reject requests
            }

            CircuitState::HalfOpen => false, // Allow test request
        }
    }

    /// Record a successful request
    pub async fn record_success(&self) {
        let mut state = self.state.lock().await;

        match *state {
            CircuitState::HalfOpen => {
                let successes = self.consecutive_successes.fetch_add(1, Ordering::SeqCst) + 1;
                if successes >= self.success_threshold {
                    info!(
                        "Circuit breaker CLOSED - service recovered after {} consecutive successes",
                        successes
                    );
                    *state = CircuitState::Closed;
                    self.failure_count.store(0, Ordering::SeqCst);
                    self.consecutive_successes.store(0, Ordering::SeqCst);
                } else {
                    info!(
                        "Circuit breaker half-open: {}/{} successes needed to close",
                        successes, self.success_threshold
                    );
                }
            }

            CircuitState::Closed => {
                // Reset failure count on success in closed state
                let prev_failures = self.failure_count.swap(0, Ordering::SeqCst);
                if prev_failures > 0 {
                    info!("Circuit breaker reset: cleared {} previous failures", prev_failures);
                }
            }

            CircuitState::Open => {
                // Shouldn't happen, but handle gracefully
                warn!("Recorded success while circuit breaker is open - this shouldn't happen");
            }
        }
    }

    /// Record a failed request
    pub async fn record_failure(&self) {
        // Reset consecutive successes on any failure
        self.consecutive_successes.store(0, Ordering::SeqCst);

        let failures = self.failure_count.fetch_add(1, Ordering::SeqCst) + 1;
        let mut state = self.state.lock().await;

        match *state {
            CircuitState::Closed => {
                if failures >= self.failure_threshold {
                    warn!(
                        "Circuit breaker OPEN - service failing ({} consecutive failures, threshold: {})",
                        failures, self.failure_threshold
                    );
                    *state = CircuitState::Open;
                    let mut last_failure = self.last_failure_time.lock().await;
                    *last_failure = Some(Instant::now());
                } else {
                    warn!(
                        "Circuit breaker tracking failures: {}/{} (will open on threshold)",
                        failures, self.failure_threshold
                    );
                }
            }

            CircuitState::HalfOpen => {
                warn!("Circuit breaker OPEN - recovery test failed");
                *state = CircuitState::Open;
                self.failure_count.store(1, Ordering::SeqCst); // Reset to 1 failure
                let mut last_failure = self.last_failure_time.lock().await;
                *last_failure = Some(Instant::now());
            }

            CircuitState::Open => {
                // Update last failure time to extend the timeout
                let mut last_failure = self.last_failure_time.lock().await;
                *last_failure = Some(Instant::now());
            }
        }
    }

    /// Get current state for metrics and monitoring
    pub async fn get_state(&self) -> CircuitState {
        *self.state.lock().await
    }

    /// Get current failure count
    pub fn get_failure_count(&self) -> u32 {
        self.failure_count.load(Ordering::SeqCst)
    }

    /// Get current success count (only relevant in half-open state)
    pub fn get_success_count(&self) -> u32 {
        self.consecutive_successes.load(Ordering::SeqCst)
    }

    /// Get state as a numeric value for metrics
    /// - 0 = Closed (healthy)
    /// - 1 = Open (broken)
    /// - 2 = HalfOpen (testing)
    pub async fn get_state_value(&self) -> u8 {
        match self.get_state().await {
            CircuitState::Closed => 0,
            CircuitState::Open => 1,
            CircuitState::HalfOpen => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_circuit_breaker_opens_on_failures() {
        let breaker = CircuitBreaker::new(3, Duration::from_millis(100));

        assert_eq!(breaker.get_state().await, CircuitState::Closed);
        assert!(!breaker.is_open().await);

        // Record failures
        breaker.record_failure().await;
        assert_eq!(breaker.get_state().await, CircuitState::Closed);

        breaker.record_failure().await;
        assert_eq!(breaker.get_state().await, CircuitState::Closed);

        breaker.record_failure().await;
        assert_eq!(breaker.get_state().await, CircuitState::Open);
        assert!(breaker.is_open().await);
    }

    #[tokio::test]
    async fn test_circuit_breaker_transitions_to_half_open() {
        let breaker = CircuitBreaker::new(2, Duration::from_millis(50));

        // Open the circuit
        breaker.record_failure().await;
        breaker.record_failure().await;
        assert_eq!(breaker.get_state().await, CircuitState::Open);

        // Wait for timeout
        sleep(Duration::from_millis(60)).await;

        // Should transition to half-open
        assert!(!breaker.is_open().await);
        assert_eq!(breaker.get_state().await, CircuitState::HalfOpen);
    }

    #[tokio::test]
    async fn test_circuit_breaker_closes_on_success() {
        let breaker = CircuitBreaker::new(2, Duration::from_millis(50));

        // Open the circuit
        breaker.record_failure().await;
        breaker.record_failure().await;
        assert_eq!(breaker.get_state().await, CircuitState::Open);

        // Wait for timeout
        sleep(Duration::from_millis(60)).await;

        // Transition to half-open
        breaker.is_open().await;
        assert_eq!(breaker.get_state().await, CircuitState::HalfOpen);

        // Record successes
        breaker.record_success().await;
        breaker.record_success().await;
        breaker.record_success().await;

        // Should close
        assert_eq!(breaker.get_state().await, CircuitState::Closed);
        assert!(!breaker.is_open().await);
    }

    #[tokio::test]
    async fn test_circuit_breaker_reopens_on_half_open_failure() {
        let breaker = CircuitBreaker::new(2, Duration::from_millis(50));

        // Open the circuit
        breaker.record_failure().await;
        breaker.record_failure().await;

        // Wait and transition to half-open
        sleep(Duration::from_millis(60)).await;
        breaker.is_open().await;

        // Fail the test request
        breaker.record_failure().await;

        // Should go back to open
        assert_eq!(breaker.get_state().await, CircuitState::Open);
        assert!(breaker.is_open().await);
    }

    #[tokio::test]
    async fn test_circuit_breaker_resets_on_success() {
        let breaker = CircuitBreaker::new(3, Duration::from_millis(100));

        // Record some failures
        breaker.record_failure().await;
        breaker.record_failure().await;
        assert_eq!(breaker.get_failure_count(), 2);

        // Record success - should reset
        breaker.record_success().await;
        assert_eq!(breaker.get_failure_count(), 0);
        assert_eq!(breaker.get_state().await, CircuitState::Closed);
    }
}
