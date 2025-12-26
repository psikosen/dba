pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<resources::LootTableDB>()
            .init_resource::<resources::RecipeDB>()
            // Systems
            .add_systems(Update, (
                systems::pickup::process_item_pickups,
                systems::spirit_orb::consume_spirit_orbs,
                systems::inventory::manage_inventory,
                systems::crafting::process_crafting_requests,
            ).run_if(in_state(GameState::Playing)))
            // Events
            .add_event::<systems::events::ItemPickedUp>()
            .add_event::<systems::events::SpiritOrbConsumed>()
            .add_event::<systems::events::CraftingRequested>();
    }
}
