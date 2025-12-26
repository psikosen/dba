use bevy::prelude::*;

#[derive(Event)]
pub struct ItemPickedUp {
    pub entity: Entity,
    pub item_id: String,
    pub quantity: u32,
}

#[derive(Event)]
pub struct SpiritOrbConsumed {
    pub entity: Entity,
    pub spirit_restored: f32,
    pub stamina_restored: f32,
}

#[derive(Event)]
pub struct CraftingRequested {
    pub recipe_id: String,
    pub crafter: Entity,
}
