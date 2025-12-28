use crate::components::*;
use bevy::prelude::*;
use bevy_shaman_monsters::components::MonsterState;
use rand::Rng;

/// Event for when the combat wheel triggers
#[derive(Event)]
pub struct WheelTriggered {
    pub outcome: WheelOutcome,
    pub attacker: Entity,
}

/// System to randomly trigger the combat wheel during attacks
pub fn combat_wheel_trigger(
    mut wheel_query: Query<&mut CombatWheel>,
    mut attack_query: Query<(Entity, &mut Attack)>,
    mut wheel_events: EventWriter<WheelTriggered>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs_f64();
    let mut rng = rand::thread_rng();

    if let Ok(mut wheel) = wheel_query.get_single_mut() {
        // Check cooldown
        if current_time - wheel.last_trigger < wheel.cooldown as f64 {
            return;
        }

        for (entity, _attack) in attack_query.iter_mut() {
            // Roll for wheel trigger
            let roll: f32 = rng.gen();
            if roll < wheel.trigger_chance {
                let outcome = WheelOutcome::random();
                wheel.last_trigger = current_time;

                wheel_events.send(WheelTriggered {
                    outcome,
                    attacker: entity,
                });

                // Only trigger once per check
                break;
            }
        }
    }
}

/// System to apply wheel outcomes
/// OPTIMIZED: Now filters to only affect the attacker's attack instead of all attacks
pub fn apply_wheel_outcome(
    mut wheel_events: EventReader<WheelTriggered>,
    mut attack_query: Query<(Entity, &mut Attack)>,
    mut blood_lust_query: Query<&mut BloodLust>,
    mut monster_query: Query<&mut MonsterState, With<bevy_shaman_monsters::components::Tamed>>,
    weapon_query: Query<&EquippedWeapon>,
) {
    for event in wheel_events.read() {
        match event.outcome {
            WheelOutcome::CriticalHit => {
                // OPTIMIZATION: Only modify attacks from the triggering entity
                for (entity, mut attack) in attack_query.iter_mut() {
                    if entity == event.attacker {
                        attack.damage *= 2.0;
                        info!("Wheel triggered: Critical Hit! Damage doubled!");
                        break; // Only one attack per attacker
                    }
                }
            }

            WheelOutcome::DoubleSpellDamage => {
                // OPTIMIZATION: Only check weapon of triggering attacker
                if let Ok(weapon) = weapon_query.get(event.attacker) {
                    if weapon.weapon_type.can_cast_spells() {
                        for (entity, mut attack) in attack_query.iter_mut() {
                            if entity == event.attacker {
                                attack.damage *= 2.0;
                                info!("Wheel triggered: Double Spell Damage!");
                                break;
                            }
                        }
                    }
                }
            }

            WheelOutcome::SelfCorruption => {
                // Add corruption to player via blood lust
                if let Ok(mut blood_lust) = blood_lust_query.get_single_mut() {
                    blood_lust.current = (blood_lust.current + 20.0).min(100.0);
                }
                warn!("Wheel triggered: Self Corruption! Blood lust increased!");
            }

            WheelOutcome::SpiritCorruption => {
                // Corrupt all tamed monsters/spirits
                for mut monster_state in monster_query.iter_mut() {
                    monster_state.corruption_meter =
                        (monster_state.corruption_meter + 0.3).min(1.0);
                    monster_state.stability_meter = (monster_state.stability_meter - 0.2).max(0.0);
                }
                warn!("Wheel triggered: Spirit Corruption! Your companions are tainted!");
            }
        }
    }
}
