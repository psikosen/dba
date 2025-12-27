pub mod components;
pub mod events;
pub mod resources;
pub mod settings;
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
            .init_resource::<resources::GridOccupancy>()
            .init_resource::<resources::InventoryVisible>()
            .init_resource::<resources::DialogueVisible>()
            .init_resource::<settings::GameSettings>()
            .init_resource::<systems::player::PlayerSpawned>()
            .init_resource::<systems::assets::AssetLoadingState>()
            // Register events
            .add_event::<events::DialogueRequested>()
            .add_event::<events::PlayerDied>()
            .add_event::<events::EntityDied>()
            // Asset loading (runs in Boot state)
            .add_systems(Startup, (
                systems::assets::load_game_assets,
            ))
            // Player and camera spawning (runs once when entering Playing state)
            .add_systems(OnEnter(GameState::Playing), (
                systems::player::spawn_player,
                systems::player::spawn_camera,
            ))
            // Core gameplay systems
            .add_systems(Update, (
                systems::input::handle_player_input,
                systems::input::handle_inventory_toggle,
                systems::input::handle_interaction_input,
                systems::grid::update_grid_occupancy,
                systems::movement::process_movement_commands,
                systems::camera::follow_player,
                systems::animation::update_sprite_animations,
            ).run_if(in_state(GameState::Playing)));
    }
}
