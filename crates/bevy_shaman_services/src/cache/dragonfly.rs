use anyhow::{Context, Result};
use redis::{aio::ConnectionManager, AsyncCommands, Client};
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// DragonflyDB cache client
/// DragonflyDB is a modern, high-performance Redis replacement
#[derive(Clone)]
pub struct DragonflyCache {
    client: ConnectionManager,
}

impl DragonflyCache {
    /// Create a new DragonflyDB cache client with retry logic
    pub async fn new(url: &str) -> Result<Self> {
        let max_retries = 5;
        let mut attempt = 0;

        loop {
            attempt += 1;
            info!("Connecting to DragonflyDB at {} (attempt {}/{})", url, attempt, max_retries);

            match Self::try_connect(url).await {
                Ok(client) => {
                    info!("Successfully connected to DragonflyDB");
                    return Ok(client);
                }
                Err(e) if attempt >= max_retries => {
                    error!("Failed to connect to DragonflyDB after {} attempts: {}", max_retries, e);
                    return Err(e);
                }
                Err(e) => {
                    // Exponential backoff: 2^attempt seconds, capped at 64 seconds
                    let backoff_secs = 2_u64.pow(attempt.min(6));
                    warn!(
                        "DragonflyDB connection failed (attempt {}): {}. Retrying in {} seconds...",
                        attempt, e, backoff_secs
                    );
                    tokio::time::sleep(std::time::Duration::from_secs(backoff_secs)).await;
                }
            }
        }
    }

    /// Internal method to attempt connection (used by retry logic)
    async fn try_connect(url: &str) -> Result<Self> {
        let client = Client::open(url)
            .context("Failed to create Redis client for DragonflyDB")?;

        let connection = ConnectionManager::new(client)
            .await
            .context("Failed to connect to DragonflyDB")?;

        Ok(Self { client: connection })
    }

    /// Set a key-value pair with optional TTL
    pub async fn set(&self, key: &str, value: &str, ttl: Option<Duration>) -> Result<()> {
        let mut conn = self.client.clone();

        match ttl {
            Some(duration) => {
                conn.set_ex(key, value, duration.as_secs())
                    .await
                    .context(format!("Failed to set key '{}' with TTL", key))?;
            }
            None => {
                conn.set(key, value)
                    .await
                    .context(format!("Failed to set key '{}'", key))?;
            }
        }

        debug!("Set cache key: {}", key);
        Ok(())
    }

    /// Get a value by key
    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.client.clone();

        let value: Option<String> = conn
            .get(key)
            .await
            .context(format!("Failed to get key '{}'", key))?;

        match &value {
            Some(_) => debug!("Cache hit for key: {}", key),
            None => debug!("Cache miss for key: {}", key),
        }

        Ok(value)
    }

    /// Delete a key
    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.client.clone();

        conn.del(key)
            .await
            .context(format!("Failed to delete key '{}'", key))?;

        debug!("Deleted cache key: {}", key);
        Ok(())
    }

    /// Check if a key exists
    pub async fn exists(&self, key: &str) -> Result<bool> {
        let mut conn = self.client.clone();

        let exists: bool = conn
            .exists(key)
            .await
            .context(format!("Failed to check existence of key '{}'", key))?;

        Ok(exists)
    }

    /// Set multiple keys at once
    pub async fn set_multiple(&self, pairs: Vec<(String, String)>) -> Result<()> {
        let mut conn = self.client.clone();

        for (key, value) in pairs {
            conn.set(&key, &value)
                .await
                .context(format!("Failed to set key '{}' in batch", key))?;
        }

        debug!("Set multiple cache keys");
        Ok(())
    }

    /// Get multiple values at once
    pub async fn get_multiple(&self, keys: Vec<String>) -> Result<Vec<Option<String>>> {
        let mut conn = self.client.clone();

        let values: Vec<Option<String>> = conn
            .get(&keys)
            .await
            .context("Failed to get multiple keys")?;

        Ok(values)
    }

    /// Increment a counter
    pub async fn increment(&self, key: &str) -> Result<i64> {
        let mut conn = self.client.clone();

        let value: i64 = conn
            .incr(key, 1)
            .await
            .context(format!("Failed to increment key '{}'", key))?;

        Ok(value)
    }

    /// Set TTL on an existing key
    pub async fn expire(&self, key: &str, ttl: Duration) -> Result<bool> {
        let mut conn = self.client.clone();

        let success: bool = conn
            .expire(key, ttl.as_secs() as i64)
            .await
            .context(format!("Failed to set TTL on key '{}'", key))?;

        Ok(success)
    }

    /// Ping the server to check connection
    pub async fn ping(&self) -> Result<()> {
        let mut conn = self.client.clone();

        redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .context("Failed to ping DragonflyDB")?;

        Ok(())
    }

    /// Get cache statistics
    pub async fn get_stats(&self) -> Result<CacheStats> {
        let mut conn = self.client.clone();

        let info: String = redis::cmd("INFO")
            .arg("stats")
            .query_async(&mut conn)
            .await
            .context("Failed to get cache stats")?;

        Ok(CacheStats::parse(&info))
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
}

impl CacheStats {
    fn parse(info: &str) -> Self {
        let mut stats = CacheStats {
            hits: 0,
            misses: 0,
            evictions: 0,
        };

        for line in info.lines() {
            if let Some((key, value)) = line.split_once(':') {
                match key.trim() {
                    "keyspace_hits" => {
                        stats.hits = value.trim().parse().unwrap_or(0);
                    }
                    "keyspace_misses" => {
                        stats.misses = value.trim().parse().unwrap_or(0);
                    }
                    "evicted_keys" => {
                        stats.evictions = value.trim().parse().unwrap_or(0);
                    }
                    _ => {}
                }
            }
        }

        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_stats_parse() {
        let info = "keyspace_hits:100\nkeyspace_misses:50\nevicted_keys:5";
        let stats = CacheStats::parse(info);

        assert_eq!(stats.hits, 100);
        assert_eq!(stats.misses, 50);
        assert_eq!(stats.evictions, 5);
    }
}
