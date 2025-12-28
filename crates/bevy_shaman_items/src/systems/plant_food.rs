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

/// Event for feeding a blood plant (requires blood sacrifice)
#[derive(Event)]
pub struct FeedBloodPlant {
    pub feeder: Entity,
    pub plant: Entity,
    pub plant_type: PlantType,
}

/// Event for feeding a spirit plant (requires spirit energy)
#[derive(Event)]
pub struct FeedSpiritPlant {
    pub feeder: Entity,
    pub plant: Entity,
    pub plant_type: PlantType,
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
                // Blood plant harvested effects (from consuming the yield)
                PlantType::Mogodu => {
                    Some(ActiveEffect {
                        effect_type: EffectType::HealthCapacityBoost,
                        duration_remaining: 0.0,
                        strength: 20.0, // +20 max HP
                        is_permanent: true,
                    })
                }
                PlantType::Damu => {
                    Some(ActiveEffect {
                        effect_type: EffectType::HealthRegen,
                        duration_remaining: plant.effect_duration(),
                        strength: 5.0, // 5 HP per second for 5 min
                        is_permanent: false,
                    })
                }
                PlantType::Ingazi => {
                    Some(ActiveEffect {
                        effect_type: EffectType::StrengthBoost,
                        duration_remaining: plant.effect_duration(),
                        strength: plant.human_stat_boost(),
                        is_permanent: false,
                    })
                }
                PlantType::Mwazi => {
                    Some(ActiveEffect {
                        effect_type: EffectType::DamageResistance,
                        duration_remaining: plant.effect_duration(),
                        strength: 0.3, // 30% damage resistance
                        is_permanent: false,
                    })
                }
                PlantType::Jini => {
                    Some(ActiveEffect {
                        effect_type: EffectType::VitalityBoost,
                        duration_remaining: plant.effect_duration(),
                        strength: plant.spirit_stat_boost(),
                        is_permanent: false,
                    })
                }
                PlantType::Ropa => {
                    Some(ActiveEffect {
                        effect_type: EffectType::BloodFury,
                        duration_remaining: plant.effect_duration(),
                        strength: 1.5, // 50% attack boost
                        is_permanent: false,
                    })
                }
                PlantType::Samaki => {
                    Some(ActiveEffect {
                        effect_type: EffectType::StrengthBoost,
                        duration_remaining: plant.effect_duration(),
                        strength: plant.human_stat_boost(),
                        is_permanent: false,
                    })
                }
                PlantType::Umthombo => {
                    Some(ActiveEffect {
                        effect_type: EffectType::LegendaryHealing,
                        duration_remaining: plant.effect_duration(),
                        strength: 10.0, // 10 HP per second for 30 min
                        is_permanent: false,
                    })
                }
                // Spirit plant harvested effects
                PlantType::Roho => {
                    Some(ActiveEffect {
                        effect_type: EffectType::SpiritCapacityBoost,
                        duration_remaining: 0.0,
                        strength: 30.0, // +30 max spirit
                        is_permanent: true,
                    })
                }
                PlantType::Moya => {
                    Some(ActiveEffect {
                        effect_type: EffectType::SpiritSight,
                        duration_remaining: plant.effect_duration(),
                        strength: 1.0,
                        is_permanent: false,
                    })
                }
                PlantType::Emi => {
                    Some(ActiveEffect {
                        effect_type: EffectType::WisdomBoost,
                        duration_remaining: plant.effect_duration(),
                        strength: plant.human_stat_boost(),
                        is_permanent: false,
                    })
                }
                PlantType::Moyo => {
                    Some(ActiveEffect {
                        effect_type: EffectType::SpiritShield,
                        duration_remaining: plant.effect_duration(),
                        strength: 0.4, // 40% spirit damage resistance
                        is_permanent: false,
                    })
                }
                PlantType::Elima => {
                    Some(ActiveEffect {
                        effect_type: EffectType::SpiritualPower,
                        duration_remaining: plant.effect_duration(),
                        strength: plant.spirit_stat_boost(),
                        is_permanent: false,
                    })
                }
                PlantType::Pepo => {
                    Some(ActiveEffect {
                        effect_type: EffectType::EtherealMovement,
                        duration_remaining: plant.effect_duration(),
                        strength: 1.0,
                        is_permanent: false,
                    })
                }
                PlantType::Sankofa => {
                    Some(ActiveEffect {
                        effect_type: EffectType::WisdomBoost,
                        duration_remaining: plant.effect_duration(),
                        strength: plant.human_stat_boost(),
                        is_permanent: false,
                    })
                }
                PlantType::Nommo => {
                    Some(ActiveEffect {
                        effect_type: EffectType::SpiritAscension,
                        duration_remaining: plant.effect_duration(),
                        strength: 2.0, // 100% spirit power increase
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
                EffectType::HealthRegen | EffectType::LegendaryHealing => {
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
                    // Permanent boost applied when effect is first added
                    if let Some(ref mut s) = spirit {
                        // Only apply once when effect is permanent
                        if effect.is_permanent {
                            s.max += effect.strength;
                            s.current = s.max; // Fill to new max
                        }
                    }
                }
                EffectType::HealthCapacityBoost => {
                    // Permanent boost applied when effect is first added
                    if let Some(ref mut h) = health {
                        if effect.is_permanent {
                            h.max += effect.strength;
                            h.current = h.max; // Fill to new max
                        }
                    }
                }
                _ => {
                    // Other effects are passive and checked by other systems
                    // (StrengthBoost, DamageResistance, etc.)
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

/// System to handle feeding blood plants
pub fn feed_blood_plant(
    mut events: EventReader<FeedBloodPlant>,
    mut feeder_query: Query<(&mut Health, &mut BloodSacrificePenalty)>,
    mut plant_query: Query<&mut PlantCare>,
) {
    for event in events.read() {
        // Get the feeder's health and penalty tracker
        if let Ok((mut health, mut penalty)) = feeder_query.get_mut(event.feeder) {
            let blood_cost = event.plant_type.blood_cost();

            // Check if player has enough HP
            if health.current <= blood_cost {
                warn!("Not enough HP to feed blood plant! Need {}, have {}",
                    blood_cost, health.current);
                continue;
            }

            // Take blood from player (reduce current HP)
            health.damage(blood_cost);

            // Add penalty (reduces max HP until rest)
            penalty.add_penalty(blood_cost);

            info!("Sacrificed {} HP to feed {:?} plant", blood_cost, event.plant_type);

            // Feed the plant
            if let Ok(mut plant_care) = plant_query.get_mut(event.plant) {
                plant_care.feed();
                info!("Blood plant {:?} has been fed!", event.plant_type);
            }
        }
    }
}

/// System to handle feeding spirit plants
pub fn feed_spirit_plant(
    mut events: EventReader<FeedSpiritPlant>,
    mut feeder_query: Query<&mut Spirit>,
    mut plant_query: Query<&mut PlantCare>,
) {
    for event in events.read() {
        // Get the feeder's spirit
        if let Ok(mut spirit) = feeder_query.get_mut(event.feeder) {
            let spirit_cost = event.plant_type.spirit_cost();

            // Check if player has enough spirit
            if spirit.current < spirit_cost {
                warn!("Not enough spirit to feed plant! Need {}, have {}",
                    spirit_cost, spirit.current);
                continue;
            }

            // Take spirit from player
            spirit.current -= spirit_cost;

            info!("Sacrificed {} spirit to feed {:?} plant", spirit_cost, event.plant_type);

            // Feed the plant
            if let Ok(mut plant_care) = plant_query.get_mut(event.plant) {
                plant_care.feed();
                info!("Spirit plant {:?} has been fed!", event.plant_type);
            }
        }
    }
}

/// System to apply blood sacrifice penalty to max HP
pub fn apply_blood_sacrifice_penalty(
    mut query: Query<(&mut Health, &BloodSacrificePenalty), Changed<BloodSacrificePenalty>>,
) {
    for (mut health, penalty) in query.iter_mut() {
        // Temporarily reduce max HP (will be restored on rest)
        let original_max = health.max + penalty.hp_reduction;
        health.max = (original_max - penalty.hp_reduction).max(1.0);

        // Make sure current HP doesn't exceed new max
        if health.current > health.max {
            health.current = health.max;
        }
    }
}

/// System to advance plant care days (should be called once per in-game day)
pub fn advance_plant_care_days(
    mut plant_query: Query<&mut PlantCare>,
) {
    for mut plant_care in plant_query.iter_mut() {
        plant_care.advance_day();

        if plant_care.is_withering {
            warn!("Plant {:?} is withering! It needs feeding!", plant_care.plant_type);
        }
    }
}
