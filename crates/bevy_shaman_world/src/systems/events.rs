use bevy::prelude::*;

#[derive(Event)]
pub struct TilePurified {
    pub tile_entity: Entity,
    pub corruption_removed: f32,
}

#[derive(Event)]
pub struct CorruptionSpread {
    pub from_tile: Entity,
    pub to_tile: Entity,
    pub amount: f32,
}
