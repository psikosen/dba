use anyhow::{Context, Result};
use lapin::{
    options::*,
    types::FieldTable,
    Channel, Connection, ConnectionProperties,
};
use tracing::{error, info, warn};

/// RabbitMQ client for message queue operations
#[derive(Clone)]
pub struct RabbitMqClient {
    channel: Channel,
}

impl RabbitMqClient {
    /// Create a new RabbitMQ client
    pub async fn new(url: &str) -> Result<Self> {
        info!("Connecting to RabbitMQ at {}", url);

        let connection = Connection::connect(url, ConnectionProperties::default())
            .await
            .context("Failed to connect to RabbitMQ")?;

        let channel = connection
            .create_channel()
            .await
            .context("Failed to create RabbitMQ channel")?;

        info!("Successfully connected to RabbitMQ");

        Ok(Self { channel })
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
