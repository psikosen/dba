use bevy::prelude::*;
use crate::resources::{ActiveSong, BeatClock, TimingQuality};
use crate::systems::events::RhythmInputEvaluated;

/// Evaluates rhythm inputs from player (triggered by input system)
pub fn evaluate_rhythm_inputs(
    keyboard: Res<ButtonInput<KeyCode>>,
    clock: Res<BeatClock>,
    mut active_song: ResMut<ActiveSong>,
    mut rhythm_events: EventWriter<RhythmInputEvaluated>,
) {
    // Check for rhythm input (spacebar as example)
    if keyboard.just_pressed(KeyCode::Space) {
        let quality = clock.evaluate_timing();

        match quality {
            TimingQuality::Perfect | TimingQuality::Great => {
                active_song.increment_combo();
            }
            TimingQuality::Good => {
                // Maintain combo
            }
            TimingQuality::Miss => {
                active_song.break_combo();
            }
        }

        rhythm_events.send(RhythmInputEvaluated {
            quality,
            combo: active_song.combo,
        });
    }
}
