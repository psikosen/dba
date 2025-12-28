pub mod components;
pub mod resources;
pub mod systems;

#[cfg(test)]
mod tests;

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<resources::InstrumentChoice>()
            .init_resource::<resources::BrotherCleansingProgress>()
            .init_resource::<resources::AfricanNamesDB>()
            .init_resource::<resources::PortraitDB>()
            .init_resource::<systems::dialogue_tree::DialogueFlags>()
            .init_resource::<systems::dialogue_tree::DialogueReputation>()
            .init_resource::<systems::dialogue_tree::DialogueTreeRegistry>()
            .init_resource::<systems::dialogue_tree::ActiveDialogueState>()
            .init_resource::<systems::quest_system::QuestLog>()
            .init_resource::<systems::quest_system::QuestRegistry>()
            .init_resource::<systems::npc_spawning::NpcsSpawned>()
            // Systems - NPC Spawning (runs after world generation)
            .add_systems(
                Update,
                (
                    systems::npc_spawning::mark_village_tiles,
                    systems::npc_spawning::spawn_village_npcs
                        .after(systems::npc_spawning::mark_village_tiles),
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Systems
            .add_systems(
                Update,
                (
                    systems::npc_sickness::update_npc_waking_state,
                    systems::dialogue::filter_sick_npc_dialogue,
                    systems::instrument_choice::apply_instrument_modifiers,
                    systems::quests::update_brother_cleansing_progress,
                    systems::dialogue_tree::apply_dialogue_consequences,
                    systems::quest_system::handle_quest_started,
                    systems::quest_system::handle_quest_completed,
                    systems::quest_system::handle_quest_objective_updated,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Startup, (setup_dialogue_trees, setup_quests))
            // Events
            .add_event::<systems::events::NpcWokenUp>()
            .add_event::<systems::events::InstrumentChosen>()
            .add_event::<systems::events::BrotherFightCompleted>()
            .add_event::<systems::dialogue_tree::DialogueChoiceSelected>()
            .add_event::<systems::dialogue_tree::DialogueTreeStarted>()
            .add_event::<systems::dialogue_tree::DialogueTreeEnded>()
            .add_event::<systems::quest_system::QuestStarted>()
            .add_event::<systems::quest_system::QuestCompleted>()
            .add_event::<systems::quest_system::QuestFailed>()
            .add_event::<systems::quest_system::QuestObjectiveUpdated>();
    }
}

/// Setup dialogue trees on startup
fn setup_dialogue_trees(mut registry: ResMut<systems::dialogue_tree::DialogueTreeRegistry>) {
    // Register spirit choice dialogue tree
    registry.register(systems::dialogue_tree::create_spirit_choice_tree());

    info!("Registered dialogue trees");
}

/// Setup quests on startup
fn setup_quests(mut registry: ResMut<systems::quest_system::QuestRegistry>) {
    // Register quests
    registry.register(systems::quest_system::create_brother_cleansing_quest());
    registry.register(systems::quest_system::create_tutorial_quest());
    registry.register(systems::quest_system::create_village_corruption_quest());

    info!("Registered quests");
}
