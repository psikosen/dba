use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, Health, MovementCommand, MovementQueue, Player};
use crate::components::{AiBehavior, AiState, MonsterState};
use super::pathfinding::{PathfindingGrid, get_next_move, get_flee_direction};

/// AI system: monsters pursue player or flee based on health/state using A* pathfinding
pub fn process_monster_ai(
    player: Query<&GridPosition, With<Player>>,
    mut monsters: Query<
        (
            &GridPosition,
            &Health,
            &MonsterState,
            &AiBehavior,
            &mut AiState,
            &mut MovementQueue,
        ),
        Without<Player>,
    >,
    pathfinding_grid: Res<PathfindingGrid>,
) {
    let Ok(player_pos) = player.get_single() else {
        return;
    };

    for (monster_pos, health, state, behavior, mut ai_state, mut movement_queue) in monsters.iter_mut() {
        // Fleeing logic
        if health.current / health.max < behavior.flee_threshold {
            *ai_state = AiState::Fleeing;
            // Use pathfinding-aware flee direction
            let flee_dir = get_flee_direction(*monster_pos, *player_pos, &pathfinding_grid);
            if flee_dir != IVec2::ZERO {
                movement_queue.commands.push(MovementCommand::Move(flee_dir));
            }
            continue;
        }

        // Aggressive pursuit
        if state.is_controllable() {
            *ai_state = AiState::Idle;
        } else {
            *ai_state = AiState::Aggressive;
            // Use A* pathfinding to pursue player
            if let Some(next_move) = get_next_move(*monster_pos, *player_pos, &pathfinding_grid) {
                movement_queue.commands.push(MovementCommand::Move(next_move));
            }
        }
    }
}
