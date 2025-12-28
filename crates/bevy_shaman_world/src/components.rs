use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// TILE CORRUPTION
// ============================================================================

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TileCorruption {
    pub level: f32, // 0.0 = pure, 1.0 = fully corrupt
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

    /// Returns the corruption tier (1-4) based on level
    /// - Tier 1: 0.0 - 0.25 (light corruption)
    /// - Tier 2: 0.25 - 0.50 (moderate corruption)
    /// - Tier 3: 0.50 - 0.75 (heavy corruption)
    /// - Tier 4: 0.75 - 1.0 (severe corruption)
    pub fn corruption_tier(&self) -> u8 {
        if self.purified {
            return 0;
        }
        match self.level {
            x if x < 0.25 => 1,
            x if x < 0.50 => 2,
            x if x < 0.75 => 3,
            _ => 4,
        }
    }

    /// Returns the asset path suffix for this corruption tier
    /// Format: "{corruption_type}_lv{tier}" e.g., "chaos_lv3"
    pub fn asset_suffix(&self) -> String {
        if self.purified {
            return "purified".to_string();
        }
        if self.corruption_type == CorruptionType::None {
            return "pure".to_string();
        }
        format!(
            "{}_lv{}",
            self.corruption_type.asset_name(),
            self.corruption_tier()
        )
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

    /// Returns corruption color adjusted for tier intensity
    pub fn corruption_color_by_tier(&self) -> Color {
        let base = self.corruption_color();
        let tier = self.corruption_tier();
        // Increase saturation/intensity with higher tiers
        let intensity = 0.4 + (tier as f32 * 0.15); // 0.55, 0.70, 0.85, 1.0
        base.with_alpha(intensity)
    }
}

impl CorruptionType {
    /// Returns the asset name prefix for this corruption type
    pub fn asset_name(&self) -> &'static str {
        match self {
            CorruptionType::None => "pure",
            CorruptionType::Chaos => "chaos",
            CorruptionType::Decay => "decay",
            CorruptionType::Void => "void",
            CorruptionType::Ancestral => "ancestral",
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

/// Marks a tile as a dungeon entrance
#[derive(Component, Clone)]
pub struct DungeonEntrance {
    pub dungeon_id: String,
    pub ecosystem: BiomeType,
    pub difficulty_level: u8, // 1-5
    pub is_discovered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiomeType {
    // Central hub
    Village,

    // 5 Ecosystems (each contains dungeons)
    Jungle,
    Desert,
    Forest,
    Safari,
    DeadRealm,

    // Legacy/Special
    Mountains,
    SpiritRealm,
}

impl BiomeType {
    /// Returns true if this biome can contain dungeon entrances
    pub fn can_have_dungeons(&self) -> bool {
        matches!(
            self,
            BiomeType::Jungle
                | BiomeType::Desert
                | BiomeType::Forest
                | BiomeType::Safari
                | BiomeType::DeadRealm
        )
    }

    /// Returns the name of this biome for display
    pub fn display_name(&self) -> &'static str {
        match self {
            BiomeType::Village => "Village",
            BiomeType::Jungle => "Jungle",
            BiomeType::Desert => "Desert",
            BiomeType::Forest => "Forest",
            BiomeType::Safari => "Safari",
            BiomeType::DeadRealm => "Dead Realm",
            BiomeType::Mountains => "Mountains",
            BiomeType::SpiritRealm => "Spirit Realm",
        }
    }
}
