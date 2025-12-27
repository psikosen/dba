use bevy::prelude::*;
use crate::components::*;
use bevy_shaman_core::components::{Health, Spirit};

/// Event for using a plant
#[derive(Event)]
pub struct UsePlant {
    pub user: Entity,
    pub plant_type: PlantType,
}

/// Event for using food
#[derive(Event)]
pub struct UseFood {
    pub user: Entity,
    pub food_type: FoodType,
}

/// Event for reducing blood lust (sent when plants/food are consumed)
#[derive(Event)]
pub struct ReduceBloodLust {
    pub entity: Entity,
    pub amount: f32,
    pub source: BloodLustReductionSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BloodLustReductionSource {
    Plant,
    Food,
    Music,
}

/// System to handle plant usage
pub fn plant_usage(
    mut events: EventReader<UsePlant>,
    mut inventory_query: Query<&mut Inventory>,
    mut effects_query: Query<&mut ActiveEffects>,
    mut blood_lust_events: EventWriter<ReduceBloodLust>,
) {
    for event in events.read() {
        // Remove plant from inventory
        if let Ok(mut inventory) = inventory_query.get_mut(event.user) {
            let plant_id = format!("{:?}", event.plant_type);
            if !inventory.remove_item(&plant_id, 1) {
                warn!("Plant not found in inventory!");
                continue;
            }
        }

        let plant = event.plant_type;

        // Send blood lust reduction event
        let blood_lust_reduction = plant.blood_lust_reduction();
        if blood_lust_reduction > 0.0 {
            blood_lust_events.send(ReduceBloodLust {
                entity: event.user,
                amount: blood_lust_reduction,
                source: BloodLustReductionSource::Plant,
            });
            info!("Used {:?} - Blood lust reduced by {}", plant, blood_lust_reduction);
        }

        // Add active effect if applicable
        if let Ok(mut effects) = effects_query.get_mut(event.user) {
            let effect = match plant {
                PlantType::SpiritBlossom | PlantType::MoonPetal | PlantType::StarRoot => {
                    Some(ActiveEffect {
                        effect_type: EffectType::BloodLustReduction,
                        duration_remaining: plant.effect_duration(),
                        strength: blood_lust_reduction,
                        is_permanent: plant.is_permanent(),
                    })
                }
                PlantType::LifeLeaf => {
                    Some(ActiveEffect {
                        effect_type: EffectType::HealthRegen,
                        duration_remaining: plant.effect_duration(),
                        strength: 2.0, // HP per second
                        is_permanent: false,
                    })
                }
                PlantType::EternalBark => {
                    Some(ActiveEffect {
                        effect_type: EffectType::HealthRegen,
                        duration_remaining: 0.0,
                        strength: 1.0,
                        is_permanent: true,
                    })
                }
                PlantType::AetherGrass => {
                    Some(ActiveEffect {
                        effect_type: EffectType::SpiritRegen,
                        duration_remaining: plant.effect_duration(),
                        strength: 2.0, // Spirit per second
                        is_permanent: false,
                    })
                }
                PlantType::CrystalMoss => {
                    Some(ActiveEffect {
                        effect_type: EffectType::SpiritCapacityBoost,
                        duration_remaining: 0.0,
                        strength: 25.0, // +25 max spirit
                        is_permanent: true,
                    })
                }
                _ => None,
            };

            if let Some(effect) = effect {
                effects.add_effect(effect);
            }
        }
    }
}

/// System to handle food usage
pub fn food_usage(
    mut events: EventReader<UseFood>,
    mut inventory_query: Query<&mut Inventory>,
    mut effects_query: Query<&mut ActiveEffects>,
    mut blood_lust_events: EventWriter<ReduceBloodLust>,
) {
    for event in events.read() {
        // Remove food from inventory
        if let Ok(mut inventory) = inventory_query.get_mut(event.user) {
            let food_id = format!("{:?}", event.food_type);
            if !inventory.remove_item(&food_id, 1) {
                warn!("Food not found in inventory!");
                continue;
            }
        }

        let food = event.food_type;

        // Send blood lust reduction event
        let blood_lust_reduction = food.blood_lust_reduction();
        if blood_lust_reduction > 0.0 {
            blood_lust_events.send(ReduceBloodLust {
                entity: event.user,
                amount: blood_lust_reduction,
                source: BloodLustReductionSource::Food,
            });
            info!("Ate {:?} - Blood lust reduced by {}", food, blood_lust_reduction);
        }

        // Add stat boost effects
        if let Ok(mut effects) = effects_query.get_mut(event.user) {
            let effect = match food {
                FoodType::StrengthMeat => {
                    Some(ActiveEffect {
                        effect_type: EffectType::AttackBoost,
                        duration_remaining: food.effect_duration(),
                        strength: 1.3, // 30% attack boost
                        is_permanent: false,
                    })
                }
                FoodType::SwiftFish => {
                    Some(ActiveEffect {
                        effect_type: EffectType::SpeedBoost,
                        duration_remaining: food.effect_duration(),
                        strength: 1.5, // 50% speed boost
                        is_permanent: false,
                    })
                }
                FoodType::WisdomStew => {
                    Some(ActiveEffect {
                        effect_type: EffectType::SpiritRegen,
                        duration_remaining: food.effect_duration(),
                        strength: 3.0, // +3 spirit per second
                        is_permanent: false,
                    })
                }
                _ => None,
            };

            if let Some(effect) = effect {
                effects.add_effect(effect);
            }
        }
    }
}

/// System to update active effects over time
pub fn active_effects_update(
    mut effects_query: Query<&mut ActiveEffects>,
    time: Res<Time>,
) {
    for mut effects in effects_query.iter_mut() {
        effects.update(time.delta_secs());
    }
}

/// System to apply active effects
pub fn apply_active_effects(
    mut query: Query<(&ActiveEffects, Option<&mut Health>, Option<&mut Spirit>)>,
    time: Res<Time>,
) {
    for (effects, mut health, mut spirit) in query.iter_mut() {
        for effect in &effects.effects {
            match effect.effect_type {
                EffectType::HealthRegen => {
                    if let Some(ref mut h) = health {
                        h.heal(effect.strength * time.delta_secs());
                    }
                }
                EffectType::SpiritRegen => {
                    if let Some(ref mut s) = spirit {
                        s.heal(effect.strength * time.delta_secs());
                    }
                }
                EffectType::SpiritCapacityBoost => {
                    // This is a one-time boost, applied when the effect is first added
                    // The system checks for permanent effects
                }
                _ => {
                    // Other effects are passive and checked by other systems
                }
            }
        }
    }
}

/// System to clear permanent effects on rest
pub fn clear_permanent_effects_on_rest(
    _effects_query: Query<&mut ActiveEffects>,
    // TODO: Add rest event
) {
    // This would be triggered by a rest event
    // for mut effects in effects_query.iter_mut() {
    //     effects.remove_permanent_effects();
    // }
}
