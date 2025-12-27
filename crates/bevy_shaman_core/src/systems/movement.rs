use bevy::prelude::*;
use crate::components::{GridPosition, MovementCommand, MovementQueue};
use crate::resources::GridOccupancy;

const TILE_SIZE: f32 = 32.0;

/// Processes movement commands from queue, updates GridPosition and Transform
pub fn process_movement_commands(
    occupancy: Res<GridOccupancy>,
    mut movers: Query<(&mut GridPosition, &mut Transform, &mut MovementQueue)>,
) {
    for (mut grid_pos, mut transform, mut queue) in movers.iter_mut() {
        if queue.commands.is_empty() {
            continue;
        }

        let command = queue.commands.remove(0);

        match command {
            MovementCommand::Move(delta) => {
                let new_x = grid_pos.x + delta.x;
                let new_y = grid_pos.y + delta.y;

                if !occupancy.is_occupied(new_x, new_y) {
                    grid_pos.x = new_x;
                    grid_pos.y = new_y;
                    transform.translation.x = new_x as f32 * TILE_SIZE;
                    transform.translation.y = new_y as f32 * TILE_SIZE;
                }
            }
            MovementCommand::Dash(delta) => {
                // Dash ignores obstacles but costs stamina (handled elsewhere)
                let new_x = grid_pos.x + delta.x * 2;
                let new_y = grid_pos.y + delta.y * 2;
                grid_pos.x = new_x;
                grid_pos.y = new_y;
                transform.translation.x = new_x as f32 * TILE_SIZE;
                transform.translation.y = new_y as f32 * TILE_SIZE;
            }
            MovementCommand::Teleport(target) => {
                grid_pos.x = target.x;
                grid_pos.y = target.y;
                transform.translation.x = target.x as f32 * TILE_SIZE;
                transform.translation.y = target.y as f32 * TILE_SIZE;
            }
        }
    }
}
