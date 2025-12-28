use bevy::prelude::*;

mod error;

pub use error::{LoadError, SaveError};

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_error;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<systems::autosave::AutosaveTimer>()
            .init_resource::<systems::save_load::PendingLoadData>()
            .add_systems(
                Update,
                (
                    systems::autosave::autosave_system,
                    systems::save_load::handle_save_requests,
                    systems::save_load::apply_loaded_data,
                ),
            )
            .add_event::<systems::events::SaveRequested>()
            .add_event::<systems::events::LoadRequested>();
    }
}

// Make systems module public for integration tests
pub mod systems {
    pub mod events {
        use bevy::prelude::*;

        #[derive(Event)]
        pub struct SaveRequested;

        #[derive(Event)]
        pub struct LoadRequested {
            pub save_slot: u8,
        }
    }

    pub mod autosave {
        use super::events::SaveRequested;
        use bevy::prelude::*;

        #[derive(Resource)]
        pub struct AutosaveTimer {
            pub timer: Timer,
        }

        impl Default for AutosaveTimer {
            fn default() -> Self {
                Self {
                    timer: Timer::from_seconds(300.0, TimerMode::Repeating), // 5 minutes
                }
            }
        }

        pub fn autosave_system(
            time: Res<Time>,
            mut timer: ResMut<AutosaveTimer>,
            mut save_events: EventWriter<SaveRequested>,
        ) {
            timer.timer.tick(time.delta());

            if timer.timer.just_finished() {
                info!("Autosave triggered");
                save_events.send(SaveRequested);
            }
        }
    }

    pub mod save_load {
        use bevy::prelude::*;
        use bevy_shaman_core::components::{GridPosition, Health, Player, Spirit, Stamina};
        use bevy_shaman_world::components::TileCorruption;
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        use std::fs;

        #[derive(Serialize, Deserialize, Clone)]
        pub struct TutorialProgressData {
            pub tutorial_started: bool,
            pub tutorial_completed: bool,
            pub current_mission: Option<String>,
            pub current_step: u8,
            pub completed_missions: Vec<String>,
            pub mission_flags: HashMap<String, bool>,
            pub cutscene_viewed: HashMap<String, bool>,
        }

        impl Default for TutorialProgressData {
            fn default() -> Self {
                Self {
                    tutorial_started: false,
                    tutorial_completed: false,
                    current_mission: None,
                    current_step: 0,
                    completed_missions: Vec::new(),
                    mission_flags: HashMap::new(),
                    cutscene_viewed: HashMap::new(),
                }
            }
        }

        #[derive(Serialize, Deserialize, Clone)]
        pub struct MonsterSaveData {
            pub monster_id: String,
            pub position: (i32, i32),
            pub health: (f32, f32),
            pub state: String, // Serialized StateType
            pub stability_meter: f32,
            pub corruption_meter: f32,
            pub obedience_meter: f32,
            pub is_tamed: bool,
        }

        #[derive(Serialize, Deserialize, Clone)]
        pub struct NpcSaveData {
            pub name: String,
            pub position: (i32, i32),
            pub sickness_state: String, // "AsleepSick", "Waking", or "Awake"
            pub full_dialogue: String,
            pub partial_dialogue: Option<String>,
            pub sick_dialogue: String,
            #[serde(default)]
            pub is_head_shaman: bool,
            #[serde(default)]
            pub is_player_brother: bool,
            #[serde(default)]
            pub soul_corruption: f32,
            #[serde(default)]
            pub fights_remaining: u8,
        }

        #[derive(Serialize, Deserialize, Clone)]
        pub struct DungeonEntranceSaveData {
            pub position: (i32, i32),
            pub dungeon_id: String,
            pub ecosystem: String,
            pub difficulty_level: u8,
            pub is_discovered: bool,
        }

        #[derive(Serialize, Deserialize, Clone)]
        pub struct SaveData {
            pub player_position: (i32, i32),
            pub player_health: (f32, f32),
            pub player_spirit: (f32, f32),
            pub player_stamina: (f32, f32),
            #[serde(default)]
            pub player_gold: u32,
            pub corrupted_tiles: Vec<((i32, i32), f32)>,
            pub inventory_items: Vec<(String, String, u32)>, // (id, display_name, quantity)
            #[serde(default)]
            pub tutorial_progress: TutorialProgressData,
            #[serde(default)]
            pub monsters: Vec<MonsterSaveData>,
            #[serde(default)]
            pub npcs: Vec<NpcSaveData>,
            #[serde(default)]
            pub minion_entities: Vec<String>, // Monster IDs that are tamed minions
            #[serde(default)]
            pub dungeon_entrances: Vec<DungeonEntranceSaveData>,
            pub timestamp: f64,
            pub save_version: u32,
        }

        // Resource to store loaded save data for applying in next frame
        #[derive(Resource, Default)]
        pub struct PendingLoadData {
            pub data: Option<SaveData>,
        }

        pub fn handle_save_requests(
            mut save_events: EventReader<super::events::SaveRequested>,
            mut load_events: EventReader<super::events::LoadRequested>,
            player: Query<
                (
                    &GridPosition,
                    &Health,
                    &Spirit,
                    &Stamina,
                    Option<&bevy_shaman_items::components::Inventory>,
                ),
                With<Player>,
            >,
            currency: Option<Res<bevy_shaman_shop::resources::Currency>>,
            corrupted: Query<(&GridPosition, &TileCorruption)>,
            monsters: Query<(
                &bevy_shaman_monsters::components::MonsterId,
                &GridPosition,
                &Health,
                &bevy_shaman_monsters::components::MonsterState,
                Option<&bevy_shaman_monsters::components::Tamed>,
            )>,
            npcs: Query<(
                &bevy_shaman_story::components::NpcName,
                &GridPosition,
                &bevy_shaman_story::components::NpcSicknessState,
                &bevy_shaman_story::components::NpcDialogue,
                Option<&bevy_shaman_story::components::HeadShaman>,
                Option<&bevy_shaman_story::components::PlayerBrother>,
            )>,
            dungeon_entrances: Query<(
                &GridPosition,
                &bevy_shaman_world::components::DungeonEntrance,
            )>,
            time: Res<Time>,
            mut pending_load: ResMut<PendingLoadData>,
            mut loading_flag: ResMut<bevy_shaman_core::resources::LoadingFromSave>,
        ) {
            // Handle save requests
            for _event in save_events.read() {
                if let Ok((pos, health, spirit, stamina, inventory)) = player.get_single() {
                    // Extract inventory data
                    let inventory_items = if let Some(inv) = inventory {
                        inv.items
                            .iter()
                            .map(|stack| {
                                (
                                    stack.item.id.clone(),
                                    stack.item.display_name.clone(),
                                    stack.quantity,
                                )
                            })
                            .collect()
                    } else {
                        vec![]
                    };

                    // Extract gold from Currency resource
                    let player_gold = currency.as_ref().map(|c| c.gold).unwrap_or(0);

                    // Extract monster data
                    let monster_data: Vec<MonsterSaveData> = monsters
                        .iter()
                        .map(|(monster_id, pos, health, state, tamed)| MonsterSaveData {
                            monster_id: monster_id.0.clone(),
                            position: (pos.x, pos.y),
                            health: (health.current, health.max),
                            state: format!("{:?}", state.state),
                            stability_meter: state.stability_meter,
                            corruption_meter: state.corruption_meter,
                            obedience_meter: state.obedience_meter,
                            is_tamed: tamed.is_some(),
                        })
                        .collect();

                    // Extract NPC data
                    let npc_data: Vec<NpcSaveData> = npcs
                        .iter()
                        .map(
                            |(name, pos, sickness, dialogue, head_shaman, player_brother)| {
                                NpcSaveData {
                                    name: name.name.clone(),
                                    position: (pos.x, pos.y),
                                    sickness_state: format!("{:?}", sickness),
                                    full_dialogue: dialogue.full_dialogue.clone(),
                                    partial_dialogue: dialogue.partial_dialogue.clone(),
                                    sick_dialogue: dialogue.sick_dialogue.clone(),
                                    is_head_shaman: head_shaman.is_some(),
                                    is_player_brother: player_brother.is_some(),
                                    soul_corruption: player_brother
                                        .map(|b| b.soul_corruption)
                                        .unwrap_or(1.0),
                                    fights_remaining: player_brother
                                        .map(|b| b.fights_remaining)
                                        .unwrap_or(4),
                                }
                            },
                        )
                        .collect();

                    // Tutorial progress will be saved by the tutorial crate to avoid circular dependencies
                    let tutorial_data = TutorialProgressData::default();

                    // Get list of minion monster IDs
                    let minion_entities: Vec<String> = monsters
                        .iter()
                        .filter(|(_, _, _, _, tamed)| tamed.is_some())
                        .map(|(monster_id, _, _, _, _)| monster_id.0.clone())
                        .collect();

                    // Extract dungeon entrance data
                    let dungeon_entrance_data: Vec<DungeonEntranceSaveData> = dungeon_entrances
                        .iter()
                        .map(|(pos, entrance)| DungeonEntranceSaveData {
                            position: (pos.x, pos.y),
                            dungeon_id: entrance.dungeon_id.clone(),
                            ecosystem: format!("{:?}", entrance.ecosystem),
                            difficulty_level: entrance.difficulty_level,
                            is_discovered: entrance.is_discovered,
                        })
                        .collect();

                    let save_data = SaveData {
                        player_position: (pos.x, pos.y),
                        player_health: (health.current, health.max),
                        player_spirit: (spirit.current, spirit.max),
                        player_stamina: (stamina.current, stamina.max),
                        player_gold,
                        corrupted_tiles: corrupted
                            .iter()
                            .filter(|(_, corruption)| corruption.is_corrupt())
                            .map(|(pos, corruption)| ((pos.x, pos.y), corruption.level))
                            .collect(),
                        inventory_items,
                        tutorial_progress: tutorial_data,
                        monsters: monster_data,
                        npcs: npc_data,
                        minion_entities,
                        dungeon_entrances: dungeon_entrance_data,
                        timestamp: time.elapsed_secs_f64(),
                        save_version: 1,
                    };

                    match serde_json::to_string_pretty(&save_data) {
                        Ok(json) => {
                            // Create saves directory if it doesn't exist
                            if let Err(e) = fs::create_dir_all("saves") {
                                error!("Failed to create saves directory: {}", e);
                                return;
                            }

                            // Write to file
                            match fs::write("saves/autosave.json", json) {
                                Ok(_) => info!("Game saved successfully"),
                                Err(e) => error!("Failed to save game: {}", e),
                            }
                        }
                        Err(e) => error!("Failed to serialize save data: {}", e),
                    }
                }
            }

            // Handle load requests - store data in resource for next frame
            for event in load_events.read() {
                // Slot 0 is reserved for autosave
                let filename = if event.save_slot == 0 {
                    "saves/autosave.json".to_string()
                } else {
                    format!("saves/save_{}.json", event.save_slot)
                };

                match fs::read_to_string(&filename) {
                    Ok(json) => match serde_json::from_str::<SaveData>(&json) {
                        Ok(save_data) => {
                            info!("Save data loaded from {}, will apply next frame", filename);
                            pending_load.data = Some(save_data);
                            loading_flag.is_loading = true;
                        }
                        Err(e) => error!("Failed to deserialize save data: {}", e),
                    },
                    Err(e) => error!("Failed to load game from {}: {}", filename, e),
                }
            }
        }

        // System to apply loaded save data to the world
        pub fn apply_loaded_data(
            mut commands: Commands,
            mut pending_load: ResMut<PendingLoadData>,
            mut loading_flag: ResMut<bevy_shaman_core::resources::LoadingFromSave>,
            mut player_query: Query<
                (
                    Entity,
                    &mut GridPosition,
                    &mut Transform,
                    &mut Health,
                    &mut Spirit,
                    &mut Stamina,
                    Option<&mut bevy_shaman_items::components::Inventory>,
                ),
                With<Player>,
            >,
            mut currency: Option<ResMut<bevy_shaman_shop::resources::Currency>>,
            mut corrupted_query: Query<(&GridPosition, &mut TileCorruption)>,
            mut player_spawned: ResMut<bevy_shaman_core::systems::player::PlayerSpawned>,
            mut world_generated: ResMut<bevy_shaman_world::systems::generation::WorldGenerated>,
            sprite_handle: Option<Res<bevy_shaman_core::systems::assets::PlayerSpriteHandle>>,
            monster_sprites: Option<Res<bevy_shaman_core::systems::assets::MonsterSpriteHandles>>,
            monster_template_db: Option<Res<bevy_shaman_monsters::resources::MonsterTemplateDB>>,
            item_db: Option<Res<bevy_shaman_items::resources::ItemDB>>,
            // Queries for cleaning up existing entities
            existing_monsters: Query<Entity, With<bevy_shaman_monsters::components::MonsterId>>,
            existing_npcs: Query<Entity, With<bevy_shaman_story::components::NpcName>>,
        ) {
            if let Some(save_data) = pending_load.data.take() {
                info!("Applying loaded save data to world");

                // Despawn existing monsters and NPCs to prevent duplicates
                for entity in existing_monsters.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                for entity in existing_npcs.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                info!("Cleared existing monsters and NPCs");

                // Mark that we've generated world and spawned player to prevent duplicates
                player_spawned.0 = true;
                world_generated.0 = true;

                // Restore currency resource
                if let Some(ref mut curr) = currency {
                    curr.gold = save_data.player_gold;
                    info!("Restored {} gold", save_data.player_gold);
                } else if save_data.player_gold > 0 {
                    commands.insert_resource(bevy_shaman_shop::resources::Currency {
                        gold: save_data.player_gold,
                    });
                    info!(
                        "Created currency resource with {} gold",
                        save_data.player_gold
                    );
                }

                // Tutorial progress will be restored by the tutorial crate

                // Restore or spawn player data
                match player_query.get_single_mut() {
                    Ok((
                        entity,
                        mut pos,
                        mut transform,
                        mut health,
                        mut spirit,
                        mut stamina,
                        inventory,
                    )) => {
                        // Player exists, update components
                        pos.x = save_data.player_position.0;
                        pos.y = save_data.player_position.1;
                        transform.translation.x = pos.x as f32 * 32.0;
                        transform.translation.y = pos.y as f32 * 32.0;
                        health.current = save_data.player_health.0;
                        health.max = save_data.player_health.1;
                        spirit.current = save_data.player_spirit.0;
                        spirit.max = save_data.player_spirit.1;
                        stamina.current = save_data.player_stamina.0;
                        stamina.max = save_data.player_stamina.1;

                        // Restore inventory
                        if let Some(item_db) = item_db.as_ref() {
                            let mut restored_count = 0;
                            let mut missing_count = 0;

                            if let Some(mut inv) = inventory {
                                inv.items.clear();
                                for (item_id, display_name, quantity) in &save_data.inventory_items
                                {
                                    if let Some(item) = item_db.get(item_id) {
                                        inv.add_item(item.clone(), *quantity);
                                        restored_count += 1;
                                    } else {
                                        error!("Failed to restore item '{}' ({}): not found in database", display_name, item_id);
                                        missing_count += 1;
                                    }
                                }
                                if missing_count > 0 {
                                    warn!(
                                        "Restored {}/{} inventory items ({} missing from database)",
                                        restored_count,
                                        save_data.inventory_items.len(),
                                        missing_count
                                    );
                                } else {
                                    info!(
                                        "Restored {} inventory items successfully",
                                        restored_count
                                    );
                                }
                            } else {
                                // Player doesn't have inventory component, add it
                                let mut new_inv = bevy_shaman_items::components::Inventory::new(20);
                                for (item_id, display_name, quantity) in &save_data.inventory_items
                                {
                                    if let Some(item) = item_db.get(item_id) {
                                        new_inv.add_item(item.clone(), *quantity);
                                        restored_count += 1;
                                    } else {
                                        error!("Failed to restore item '{}' ({}): not found in database", display_name, item_id);
                                        missing_count += 1;
                                    }
                                }
                                commands.entity(entity).insert(new_inv);
                                if missing_count > 0 {
                                    warn!("Created inventory with {}/{} items ({} missing from database)",
                                        restored_count, save_data.inventory_items.len(), missing_count);
                                } else {
                                    info!(
                                        "Created inventory with {} items successfully",
                                        restored_count
                                    );
                                }
                            }
                        } else {
                            warn!(
                                "Item database not available - cannot restore {} inventory items",
                                save_data.inventory_items.len()
                            );
                        }

                        info!("Player data restored to position ({}, {})", pos.x, pos.y);
                    }
                    Err(_) => {
                        // Player doesn't exist, spawn new one with save data
                        info!("Player entity not found, spawning from save data");

                        if let Some(sprite_handle) = sprite_handle {
                            use bevy_shaman_core::components::*;

                            // Build inventory if we have items to restore
                            let mut inventory_component = None;
                            if let Some(item_db) = item_db.as_ref() {
                                if !save_data.inventory_items.is_empty() {
                                    let mut inv = bevy_shaman_items::components::Inventory::new(20);
                                    let mut restored = 0;
                                    let mut missing = 0;
                                    for (item_id, display_name, quantity) in
                                        &save_data.inventory_items
                                    {
                                        if let Some(item) = item_db.get(item_id) {
                                            inv.add_item(item.clone(), *quantity);
                                            restored += 1;
                                        } else {
                                            error!("Failed to restore item '{}' ({}): not found in database", display_name, item_id);
                                            missing += 1;
                                        }
                                    }
                                    if missing > 0 {
                                        warn!(
                                            "Player spawn: restored {}/{} items ({} missing)",
                                            restored,
                                            save_data.inventory_items.len(),
                                            missing
                                        );
                                    }
                                    inventory_component = Some(inv);
                                }
                            } else if !save_data.inventory_items.is_empty() {
                                warn!("Item database not available - cannot restore {} inventory items for spawned player", save_data.inventory_items.len());
                            }

                            let mut entity_commands = commands.spawn((
                                Player,
                                GridPosition {
                                    x: save_data.player_position.0,
                                    y: save_data.player_position.1,
                                },
                                Transform::from_xyz(
                                    save_data.player_position.0 as f32 * 32.0,
                                    save_data.player_position.1 as f32 * 32.0,
                                    10.0,
                                ),
                                Sprite {
                                    image: sprite_handle.0.clone(),
                                    custom_size: Some(Vec2::new(32.0, 32.0)),
                                    ..default()
                                },
                                Health {
                                    current: save_data.player_health.0,
                                    max: save_data.player_health.1,
                                },
                                Spirit {
                                    current: save_data.player_spirit.0,
                                    max: save_data.player_spirit.1,
                                    regen_rate: 5.0,
                                },
                                Stamina {
                                    current: save_data.player_stamina.0,
                                    max: save_data.player_stamina.1,
                                    regen_rate: 10.0,
                                },
                                MovementQueue::default(),
                                BlocksMovement,
                                CameraTarget,
                                GlobalTransform::default(),
                                Visibility::default(),
                            ));

                            // Add inventory if we created one
                            if let Some(inv) = inventory_component {
                                entity_commands.insert(inv);
                                info!(
                                    "Spawned player with {} inventory stacks",
                                    save_data.inventory_items.len()
                                );
                            }

                            player_spawned.0 = true;
                            info!(
                                "Player spawned from save at ({}, {})",
                                save_data.player_position.0, save_data.player_position.1
                            );
                        } else {
                            error!("Cannot spawn player: PlayerSpriteHandle not available");
                        }
                    }
                }

                // Restore corruption data
                for ((tile_x, tile_y), corruption_level) in &save_data.corrupted_tiles {
                    for (tile_pos, mut corruption) in corrupted_query.iter_mut() {
                        if tile_pos.x == *tile_x && tile_pos.y == *tile_y {
                            corruption.level = *corruption_level;
                        }
                    }
                }
                info!(
                    "Restored {} corrupted tiles",
                    save_data.corrupted_tiles.len()
                );

                // Spawn monsters from save data
                if let (Some(monster_sprites), Some(template_db)) =
                    (monster_sprites, monster_template_db)
                {
                    use bevy_shaman_core::components::*;
                    use bevy_shaman_monsters::components::*;

                    for monster_data in &save_data.monsters {
                        // Parse state type
                        let state_type = match monster_data.state.as_str() {
                            "Stable" => StateType::Stable,
                            "Chaos" => StateType::Chaos,
                            "Corrupt" => StateType::Corrupt,
                            "Harmony" => StateType::Harmony,
                            "Decay" => StateType::Decay,
                            "Rage" => StateType::Rage,
                            "Void" => StateType::Void,
                            "Ancestral" => StateType::Ancestral,
                            _ => StateType::Stable,
                        };

                        // Get monster template for default stats
                        let template = template_db.get(&monster_data.monster_id);

                        // Select appropriate sprite
                        let sprite_handle = match monster_data.monster_id.as_str() {
                            "forest_spirit" => monster_sprites.forest_spirit.clone(),
                            "chaos_hound" => monster_sprites.chaos_hound.clone(),
                            "corrupt_shade" => monster_sprites.corrupt_shade.clone(),
                            "shadow_beast" => monster_sprites.shadow_beast.clone(),
                            "spirit_wisp" => monster_sprites.spirit_wisp.clone(),
                            "rock_golem" => monster_sprites.rock_golem.clone(),
                            "flame_wraith" => monster_sprites.flame_wraith.clone(),
                            "void_stalker" => monster_sprites.void_stalker.clone(),
                            _ => monster_sprites.forest_spirit.clone(),
                        };

                        let mut entity_commands = commands.spawn((
                            MonsterId(monster_data.monster_id.clone()),
                            GridPosition {
                                x: monster_data.position.0,
                                y: monster_data.position.1,
                            },
                            Transform::from_xyz(
                                monster_data.position.0 as f32 * 32.0,
                                monster_data.position.1 as f32 * 32.0,
                                5.0,
                            ),
                            Sprite {
                                image: sprite_handle,
                                custom_size: Some(Vec2::new(32.0, 32.0)),
                                ..default()
                            },
                            Health {
                                current: monster_data.health.0,
                                max: monster_data.health.1,
                            },
                            MonsterState {
                                state: state_type,
                                stability_meter: monster_data.stability_meter,
                                corruption_meter: monster_data.corruption_meter,
                                obedience_meter: monster_data.obedience_meter,
                                chaos_output: 1.0,
                            },
                            MonsterStats {
                                attack: template.map(|t| t.base_stats.attack).unwrap_or(10.0),
                                defense: template.map(|t| t.base_stats.defense).unwrap_or(5.0),
                                speed: template.map(|t| t.base_stats.speed).unwrap_or(5.0),
                                spirit_affinity: template
                                    .map(|t| t.base_stats.spirit_affinity)
                                    .unwrap_or(0.5),
                            },
                            MusicAffinityProfile {
                                prefers_calm: template
                                    .map(|t| t.affinity_profile.prefers_calm)
                                    .unwrap_or(0.5),
                                prefers_aggressive: template
                                    .map(|t| t.affinity_profile.prefers_aggressive)
                                    .unwrap_or(0.5),
                                corruption_resistance: template
                                    .map(|t| t.affinity_profile.corruption_resistance)
                                    .unwrap_or(0.5),
                                trust_level: template
                                    .map(|t| t.affinity_profile.trust_level)
                                    .unwrap_or(0.0),
                            },
                            AiBehavior::default(),
                            AiState::default(),
                            BlocksMovement,
                            GlobalTransform::default(),
                            Visibility::default(),
                        ));

                        // Add Tamed component if monster was tamed
                        if monster_data.is_tamed {
                            entity_commands.insert(Tamed {
                                tamed_at: save_data.timestamp,
                            });
                        }
                    }
                    info!(
                        "Spawned {} monsters from save data",
                        save_data.monsters.len()
                    );
                } else {
                    warn!("Cannot spawn monsters: sprite handles or template DB not available");
                }

                // Spawn NPCs from save data
                for npc_data in &save_data.npcs {
                    use bevy_shaman_core::components::*;
                    use bevy_shaman_story::components::*;
                    use bevy_shaman_story::resources::PortraitEmotion;

                    // Parse sickness state
                    let sickness_state = match npc_data.sickness_state.as_str() {
                        "AsleepSick" => NpcSicknessState::AsleepSick,
                        "Waking" => NpcSicknessState::Waking,
                        "Awake" => NpcSicknessState::Awake,
                        _ => NpcSicknessState::AsleepSick,
                    };

                    // Determine emotion based on role
                    let emotion = if npc_data.is_player_brother {
                        PortraitEmotion::Angry
                    } else {
                        PortraitEmotion::Neutral
                    };

                    // Create NPC entity
                    let mut entity_commands = commands.spawn((
                        NpcName {
                            name: npc_data.name.clone(),
                            current_emotion: emotion,
                        },
                        GridPosition {
                            x: npc_data.position.0,
                            y: npc_data.position.1,
                        },
                        Transform::from_xyz(
                            npc_data.position.0 as f32 * 32.0,
                            npc_data.position.1 as f32 * 32.0,
                            1.0,
                        ),
                        sickness_state,
                        NpcDialogue {
                            full_dialogue: npc_data.full_dialogue.clone(),
                            partial_dialogue: npc_data.partial_dialogue.clone(),
                            sick_dialogue: npc_data.sick_dialogue.clone(),
                        },
                        GlobalTransform::default(),
                        Visibility::default(),
                        Name::new(format!("NPC: {}", npc_data.name)),
                    ));

                    // Add special components based on role
                    if npc_data.is_head_shaman {
                        entity_commands.insert(HeadShaman);
                    }
                    if npc_data.is_player_brother {
                        entity_commands.insert(PlayerBrother {
                            soul_corruption: npc_data.soul_corruption,
                            fights_remaining: npc_data.fights_remaining,
                        });
                    }
                }
                info!("Spawned {} NPCs from save data", save_data.npcs.len());

                // Spawn dungeon entrances from save data (NOTE: World tiles should already exist)
                // We need to query existing world tiles and add DungeonEntrance component to them
                for entrance_data in &save_data.dungeon_entrances {
                    use bevy_shaman_world::components::{BiomeType, DungeonEntrance};

                    // Parse ecosystem type
                    let ecosystem = match entrance_data.ecosystem.as_str() {
                        "Jungle" => BiomeType::Jungle,
                        "Desert" => BiomeType::Desert,
                        "Forest" => BiomeType::Forest,
                        "Safari" => BiomeType::Safari,
                        "DeadRealm" => BiomeType::DeadRealm,
                        _ => BiomeType::Forest,
                    };

                    // Spawn dungeon entrance marker (or find existing tile and add component)
                    // For now, we'll just spawn a new entity
                    commands.spawn((
                        DungeonEntrance {
                            dungeon_id: entrance_data.dungeon_id.clone(),
                            ecosystem,
                            difficulty_level: entrance_data.difficulty_level,
                            is_discovered: entrance_data.is_discovered,
                        },
                        GridPosition {
                            x: entrance_data.position.0,
                            y: entrance_data.position.1,
                        },
                    ));
                }
                info!(
                    "Restored {} dungeon entrances from save data",
                    save_data.dungeon_entrances.len()
                );

                // TODO: Tutorial progress restoration will be handled by tutorial crate
                // listening to save data events to avoid circular dependency

                // Clear loading flag
                loading_flag.is_loading = false;
                info!("Save load complete!");
            }
        }
    }
}
