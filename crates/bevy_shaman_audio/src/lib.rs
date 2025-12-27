pub mod components;
pub mod resources;
pub mod systems;

#[cfg(test)]
mod tests;

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
            .init_resource::<systems::audio_playback::AudioAssets>()
            // Systems
            .add_systems(Startup, systems::audio_playback::load_audio_assets)
            .add_systems(Update, (
                systems::beat_clock::update_beat_clock,
                systems::rhythm::evaluate_rhythm_inputs,
                systems::song_manager::manage_active_song,
                systems::audio_playback::play_music,
                systems::audio_playback::play_sound_effects,
                // Commented out due to circular dependency - these need combat/story events
                // systems::audio_playback::play_hit_sounds,
                systems::audio_playback::play_level_up_sound,
                // systems::audio_playback::play_quest_complete_sound,
            ).run_if(in_state(GameState::Playing)))
            // Events
            .add_event::<systems::events::BeatHit>()
            .add_event::<systems::events::RhythmInputEvaluated>()
            .add_event::<systems::audio_playback::PlaySoundEffect>();
    }
}
