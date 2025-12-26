use bevy::prelude::*;
use crate::components::*;

/// Handle keyboard input for player movement
pub fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut MovementQueue, &Stamina), With<Player>>,
) {
    let Ok((mut movement_queue, stamina)) = player.get_single_mut() else {
        return;
    };

    // Only allow input if not already moving (prevent input buffering spam)
    if !movement_queue.commands.is_empty() {
        return;
    }

    // Regular movement (WASD or Arrow keys)
    let mut direction = IVec2::ZERO;

    if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::ArrowUp) {
        direction.y += 1;
    }
    if keyboard.just_pressed(KeyCode::KeyS) || keyboard.just_pressed(KeyCode::ArrowDown) {
        direction.y -= 1;
    }
    if keyboard.just_pressed(KeyCode::KeyA) || keyboard.just_pressed(KeyCode::ArrowLeft) {
        direction.x -= 1;
    }
    if keyboard.just_pressed(KeyCode::KeyD) || keyboard.just_pressed(KeyCode::ArrowRight) {
        direction.x += 1;
    }

    // If shift is held, try to dash (costs stamina)
    if direction != IVec2::ZERO {
        if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            if stamina.current >= 10.0 {
                // Dash costs stamina
                movement_queue.commands.push(MovementCommand::Dash(direction));
            } else {
                // Not enough stamina for dash, do regular move
                movement_queue.commands.push(MovementCommand::Move(direction));
            }
        } else {
            // Regular movement
            movement_queue.commands.push(MovementCommand::Move(direction));
        }
    }
}

/// Handle inventory UI toggle
pub fn handle_inventory_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inventory_visible: ResMut<crate::resources::InventoryVisible>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        inventory_visible.0 = !inventory_visible.0;
    }
}

/// Marker component for NPCs (defined here to avoid circular dependency)
#[derive(Component)]
pub struct NpcMarker;

/// Handle interaction with NPCs and objects
pub fn handle_interaction_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<(Entity, &GridPosition), With<Player>>,
    npc_query: Query<(Entity, &GridPosition), (With<NpcMarker>, Without<Player>)>,
    mut dialogue_events: EventWriter<crate::events::DialogueRequested>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok((player_entity, player_pos)) = player_query.get_single() else {
        return;
    };

    // Check for adjacent NPCs
    for (npc_entity, npc_pos) in npc_query.iter() {
        let distance = ((player_pos.x - npc_pos.x).abs() + (player_pos.y - npc_pos.y).abs()) as f32;

        if distance <= 1.5 {
            dialogue_events.send(crate::events::DialogueRequested {
                npc_entity,
                player_entity,
            });
            break;
        }
    }
}
