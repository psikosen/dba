use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// PURIFICATION ABILITY
// ============================================================================

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct PurificationAbility {
    pub range: u32,
    pub shape: PurificationShape,
    pub spirit_cost: f32,
}

impl Default for PurificationAbility {
    fn default() -> Self {
        Self {
            range: 1,
            shape: PurificationShape::Single,
            spirit_cost: 20.0,
        }
    }
}

/// Purification shape progression: 1 → 3 → 6 → 8 → Screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PurificationShape {
    /// Starting ability: 1 tile at a time
    Single,
    /// First upgrade: 3 tiles
    Triangle,
    /// Second upgrade: 6 tiles
    Hexagon,
    /// Third upgrade: 8 tiles
    Octagon,
    /// Final upgrade: All tiles on screen
    Screen,
}

impl PurificationShape {
    /// Returns relative tile offsets for this shape
    pub fn get_offsets(&self, _range: u32) -> Vec<(i32, i32)> {
        match self {
            PurificationShape::Single => vec![(0, 0)],
            PurificationShape::Triangle => {
                // 3 tiles: center and two adjacent
                vec![(0, 0), (1, 0), (0, 1)]
            }
            PurificationShape::Hexagon => {
                // 6 tiles: center + 5 surrounding
                vec![(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1), (1, 1)]
            }
            PurificationShape::Octagon => {
                // 8 tiles: center + 7 surrounding
                vec![
                    (0, 0),
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                    (1, 1),
                    (-1, -1),
                    (1, -1),
                ]
            }
            PurificationShape::Screen => {
                // All tiles on screen (approximate 15x15 area around player)
                let screen_range = 7i32;
                let mut offsets = Vec::new();
                for dx in -screen_range..=screen_range {
                    for dy in -screen_range..=screen_range {
                        offsets.push((dx, dy));
                    }
                }
                offsets
            }
        }
    }

    /// Get the tile count for this shape
    pub fn tile_count(&self) -> u32 {
        match self {
            PurificationShape::Single => 1,
            PurificationShape::Triangle => 3,
            PurificationShape::Hexagon => 6,
            PurificationShape::Octagon => 8,
            PurificationShape::Screen => 225, // 15x15
        }
    }
}

// ============================================================================
// BOSS UNLOCK FLAGS
// ============================================================================

/// Tracks boss defeats and unlocked progression gates
#[derive(Resource, Default, Serialize, Deserialize)]
pub struct BossUnlockFlags {
    pub bosses_defeated: Vec<String>,
    pub songs_unlocked: Vec<String>,
    pub spirit_worlds_unlocked: Vec<crate::components::CorruptionType>,
}

impl BossUnlockFlags {
    pub fn defeat_boss(&mut self, boss_id: String) {
        if !self.bosses_defeated.contains(&boss_id) {
            self.bosses_defeated.push(boss_id);
        }
    }

    pub fn unlock_song(&mut self, song_id: String) {
        if !self.songs_unlocked.contains(&song_id) {
            self.songs_unlocked.push(song_id);
        }
    }

    pub fn bosses_defeated_count(&self) -> usize {
        self.bosses_defeated.len()
    }
}

// ============================================================================
// WORLD STATE RULES
// ============================================================================

/// Rules for each Spirit World (passive effects, encounter modifiers)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritWorldRules {
    pub world_id: String,
    pub corruption_growth_rate_modifier: f32,
    pub stability_decay_modifier: f32,
    pub encounter_pool: Vec<String>, // monster IDs
}

#[derive(Resource, Default)]
pub struct WorldStateRulesDB {
    pub rules: HashMap<String, SpiritWorldRules>,
}

impl WorldStateRulesDB {
    pub fn register(&mut self, rules: SpiritWorldRules) {
        self.rules.insert(rules.world_id.clone(), rules);
    }

    pub fn get(&self, world_id: &str) -> Option<&SpiritWorldRules> {
        self.rules.get(world_id)
    }

    pub fn populate_defaults(&mut self) {
        self.register(SpiritWorldRules {
            world_id: "chaos_world".to_string(),
            corruption_growth_rate_modifier: 2.0,
            stability_decay_modifier: 1.5,
            encounter_pool: vec!["chaos_hound".to_string(), "chaos_sprite".to_string()],
        });

        self.register(SpiritWorldRules {
            world_id: "harmony_world".to_string(),
            corruption_growth_rate_modifier: 0.5,
            stability_decay_modifier: 0.5,
            encounter_pool: vec!["forest_spirit".to_string(), "healing_wisp".to_string()],
        });
    }
}
