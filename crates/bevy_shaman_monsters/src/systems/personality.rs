use crate::components::{MonsterState, MusicAffinityProfile};
use bevy::prelude::*;

/// Placeholder music influence event (defined in audio crate)
#[derive(Event)]
pub struct MusicPlayed {
    pub style: MusicStyle,
    pub intensity: f32,
    pub position: Vec2,
    pub radius: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum MusicStyle {
    Calm,
    Aggressive,
    Purifying,
}

/// Applies music influence to nearby monsters
pub fn apply_music_influence(
    mut music_events: EventReader<MusicPlayed>,
    mut monsters: Query<(&Transform, &mut MonsterState, &MusicAffinityProfile)>,
) {
    for music in music_events.read() {
        for (transform, mut state, affinity) in monsters.iter_mut() {
            let distance = transform.translation.truncate().distance(music.position);

            if distance > music.radius {
                continue;
            }

            let influence_strength = (1.0 - distance / music.radius) * music.intensity;

            match music.style {
                MusicStyle::Calm => {
                    let stability_gain = influence_strength * affinity.prefers_calm * 0.1;
                    state.stability_meter = (state.stability_meter + stability_gain).min(1.0);
                    state.obedience_meter = (state.obedience_meter + stability_gain * 0.5).min(1.0);
                }
                MusicStyle::Aggressive => {
                    let chaos_gain = influence_strength * affinity.prefers_aggressive * 0.15;
                    state.stability_meter = (state.stability_meter - chaos_gain).max(0.0);
                    state.chaos_output += chaos_gain;
                }
                MusicStyle::Purifying => {
                    let purify_strength = influence_strength * 0.2;
                    state.corruption_meter = (state.corruption_meter - purify_strength).max(0.0);
                }
            }
        }
    }
}
