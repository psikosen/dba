use bevy::prelude::*;
use bevy_shaman_audio::systems::events::RhythmInputEvaluated;
use bevy_shaman_monsters::components::MonsterStats;
use crate::components::Attack;

pub fn apply_rhythm_based_damage(
    mut commands: Commands,
    mut rhythm_events: EventReader<RhythmInputEvaluated>,
    monsters: Query<(Entity, &MonsterStats)>,
) {
    for event in rhythm_events.read() {
        let damage_multiplier = event.quality.damage_multiplier();

        // Example: create attack against nearest monster
        for (monster_entity, stats) in monsters.iter() {
            let base_damage = 10.0;
            let final_damage = base_damage * damage_multiplier;

            commands.spawn(Attack {
                damage: final_damage,
                target: monster_entity,
                rhythm_quality: event.quality,
            });

            break; // Only attack one for now
        }
    }
}
