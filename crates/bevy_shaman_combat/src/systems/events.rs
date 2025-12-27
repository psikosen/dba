use bevy::prelude::*;

// Re-export from core to avoid circular dependency
pub use bevy_shaman_core::events::HitLanded;

#[derive(Event)]
pub struct StatusEffectApplied {
    pub target: Entity,
    pub effect_type: crate::components::StatusEffectType,
}
