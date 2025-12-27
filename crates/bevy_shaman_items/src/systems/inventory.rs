use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, Player, Health, Spirit, Stamina, BloodLust};
use crate::components::{
    Inventory, Pickupable, Item, ItemType, PlantType, FoodType,
    ActiveEffects, ActiveEffect, EffectType
};

/// Event fired when player picks up an item
#[derive(Event)]
pub struct ItemPickedUp {
    pub player: Entity,
    pub item: Item,
    pub quantity: u32,
}

/// Event fired when player uses an item
#[derive(Event)]
pub struct ItemUsed {
    pub player: Entity,
    pub item_id: String,
}

/// Event fired when player drops an item
#[derive(Event)]
pub struct ItemDropped {
    pub player: Entity,
    pub item_id: String,
    pub quantity: u32,
}

/// Handle picking up items when player is near them
pub fn pickup_items(
    mut commands: Commands,
    mut player: Query<(Entity, &GridPosition, &mut Inventory), With<Player>>,
    pickupables: Query<(Entity, &GridPosition, &Pickupable)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut pickup_events: EventWriter<ItemPickedUp>,
) {
    let Ok((player_entity, player_pos, mut inventory)) = player.get_single_mut() else {
        return;
    };

    let pickup_range = 1.0; // Grid units
    let pickup_pressed = keyboard.just_pressed(KeyCode::KeyE);

    for (item_entity, item_pos, pickupable) in pickupables.iter() {
        let distance = ((player_pos.x - item_pos.x).abs() + (player_pos.y - item_pos.y).abs()) as f32;

        if distance <= pickup_range && (pickup_pressed || pickupable.auto_pickup) {
            if inventory.add_item(pickupable.item.clone(), pickupable.quantity) {
                pickup_events.send(ItemPickedUp {
                    player: player_entity,
                    item: pickupable.item.clone(),
                    quantity: pickupable.quantity,
                });
                commands.entity(item_entity).despawn();
            }
        }
    }
}

/// Use items from inventory (consume consumables)
pub fn use_items(
    mut use_events: EventReader<ItemUsed>,
    mut player: Query<(&mut Inventory, &mut Health, Option<&mut Spirit>, Option<&mut Stamina>, Option<&mut BloodLust>, Option<&mut ActiveEffects>), With<Player>>,
    time: Res<Time>,
) {
    for event in use_events.read() {
        let Ok((mut inventory, mut health, mut spirit_opt, mut stamina_opt, mut blood_lust_opt, effects_opt)) = player.get_single_mut() else {
            continue;
        };

        // Find the item in inventory
        let item_stack = inventory.items.iter().find(|s| s.item.id == event.item_id).cloned();

        let Some(stack) = item_stack else {
            continue;
        };

        let item = &stack.item;

        // Apply item effects based on type
        match item.item_type {
            ItemType::SpiritOrb(size) => {
                if let Some(spirit) = spirit_opt.as_mut() {
                    spirit.current = (spirit.current + size.spirit_restore()).min(spirit.max);
                }
                if let Some(stamina) = stamina_opt.as_mut() {
                    stamina.current = (stamina.current + size.stamina_restore()).min(stamina.max);
                }
                inventory.remove_item(&item.id, 1);
            }

            ItemType::Plant(plant_type) => {
                // Apply plant effects
                if plant_type.blood_lust_reduction() > 0.0 {
                    if let Some(blood_lust) = blood_lust_opt.as_mut() {
                        blood_lust.reduce_with_plant(plant_type.blood_lust_reduction());
                    }
                }

                // Add active effect if it has duration
                if let Some(mut effects) = effects_opt {
                    let effect_type = match plant_type {
                        PlantType::LifeLeaf => Some(EffectType::HealthRegen),
                        PlantType::AetherGrass => Some(EffectType::SpiritRegen),
                        PlantType::CrystalMoss => Some(EffectType::SpiritCapacityBoost),
                        _ => None,
                    };

                    if let Some(eff_type) = effect_type {
                        effects.add_effect(ActiveEffect {
                            effect_type: eff_type,
                            duration_remaining: plant_type.effect_duration(),
                            strength: 1.0,
                            is_permanent: plant_type.is_permanent(),
                        });
                    }
                }

                inventory.remove_item(&item.id, 1);
            }

            ItemType::Food(food_type) => {
                // Apply food effects
                if food_type.blood_lust_reduction() > 0.0 {
                    if let Some(blood_lust) = blood_lust_opt.as_mut() {
                        blood_lust.reduce_with_food(food_type.blood_lust_reduction());
                    }
                }

                // Add active effect for stat boost foods
                if let Some(mut effects) = effects_opt {
                    let (effect_type, strength) = match food_type {
                        FoodType::StrengthMeat => Some((EffectType::AttackBoost, 1.2)),
                        FoodType::SwiftFish => Some((EffectType::SpeedBoost, 1.3)),
                        FoodType::WisdomStew => Some((EffectType::SpiritRegen, 1.5)),
                        _ => None,
                    }.unwrap_or((EffectType::HealthRegen, 1.0));

                    if food_type.effect_duration() > 0.0 {
                        effects.add_effect(ActiveEffect {
                            effect_type,
                            duration_remaining: food_type.effect_duration(),
                            strength,
                            is_permanent: false,
                        });
                    }
                }

                inventory.remove_item(&item.id, 1);
            }

            ItemType::Remedy => {
                // Heal health
                health.heal(30.0);
                inventory.remove_item(&item.id, 1);
            }

            ItemType::Herb => {
                // Minor health heal
                health.heal(15.0);
                inventory.remove_item(&item.id, 1);
            }

            _ => {
                // KeyItem, CraftingMaterial - not consumable
            }
        }
    }
}

/// Update active effects from items
pub fn update_active_effects(
    time: Res<Time>,
    mut entities: Query<(&mut ActiveEffects, &mut Health, Option<&mut Spirit>)>,
) {
    let delta = time.delta_secs();

    for (mut effects, mut health, mut spirit_opt) in entities.iter_mut() {
        // Apply effect bonuses
        for effect in &effects.effects {
            match effect.effect_type {
                EffectType::HealthRegen => {
                    health.heal(effect.strength * 2.0 * delta);
                }
                EffectType::SpiritRegen => {
                    if let Some(spirit) = spirit_opt.as_mut() {
                        spirit.current = (spirit.current + effect.strength * 3.0 * delta).min(spirit.max);
                    }
                }
                EffectType::SpiritCapacityBoost => {
                    if let Some(_spirit) = spirit_opt.as_mut() {
                        // Permanent boost - apply once when added
                        // This would be handled when the effect is first applied
                    }
                }
                _ => {}
            }
        }

        // Update effect timers
        effects.update(delta);
    }
}

/// Drop items from inventory into the world
pub fn drop_items(
    mut commands: Commands,
    mut drop_events: EventReader<ItemDropped>,
    mut player: Query<(&GridPosition, &mut Inventory), With<Player>>,
) {
    for event in drop_events.read() {
        let Ok((player_pos, mut inventory)) = player.get_single_mut() else {
            continue;
        };

        // Find the item in inventory
        let item_stack = inventory.items.iter().find(|s| s.item.id == event.item_id).cloned();

        let Some(stack) = item_stack else {
            continue;
        };

        let drop_quantity = event.quantity.min(stack.quantity);

        if inventory.remove_item(&event.item_id, drop_quantity) {
            // Spawn item in world near player
            commands.spawn((
                Pickupable {
                    item: stack.item.clone(),
                    quantity: drop_quantity,
                    auto_pickup: false,
                },
                GridPosition {
                    x: player_pos.x + 1,
                    y: player_pos.y,
                },
            ));
        }
    }
}
