pub mod components;
pub mod resources;
pub mod states;
pub mod systems;

#[cfg(test)]
mod tests;

use bevy::prelude::*;
use states::{CombatState, GameState, WorldState};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            // Register state machines
            .init_state::<GameState>()
            .init_state::<WorldState>()
            .init_state::<CombatState>()
            // Register common resources
            .init_resource::<resources::TimeOfDay>()
            .init_resource::<resources::PlayerLevel>()
            // Core systems
            .add_systems(Update, (
                systems::grid::update_grid_occupancy,
                systems::movement::process_movement_commands,
                systems::camera::follow_player,
                systems::animation::update_sprite_animations,
            ).run_if(in_state(GameState::Playing)));
    }
}
