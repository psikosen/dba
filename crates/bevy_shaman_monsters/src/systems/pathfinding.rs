use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, BlocksMovement};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

/// A* pathfinding node
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PathNode {
    position: GridPosition,
    g_cost: u32,  // Cost from start
    h_cost: u32,  // Heuristic cost to goal
}

impl PathNode {
    fn f_cost(&self) -> u32 {
        self.g_cost + self.h_cost
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.f_cost().cmp(&self.f_cost())
            .then_with(|| other.h_cost.cmp(&self.h_cost))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Resource to cache blocked positions
#[derive(Resource, Default)]
pub struct PathfindingGrid {
    pub blocked: HashSet<GridPosition>,
    pub dirty: bool,
}

impl PathfindingGrid {
    pub fn rebuild(&mut self, obstacles: &Query<&GridPosition, With<BlocksMovement>>) {
        self.blocked.clear();
        for pos in obstacles.iter() {
            self.blocked.insert(*pos);
        }
        self.dirty = false;
    }

    pub fn is_walkable(&self, pos: &GridPosition) -> bool {
        !self.blocked.contains(pos)
    }
}

/// Update the pathfinding grid when obstacles change
pub fn update_pathfinding_grid(
    obstacles: Query<&GridPosition, With<BlocksMovement>>,
    mut grid: ResMut<PathfindingGrid>,
) {
    // Rebuild every frame for simplicity (can be optimized to only rebuild when obstacles change)
    grid.rebuild(&obstacles);
}

/// Find a path from start to goal using A* algorithm
pub fn find_path(
    start: GridPosition,
    goal: GridPosition,
    grid: &PathfindingGrid,
    max_iterations: usize,
) -> Option<Vec<GridPosition>> {
    if start == goal {
        return Some(vec![start]);
    }

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<GridPosition, GridPosition> = HashMap::new();
    let mut g_scores: HashMap<GridPosition, u32> = HashMap::new();

    g_scores.insert(start, 0);
    open_set.push(PathNode {
        position: start,
        g_cost: 0,
        h_cost: start.distance(&goal),
    });

    let mut iterations = 0;

    while let Some(current_node) = open_set.pop() {
        iterations += 1;
        if iterations > max_iterations {
            // Prevent infinite loops, return best path found so far
            break;
        }

        let current = current_node.position;

        // Goal reached
        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }

        // Check all neighbors (4-directional movement)
        let neighbors = [
            GridPosition::new(current.x + 1, current.y),
            GridPosition::new(current.x - 1, current.y),
            GridPosition::new(current.x, current.y + 1),
            GridPosition::new(current.x, current.y - 1),
        ];

        for neighbor in neighbors {
            if !grid.is_walkable(&neighbor) {
                continue;
            }

            let tentative_g_score = g_scores.get(&current).unwrap_or(&u32::MAX) + 1;

            if tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, current);
                g_scores.insert(neighbor, tentative_g_score);

                open_set.push(PathNode {
                    position: neighbor,
                    g_cost: tentative_g_score,
                    h_cost: neighbor.distance(&goal),
                });
            }
        }
    }

    // No path found
    None
}

/// Reconstruct the path from the came_from map
fn reconstruct_path(came_from: &HashMap<GridPosition, GridPosition>, mut current: GridPosition) -> Vec<GridPosition> {
    let mut path = vec![current];
    while let Some(&previous) = came_from.get(&current) {
        path.push(previous);
        current = previous;
    }
    path.reverse();
    path
}

/// Get the next move direction toward a goal
pub fn get_next_move(
    start: GridPosition,
    goal: GridPosition,
    grid: &PathfindingGrid,
) -> Option<IVec2> {
    let path = find_path(start, goal, grid, 500)?;

    if path.len() < 2 {
        return None;
    }

    // Get the second position in the path (first is current position)
    let next = path[1];
    let dx = next.x - start.x;
    let dy = next.y - start.y;

    Some(IVec2::new(dx, dy))
}

/// Simple flee direction calculation (move away from target)
pub fn get_flee_direction(
    from: GridPosition,
    away_from: GridPosition,
    grid: &PathfindingGrid,
) -> IVec2 {
    let dx = (from.x - away_from.x).signum();
    let dy = (from.y - away_from.y).signum();

    // Try primary direction
    let primary = IVec2::new(dx, dy);
    if grid.is_walkable(&GridPosition::new(from.x + dx, from.y + dy)) {
        return primary;
    }

    // Try horizontal only
    if dx != 0 && grid.is_walkable(&GridPosition::new(from.x + dx, from.y)) {
        return IVec2::new(dx, 0);
    }

    // Try vertical only
    if dy != 0 && grid.is_walkable(&GridPosition::new(from.x, from.y + dy)) {
        return IVec2::new(0, dy);
    }

    // No valid flee direction, stay in place
    IVec2::ZERO
}
