use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, Player};
use crate::components::{NpcDialogue, NpcSicknessState, NpcName};
use crate::systems::dialogue_tree::{
    ActiveDialogueState, DialogueTreeRegistry, DialogueTreeStarted
};

/// Event for starting a dialogue with an NPC
#[derive(Event)]
pub struct StartDialogue {
    pub npc_entity: Entity,
    pub tree_id: Option<String>, // If None, use simple NpcDialogue
}

/// Component to mark an NPC as having a dialogue tree
#[derive(Component)]
pub struct HasDialogueTree {
    pub tree_id: String,
}

/// Initiate dialogue when player interacts with NPC
pub fn initiate_npc_dialogue(
    player: Query<&GridPosition, With<Player>>,
    npcs: Query<(Entity, &GridPosition, Option<&HasDialogueTree>, Option<&NpcDialogue>, Option<&NpcSicknessState>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut dialogue_state: ResMut<ActiveDialogueState>,
    mut start_events: EventWriter<DialogueTreeStarted>,
    mut simple_dialogue_events: EventWriter<StartDialogue>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok(player_pos) = player.get_single() else {
        return;
    };

    let interact_range = 1.5; // Grid units

    for (npc_entity, npc_pos, tree_opt, dialogue_opt, sickness_opt) in npcs.iter() {
        let distance = ((player_pos.x - npc_pos.x).abs() + (player_pos.y - npc_pos.y).abs()) as f32;

        if distance <= interact_range {
            // Check if NPC has a dialogue tree
            if let Some(tree) = tree_opt {
                // Start dialogue tree
                dialogue_state.start_dialogue(tree.tree_id.clone(), npc_entity);
                start_events.send(DialogueTreeStarted {
                    tree_id: tree.tree_id.clone(),
                    npc_entity,
                });
                return;
            } else if dialogue_opt.is_some() {
                // Start simple dialogue
                simple_dialogue_events.send(StartDialogue {
                    npc_entity,
                    tree_id: None,
                });
                return;
            }
        }
    }
}

/// System to show simple NPC dialogue in the UI
pub fn show_simple_dialogue(
    mut dialogue_events: EventReader<StartDialogue>,
    npcs: Query<(&NpcDialogue, &NpcSicknessState, &NpcName)>,
) {
    for event in dialogue_events.read() {
        if let Ok((dialogue, sickness, name)) = npcs.get(event.npc_entity) {
            let text = dialogue.get_dialogue(*sickness);
            info!("{}: {}", name.name, text);
            // UI system will pick this up from the ActiveDialogueState
        }
    }
}
