use bevy::prelude::*;
use crate::components::{NpcDialogue, NpcSicknessState};

/// Filters NPC dialogue based on sickness state
pub fn filter_sick_npc_dialogue(
    npcs: Query<(&NpcSicknessState, &NpcDialogue)>,
) {
    // Placeholder: integrate with dialogue UI system
    // This would fetch appropriate dialogue based on NPC state
}
