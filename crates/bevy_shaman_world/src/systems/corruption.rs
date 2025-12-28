use crate::components::TileCorruption;
use crate::systems::events::CorruptionSpread;
use bevy::prelude::*;
use bevy_shaman_core::components::GridPosition;
use std::collections::HashMap;

const CORRUPTION_SPREAD_RATE: f32 = 0.01;
const CORRUPTION_SPREAD_INTERVAL: f32 = 2.0;
const CORRUPTION_SPREAD_RADIUS: i32 = 2;

/// Spreads corruption from highly corrupt tiles to nearby tiles
/// OPTIMIZED: Uses spatial partitioning to avoid O(n²) nested loops
pub fn spread_corruption(
    time: Res<Time>,
    tiles: Query<(Entity, &GridPosition, &TileCorruption)>,
    mut tile_corruption: Query<&mut TileCorruption>,
    mut spread_events: EventWriter<CorruptionSpread>,
) {
    // Time-sliced: only run every N seconds
    if time.elapsed_secs() % CORRUPTION_SPREAD_INTERVAL > 0.1 {
        return;
    }

    // OPTIMIZATION: Build spatial hashmap for O(1) neighbor lookups
    // This replaces O(n²) nested iteration with O(n) single pass
    let mut spatial_map: HashMap<(i32, i32), Vec<(Entity, &TileCorruption)>> = HashMap::new();

    for (entity, pos, corruption) in tiles.iter() {
        spatial_map
            .entry((pos.x, pos.y))
            .or_insert_with(Vec::new)
            .push((entity, corruption));
    }

    // Collect highly corrupt source tiles
    let source_tiles: Vec<_> = tiles
        .iter()
        .filter(|(_, _, corruption)| corruption.level >= 0.7 && !corruption.purified)
        .collect();

    // For each highly corrupt tile, only check nearby cells (O(n) instead of O(n²))
    for (source_entity, source_pos, source_corruption) in source_tiles {
        // Check 5x5 grid around source (max distance 2)
        for dx in -CORRUPTION_SPREAD_RADIUS..=CORRUPTION_SPREAD_RADIUS {
            for dy in -CORRUPTION_SPREAD_RADIUS..=CORRUPTION_SPREAD_RADIUS {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let neighbor_pos = (source_pos.x + dx, source_pos.y + dy);

                if let Some(neighbors) = spatial_map.get(&neighbor_pos) {
                    for (target_entity, _) in neighbors {
                        if source_entity == *target_entity {
                            continue;
                        }

                        let Ok(mut target_corruption) = tile_corruption.get_mut(*target_entity) else {
                            continue;
                        };

                        if target_corruption.purified {
                            continue;
                        }

                        // Calculate distance (Manhattan distance is faster than Euclidean)
                        let distance = dx.abs() + dy.abs();
                        if distance > CORRUPTION_SPREAD_RADIUS {
                            continue;
                        }

                        let spread_amount = CORRUPTION_SPREAD_RATE * (1.0 - distance as f32 / 2.0);
                        target_corruption.level = (target_corruption.level + spread_amount).min(1.0);
                        target_corruption.corruption_type = source_corruption.corruption_type;

                        spread_events.send(CorruptionSpread {
                            from_tile: source_entity,
                            to_tile: *target_entity,
                            amount: spread_amount,
                        });
                    }
                }
            }
        }
    }
}
