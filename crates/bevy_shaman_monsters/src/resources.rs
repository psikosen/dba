use bevy::prelude::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::components::{MonsterStats, MusicAffinityProfile, StateType};

// ============================================================================
// MONSTER SPRITE DATABASE
// ============================================================================

/// Maps (monster_id, state) -> sprite handles for visual swapping
#[derive(Resource, Default)]
pub struct MonsterSpriteDB {
    pub sprites: HashMap<(String, StateType), Handle<Image>>,
}

impl MonsterSpriteDB {
    pub fn get(&self, monster_id: &str, state: StateType) -> Option<&Handle<Image>> {
        self.sprites.get(&(monster_id.to_string(), state))
    }

    pub fn register(&mut self, monster_id: String, state: StateType, handle: Handle<Image>) {
        self.sprites.insert((monster_id, state), handle);
    }
}

// ============================================================================
// MONSTER TEMPLATE DATABASE
// ============================================================================

/// Monster archetypes with default stats/behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterTemplate {
    pub id: String,
    pub display_name: String,
    pub base_stats: MonsterStats,
    pub affinity_profile: MusicAffinityProfile,
    pub default_state: StateType,
    pub tameable: bool,
}

#[derive(Resource, Default)]
pub struct MonsterTemplateDB {
    pub templates: HashMap<String, MonsterTemplate>,
}

impl MonsterTemplateDB {
    pub fn get(&self, id: &str) -> Option<&MonsterTemplate> {
        self.templates.get(id)
    }

    pub fn register(&mut self, template: MonsterTemplate) {
        self.templates.insert(template.id.clone(), template);
    }

    /// Example: populate with initial monster types
    pub fn populate_defaults(&mut self) {
        self.register(MonsterTemplate {
            id: "forest_spirit".to_string(),
            display_name: "Forest Spirit".to_string(),
            base_stats: MonsterStats {
                attack: 8.0,
                defense: 6.0,
                speed: 7.0,
                spirit_affinity: 0.8,
            },
            affinity_profile: MusicAffinityProfile {
                prefers_calm: 0.9,
                prefers_aggressive: 0.2,
                corruption_resistance: 0.7,
                trust_level: 0.0,
            },
            default_state: StateType::Stable,
            tameable: true,
        });

        self.register(MonsterTemplate {
            id: "chaos_hound".to_string(),
            display_name: "Chaos Hound".to_string(),
            base_stats: MonsterStats {
                attack: 15.0,
                defense: 4.0,
                speed: 10.0,
                spirit_affinity: 0.3,
            },
            affinity_profile: MusicAffinityProfile {
                prefers_calm: 0.1,
                prefers_aggressive: 0.95,
                corruption_resistance: 0.2,
                trust_level: 0.0,
            },
            default_state: StateType::Chaos,
            tameable: false,
        });

        self.register(MonsterTemplate {
            id: "corrupt_shade".to_string(),
            display_name: "Corrupt Shade".to_string(),
            base_stats: MonsterStats {
                attack: 12.0,
                defense: 8.0,
                speed: 6.0,
                spirit_affinity: 0.1,
            },
            affinity_profile: MusicAffinityProfile {
                prefers_calm: 0.0,
                prefers_aggressive: 0.5,
                corruption_resistance: 0.0,
                trust_level: 0.0,
            },
            default_state: StateType::Corrupt,
            tameable: false,
        });
    }
}
