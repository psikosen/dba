pub mod components;
pub mod systems;

use bevy::prelude::*;
use bevy_shaman_core::states::{CombatState, GameState};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            // Combat systems
            .add_systems(Update, (
                systems::hit_resolution::resolve_hits,
                systems::status_effects::apply_status_effects,
                systems::damage::apply_rhythm_based_damage,
            ).run_if(in_state(GameState::Playing)))
            // Events
            .add_event::<systems::events::HitLanded>()
            .add_event::<systems::events::StatusEffectApplied>();
    }
}
