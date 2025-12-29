use super::dragonfly::DragonflyCache;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{debug, info};

/// Cache for dungeon seeds to enable faster level loading
#[derive(Clone)]
pub struct DungeonSeedCache {
    cache: DragonflyCache,
    ttl: Duration,
}

impl DungeonSeedCache {
    /// Create a new dungeon seed cache
    ///
    /// # Arguments
    /// * `cache` - DragonflyDB cache instance
    /// * `ttl` - Time-to-live for cached seeds (default: 24 hours)
    pub fn new(cache: DragonflyCache, ttl: Option<Duration>) -> Self {
        Self {
            cache,
            ttl: ttl.unwrap_or(Duration::from_secs(86400)), // 24 hours default
        }
    }

    /// Generate cache key for dungeon seed
    fn cache_key(&self, dungeon_id: &str, level: u32, difficulty: &str) -> String {
        format!("dungeon:seed:{}:{}:{}", dungeon_id, level, difficulty)
    }

    /// Get cached dungeon seed
    pub async fn get_seed(
        &self,
        dungeon_id: &str,
        level: u32,
        difficulty: &str,
    ) -> Result<Option<DungeonSeed>> {
        let key = self.cache_key(dungeon_id, level, difficulty);

        if let Some(value) = self.cache.get(&key).await? {
            match serde_json::from_str::<DungeonSeed>(&value) {
                Ok(seed) => {
                    info!(
                        "Dungeon seed cache hit for {}/level{}/{}",
                        dungeon_id, level, difficulty
                    );
                    Ok(Some(seed))
                }
                Err(e) => {
                    debug!("Failed to deserialize cached dungeon seed: {}", e);
                    Ok(None)
                }
            }
        } else {
            debug!("Dungeon seed cache miss");
            Ok(None)
        }
    }

    /// Cache dungeon seed
    pub async fn cache_seed(&self, seed: &DungeonSeed) -> Result<()> {
        let key = self.cache_key(&seed.dungeon_id, seed.level, &seed.difficulty);
        let value = serde_json::to_string(seed)?;

        self.cache.set(&key, &value, Some(self.ttl)).await?;
        info!(
            "Cached dungeon seed for {}/level{}/{}",
            seed.dungeon_id, seed.level, seed.difficulty
        );

        Ok(())
    }

    /// Invalidate dungeon seed cache for a specific dungeon
    pub async fn invalidate_dungeon(&self, dungeon_id: &str, level: u32) -> Result<()> {
        // In production, you'd use SCAN to find all matching keys
        // For now, we'll delete common difficulty levels
        for difficulty in &["easy", "normal", "hard", "nightmare"] {
            let key = self.cache_key(dungeon_id, level, difficulty);
            let _ = self.cache.delete(&key).await;
        }

        info!("Invalidated dungeon cache for {}/level{}", dungeon_id, level);
        Ok(())
    }

    /// Preload dungeon seeds for faster loading
    pub async fn preload_seeds(&self, seeds: Vec<DungeonSeed>) -> Result<()> {
        let count = seeds.len();
        for seed in seeds {
            self.cache_seed(&seed).await?;
        }

        info!("Preloaded {} dungeon seeds", count);
        Ok(())
    }
}

/// Dungeon seed structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonSeed {
    pub dungeon_id: String,
    pub level: u32,
    pub difficulty: String,
    pub seed: u64,
    pub layout_hash: String,
    pub room_count: u32,
    pub monster_placements: Vec<MonsterPlacement>,
    pub item_placements: Vec<ItemPlacement>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterPlacement {
    pub monster_id: String,
    pub room_index: u32,
    pub position: (f32, f32),
    pub level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPlacement {
    pub item_id: String,
    pub room_index: u32,
    pub position: (f32, f32),
    pub rarity: String,
}

impl DungeonSeed {
    pub fn new(
        dungeon_id: String,
        level: u32,
        difficulty: String,
        seed: u64,
        layout_hash: String,
    ) -> Self {
        Self {
            dungeon_id,
            level,
            difficulty,
            seed,
            layout_hash,
            room_count: 0,
            monster_placements: Vec::new(),
            item_placements: Vec::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn with_room_count(mut self, count: u32) -> Self {
        self.room_count = count;
        self
    }

    pub fn with_monsters(mut self, placements: Vec<MonsterPlacement>) -> Self {
        self.monster_placements = placements;
        self
    }

    pub fn with_items(mut self, placements: Vec<ItemPlacement>) -> Self {
        self.item_placements = placements;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dungeon_seed_builder() {
        let seed = DungeonSeed::new(
            "forest_temple".to_string(),
            1,
            "normal".to_string(),
            12345,
            "abc123".to_string(),
        )
        .with_room_count(15);

        assert_eq!(seed.dungeon_id, "forest_temple");
        assert_eq!(seed.level, 1);
        assert_eq!(seed.difficulty, "normal");
        assert_eq!(seed.seed, 12345);
        assert_eq!(seed.room_count, 15);
    }
}
