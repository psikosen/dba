use crate::components::{BiomeType, CorruptionType, DungeonEntrance, TileCorruption, WorldTile};
use bevy::prelude::*;
use bevy_shaman_core::components::GridPosition;
use bevy_shaman_core::systems::assets::TileSpriteHandles;
use rand::Rng;
use std::f32::consts::PI;

/// Resource to track if world has been generated
#[derive(Resource, Default)]
pub struct WorldGenerated(pub bool);

/// World generation parameters
#[derive(Resource)]
pub struct WorldGenConfig {
    pub world_radius: i32,
    pub village_radius: i32,
    pub ecosystem_count: usize,
    pub dungeons_per_ecosystem: usize,
}

impl Default for WorldGenConfig {
    fn default() -> Self {
        Self {
            world_radius: 150,         // Total world radius (150 tiles from center)
            village_radius: 20,        // Central village safe zone
            ecosystem_count: 5,        // 5 ecosystems
            dungeons_per_ecosystem: 3, // 3 dungeons per ecosystem
        }
    }
}

/// Ecosystem region definition
struct EcosystemRegion {
    biome: BiomeType,
    center_angle: f32,  // Angle from world center (radians)
    angular_width: f32, // Width of region in radians
    min_radius: i32,    // Distance from center
    max_radius: i32,
    corruption_base: f32, // Base corruption level for this ecosystem
}

/// Generate the overworld when entering the game
pub fn generate_overworld(
    mut commands: Commands,
    mut generated: ResMut<WorldGenerated>,
    config: Res<WorldGenConfig>,
    tile_handles: Option<Res<TileSpriteHandles>>,
    loading_from_save: Option<Res<bevy_shaman_core::resources::LoadingFromSave>>,
) {
    if generated.0 {
        return;
    }

    // Don't generate world if we're loading from a save
    if let Some(loading) = loading_from_save {
        if loading.is_loading {
            info!("Skipping world generation - loading from save");
            return;
        }
    }

    // Wait for assets to load
    let Some(tile_handles) = tile_handles else {
        return;
    };

    info!("Generating world with village hub and 5 ecosystems...");

    let mut rng = rand::thread_rng();

    // Define the 5 ecosystem regions radiating from center
    let ecosystems = create_ecosystem_layout(&config);

    let mut dungeon_counter = 0;

    // Generate tiles
    for x in -config.world_radius..config.world_radius {
        for y in -config.world_radius..config.world_radius {
            let world_x = x;
            let world_y = y;

            // Calculate distance from center
            let distance = ((x * x + y * y) as f32).sqrt();

            // Skip tiles outside world radius
            if distance > config.world_radius as f32 {
                continue;
            }

            // Calculate angle from center (for ecosystem determination)
            let angle = (y as f32).atan2(x as f32);

            // Determine biome based on distance and angle
            let (biome, corruption_base) = if distance <= config.village_radius as f32 {
                // Central village - safe zone
                (BiomeType::Village, 0.0)
            } else {
                // Determine which ecosystem this tile belongs to
                determine_ecosystem(&ecosystems, angle, distance)
            };

            // Determine if tile is walkable
            let walkable = match biome {
                BiomeType::Village => true,
                BiomeType::Forest | BiomeType::Jungle => rng.gen_bool(0.9),
                BiomeType::Desert => rng.gen_bool(0.95),
                BiomeType::Safari => rng.gen_bool(0.85),
                BiomeType::DeadRealm => rng.gen_bool(0.7),
                BiomeType::Mountains => rng.gen_bool(0.3),
                BiomeType::SpiritRealm => true,
            };

            // Determine corruption level
            let corruption_chance = corruption_base + (distance / config.world_radius as f32) * 0.3;
            let (corruption_level, corruption_type) = if rng.gen_bool(corruption_chance as f64) {
                let level = rng.gen_range(0.3..0.9);
                let corruption_type = match biome {
                    BiomeType::Jungle => CorruptionType::Decay,
                    BiomeType::Desert => CorruptionType::Void,
                    BiomeType::Forest => CorruptionType::Chaos,
                    BiomeType::Safari => CorruptionType::Chaos,
                    BiomeType::DeadRealm => CorruptionType::Ancestral,
                    _ => CorruptionType::None,
                };
                (level, corruption_type)
            } else {
                (0.0, CorruptionType::None)
            };

            // Choose sprite based on corruption and biome
            let sprite_handle = if corruption_level > 0.5 {
                tile_handles.corrupted.clone()
            } else {
                match biome {
                    BiomeType::Village => tile_handles.village.clone(),
                    BiomeType::Forest | BiomeType::Jungle => tile_handles.forest.clone(),
                    BiomeType::Desert => tile_handles.grass.clone(), // Use grass for desert (placeholder)
                    BiomeType::Safari => tile_handles.grass.clone(),
                    BiomeType::DeadRealm => tile_handles.corrupted.clone(),
                    BiomeType::Mountains => tile_handles.mountain.clone(),
                    BiomeType::SpiritRealm => tile_handles.grass.clone(),
                }
            };

            // Spawn tile entity
            let mut entity = commands.spawn((
                WorldTile { biome, walkable },
                TileCorruption {
                    level: corruption_level,
                    corruption_type,
                    purified: false,
                    purified_timestamp: None,
                },
                GridPosition {
                    x: world_x,
                    y: world_y,
                },
                Transform::from_xyz(world_x as f32 * 32.0, world_y as f32 * 32.0, 0.0),
                Sprite {
                    image: sprite_handle,
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                GlobalTransform::default(),
                Visibility::default(),
            ));

            // Spawn dungeon entrances in ecosystem regions
            if biome.can_have_dungeons() && walkable && corruption_level < 0.3 {
                // Place dungeons at strategic points within each ecosystem
                let should_spawn_dungeon = is_dungeon_location(
                    x,
                    y,
                    distance,
                    angle,
                    &ecosystems,
                    biome,
                    &mut dungeon_counter,
                    config.dungeons_per_ecosystem,
                );

                if should_spawn_dungeon {
                    let difficulty_level =
                        calculate_difficulty(distance, config.world_radius as f32);
                    let dungeon_id = format!(
                        "{}_{}",
                        biome.display_name().to_lowercase(),
                        dungeon_counter
                    );

                    entity.insert(DungeonEntrance {
                        dungeon_id: dungeon_id.clone(),
                        ecosystem: biome,
                        difficulty_level,
                        is_discovered: false,
                    });

                    info!(
                        "Spawned dungeon entrance '{}' at ({}, {}) in {} (Level {})",
                        dungeon_id,
                        world_x,
                        world_y,
                        biome.display_name(),
                        difficulty_level
                    );
                }
            }
        }
    }

    generated.0 = true;
    info!("World generation complete! Village hub with 5 ecosystems created.");
    info!("Total dungeons spawned: {}", dungeon_counter);
}

/// Create the 5 ecosystem layout radiating from center
fn create_ecosystem_layout(config: &WorldGenConfig) -> Vec<EcosystemRegion> {
    let angle_per_ecosystem = (2.0 * PI) / config.ecosystem_count as f32;

    vec![
        // Jungle (North)
        EcosystemRegion {
            biome: BiomeType::Jungle,
            center_angle: PI / 2.0, // 90 degrees (North)
            angular_width: angle_per_ecosystem,
            min_radius: config.village_radius,
            max_radius: config.world_radius,
            corruption_base: 0.2,
        },
        // Desert (East)
        EcosystemRegion {
            biome: BiomeType::Desert,
            center_angle: 0.0, // 0 degrees (East)
            angular_width: angle_per_ecosystem,
            min_radius: config.village_radius,
            max_radius: config.world_radius,
            corruption_base: 0.3,
        },
        // Forest (South)
        EcosystemRegion {
            biome: BiomeType::Forest,
            center_angle: -PI / 2.0, // -90 degrees (South)
            angular_width: angle_per_ecosystem,
            min_radius: config.village_radius,
            max_radius: config.world_radius,
            corruption_base: 0.15,
        },
        // Safari (West)
        EcosystemRegion {
            biome: BiomeType::Safari,
            center_angle: PI, // 180 degrees (West)
            angular_width: angle_per_ecosystem,
            min_radius: config.village_radius,
            max_radius: config.world_radius,
            corruption_base: 0.25,
        },
        // Dead Realm (Northwest)
        EcosystemRegion {
            biome: BiomeType::DeadRealm,
            center_angle: 3.0 * PI / 4.0, // 135 degrees (Northwest)
            angular_width: angle_per_ecosystem,
            min_radius: config.village_radius,
            max_radius: config.world_radius,
            corruption_base: 0.6,
        },
    ]
}

/// Determine which ecosystem a tile belongs to based on angle and distance
fn determine_ecosystem(
    ecosystems: &[EcosystemRegion],
    angle: f32,
    distance: f32,
) -> (BiomeType, f32) {
    for ecosystem in ecosystems {
        if distance >= ecosystem.min_radius as f32 && distance <= ecosystem.max_radius as f32 {
            // Normalize angle to match ecosystem center
            let angle_diff = normalize_angle(angle - ecosystem.center_angle);

            if angle_diff.abs() <= ecosystem.angular_width / 2.0 {
                return (ecosystem.biome, ecosystem.corruption_base);
            }
        }
    }

    // Default to forest if no match
    (BiomeType::Forest, 0.15)
}

/// Normalize angle to -PI to PI range
fn normalize_angle(angle: f32) -> f32 {
    let mut normalized = angle;
    while normalized > PI {
        normalized -= 2.0 * PI;
    }
    while normalized < -PI {
        normalized += 2.0 * PI;
    }
    normalized
}

/// Determine if this location should have a dungeon entrance
fn is_dungeon_location(
    x: i32,
    y: i32,
    distance: f32,
    angle: f32,
    ecosystems: &[EcosystemRegion],
    biome: BiomeType,
    dungeon_counter: &mut usize,
    max_per_ecosystem: usize,
) -> bool {
    // Find the ecosystem this tile belongs to
    for ecosystem in ecosystems {
        if ecosystem.biome != biome {
            continue;
        }

        // Place dungeons at specific radial distances (near, mid, far)
        let dungeon_radii = [
            ecosystem.min_radius as f32 + 20.0, // Near dungeon
            (ecosystem.min_radius + ecosystem.max_radius) as f32 / 2.0, // Mid dungeon
            ecosystem.max_radius as f32 - 30.0, // Far dungeon
        ];

        for (idx, &target_radius) in dungeon_radii.iter().enumerate() {
            if idx >= max_per_ecosystem {
                break;
            }

            // Check if close to target radius and at center angle of ecosystem
            let angle_diff = normalize_angle(angle - ecosystem.center_angle);
            let is_at_center_angle = angle_diff.abs() < 0.2; // Within ~11 degrees of center
            let is_at_radius = (distance - target_radius).abs() < 5.0; // Within 5 tiles

            if is_at_center_angle && is_at_radius {
                // Use grid position as unique seed for deterministic placement
                let seed = ((x + 1000) * 1000 + (y + 1000)) as u32;
                if seed % 100 == 0 {
                    // 1% chance per valid tile (ensures we place some)
                    *dungeon_counter += 1;
                    return true;
                }
            }
        }
    }

    false
}

/// Calculate dungeon difficulty based on distance from center
fn calculate_difficulty(distance: f32, world_radius: f32) -> u8 {
    let normalized_distance = distance / world_radius;

    if normalized_distance < 0.3 {
        1 // Easy
    } else if normalized_distance < 0.5 {
        2 // Medium
    } else if normalized_distance < 0.7 {
        3 // Hard
    } else if normalized_distance < 0.85 {
        4 // Very Hard
    } else {
        5 // Extreme
    }
}

/// Resource tracking world seed for procedural generation
#[derive(Resource)]
pub struct WorldSeed(pub u64);

impl Default for WorldSeed {
    fn default() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or_else(|e| {
                // If system time is before UNIX_EPOCH or unavailable, use a fixed seed
                bevy::log::warn!(
                    "Failed to get system time for seed: {}. Using fallback seed.",
                    e
                );
                12345678901234567890_u64
            });
        Self(seed)
    }
}
