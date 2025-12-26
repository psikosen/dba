pub mod components;
pub mod resources;
pub mod systems;

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
            // Systems
            .add_systems(Update, (
                systems::corruption::spread_corruption,
                systems::purification::process_purification_casts,
                systems::tiles::update_tile_visuals,
                systems::progression::update_purification_range,
            ).run_if(in_state(GameState::Playing)))
            // Events
            .add_event::<systems::events::TilePurified>()
            .add_event::<systems::events::CorruptionSpread>();
    }
}
