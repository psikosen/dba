use bevy::prelude::*;
use crate::components::Inventory;
use crate::resources::{ItemDB, RecipeDB};
use crate::systems::events::CraftingRequested;

/// Processes crafting requests
pub fn process_crafting_requests(
    mut craft_events: EventReader<CraftingRequested>,
    recipe_db: Res<RecipeDB>,
    item_db: Res<ItemDB>,
    mut crafters: Query<&mut Inventory>,
) {
    for event in craft_events.read() {
        let Some(recipe) = recipe_db.get(&event.recipe_id) else {
            warn!("Recipe not found: {}", event.recipe_id);
            continue;
        };

        let Ok(mut inventory) = crafters.get_mut(event.crafter) else {
            continue;
        };

        // Check if player has ingredients
        let has_ingredients = recipe
            .ingredients
            .iter()
            .all(|ing| inventory.count_item(&ing.item_id) >= ing.quantity);

        if !has_ingredients {
            warn!("Insufficient ingredients for recipe: {}", event.recipe_id);
            continue;
        }

        // Remove ingredients
        for ingredient in &recipe.ingredients {
            inventory.remove_item(&ingredient.item_id, ingredient.quantity);
        }

        // Add output item
        if let Some(output_item) = item_db.get(&recipe.output_item_id) {
            inventory.add_item(output_item.clone(), recipe.output_quantity);
            info!("Crafted: {} x{}", output_item.display_name, recipe.output_quantity);
        }
    }
}
