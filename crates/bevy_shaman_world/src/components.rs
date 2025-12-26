use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// TILE CORRUPTION
// ============================================================================

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TileCorruption {
    pub level: f32,              // 0.0 = pure, 1.0 = fully corrupt
    pub corruption_type: CorruptionType,
    pub purified: bool,
    pub purified_timestamp: Option<f64>,
}

impl Default for TileCorruption {
    fn default() -> Self {
        Self {
            level: 0.0,
            corruption_type: CorruptionType::None,
            purified: false,
            purified_timestamp: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorruptionType {
    None,
    Chaos,
    Decay,
    Void,
    Ancestral,
}

impl TileCorruption {
    pub fn is_corrupt(&self) -> bool {
        self.level > 0.3
    }

    pub fn corruption_color(&self) -> Color {
        match self.corruption_type {
            CorruptionType::None => Color::WHITE,
            CorruptionType::Chaos => Color::srgb(0.8, 0.2, 0.2),
            CorruptionType::Decay => Color::srgb(0.4, 0.6, 0.2),
            CorruptionType::Void => Color::srgb(0.1, 0.1, 0.3),
            CorruptionType::Ancestral => Color::srgb(0.6, 0.4, 0.8),
        }
    }
}

// ============================================================================
// TILE METADATA
// ============================================================================

#[derive(Component)]
pub struct WorldTile {
    pub biome: BiomeType,
    pub walkable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiomeType {
    Village,
    Forest,
    Mountains,
    SpiritRealm,
}
