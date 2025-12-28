use crate::components::TileCorruption;
use crate::systems::events::CorruptionSpread;
use bevy::prelude::*;
use bevy_shaman_core::components::GridPosition;

const CORRUPTION_SPREAD_RATE: f32 = 0.01;
const CORRUPTION_SPREAD_INTERVAL: f32 = 2.0;

/// Spreads corruption from highly corrupt tiles to nearby tiles
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

    let tile_data: Vec<_> = tiles.iter().collect();

    for (source_entity, source_pos, source_corruption) in &tile_data {
        if source_corruption.level < 0.7 || source_corruption.purified {
            continue;
        }

        for (target_entity, target_pos, _) in &tile_data {
            if source_entity == target_entity {
                continue;
            }

            let distance = source_pos.distance(target_pos);
            if distance > 2 {
                continue;
            }

            let Ok(mut target_corruption) = tile_corruption.get_mut(*target_entity) else {
                continue;
            };

            if target_corruption.purified {
                continue;
            }

            let spread_amount = CORRUPTION_SPREAD_RATE * (1.0 - distance as f32 / 2.0);
            target_corruption.level = (target_corruption.level + spread_amount).min(1.0);
            target_corruption.corruption_type = source_corruption.corruption_type;

            spread_events.send(CorruptionSpread {
                from_tile: *source_entity,
                to_tile: *target_entity,
                amount: spread_amount,
            });
        }
    }
}
