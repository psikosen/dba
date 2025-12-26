pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct StoryPlugin;

impl Plugin for StoryPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<resources::InstrumentChoice>()
            .init_resource::<resources::BrotherCleansingProgress>()
            // Systems
            .add_systems(Update, (
                systems::npc_sickness::update_npc_waking_state,
                systems::dialogue::filter_sick_npc_dialogue,
                systems::instrument_choice::apply_instrument_modifiers,
                systems::quests::update_brother_cleansing_progress,
            ).run_if(in_state(GameState::Playing)))
            // Events
            .add_event::<systems::events::NpcWokenUp>()
            .add_event::<systems::events::InstrumentChosen>()
            .add_event::<systems::events::BrotherFightCompleted>();
    }
}
