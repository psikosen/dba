use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::resources::PortraitEmotion;

// ============================================================================
// NPC SICKNESS
// ============================================================================

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcSicknessState {
    AsleepSick,  // Cannot talk, outputs dots ("... ...")
    Waking,      // Partial dialogue
    Awake,       // Full dialogue
}

impl Default for NpcSicknessState {
    fn default() -> Self {
        Self::AsleepSick
    }
}

#[derive(Component)]
pub struct NpcDialogue {
    pub full_dialogue: String,
    pub partial_dialogue: Option<String>,
    pub sick_dialogue: String, // Usually dots
}

impl Default for NpcDialogue {
    fn default() -> Self {
        Self {
            full_dialogue: String::new(),
            partial_dialogue: None,
            sick_dialogue: "... ... ...".to_string(),
        }
    }
}

impl NpcDialogue {
    pub fn get_dialogue(&self, state: NpcSicknessState) -> &str {
        match state {
            NpcSicknessState::AsleepSick => &self.sick_dialogue,
            NpcSicknessState::Waking => {
                self.partial_dialogue.as_deref().unwrap_or(&self.full_dialogue)
            }
            NpcSicknessState::Awake => &self.full_dialogue,
        }
    }
}

// ============================================================================
// KEY NPCs
// ============================================================================

#[derive(Component, Clone)]
pub struct NpcName {
    pub name: String,
    pub current_emotion: PortraitEmotion,
}

impl Default for NpcName {
    fn default() -> Self {
        Self {
            name: "Villager".to_string(),
            current_emotion: PortraitEmotion::Neutral,
        }
    }
}

#[derive(Component)]
pub struct HeadShaman;

#[derive(Component)]
pub struct PlayerBrother {
    pub soul_corruption: f32, // 0.0 = cleansed, 1.0 = fully corrupt
    pub fights_remaining: u8, // 4 fights total
}

impl Default for PlayerBrother {
    fn default() -> Self {
        Self {
            soul_corruption: 1.0,
            fights_remaining: 4,
        }
    }
}

// ============================================================================
// WORLD MARKERS
// ============================================================================

/// Marker component for village tiles
#[derive(Component)]
pub struct VillageMarker;

// ============================================================================
// QUESTS
// ============================================================================

#[derive(Component)]
pub struct Quest {
    pub quest_id: String,
    pub description: String,
    pub completed: bool,
}
