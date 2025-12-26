use bevy::prelude::*;

#[derive(Event)]
pub struct NpcWokenUp {
    pub npc_entity: Entity,
    pub npc_name: String,
}

#[derive(Event)]
pub struct InstrumentChosen {
    pub instrument: crate::resources::InstrumentChoice,
}

#[derive(Event)]
pub struct BrotherFightCompleted {
    pub fight_number: u8,
    pub corruption_remaining: f32,
}
