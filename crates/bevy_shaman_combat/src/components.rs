use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Default)]
pub struct StatusEffects {
    pub effects: Vec<StatusEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: f32,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusEffectType {
    Burn,
    Poison,
    Stun,
    Slow,
    Purifying,
}

#[derive(Component)]
pub struct Attack {
    pub damage: f32,
    pub target: Entity,
    pub rhythm_quality: bevy_shaman_audio::resources::TimingQuality,
}
