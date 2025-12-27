pub mod components;
pub mod systems;

#[cfg(test)]
mod tests;

use bevy::prelude::*;
use bevy_shaman_core::states::{CombatState, GameState};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        let mut app = app;

        // Core combat systems
        app.add_systems(Update, (
                systems::hit_resolution::resolve_hits,
                systems::death::handle_entity_deaths,
                systems::death::cleanup_dead_entities,
                systems::status_effects::apply_status_effects,
            ).run_if(in_state(GameState::Playing)));

        // Rhythm-based damage (requires audio feature)
        #[cfg(feature = "audio")]
        app.add_systems(Update, systems::damage::apply_rhythm_based_damage.run_if(in_state(GameState::Playing)));

        app
            // Weapon systems
            .add_systems(Update, (
                systems::weapon::weapon_attack_system,
                systems::weapon::mambele_poison_system,
                systems::weapon::weapon_durability_system,
            ).run_if(in_state(GameState::Playing)))
            // Blood lust systems
            .add_systems(Update, (
                systems::blood_lust::blood_lust_combat_gain,
                systems::blood_lust::blood_lust_decay,
                systems::blood_lust::blood_lust_corruption_spread,
                systems::blood_lust::blood_lust_reduction_from_items,
            ).run_if(in_state(GameState::Playing)))
            // Combat wheel systems
            .add_systems(Update, (
                systems::wheel::combat_wheel_trigger,
                systems::wheel::apply_wheel_outcome,
            ).run_if(in_state(GameState::Playing)))
            // Monster control systems
            .add_systems(Update, (
                systems::monster_control::monster_control_attempt,
                systems::monster_control::monster_control_update,
                systems::monster_control::monster_control_release,
                systems::monster_control::controlled_monster_abilities,
            ).run_if(in_state(GameState::Playing)))
            // Rhythm combo systems
            .add_systems(Update, (
                systems::rhythm_combo::rhythm_combo_tracking,
                systems::rhythm_combo::rhythm_combo_specials,
                systems::rhythm_combo::apply_special_moves,
                systems::rhythm_combo::rhythm_combo_reset,
            ).run_if(in_state(GameState::Playing)))
            // Events
            .add_event::<systems::events::HitLanded>()
            .add_event::<systems::events::StatusEffectApplied>()
            .add_event::<systems::wheel::WheelTriggered>()
            .add_event::<systems::monster_control::AttemptMonsterControl>()
            .add_event::<systems::monster_control::ReleaseMonsterControl>()
            .add_event::<systems::rhythm_combo::SpecialMoveExecuted>();
    }
}
