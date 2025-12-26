use bevy::prelude::*;

/// Marks an entity as responding to rhythm input
#[derive(Component)]
pub struct RhythmResponder {
    pub enabled: bool,
}

impl Default for RhythmResponder {
    fn default() -> Self {
        Self { enabled: true }
    }
}
