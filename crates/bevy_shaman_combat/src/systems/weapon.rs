use bevy::prelude::*;
use crate::components::*;
use bevy_shaman_core::components::Spirit;

/// System to handle weapon attacks with their specific properties
pub fn weapon_attack_system(
    mut attack_query: Query<(&mut Attack, &EquippedWeapon)>,
    mut spirit_query: Query<&mut Spirit>,
) {
    for (mut attack, weapon) in attack_query.iter_mut() {
        // Apply weapon-specific damage
        let base_damage = weapon.weapon_type.base_damage();
        attack.damage *= base_damage * weapon.damage_multiplier;

        // Consume spirit for magical weapons
        let spirit_cost = weapon.weapon_type.spirit_cost();
        if spirit_cost > 0.0 {
            if let Ok(mut spirit) = spirit_query.get_single_mut() {
                if spirit.current >= spirit_cost {
                    spirit.current -= spirit_cost;
                } else {
                    // Not enough spirit, reduce damage
                    attack.damage *= 0.5;
                }
            }
        }
    }
}

/// System to apply poison from Mambele weapons
pub fn mambele_poison_system(
    attack_query: Query<(&Attack, &EquippedWeapon)>,
    mut status_query: Query<&mut StatusEffects>,
    plant_inventory: Query<&bevy_shaman_items::components::Inventory>,
) {
    for (attack, weapon) in attack_query.iter() {
        if weapon.weapon_type.can_apply_poison() {
            // Check if we have poison plants
            if let Ok(inventory) = plant_inventory.get_single() {
                let has_poison = inventory.items.iter().any(|stack| {
                    matches!(
                        stack.item.item_type,
                        bevy_shaman_items::components::ItemType::Plant(plant_type)
                        if matches!(
                            plant_type,
                            bevy_shaman_items::components::PlantType::VenomVine
                            | bevy_shaman_items::components::PlantType::ShadowMushroom
                            | bevy_shaman_items::components::PlantType::DeathBloom
                        )
                    )
                });

                if has_poison {
                    // Apply spiritual poison to target
                    if let Ok(mut effects) = status_query.get_mut(attack.target) {
                        effects.effects.push(StatusEffect {
                            effect_type: StatusEffectType::SpiritualPoison,
                            duration: 10.0,
                            strength: 5.0,
                        });
                    }
                }
            }
        }
    }
}

/// System to handle weapon durability
pub fn weapon_durability_system(
    mut weapon_query: Query<&mut EquippedWeapon>,
    attack_events: EventReader<crate::systems::events::HitLanded>,
) {
    if attack_events.is_empty() {
        return;
    }

    for mut weapon in weapon_query.iter_mut() {
        weapon.durability = (weapon.durability - 0.5).max(0.0);

        // Reduce effectiveness if durability is low
        weapon.damage_multiplier = (weapon.durability / weapon.max_durability).max(0.3);
    }
}
