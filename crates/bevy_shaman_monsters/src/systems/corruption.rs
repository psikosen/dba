use crate::components::{CorruptionExposure, CorruptionInfluence, MonsterState};
use bevy::prelude::*;

const CORRUPTION_PROPAGATION_INTERVAL: f32 = 1.0; // seconds

/// Propagates corruption influence from corrupt monsters to nearby monsters
pub fn propagate_corruption_influence(
    time: Res<Time>,
    sources: Query<(Entity, &Transform, &CorruptionInfluence, &MonsterState)>,
    mut targets: Query<(Entity, &Transform, &mut CorruptionExposure)>,
) {
    // Simple time-slicing: only run every N seconds
    if time.elapsed_secs() % CORRUPTION_PROPAGATION_INTERVAL > 0.1 {
        return;
    }

    for (target_entity, target_transform, mut exposure) in targets.iter_mut() {
        exposure.accumulated = 0.0;
        exposure.sources.clear();

        for (source_entity, source_transform, influence, source_state) in sources.iter() {
            if target_entity == source_entity {
                continue;
            }

            // Only corrupt/chaos monsters spread corruption
            if !matches!(
                source_state.state,
                crate::components::StateType::Corrupt | crate::components::StateType::Chaos
            ) {
                continue;
            }

            let distance = target_transform
                .translation
                .truncate()
                .distance(source_transform.translation.truncate());

            if distance < influence.radius {
                let strength = influence.strength * (1.0 - distance / influence.radius);
                exposure.accumulated += strength;
                exposure.sources.push(source_entity);
            }
        }
    }
}
