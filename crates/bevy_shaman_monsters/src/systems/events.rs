use crate::components::StateType;
use bevy::prelude::*;

#[derive(Event)]
pub struct MonsterStateChanged {
    pub entity: Entity,
    pub old_state: StateType,
    pub new_state: StateType,
}

#[derive(Event)]
pub struct MonsterCorrupted {
    pub entity: Entity,
    pub corruption_level: f32,
}
