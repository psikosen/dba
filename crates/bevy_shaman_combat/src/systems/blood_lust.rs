use bevy::prelude::*;
use crate::components::*;
use bevy_shaman_core::components::Health;
use bevy_shaman_monsters::components::MonsterState;

/// System to increase blood lust during combat
pub fn blood_lust_combat_gain(
    mut player_query: Query<&mut BloodLust, With<bevy_shaman_core::components::Player>>,
    mut event_reader: EventReader<crate::systems::events::HitLanded>,
    health_query: Query<&Health>,
) {
    for event in event_reader.read() {
        if let Ok(mut blood_lust) = player_query.get_single_mut() {
            if let Ok(target_health) = health_query.get(event.target) {
                // Check if it was overkill (dealt more damage than remaining health)
                let was_overkill = event.damage > target_health.current;

                // Determine combat difficulty based on enemy health
                let difficulty = if target_health.max > 200.0 {
                    CombatDifficulty::Boss
                } else if target_health.max > 100.0 {
                    CombatDifficulty::Hard
                } else if target_health.max > 50.0 {
                    CombatDifficulty::Normal
                } else {
                    CombatDifficulty::Easy
                };

                blood_lust.add_from_combat(target_health.max, was_overkill, difficulty);
            }
        }
    }
}

/// System to decay blood lust over time when not in combat
pub fn blood_lust_decay(
    mut query: Query<&mut BloodLust>,
    time: Res<Time>,
    combat_state: Res<State<bevy_shaman_core::states::CombatState>>,
) {
    // Only decay when not in combat
    if *combat_state.get() == bevy_shaman_core::states::CombatState::None {
        for mut blood_lust in query.iter_mut() {
            blood_lust.current = (blood_lust.current - blood_lust.decay_rate * time.delta_secs()).max(0.0);
        }
    }
}

/// System to corrupt monsters when blood lust is high
pub fn blood_lust_corruption_spread(
    player_query: Query<&BloodLust, With<bevy_shaman_core::components::Player>>,
    mut monster_query: Query<&mut MonsterState>,
    time: Res<Time>,
) {
    if let Ok(blood_lust) = player_query.get_single() {
        if blood_lust.is_corrupting() {
            let corruption_rate = (blood_lust.current - blood_lust.threshold) * 0.01;

            for mut monster_state in monster_query.iter_mut() {
                monster_state.corruption_meter = (monster_state.corruption_meter + corruption_rate * time.delta_secs()).min(1.0);
            }
        }
    }
}

/// System to reduce blood lust from external events (plants, food, music)
pub fn blood_lust_reduction_from_items(
    mut blood_lust_query: Query<&mut BloodLust>,
    mut event_reader: EventReader<bevy_shaman_items::systems::plant_food::ReduceBloodLust>,
) {
    for event in event_reader.read() {
        if let Ok(mut blood_lust) = blood_lust_query.get_mut(event.entity) {
            match event.source {
                bevy_shaman_items::systems::plant_food::BloodLustReductionSource::Plant => {
                    blood_lust.reduce_with_plant(event.amount);
                }
                bevy_shaman_items::systems::plant_food::BloodLustReductionSource::Food => {
                    blood_lust.reduce_with_food(event.amount);
                }
                bevy_shaman_items::systems::plant_food::BloodLustReductionSource::Music => {
                    blood_lust.reduce_with_music(event.amount);
                }
            }
        }
    }
}
