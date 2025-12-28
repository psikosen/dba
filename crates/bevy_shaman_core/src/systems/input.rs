use crate::components::*;
use crate::settings::GameSettings;
use bevy::input::gamepad::{GamepadAxis, GamepadButton, GamepadConnection, GamepadEvent};
use bevy::prelude::*;

/// Resource to track connected gamepads
#[derive(Resource, Default)]
pub struct ConnectedGamepads {
    pub gamepads: Vec<Entity>,
}

/// System to handle gamepad connection/disconnection events
pub fn gamepad_connections(
    mut connection_events: EventReader<GamepadEvent>,
    mut connected_gamepads: ResMut<ConnectedGamepads>,
) {
    for event in connection_events.read() {
        match &event {
            GamepadEvent::Connection(connection_event) => match &connection_event.connection {
                GamepadConnection::Connected { name, .. } => {
                    info!(
                        "Gamepad connected: {:?} (ID: {:?})",
                        name, connection_event.gamepad
                    );
                    if !connected_gamepads
                        .gamepads
                        .contains(&connection_event.gamepad)
                    {
                        connected_gamepads.gamepads.push(connection_event.gamepad);
                    }
                }
                GamepadConnection::Disconnected => {
                    info!("Gamepad disconnected: {:?}", connection_event.gamepad);
                    connected_gamepads
                        .gamepads
                        .retain(|g| *g != connection_event.gamepad);
                }
            },
            _ => {}
        }
    }
}

/// Handle keyboard and gamepad input for player movement
pub fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    gamepad_button: Res<ButtonInput<GamepadButton>>,
    gamepad_axis: Res<Axis<GamepadAxis>>,
    connected_gamepads: Res<ConnectedGamepads>,
    settings: Res<GameSettings>,
    mut player: Query<(&mut MovementQueue, &Stamina), With<Player>>,
) {
    let Ok((mut movement_queue, stamina)) = player.get_single_mut() else {
        return;
    };

    // Only allow input if not already moving (prevent input buffering spam)
    if !movement_queue.commands.is_empty() {
        return;
    }

    let mut direction = IVec2::ZERO;
    let mut is_dashing = false;

    // Get movement keys based on control scheme
    let keys = settings.controls.scheme.movement_keys();

    // Handle keyboard input
    if keyboard.just_pressed(keys.up) {
        direction.y += 1;
    }
    if keyboard.just_pressed(keys.down) {
        direction.y -= 1;
    }
    if keyboard.just_pressed(keys.left) {
        direction.x -= 1;
    }
    if keyboard.just_pressed(keys.right) {
        direction.x += 1;
    }

    // Handle gamepad input if enabled
    if settings.controls.gamepad_enabled && !connected_gamepads.gamepads.is_empty() {
        let gamepad = connected_gamepads.gamepads[0];
        let deadzone = settings.controls.gamepad_deadzone as f32 / 100.0;

        // Left stick for movement
        let left_stick_x: f32 = gamepad_axis.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
        let left_stick_y: f32 = gamepad_axis.get(GamepadAxis::LeftStickY).unwrap_or(0.0);

        // D-pad for movement (discrete) - check all buttons
        if gamepad_button.just_pressed(GamepadButton::DPadUp) {
            direction.y += 1;
        }
        if gamepad_button.just_pressed(GamepadButton::DPadDown) {
            direction.y -= 1;
        }
        if gamepad_button.just_pressed(GamepadButton::DPadLeft) {
            direction.x -= 1;
        }
        if gamepad_button.just_pressed(GamepadButton::DPadRight) {
            direction.x += 1;
        }

        // Convert analog stick to discrete movement (with deadzone)
        if left_stick_x.abs() > deadzone || left_stick_y.abs() > deadzone {
            if left_stick_x < -deadzone {
                direction.x -= 1;
            } else if left_stick_x > deadzone {
                direction.x += 1;
            }

            if left_stick_y < -deadzone {
                direction.y -= 1;
            } else if left_stick_y > deadzone {
                direction.y += 1;
            }
        }

        // R1/RB or R2/RT for dash
        if gamepad_button.pressed(GamepadButton::RightTrigger)
            || gamepad_button.pressed(GamepadButton::RightTrigger2)
        {
            is_dashing = true;
        }
    }

    // Check for keyboard dash (shift key)
    if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
        is_dashing = true;
    }

    // Apply movement command if direction is valid
    if direction != IVec2::ZERO {
        // Normalize diagonal movement
        if direction.x != 0 && direction.y != 0 {
            direction.x = direction.x.signum();
            direction.y = direction.y.signum();
        }

        if is_dashing && stamina.current >= 10.0 {
            movement_queue
                .commands
                .push(MovementCommand::Dash(direction));
        } else {
            movement_queue
                .commands
                .push(MovementCommand::Move(direction));
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
