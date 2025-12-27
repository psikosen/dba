pub mod components;
pub mod resources;
pub mod systems;

#[cfg(test)]
mod tests;

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
                systems::inventory::pickup_items,
                systems::inventory::use_items,
                systems::inventory::update_active_effects,
                systems::inventory::drop_items,
                systems::crafting::process_crafting_requests,
                // Plant and food systems
                systems::plant_food::plant_usage,
                systems::plant_food::food_usage,
                systems::plant_food::active_effects_update,
                systems::plant_food::apply_active_effects,
            ).run_if(in_state(GameState::Playing)))
            // Events
            .add_event::<systems::events::ItemPickedUp>()
            .add_event::<systems::events::SpiritOrbConsumed>()
            .add_event::<systems::events::CraftingRequested>()
            .add_event::<systems::inventory::ItemPickedUp>()
            .add_event::<systems::inventory::ItemUsed>()
            .add_event::<systems::inventory::ItemDropped>()
            .add_event::<systems::plant_food::UsePlant>()
            .add_event::<systems::plant_food::UseFood>()
            .add_event::<systems::plant_food::ReduceBloodLust>();
    }
}
