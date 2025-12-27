use bevy::prelude::*;
use bevy_shaman_core::components::GridPosition;
use bevy_shaman_core::systems::assets::TileSpriteHandles;
use crate::components::{WorldTile, BiomeType, TileCorruption, CorruptionType};
use rand::Rng;

/// Resource to track if world has been generated
#[derive(Resource, Default)]
pub struct WorldGenerated(pub bool);

/// World generation parameters
#[derive(Resource)]
pub struct WorldGenConfig {
    pub width: i32,
    pub height: i32,
    pub village_count: usize,
    pub initial_corruption_level: f32,
}

impl Default for WorldGenConfig {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            village_count: 5,
            initial_corruption_level: 0.3,
        }
    }
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

    info!("Generating overworld map ({} x {})...", config.width, config.height);

    let mut rng = rand::thread_rng();

    // Generate village positions first
    let mut village_positions = Vec::new();
    for _ in 0..config.village_count {
        let x = rng.gen_range(5..config.width - 5);
        let y = rng.gen_range(5..config.height - 5);
        village_positions.push((x, y));
    }

    // Generate tiles
    for x in 0..config.width {
        for y in 0..config.height {
            let distance_to_nearest_village = village_positions
                .iter()
                .map(|(vx, vy)| ((x - vx).abs() + (y - vy).abs()) as f32)
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(1000.0);

            // Determine biome based on noise and distance to villages
            let biome = if distance_to_nearest_village < 3.0 {
                BiomeType::Village
            } else if distance_to_nearest_village < 15.0 {
                BiomeType::Forest
            } else if rng.gen_bool(0.3) {
                BiomeType::Mountains
            } else {
                BiomeType::Forest
            };

            // Determine if tile is walkable
            let walkable = match biome {
                BiomeType::Village => true,
                BiomeType::Forest => true,
                BiomeType::Mountains => rng.gen_bool(0.3), // Some mountains are passable
                BiomeType::SpiritRealm => true,
            };

            // Determine corruption (more corruption away from villages)
            let corruption_chance = if distance_to_nearest_village < 10.0 {
                0.1
            } else if distance_to_nearest_village < 30.0 {
                0.3
            } else {
                0.6
            };

            let (corruption_level, corruption_type) = if rng.gen_bool(corruption_chance as f64) {
                let level = rng.gen_range(0.3..0.8);
                let corruption_type = match rng.gen_range(0..4) {
                    0 => CorruptionType::Chaos,
                    1 => CorruptionType::Decay,
                    2 => CorruptionType::Void,
                    _ => CorruptionType::Ancestral,
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
                    BiomeType::Forest => tile_handles.forest.clone(),
                    BiomeType::Mountains => tile_handles.mountain.clone(),
                    BiomeType::SpiritRealm => tile_handles.grass.clone(),
                }
            };

            // Spawn tile entity
            commands.spawn((
                WorldTile { biome, walkable },
                TileCorruption {
                    level: corruption_level,
                    corruption_type,
                    purified: false,
                    purified_timestamp: None,
                },
                GridPosition { x, y },
                Transform::from_xyz(x as f32 * 32.0, y as f32 * 32.0, 0.0),
                Sprite {
                    image: sprite_handle,
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                GlobalTransform::default(),
                Visibility::default(),
            ));
        }
    }

    generated.0 = true;
    info!("Overworld generation complete! {} tiles created.", config.width * config.height);
}

/// Resource tracking world seed for procedural generation
#[derive(Resource)]
pub struct WorldSeed(pub u64);

impl Default for WorldSeed {
    fn default() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self(seed)
    }
}
