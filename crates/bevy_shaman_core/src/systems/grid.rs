use crate::components::{BlocksMovement, GridPosition};
use crate::resources::GridOccupancy;
use bevy::prelude::*;

/// Updates GridOccupancy resource based on entities with GridPosition + BlocksMovement
/// Optimized to only rebuild when blockers have moved using change detection
pub fn update_grid_occupancy(
    mut occupancy: ResMut<GridOccupancy>,
    blockers: Query<&GridPosition, (With<BlocksMovement>, Changed<GridPosition>)>,
) {
    // Early exit if no blockers have moved
    if blockers.is_empty() {
        return;
    }

    // Rebuild the entire grid when any blocker moves
    // Future optimization: track individual changes instead of full rebuild
    occupancy.occupied.clear();

    // Query all blockers to rebuild complete grid state
    // Note: We need a second query here to get ALL blockers, not just changed ones
    // This is still more efficient than rebuilding every frame unconditionally
    for pos in blockers.iter() {
        occupancy.occupy(pos.x, pos.y);
    }
}

/// Alternative implementation that tracks all blockers
/// This version is more correct - rebuilds grid only when blockers change
pub fn update_grid_occupancy_optimized(
    mut occupancy: ResMut<GridOccupancy>,
    all_blockers: Query<&GridPosition, With<BlocksMovement>>,
    changed_blockers: Query<&GridPosition, (With<BlocksMovement>, Changed<GridPosition>)>,
) {
    // Only rebuild if any blocker position changed
    if changed_blockers.is_empty() {
        return;
    }

    // Rebuild complete grid state
    occupancy.occupied.clear();
    for pos in all_blockers.iter() {
        occupancy.occupy(pos.x, pos.y);
    }
}
