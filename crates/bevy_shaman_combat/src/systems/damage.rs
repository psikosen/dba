use bevy::prelude::*;
use bevy_shaman_audio::systems::events::RhythmInputEvaluated;
use bevy_shaman_monsters::components::MonsterStats;
use crate::components::Attack;

pub fn apply_rhythm_based_damage(
    mut commands: Commands,
    mut rhythm_events: EventReader<RhythmInputEvaluated>,
    monsters: Query<(Entity, &MonsterStats)>,
    player: Query<Entity, With<bevy_shaman_core::components::Player>>,
) {
    for event in rhythm_events.read() {
        let damage_multiplier = event.quality.damage_multiplier();

        // Get the player entity as the attacker
        let Ok(player_entity) = player.get_single() else {
            continue;
        };

        // Example: create attack against nearest monster
        for (monster_entity, _stats) in monsters.iter() {
            let base_damage = 10.0;
            let final_damage = base_damage * damage_multiplier;

            commands.spawn(Attack {
                attacker: player_entity,
                damage: final_damage,
                target: monster_entity,
                rhythm_quality: event.quality,
            });

            break; // Only attack one for now
        }
    }
}
