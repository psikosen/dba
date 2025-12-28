use anyhow::{Context, Result};
use lapin::{
    options::*,
    types::FieldTable,
    Channel, Connection, ConnectionProperties,
};
use std::time::Duration;
use tracing::{error, info, warn};

use crate::cache::CircuitBreaker;

/// RabbitMQ client for message queue operations with circuit breaker protection
#[derive(Clone)]
pub struct RabbitMqClient {
    channel: Channel,
    circuit_breaker: CircuitBreaker,
}

impl RabbitMqClient {
    /// Create a new RabbitMQ client with retry logic
    pub async fn new(url: &str) -> Result<Self> {
        let max_retries = 5;
        let mut attempt = 0;

        loop {
            attempt += 1;
            info!("Connecting to RabbitMQ at {} (attempt {}/{})", url, attempt, max_retries);

            match Self::try_connect(url).await {
                Ok(client) => {
                    info!("Successfully connected to RabbitMQ");
                    return Ok(client);
                }
                Err(e) if attempt >= max_retries => {
                    error!("Failed to connect to RabbitMQ after {} attempts: {}", max_retries, e);
                    return Err(e);
                }
                Err(e) => {
                    // Exponential backoff: 2^attempt seconds, capped at 64 seconds
                    let backoff_secs = 2_u64.pow(attempt.min(6));
                    warn!(
                        "RabbitMQ connection failed (attempt {}): {}. Retrying in {} seconds...",
                        attempt, e, backoff_secs
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(backoff_secs)).await;
                }
            }
        }
    }

    /// Internal method to attempt connection (used by retry logic)
    async fn try_connect(url: &str) -> Result<Self> {
        let connection = Connection::connect(url, ConnectionProperties::default())
            .await
            .context("Failed to connect to RabbitMQ")?;

        let channel = connection
            .create_channel()
            .await
            .context("Failed to create RabbitMQ channel")?;

        // Create circuit breaker: trip after 5 failures, wait 60s before testing recovery
        let circuit_breaker = CircuitBreaker::new(5, Duration::from_secs(60));

        Ok(Self {
            channel,
            circuit_breaker,
        })
    }

    /// Get circuit breaker reference for monitoring
    pub fn circuit_breaker(&self) -> &CircuitBreaker {
        &self.circuit_breaker
    }

    /// Declare a queue
    pub async fn declare_queue(
        &self,
        queue_name: &str,
        durable: bool,
    ) -> Result<()> {
        self.channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions {
                    durable,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .context(format!("Failed to declare queue '{}'", queue_name))?;

        info!("Declared queue: {}", queue_name);
        Ok(())
    }

    /// Declare an exchange
    pub async fn declare_exchange(
        &self,
        exchange_name: &str,
        exchange_type: &str,
        durable: bool,
    ) -> Result<()> {
        self.channel
            .exchange_declare(
                exchange_name,
                lapin::ExchangeKind::from(exchange_type),
                ExchangeDeclareOptions {
                    durable,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .context(format!("Failed to declare exchange '{}'", exchange_name))?;

        info!("Declared exchange: {} (type: {})", exchange_name, exchange_type);
        Ok(())
    }

    /// Bind queue to exchange
    pub async fn bind_queue(
        &self,
        queue_name: &str,
        exchange_name: &str,
        routing_key: &str,
    ) -> Result<()> {
        self.channel
            .queue_bind(
                queue_name,
                exchange_name,
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .context(format!(
                "Failed to bind queue '{}' to exchange '{}'",
                queue_name, exchange_name
            ))?;

        info!(
            "Bound queue '{}' to exchange '{}' with routing key '{}'",
            queue_name, exchange_name, routing_key
        );
        Ok(())
    }

    /// Publish a message to a queue
    pub async fn publish_to_queue(
        &self,
        queue_name: &str,
        message: &[u8],
    ) -> Result<()> {
        self.channel
            .basic_publish(
                "",
                queue_name,
                BasicPublishOptions::default(),
                message,
                lapin::BasicProperties::default(),
            )
            .await
            .context(format!("Failed to publish to queue '{}'", queue_name))?
            .await
            .context("Failed to confirm publish")?;

        Ok(())
    }

    /// Publish a message to an exchange
    pub async fn publish_to_exchange(
        &self,
        exchange_name: &str,
        routing_key: &str,
        message: &[u8],
    ) -> Result<()> {
        self.channel
            .basic_publish(
                exchange_name,
                routing_key,
                BasicPublishOptions::default(),
                message,
                lapin::BasicProperties::default(),
            )
            .await
            .context(format!("Failed to publish to exchange '{}'", exchange_name))?
            .await
            .context("Failed to confirm publish")?;

        Ok(())
    }

    /// Get the channel for advanced operations
    pub fn channel(&self) -> &Channel {
        &self.channel
    }

    /// Health check
    pub async fn health_check(&self) -> Result<()> {
        // Check if channel is still open
        if self.channel.status().connected() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("RabbitMQ channel is not connected"))
        }
    }
}

impl From<&str> for lapin::ExchangeKind {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "direct" => lapin::ExchangeKind::Direct,
            "fanout" => lapin::ExchangeKind::Fanout,
            "topic" => lapin::ExchangeKind::Topic,
            "headers" => lapin::ExchangeKind::Headers,
            _ => lapin::ExchangeKind::Direct,
        }
    }
}
