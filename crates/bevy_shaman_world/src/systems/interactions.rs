use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, Player};
use bevy_shaman_items::components::{Inventory, Item, ItemType, SpiritOrbSize, PlantType};
use rand::Rng;

#[derive(Component)]
pub struct ForageableSpot {
    pub seed_type: PlantType,
    pub foraged: bool,
}

#[derive(Component)]
pub struct DigSpot {
    pub dug: bool,
}

/// System to process foraging attempts
pub fn process_foraging(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&GridPosition, &mut Inventory), With<Player>>,
    mut forageable_query: Query<(&GridPosition, &mut ForageableSpot)>,
) {
    if keyboard.just_pressed(KeyCode::KeyF) {
        if let Ok((player_pos, mut inventory)) = player_query.get_single_mut() {
            // Check for forageable spots within range (adjacent tiles)
            for (spot_pos, mut forageable) in forageable_query.iter_mut() {
                let distance = ((player_pos.x - spot_pos.x).abs() + (player_pos.y - spot_pos.y).abs()) as u32;

                if distance <= 1 && !forageable.foraged {
                    // Forage the spot
                    let seed_item = Item {
                        id: format!("{:?}_seed", forageable.seed_type).to_lowercase(),
                        display_name: format!("{:?} Seed", forageable.seed_type),
                        item_type: ItemType::Plant(forageable.seed_type),
                        max_stack: 99,
                    };

                    let mut rng = rand::thread_rng();
                    let quantity = rng.gen_range(1..=3);

                    inventory.add_item(seed_item, quantity);
                    forageable.foraged = true;

                    info!("Foraged {} {:?} seeds!", quantity, forageable.seed_type);
                    break;
                }
            }
        }
    }
}

/// System to spawn random forageable spots
pub fn spawn_forageable_spots(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: Local<f32>,
    existing_spots: Query<&ForageableSpot>,
) {
    *spawn_timer += time.delta_secs();

    // Spawn a new forageable spot every 30 seconds (if less than 10 exist)
    if *spawn_timer >= 30.0 && existing_spots.iter().count() < 10 {
        *spawn_timer = 0.0;

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-20..20);
        let y = rng.gen_range(-20..20);

        let plant_types = [
            PlantType::MoonPetal,
            PlantType::StarRoot,
            PlantType::EternalBark,
            PlantType::CrystalMoss,
            PlantType::AetherGrass,
        ];

        let seed_type = plant_types[rng.gen_range(0..plant_types.len())];

        commands.spawn((
            ForageableSpot {
                seed_type,
                foraged: false,
            },
            GridPosition { x, y },
        ));

        info!("Spawned forageable spot at ({}, {}) with {:?} seeds", x, y, seed_type);
    }
}

/// System to process digging in tunnels/dungeons
pub fn process_digging(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&GridPosition, &mut Inventory), With<Player>>,
    mut dig_spots: Query<(&GridPosition, &mut DigSpot)>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::KeyG) {
        if let Ok((player_pos, mut inventory)) = player_query.get_single_mut() {
            // Check if player has a shovel
            let has_shovel = inventory.count_item("shovel") > 0;

            if !has_shovel {
                info!("You need a shovel to dig! Buy one from the shop.");
                return;
            }

            // Check for dig spots or create one at player position
            let mut found_spot = false;

            for (spot_pos, mut dig_spot) in dig_spots.iter_mut() {
                let distance = ((player_pos.x - spot_pos.x).abs() + (player_pos.y - spot_pos.y).abs()) as u32;

                if distance == 0 && !dig_spot.dug {
                    found_spot = true;
                    dig_spot.dug = true;

                    // Random loot from digging
                    let mut rng = rand::thread_rng();
                    let loot_roll = rng.gen_range(0..100);

                    if loot_roll < 30 {
                        // 30% chance: Spirit orb
                        let orb_size = if loot_roll < 10 {
                            SpiritOrbSize::Large
                        } else if loot_roll < 20 {
                            SpiritOrbSize::Medium
                        } else {
                            SpiritOrbSize::Small
                        };

                        let orb_item = Item {
                            id: format!("{:?}_spirit_orb", orb_size).to_lowercase(),
                            display_name: format!("{:?} Spirit Orb", orb_size),
                            item_type: ItemType::SpiritOrb(orb_size),
                            max_stack: 10,
                        };

                        inventory.add_item(orb_item, 1);
                        info!("Found a {:?} spirit orb!", orb_size);
                    } else if loot_roll < 60 {
                        // 30% chance: Crafting materials
                        let materials = ["wood", "stone", "iron_ore"];
                        let material = materials[rng.gen_range(0..materials.len())];
                        let quantity = rng.gen_range(1..=5);

                        let material_item = Item {
                            id: material.to_string(),
                            display_name: material.replace('_', " ").to_string(),
                            item_type: ItemType::CraftingMaterial,
                            max_stack: 99,
                        };

                        inventory.add_item(material_item, quantity);
                        info!("Found {} {}!", quantity, material);
                    } else if loot_roll < 80 {
                        // 20% chance: Herbs or remedies
                        let healing_items = [
                            ("herb", ItemType::Herb),
                            ("remedy", ItemType::Remedy),
                        ];
                        let (item_id, item_type) = healing_items[rng.gen_range(0..healing_items.len())];

                        let item = Item {
                            id: item_id.to_string(),
                            display_name: item_id.to_string(),
                            item_type,
                            max_stack: 10,
                        };

                        inventory.add_item(item, 1);
                        info!("Found a {}!", item_id);
                    } else {
                        // 20% chance: Seeds
                        let plant_types = [
                            PlantType::MoonPetal,
                            PlantType::StarRoot,
                            PlantType::CrystalMoss,
                        ];
                        let seed_type = plant_types[rng.gen_range(0..plant_types.len())];

                        let seed_item = Item {
                            id: format!("{:?}_seed", seed_type).to_lowercase(),
                            display_name: format!("{:?} Seed", seed_type),
                            item_type: ItemType::Plant(seed_type),
                            max_stack: 99,
                        };

                        let quantity = rng.gen_range(1..=3);
                        inventory.add_item(seed_item, quantity);
                        info!("Found {} {:?} seeds!", quantity, seed_type);
                    }

                    break;
                }
            }

            if !found_spot {
                // Create a new dig spot at player position
                commands.spawn((
                    DigSpot { dug: false },
                    GridPosition {
                        x: player_pos.x,
                        y: player_pos.y
                    },
                ));
                info!("Started digging at position ({}, {}). Press G again to dig!", player_pos.x, player_pos.y);
            }
        }
    }
}

/// System to spawn dig spots in dungeons
pub fn spawn_dungeon_dig_spots(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: Local<f32>,
    existing_spots: Query<&DigSpot>,
) {
    *spawn_timer += time.delta_secs();

    // Spawn dig spots every 20 seconds in dungeons (if less than 15 exist)
    if *spawn_timer >= 20.0 && existing_spots.iter().count() < 15 {
        *spawn_timer = 0.0;

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-30..30);
        let y = rng.gen_range(-30..30);

        commands.spawn((
            DigSpot { dug: false },
            GridPosition { x, y },
        ));
    }
}
