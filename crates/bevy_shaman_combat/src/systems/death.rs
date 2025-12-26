use bevy::prelude::*;
use bevy_shaman_core::components::{Health, Player};
use bevy_shaman_core::events::EntityDied;
use bevy_shaman_core::states::GameState;

/// Handle entity deaths by despawning them
pub fn handle_entity_deaths(
    mut commands: Commands,
    mut death_events: EventReader<EntityDied>,
    player_query: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in death_events.read() {
        // Check if this is the player
        let is_player = player_query.iter().any(|e| e == event.entity);

        if is_player {
            info!("Player died! Game Over.");
            // Transition to game over state (or main menu for now)
            next_state.set(GameState::MainMenu);
        } else {
            info!("Entity {:?} died, despawning...", event.entity);
            commands.entity(event.entity).despawn_recursive();
        }
    }
}

/// System to check for dead entities that weren't properly handled
pub fn cleanup_dead_entities(
    mut commands: Commands,
    dead_entities: Query<(Entity, &Health), Without<Player>>,
    mut death_events: EventWriter<EntityDied>,
) {
    for (entity, health) in dead_entities.iter() {
        if health.is_dead() {
            death_events.send(EntityDied {
                entity,
                was_player: false,
            });
        }
    }
}
