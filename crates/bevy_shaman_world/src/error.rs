use std::fmt;

/// Errors that can occur during world generation
#[derive(Debug)]
pub enum WorldGenerationError {
    /// Invalid world configuration parameters
    InvalidConfiguration(String),
    /// Failed to generate world with given seed
    GenerationFailed(String),
    /// Biome placement failed
    BiomePlacementFailed(String),
    /// Dungeon spawning failed
    DungeonSpawnFailed(String),
    /// System time error (fallback to default seed)
    TimeError(String),
}

impl fmt::Display for WorldGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorldGenerationError::InvalidConfiguration(msg) => {
                write!(f, "Invalid world generation configuration: {}", msg)
            }
            WorldGenerationError::GenerationFailed(msg) => {
                write!(f, "World generation failed: {}", msg)
            }
            WorldGenerationError::BiomePlacementFailed(msg) => {
                write!(f, "Biome placement failed: {}", msg)
            }
            WorldGenerationError::DungeonSpawnFailed(msg) => {
                write!(f, "Dungeon spawn failed: {}", msg)
            }
            WorldGenerationError::TimeError(msg) => {
                write!(f, "System time error: {}", msg)
            }
        }
    }
}

impl std::error::Error for WorldGenerationError {}

impl From<std::time::SystemTimeError> for WorldGenerationError {
    fn from(err: std::time::SystemTimeError) -> Self {
        WorldGenerationError::TimeError(err.to_string())
    }
}
