//! # Prime Vessel System
//!
//! A dynamic enemy evolution mechanic driven by "Biological Chaos."
//!
//! ## Core Mechanics
//!
//! ### The Prime Vessel
//! A single roaming AI entity that actively traverses the open world, competing
//! with the player to locate and absorb randomly spawned Spirit resources.
//!
//! ### Metabolic Decay (800-day FIFO)
//! Any Spirit absorbed by the Prime Vessel is fully digested and removed from
//! its stat pool after 800 in-game days. This prevents the Vessel from resting
//! on ancient power - it must constantly hunt new Spirits.
//!
//! ### Shedding Mechanic (Lesser Selves)
//! Upon reaching evolution thresholds, the Prime Vessel "sheds" its current
//! physical form, leaving behind a "Lesser Self" - a static snapshot that
//! patrols as a permanent hazard.
//!
//! ### Global Corruption Index (Doomsday Clock)
//! Tracks the total free Spirits in the environment (~15,000 initial).
//! As spirits are consumed, the world's "Soul Alignment" shifts toward chaos.
//! Zero spirits = Total Collapse (all entities become hostile).
//!
//! ### Hidden Mechanic
//! The Corruption Index is hidden until completing "Purify Your Brother" quest,
//! revealing the player's role in the world's decay.

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

#[cfg(test)]
mod tests;

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub use components::*;
pub use events::*;
pub use resources::*;

/// Plugin that adds the Prime Vessel system to the game
pub struct PrimeVesselPlugin;

impl Plugin for PrimeVesselPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register resources
            .init_resource::<GlobalCorruptionIndex>()
            .init_resource::<PrimeVesselState>()
            .init_resource::<SpiritSpawnConfig>()
            .init_resource::<systems::DungeonVesselEncounters>()
            // Register events
            .add_event::<VesselAbsorbedSpirit>()
            .add_event::<VesselShedLesserSelf>()
            .add_event::<VesselEvolved>()
            .add_event::<VesselDecayProcessed>()
            .add_event::<VesselDefeated>()
            .add_event::<VesselResurrected>()
            .add_event::<CorruptionIndexChanged>()
            .add_event::<TotalCollapseTriggered>()
            .add_event::<CorruptionIndexRevealed>()
            .add_event::<WorldSpiritSpawned>()
            .add_event::<SpiritAbsorbed>()
            .add_event::<SpiritPurified>()
            .add_event::<SpiritFreed>()
            .add_event::<LesserSelfEncountered>()
            .add_event::<LesserSelfDefeated>()
            .add_event::<SoulAlignmentShifted>()
            .add_event::<EntityChaosLocked>()
            // Startup systems
            .add_systems(
                OnEnter(GameState::Playing),
                (systems::spawn_initial_spirits,),
            )
            // Vessel spawning and AI systems
            .add_systems(
                Update,
                (
                    systems::delayed_vessel_spawn,
                    systems::spawn_prime_vessel,
                    systems::update_vessel_behavior,
                    systems::move_vessel,
                    systems::check_player_encounter,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Spirit mechanics systems
            .add_systems(
                Update,
                (
                    systems::process_vessel_absorption,
                    systems::process_player_absorption,
                    systems::process_spirit_purification,
                    systems::free_trapped_spirits,
                    systems::manage_visible_spirits,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Metabolic and evolution systems
            .add_systems(
                Update,
                (
                    systems::process_metabolic_decay,
                    systems::check_tier_downgrade,
                    systems::check_evolution,
                    systems::process_shedding_recovery,
                    systems::update_lesser_self_patrol,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Corruption index systems
            .add_systems(
                Update,
                (
                    systems::update_soul_alignments,
                    systems::monitor_corruption_changes,
                    systems::check_total_collapse,
                    systems::check_ui_reveal,
                    systems::apply_corruption_world_effects,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Resurrection systems
            .add_systems(
                Update,
                (
                    systems::handle_vessel_defeat,
                    systems::check_resurrection_availability,
                    systems::process_resurrection_ritual,
                    systems::complete_resurrection,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Dungeon encounter systems
            .add_systems(
                Update,
                (
                    systems::check_prime_vessel_dungeon_spawn,
                    systems::spawn_lesser_selves_in_dungeon,
                    systems::move_vessel_to_dungeon,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Vessel combat systems
            .add_systems(
                Update,
                (
                    systems::initialize_vessel_health,
                    systems::initialize_lesser_self_health,
                    systems::check_vessel_defeat,
                    systems::check_lesser_self_defeat,
                    systems::apply_mutation_effects,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

/// Configuration for the Prime Vessel system
#[derive(Debug, Clone)]
pub struct PrimeVesselConfig {
    /// Initial spirit count in the world
    pub initial_spirits: u32,
    /// Days until absorbed spirits decay
    pub decay_days: u32,
    /// Maximum power for Lesser Selves
    pub lesser_self_power_cap: f32,
    /// Days until resurrection is available after defeat
    pub resurrection_delay_days: u32,
    /// Spirits required for resurrection ritual
    pub resurrection_cost: u32,
}

impl Default for PrimeVesselConfig {
    fn default() -> Self {
        Self {
            initial_spirits: 15_000,
            decay_days: 800,
            lesser_self_power_cap: 500.0,
            resurrection_delay_days: 100,
            resurrection_cost: 1000,
        }
    }
}
