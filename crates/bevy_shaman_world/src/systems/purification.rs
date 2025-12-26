use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, Player, Spirit};
use crate::components::TileCorruption;
use crate::resources::PurificationAbility;
use crate::systems::events::TilePurified;

/// Processes purification casts from player
pub fn process_purification_casts(
    keyboard: Res<ButtonInput<KeyCode>>,
    ability: Res<PurificationAbility>,
    time: Res<Time>,
    mut player: Query<(&GridPosition, &mut Spirit), With<Player>>,
    mut tiles: Query<(Entity, &GridPosition, &mut TileCorruption)>,
    mut purify_events: EventWriter<TilePurified>,
) {
    if !keyboard.just_pressed(KeyCode::KeyP) {
        return;
    }

    let Ok((player_pos, mut player_spirit)) = player.get_single_mut() else {
        return;
    };

    // Check spirit cost
    if player_spirit.current < ability.spirit_cost {
        warn!("Not enough Spirit to purify!");
        return;
    }

    player_spirit.current -= ability.spirit_cost;

    // Get affected tiles based on shape
    let offsets = ability.shape.get_offsets(ability.range);

    for (dx, dy) in offsets {
        let target_x = player_pos.x + dx;
        let target_y = player_pos.y + dy;

        for (tile_entity, tile_pos, mut corruption) in tiles.iter_mut() {
            if tile_pos.x == target_x && tile_pos.y == target_y {
                let removed = corruption.level;
                corruption.level = 0.0;
                corruption.purified = true;
                corruption.purified_timestamp = Some(time.elapsed_secs_f64());

                purify_events.send(TilePurified {
                    tile_entity,
                    corruption_removed: removed,
                });

                info!("Purified tile at ({}, {})", target_x, target_y);
            }
        }
    }
}
