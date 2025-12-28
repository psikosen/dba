pub mod cache;
pub mod config;
pub mod queue;

pub use cache::{
    CircuitBreaker, CircuitState, DragonflyCache, LlmResponseCache,
    DungeonSeedCache, SessionStateCache,
};

pub use config::ServiceConfig;

pub use queue::{
    RabbitMqClient, BackgroundJobQueue, EventBus, OfflineProcessor,
    JobType, JobPayload, GameEvent, EventType, EventPayload,
    OfflineJob, OfflineJobType, OfflineJobPayload,
};

use anyhow::Result;
use tracing::info;

/// Initialize all services with validated configuration
pub async fn initialize_services(config: ServiceConfig) -> Result<Services> {
    info!("Initializing DragonflyDB and RabbitMQ services...");

    // Validate TLS is enabled in production
    config.ensure_tls_enabled()?;

    // Initialize DragonflyDB
    let dragonfly = DragonflyCache::new(&config.dragonfly_url).await?;

    // Initialize RabbitMQ
    let rabbitmq = RabbitMqClient::new(&config.rabbitmq_url).await?;

    // Create cache services
    let llm_cache = LlmResponseCache::new(dragonfly.clone(), None);
    let dungeon_cache = DungeonSeedCache::new(dragonfly.clone(), None);
    let session_cache = SessionStateCache::new(dragonfly.clone(), None);

    // Create queue services
    let background_jobs = BackgroundJobQueue::new(
        rabbitmq.clone(),
        "background_jobs".to_string()
    ).await?;

    let event_bus = EventBus::new(
        rabbitmq.clone(),
        "game_events".to_string()
    ).await?;

    let offline_processor = OfflineProcessor::new(
        rabbitmq.clone(),
        "offline_jobs".to_string()
    ).await?;

    info!("All services initialized successfully");

    Ok(Services {
        dragonfly,
        rabbitmq,
        llm_cache,
        dungeon_cache,
        session_cache,
        background_jobs,
        event_bus,
        offline_processor,
    })
}

/// Container for all initialized services
pub struct Services {
    pub dragonfly: DragonflyCache,
    pub rabbitmq: RabbitMqClient,
    pub llm_cache: LlmResponseCache,
    pub dungeon_cache: DungeonSeedCache,
    pub session_cache: SessionStateCache,
    pub background_jobs: BackgroundJobQueue,
    pub event_bus: EventBus,
    pub offline_processor: OfflineProcessor,
}

impl Services {
    /// Health check for all services
    pub async fn health_check(&self) -> Result<HealthStatus> {
        let dragonfly_ok = self.dragonfly.ping().await.is_ok();
        let rabbitmq_ok = self.rabbitmq.health_check().await.is_ok();

        Ok(HealthStatus {
            dragonfly: dragonfly_ok,
            rabbitmq: rabbitmq_ok,
            overall: dragonfly_ok && rabbitmq_ok,
        })
    }

    /// Get circuit breaker states for monitoring
    pub async fn get_circuit_breaker_states(&self) -> CircuitBreakerStates {
        CircuitBreakerStates {
            dragonfly: self.dragonfly.circuit_breaker().get_state().await,
            rabbitmq: self.rabbitmq.circuit_breaker().get_state().await,
        }
    }
}

/// Health status for services
#[derive(Debug, Clone)]
pub struct HealthStatus {
    pub dragonfly: bool,
    pub rabbitmq: bool,
    pub overall: bool,
}

/// Circuit breaker states for monitoring
#[derive(Debug, Clone)]
pub struct CircuitBreakerStates {
    pub dragonfly: CircuitState,
    pub rabbitmq: CircuitState,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        let config = ServiceConfig::from_env();
        assert!(!config.dragonfly_url.is_empty());
        assert!(!config.rabbitmq_url.is_empty());
    }
}
