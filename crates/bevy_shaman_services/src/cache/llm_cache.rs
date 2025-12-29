use super::dragonfly::DragonflyCache;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info};

/// Cache for LLM responses to reduce generation load
#[derive(Clone)]
pub struct LlmResponseCache {
    cache: DragonflyCache,
    ttl: Duration,
}

impl LlmResponseCache {
    /// Create a new LLM response cache
    ///
    /// # Arguments
    /// * `cache` - DragonflyDB cache instance
    /// * `ttl` - Time-to-live for cached responses (default: 1 hour)
    pub fn new(cache: DragonflyCache, ttl: Option<Duration>) -> Self {
        Self {
            cache,
            ttl: ttl.unwrap_or(Duration::from_secs(3600)), // 1 hour default
        }
    }

    /// Generate cache key for LLM request
    fn cache_key(&self, prompt: &str, context: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        prompt.hash(&mut hasher);
        context.hash(&mut hasher);
        let hash = hasher.finish();

        format!("llm:response:{:x}", hash)
    }

    /// Get cached LLM response
    pub async fn get_response(&self, prompt: &str, context: &str) -> Result<Option<LlmResponse>> {
        let key = self.cache_key(prompt, context);

        if let Some(value) = self.cache.get(&key).await? {
            match serde_json::from_str::<LlmResponse>(&value) {
                Ok(response) => {
                    info!("LLM cache hit for prompt hash: {}", key.split(':').last().unwrap_or(""));
                    Ok(Some(response))
                }
                Err(e) => {
                    debug!("Failed to deserialize cached LLM response: {}", e);
                    Ok(None)
                }
            }
        } else {
            debug!("LLM cache miss");
            Ok(None)
        }
    }

    /// Cache LLM response
    pub async fn cache_response(
        &self,
        prompt: &str,
        context: &str,
        response: &LlmResponse,
    ) -> Result<()> {
        let key = self.cache_key(prompt, context);
        let value = serde_json::to_string(response)?;

        self.cache.set(&key, &value, Some(self.ttl)).await?;
        info!("Cached LLM response");

        Ok(())
    }

    /// Clear all LLM caches
    pub async fn clear_all(&self) -> Result<()> {
        // This would require SCAN command in production
        info!("LLM cache clear requested (not implemented - requires SCAN)");
        Ok(())
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> Result<super::dragonfly::CacheStats> {
        self.cache.get_stats().await
    }
}

/// LLM response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub text: String,
    pub tokens_used: u32,
    pub model: String,
    pub timestamp: u64,
}

impl LlmResponse {
    pub fn new(text: String, tokens_used: u32, model: String) -> Self {
        Self {
            text,
            tokens_used,
            model,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_response_creation() {
        let response = LlmResponse::new(
            "Hello, adventurer!".to_string(),
            10,
            "llama3".to_string(),
        );

        assert_eq!(response.text, "Hello, adventurer!");
        assert_eq!(response.tokens_used, 10);
        assert_eq!(response.model, "llama3");
        assert!(response.timestamp > 0);
    }
}
