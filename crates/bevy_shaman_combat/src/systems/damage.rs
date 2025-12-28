#[cfg(feature = "audio")]
use super::super::components::{Attack, EquippedWeapon};
#[cfg(feature = "audio")]
use bevy::prelude::*;
#[cfg(feature = "audio")]
use bevy_shaman_audio::systems::events::RhythmInputEvaluated;
#[cfg(feature = "audio")]
use bevy_shaman_core::components::GridPosition;
#[cfg(feature = "audio")]
use bevy_shaman_monsters::components::MonsterStats;

#[cfg(feature = "audio")]
pub fn apply_rhythm_based_damage(
    mut commands: Commands,
    mut rhythm_events: EventReader<RhythmInputEvaluated>,
    monsters: Query<(Entity, &GridPosition, &MonsterStats)>,
    player: Query<
        (Entity, &GridPosition, Option<&EquippedWeapon>),
        With<bevy_shaman_core::components::Player>,
    >,
) {
    for event in rhythm_events.read() {
        let damage_multiplier = event.quality.damage_multiplier();

        // Get the player entity and position as the attacker
        let Ok((player_entity, player_pos, equipped_weapon)) = player.get_single() else {
            continue;
        };

        // Calculate base damage from weapon or use default
        let base_damage = if let Some(weapon) = equipped_weapon {
            weapon.weapon_type.base_damage() * weapon.damage_multiplier
        } else {
            10.0 // Default unarmed damage
        };

        // Find nearest monster within attack range
        let attack_range = 2.0; // Grid units
        let mut nearest_monster: Option<(Entity, f32)> = None;

        for (monster_entity, monster_pos, _stats) in monsters.iter() {
            let distance = (((player_pos.x - monster_pos.x).pow(2)
                + (player_pos.y - monster_pos.y).pow(2)) as f32)
                .sqrt();

            if distance <= attack_range {
                if let Some((_, current_nearest_dist)) = nearest_monster {
                    if distance < current_nearest_dist {
                        nearest_monster = Some((monster_entity, distance));
                    }
                } else {
                    nearest_monster = Some((monster_entity, distance));
                }
            }
        }

        // Create attack against nearest monster in range
        if let Some((target_entity, _)) = nearest_monster {
            let final_damage = base_damage * damage_multiplier;

            commands.spawn(Attack {
                attacker: player_entity,
                damage: final_damage,
                target: target_entity,
                rhythm_quality: event.quality,
            });
        }
    }
}
