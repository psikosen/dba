use std::fmt;
use std::error::Error;

/// Dungeon system errors
#[derive(Debug, Clone)]
pub enum DungeonError {
    /// Dungeon generation failed
    GenerationFailed {
        dungeon_id: String,
        level: u32,
        reason: String,
    },

    /// Invalid dungeon seed
    InvalidSeed {
        seed: String,
        reason: String,
    },

    /// Dungeon not found
    DungeonNotFound {
        dungeon_id: String,
    },

    /// Invalid dungeon layout
    InvalidLayout {
        reason: String,
    },

    /// Monster placement failed
    MonsterPlacementFailed {
        monster_id: String,
        position: (i32, i32),
        reason: String,
    },

    /// Item placement failed
    ItemPlacementFailed {
        item_id: String,
        position: (i32, i32),
        reason: String,
    },

    /// Boss encounter error
    BossEncounterError {
        boss_id: String,
        reason: String,
    },

    /// Dungeon progression error
    ProgressionError {
        current_level: u32,
        reason: String,
    },

    /// Room connection error
    RoomConnectionError {
        from: String,
        to: String,
        reason: String,
    },

    /// Serialization/deserialization error
    SerializationError {
        reason: String,
    },

    /// Generic dungeon system error
    SystemError(String),
}

impl fmt::Display for DungeonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DungeonError::GenerationFailed { dungeon_id, level, reason } => {
                write!(
                    f,
                    "Failed to generate dungeon '{}' level {}: {}",
                    dungeon_id, level, reason
                )
            }
            DungeonError::InvalidSeed { seed, reason } => {
                write!(f, "Invalid dungeon seed '{}': {}", seed, reason)
            }
            DungeonError::DungeonNotFound { dungeon_id } => {
                write!(f, "Dungeon '{}' not found", dungeon_id)
            }
            DungeonError::InvalidLayout { reason } => {
                write!(f, "Invalid dungeon layout: {}", reason)
            }
            DungeonError::MonsterPlacementFailed { monster_id, position, reason } => {
                write!(
                    f,
                    "Failed to place monster '{}' at ({}, {}): {}",
                    monster_id, position.0, position.1, reason
                )
            }
            DungeonError::ItemPlacementFailed { item_id, position, reason } => {
                write!(
                    f,
                    "Failed to place item '{}' at ({}, {}): {}",
                    item_id, position.0, position.1, reason
                )
            }
            DungeonError::BossEncounterError { boss_id, reason } => {
                write!(f, "Boss encounter error for '{}': {}", boss_id, reason)
            }
            DungeonError::ProgressionError { current_level, reason } => {
                write!(
                    f,
                    "Dungeon progression error at level {}: {}",
                    current_level, reason
                )
            }
            DungeonError::RoomConnectionError { from, to, reason } => {
                write!(
                    f,
                    "Failed to connect room '{}' to '{}': {}",
                    from, to, reason
                )
            }
            DungeonError::SerializationError { reason } => {
                write!(f, "Dungeon serialization error: {}", reason)
            }
            DungeonError::SystemError(msg) => {
                write!(f, "Dungeon system error: {}", msg)
            }
        }
    }
}

impl Error for DungeonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<serde_json::Error> for DungeonError {
    fn from(err: serde_json::Error) -> Self {
        DungeonError::SerializationError {
            reason: err.to_string(),
        }
    }
}

/// Result type for dungeon operations
pub type DungeonResult<T> = Result<T, DungeonError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = DungeonError::GenerationFailed {
            dungeon_id: "forest_temple".to_string(),
            level: 1,
            reason: "Not enough space".to_string(),
        };

        assert_eq!(
            error.to_string(),
            "Failed to generate dungeon 'forest_temple' level 1: Not enough space"
        );
    }

    #[test]
    fn test_monster_placement_error() {
        let error = DungeonError::MonsterPlacementFailed {
            monster_id: "goblin".to_string(),
            position: (5, 10),
            reason: "Tile occupied".to_string(),
        };

        assert!(error.to_string().contains("(5, 10)"));
    }
}
