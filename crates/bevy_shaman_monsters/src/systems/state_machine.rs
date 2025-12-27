use bevy::prelude::*;
use crate::components::{CorruptionExposure, MonsterState};
use crate::systems::events::{MonsterCorrupted, MonsterStateChanged};

const STABILITY_DECAY_RATE: f32 = 0.01;
const CORRUPTION_GROWTH_RATE: f32 = 0.005;

/// Updates monster state meters based on various factors
pub fn update_monster_state_meters(
    time: Res<Time>,
    mut monsters: Query<(Entity, &mut MonsterState, Option<&CorruptionExposure>)>,
    mut corruption_events: EventWriter<MonsterCorrupted>,
) {
    for (entity, mut state, exposure) in monsters.iter_mut() {
        let dt = time.delta_secs();

        // Passive stability decay in chaotic environments
        if state.state == crate::components::StateType::Chaos {
            state.stability_meter = (state.stability_meter - STABILITY_DECAY_RATE * dt).max(0.0);
        }

        // Corruption growth from exposure
        if let Some(exp) = exposure {
            if exp.accumulated > 0.0 {
                let growth = CORRUPTION_GROWTH_RATE * exp.accumulated * dt;
                state.corruption_meter = (state.corruption_meter + growth).min(1.0);

                if state.corruption_meter > 0.7 {
                    corruption_events.send(MonsterCorrupted {
                        entity,
                        corruption_level: state.corruption_meter,
                    });
                }
            }
        }

        // Chaos output affects damage multiplier
        state.chaos_output = if state.state == crate::components::StateType::Chaos {
            1.0 + (1.0 - state.stability_meter) * 0.5 // up to 50% bonus
        } else {
            1.0
        };
    }
}

/// Evaluates state transitions based on meters
pub fn evaluate_state_transitions(
    mut monsters: Query<(Entity, &mut MonsterState)>,
    mut state_events: EventWriter<MonsterStateChanged>,
) {
    for (entity, mut state) in monsters.iter_mut() {
        let old_state = state.state;
        let new_state = state.evaluate_state();

        if old_state != new_state {
            state.state = new_state;
            state_events.send(MonsterStateChanged {
                entity,
                old_state,
                new_state,
            });
        }
    }
}
