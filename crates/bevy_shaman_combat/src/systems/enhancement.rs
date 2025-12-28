use bevy::prelude::*;
use crate::components::*;
use bevy_shaman_core::components::Spirit;
use bevy_shaman_items::components::{Inventory, PlantType, SpiritOrbSize};

/// System to handle spirit merging into weapons
pub fn spirit_merging_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut weapon_query: Query<&mut EquippedWeapon>,
    mut spirit_query: Query<&mut Spirit>,
    mut inventory_query: Query<&mut Inventory>,
    mut enhancement_query: Query<&mut WeaponEnhancement>,
) {
    // Press 'M' to open merging interface (will be UI-driven later)
    if !keyboard.just_pressed(KeyCode::KeyM) {
        return;
    }

    for mut weapon in weapon_query.iter_mut() {
        for mut enhancement in enhancement_query.iter_mut() {
            for mut inventory in inventory_query.iter_mut() {
                // Check if we can enhance (not at max level)
                if enhancement.level >= WeaponEnhancement::MAX_LEVEL {
                    continue;
                }

                let cost = enhancement.next_level_cost();

                // Try to consume spirit orbs first
                if inventory.count_item("spirit_orb_large") > 0
                    && cost.spirit_orbs <= SpiritOrbSize::Large.spirit_restore() {
                    if inventory.remove_item("spirit_orb_large", 1) {
                        enhancement.apply_upgrade(&mut weapon);
                        info!("Weapon enhanced to level {}!", enhancement.level);
                    }
                }
            }
        }
    }
}

/// System to apply plant-based weapon enhancements
pub fn plant_enhancement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut weapon_query: Query<&mut EquippedWeapon>,
    mut inventory_query: Query<&mut Inventory>,
    mut enhancement_query: Query<&mut WeaponEnhancement>,
    mut spirit_query: Query<&mut Spirit>,
) {
    // Press 'N' to apply plant enhancement (temporary enchantment)
    if !keyboard.just_pressed(KeyCode::KeyN) {
        return;
    }

    for mut weapon in weapon_query.iter_mut() {
        for mut enhancement in enhancement_query.iter_mut() {
            for mut inventory in inventory_query.iter_mut() {
                // Try to apply blood plant enhancement (damage boost)
                if let Some(plant_type) = find_usable_blood_plant(&inventory) {
                    if inventory.remove_item(&plant_item_id(plant_type), 1) {
                        let duration = plant_type.effect_duration();
                        enhancement.apply_temporary_enchantment(
                            EnchantmentType::BloodFury,
                            duration,
                            plant_type.human_stat_boost()
                        );
                        info!("Applied {} blood enhancement for {}s!",
                              plant_name(plant_type), duration);
                    }
                }

                // Try to apply spirit plant enhancement (spirit efficiency)
                if let Some(plant_type) = find_usable_spirit_plant(&inventory) {
                    if inventory.remove_item(&plant_item_id(plant_type), 1) {
                        if let Ok(mut spirit) = spirit_query.get_single_mut() {
                            let cost = plant_type.spirit_cost();
                            if spirit.current >= cost {
                                spirit.current -= cost;
                                let duration = plant_type.effect_duration();
                                enhancement.apply_temporary_enchantment(
                                    EnchantmentType::SpiritInfusion,
                                    duration,
                                    plant_type.spirit_stat_boost()
                                );
                                info!("Applied {} spirit enhancement for {}s!",
                                      plant_name(plant_type), duration);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// System to update temporary enchantments (decay over time)
pub fn enchantment_decay_system(
    time: Res<Time>,
    mut enhancement_query: Query<&mut WeaponEnhancement>,
) {
    let delta = time.delta_secs();

    for mut enhancement in enhancement_query.iter_mut() {
        enhancement.update_enchantments(delta);
    }
}

/// System to apply enhancement bonuses to attacks
pub fn apply_enhancement_bonuses(
    mut attack_query: Query<&mut Attack>,
    weapon_query: Query<&EquippedWeapon>,
    enhancement_query: Query<&WeaponEnhancement>,
) {
    for mut attack in attack_query.iter_mut() {
        if let Ok(weapon) = weapon_query.get_single() {
            if let Ok(enhancement) = enhancement_query.get_single() {
                // Apply permanent enhancement bonus
                let damage_bonus = enhancement.damage_bonus();
                attack.damage *= 1.0 + (damage_bonus / 100.0);

                // Apply active enchantments
                for enchantment in &enhancement.active_enchantments {
                    match enchantment.enchantment_type {
                        EnchantmentType::BloodFury => {
                            attack.damage *= 1.0 + (enchantment.strength / 100.0);
                        }
                        EnchantmentType::SpiritInfusion => {
                            // Reduces spirit cost (handled in weapon_attack_system)
                        }
                        EnchantmentType::VenomCoating => {
                            // Apply poison (handled separately)
                        }
                        EnchantmentType::AncestralBlessing => {
                            attack.damage *= 1.15; // 15% bonus from ancestors
                        }
                    }
                }
            }
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn find_usable_blood_plant(inventory: &Inventory) -> Option<PlantType> {
    let blood_plants = [
        PlantType::Ropa,      // Blood fury
        PlantType::Ingazi,    // Strength
        PlantType::Samaki,    // Dual boost
        PlantType::Umthombo,  // Legendary
    ];

    for plant in blood_plants {
        if inventory.count_item(&plant_item_id(plant)) > 0 {
            return Some(plant);
        }
    }
    None
}

fn find_usable_spirit_plant(inventory: &Inventory) -> Option<PlantType> {
    let spirit_plants = [
        PlantType::Elima,     // Spiritual power
        PlantType::Sankofa,   // Wisdom
        PlantType::Nommo,     // Legendary
        PlantType::Roho,      // Max spirit
    ];

    for plant in spirit_plants {
        if inventory.count_item(&plant_item_id(plant)) > 0 {
            return Some(plant);
        }
    }
    None
}

fn plant_item_id(plant_type: PlantType) -> String {
    format!("plant_{:?}", plant_type).to_lowercase()
}

fn plant_name(plant_type: PlantType) -> &'static str {
    match plant_type {
        PlantType::Ropa => "Ropa (Blood Fury)",
        PlantType::Ingazi => "Ingazi (Strength)",
        PlantType::Samaki => "Samaki (Dual Power)",
        PlantType::Umthombo => "Umthombo (Life Source)",
        PlantType::Elima => "Elima (Soul Power)",
        PlantType::Sankofa => "Sankofa (Wisdom)",
        PlantType::Nommo => "Nommo (Life Force)",
        PlantType::Roho => "Roho (Spirit)",
        _ => "Unknown Plant",
    }
}
