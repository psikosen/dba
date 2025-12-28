use crate::components::*;
use bevy::prelude::*;
use bevy_shaman_core::components::Spirit;
use bevy_shaman_core::resources::TimingQuality;

/// Event for executing a special move from a combo
#[derive(Event)]
pub struct SpecialMoveExecuted {
    pub special: SpecialMove,
    pub executor: Entity,
}

/// System to track rhythm inputs and build combos
pub fn rhythm_combo_tracking(
    mut combo_query: Query<&mut RhythmCombo>,
    attack_query: Query<&Attack>,
    weapon_query: Query<&EquippedWeapon>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs_f64();

    if let Ok(mut combo) = combo_query.get_single_mut() {
        for attack in attack_query.iter() {
            // Determine combo input type based on attack and timing
            let input = match attack.rhythm_quality {
                TimingQuality::Perfect => ComboInput::Perfect,
                _ => {
                    // Check weapon type for input classification
                    if let Ok(weapon) = weapon_query.get_single() {
                        if weapon.weapon_type.can_cast_spells() {
                            ComboInput::Magic
                        } else if attack.damage > 15.0 {
                            ComboInput::Heavy
                        } else {
                            ComboInput::Light
                        }
                    } else {
                        ComboInput::Light
                    }
                }
            };

            combo.add_input(input, current_time);
        }
    }
}

/// System to check for and execute special moves
pub fn rhythm_combo_specials(
    combo_query: Query<(Entity, &RhythmCombo)>,
    mut special_events: EventWriter<SpecialMoveExecuted>,
) {
    for (entity, combo) in combo_query.iter() {
        if let Some(special) = combo.check_special() {
            special_events.send(SpecialMoveExecuted {
                special,
                executor: entity,
            });
        }
    }
}

/// System to apply special move effects
pub fn apply_special_moves(
    mut special_events: EventReader<SpecialMoveExecuted>,
    mut attack_query: Query<&mut Attack>,
    mut spirit_query: Query<&mut Spirit>,
    mut combo_query: Query<&mut RhythmCombo>,
) {
    for event in special_events.read() {
        match event.special {
            SpecialMove::FlurryFinisher => {
                // Extra damage burst
                for mut attack in attack_query.iter_mut() {
                    attack.damage *= 1.5;
                }
                info!("Special Move: Flurry Finisher! (+50% damage)");
            }

            SpecialMove::PerfectCast => {
                // Zero spirit cost for next magic cast
                // This would need to be tracked for the next cast
                if let Ok(mut spirit) = spirit_query.get_single_mut() {
                    // Refund any spirit used in last cast (approximation)
                    spirit.heal(10.0);
                }
                info!("Special Move: Perfect Cast! (Spirit refunded)");
            }

            SpecialMove::SpiritStrike => {
                // Damage + heal spirit
                for mut attack in attack_query.iter_mut() {
                    attack.damage *= 1.3;
                }
                if let Ok(mut spirit) = spirit_query.get_single_mut() {
                    spirit.heal(15.0);
                }
                info!("Special Move: Spirit Strike! (Damage + Spirit heal)");
            }
        }

        // Reset combo after special move
        if let Ok(mut combo) = combo_query.get_mut(event.executor) {
            combo.reset();
        }
    }
}

/// System to reset combos when timing window expires
pub fn rhythm_combo_reset(mut combo_query: Query<&mut RhythmCombo>, time: Res<Time>) {
    let current_time = time.elapsed_secs_f64();

    for mut combo in combo_query.iter_mut() {
        if current_time - combo.last_input_time > combo.combo_window as f64 {
            if !combo.current_combo.is_empty() {
                combo.reset();
            }
        }
    }
}
