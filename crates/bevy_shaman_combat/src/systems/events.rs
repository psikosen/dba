use bevy::prelude::*;

#[derive(Event)]
pub struct HitLanded {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: f32,
}

#[derive(Event)]
pub struct StatusEffectApplied {
    pub target: Entity,
    pub effect_type: crate::components::StatusEffectType,
}
