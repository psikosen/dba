pub mod components;
mod error;
pub mod resources;
pub mod systems;

pub use error::WorldGenerationError;

#[cfg(test)]
mod tests;

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<resources::PurificationAbility>()
            .init_resource::<resources::BossUnlockFlags>()
            .init_resource::<resources::WorldStateRulesDB>()
            .init_resource::<systems::generation::WorldGenerated>()
            .init_resource::<systems::generation::WorldGenConfig>()
            .init_resource::<systems::generation::WorldSeed>()
            // World generation (runs when entering Playing state)
            .add_systems(OnEnter(GameState::Playing), (
                systems::generation::generate_overworld,
            ))
            // Systems
            .add_systems(Update, (
                systems::corruption::spread_corruption,
                systems::purification::process_purification_casts,
                systems::tiles::update_tile_visuals,
                systems::progression::update_purification_range,
                systems::interactions::process_foraging,
                systems::interactions::spawn_forageable_spots,
                systems::interactions::process_digging,
                systems::interactions::spawn_dungeon_dig_spots,
            ).run_if(in_state(GameState::Playing)))
            // Events
            .add_event::<systems::events::TilePurified>()
            .add_event::<systems::events::CorruptionSpread>();
    }
}
