use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, Health, MovementCommand, MovementQueue, Player};
use crate::components::{AiBehavior, AiState, MonsterState};

/// Basic AI system: monsters pursue player or flee based on health/state
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
) {
    let Ok(player_pos) = player.get_single() else {
        return;
    };

    for (monster_pos, health, state, behavior, mut ai_state, mut movement_queue) in monsters.iter_mut() {
        // Fleeing logic
        if health.current / health.max < behavior.flee_threshold {
            *ai_state = AiState::Fleeing;
            // Move away from player
            let dx = (monster_pos.x - player_pos.x).signum();
            let dy = (monster_pos.y - player_pos.y).signum();
            movement_queue.commands.push(MovementCommand::Move(IVec2::new(dx, dy)));
            continue;
        }

        // Aggressive pursuit
        if state.is_controllable() {
            *ai_state = AiState::Idle;
        } else {
            *ai_state = AiState::Aggressive;
            // Simple pursuit: move toward player
            let dx = (player_pos.x - monster_pos.x).signum();
            let dy = (player_pos.y - monster_pos.y).signum();
            movement_queue.commands.push(MovementCommand::Move(IVec2::new(dx, dy)));
        }
    }
}
