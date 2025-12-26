pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<resources::BeatClock>()
            .init_resource::<resources::SongDB>()
            .init_resource::<resources::ActiveSong>()
            // Systems
            .add_systems(Update, (
                systems::beat_clock::update_beat_clock,
                systems::rhythm::evaluate_rhythm_inputs,
                systems::song_manager::manage_active_song,
            ).run_if(in_state(GameState::Playing)))
            // Events
            .add_event::<systems::events::BeatHit>()
            .add_event::<systems::events::RhythmInputEvaluated>();
    }
}
