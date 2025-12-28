use super::dragonfly::DragonflyCache;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info};

/// Cache for session state management
#[derive(Clone)]
pub struct SessionStateCache {
    cache: DragonflyCache,
    ttl: Duration,
}

impl SessionStateCache {
    /// Create a new session state cache
    ///
    /// # Arguments
    /// * `cache` - DragonflyDB cache instance
    /// * `ttl` - Time-to-live for session data (default: 30 minutes)
    pub fn new(cache: DragonflyCache, ttl: Option<Duration>) -> Self {
        Self {
            cache,
            ttl: ttl.unwrap_or(Duration::from_secs(1800)), // 30 minutes default
        }
    }

    /// Generate cache key for session
    fn cache_key(&self, session_id: &str) -> String {
        format!("session:state:{}", session_id)
    }

    /// Get session state
    pub async fn get_session(&self, session_id: &str) -> Result<Option<SessionState>> {
        let key = self.cache_key(session_id);

        if let Some(value) = self.cache.get(&key).await? {
            match serde_json::from_str::<SessionState>(&value) {
                Ok(state) => {
                    info!("Session cache hit for: {}", session_id);
                    // Extend TTL on access (sliding expiration)
                    let _ = self.cache.expire(&key, self.ttl).await;
                    Ok(Some(state))
                }
                Err(e) => {
                    debug!("Failed to deserialize cached session state: {}", e);
                    Ok(None)
                }
            }
        } else {
            debug!("Session cache miss for: {}", session_id);
            Ok(None)
        }
    }

    /// Save session state
    pub async fn save_session(&self, session_id: &str, state: &SessionState) -> Result<()> {
        let key = self.cache_key(session_id);
        let value = serde_json::to_string(state)?;

        self.cache.set(&key, &value, Some(self.ttl)).await?;
        info!("Saved session state for: {}", session_id);

        Ok(())
    }

    /// Delete session
    pub async fn delete_session(&self, session_id: &str) -> Result<()> {
        let key = self.cache_key(session_id);
        self.cache.delete(&key).await?;

        info!("Deleted session: {}", session_id);
        Ok(())
    }

    /// Check if session exists
    pub async fn session_exists(&self, session_id: &str) -> Result<bool> {
        let key = self.cache_key(session_id);
        self.cache.exists(&key).await
    }

    /// Extend session TTL (keep alive)
    pub async fn extend_session(&self, session_id: &str) -> Result<bool> {
        let key = self.cache_key(session_id);
        self.cache.expire(&key, self.ttl).await
    }

    /// Update specific field in session
    pub async fn update_field(
        &self,
        session_id: &str,
        field: &str,
        value: serde_json::Value,
    ) -> Result<()> {
        if let Some(mut state) = self.get_session(session_id).await? {
            state.custom_data.insert(field.to_string(), value);
            self.save_session(session_id, &state).await?;
        }

        Ok(())
    }

    /// Get active session count
    pub async fn get_active_sessions_count(&self) -> Result<u64> {
        // This would require SCAN in production
        // For now, return 0
        Ok(0)
    }
}

/// Session state structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub session_id: String,
    pub player_id: String,
    pub current_location: String,
    pub current_dungeon: Option<String>,
    pub current_level: u32,
    pub health: f32,
    pub mana: f32,
    pub inventory_snapshot: Vec<String>,
    pub active_quests: Vec<String>,
    pub last_activity: u64,
    pub custom_data: std::collections::HashMap<String, serde_json::Value>,
}

impl SessionState {
    pub fn new(session_id: String, player_id: String) -> Self {
        Self {
            session_id,
            player_id,
            current_location: "village".to_string(),
            current_dungeon: None,
            current_level: 1,
            health: 100.0,
            mana: 100.0,
            inventory_snapshot: Vec::new(),
            active_quests: Vec::new(),
            last_activity: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            custom_data: std::collections::HashMap::new(),
        }
    }

    pub fn update_activity(&mut self) {
        self.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn is_in_dungeon(&self) -> bool {
        self.current_dungeon.is_some()
    }

    pub fn enter_dungeon(&mut self, dungeon_id: String) {
        self.current_dungeon = Some(dungeon_id);
        self.update_activity();
    }

    pub fn exit_dungeon(&mut self) {
        self.current_dungeon = None;
        self.update_activity();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_state_creation() {
        let state = SessionState::new("session123".to_string(), "player456".to_string());

        assert_eq!(state.session_id, "session123");
        assert_eq!(state.player_id, "player456");
        assert_eq!(state.current_location, "village");
        assert!(!state.is_in_dungeon());
    }

    #[test]
    fn test_session_dungeon_management() {
        let mut state = SessionState::new("session123".to_string(), "player456".to_string());

        assert!(!state.is_in_dungeon());

        state.enter_dungeon("forest_temple".to_string());
        assert!(state.is_in_dungeon());
        assert_eq!(state.current_dungeon, Some("forest_temple".to_string()));

        state.exit_dungeon();
        assert!(!state.is_in_dungeon());
    }
}
