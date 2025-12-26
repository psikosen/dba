use bevy::prelude::*;
use crate::components::{BlocksMovement, GridPosition};
use crate::resources::GridOccupancy;

/// Updates GridOccupancy resource based on entities with GridPosition + BlocksMovement
pub fn update_grid_occupancy(
    mut occupancy: ResMut<GridOccupancy>,
    blockers: Query<&GridPosition, (With<BlocksMovement>, Changed<GridPosition>)>,
) {
    occupancy.occupied.clear();
    for pos in blockers.iter() {
        occupancy.occupy(pos.x, pos.y);
    }
}
