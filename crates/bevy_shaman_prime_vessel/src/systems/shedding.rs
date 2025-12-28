use bevy::prelude::*;
use bevy_shaman_core::components::GridPosition;
use bevy_shaman_core::resources::GameCalendar;
use bevy_shaman_monsters::components::{
    AiBehavior, AiState, MonsterState, MonsterStats, StateType,
};
use rand::Rng;

use crate::components::{
    LesserSelf, MetabolicHistory, PrimeVessel, SoulAlignment, VesselMutation, VesselRoamingState,
};
use crate::events::VesselEvolved;
use crate::events::VesselShedLesserSelf;
use crate::resources::PrimeVesselState;

/// Check if the Prime Vessel should shed a Lesser Self and evolve
pub fn check_evolution(
    mut query: Query<(&mut PrimeVessel, &GridPosition, &MetabolicHistory)>,
    mut vessel_state: ResMut<PrimeVesselState>,
    calendar: Res<GameCalendar>,
    mut commands: Commands,
    mut shed_events: EventWriter<VesselShedLesserSelf>,
    mut evolve_events: EventWriter<VesselEvolved>,
) {
    for (mut vessel, position, history) in query.iter_mut() {
        // Skip if not active or already shedding
        if !vessel.is_active || vessel.roaming_state == VesselRoamingState::Shedding {
            continue;
        }

        // Check evolution threshold
        if vessel.should_evolve() {
            // Begin shedding process
            vessel.roaming_state = VesselRoamingState::Shedding;

            // Determine mutations for this evolution
            let new_mutation = roll_mutation(vessel.evolution_tier, history.spirit_count());
            let inherited_mutations = collect_mutations(vessel.evolution_tier, new_mutation);

            // Create the Lesser Self
            let lesser_self_power = vessel.power_level.min(LesserSelf::MAX_POWER);
            let lesser_self = LesserSelf::from_prime_vessel(
                vessel.evolution_tier,
                lesser_self_power,
                vessel_state.total_lesser_selves + 1,
                calendar.current_day(),
                inherited_mutations.clone(),
                (position.x, position.y),
            );

            // Spawn the Lesser Self entity
            let lesser_self_entity =
                spawn_lesser_self(&mut commands, lesser_self.clone(), position);

            // Update vessel state
            vessel_state.add_lesser_self(lesser_self_entity);
            vessel.lesser_selves_shed += 1;

            // Fire shedding event
            shed_events.send(VesselShedLesserSelf {
                lesser_self_entity,
                origin_tier: vessel.evolution_tier,
                power_level: lesser_self.power_level,
                mutations: inherited_mutations,
                position: (position.x, position.y),
            });

            // Evolve the Prime Vessel
            vessel.evolution_tier += 1;
            vessel.max_power_reached = vessel.max_power_reached.max(vessel.power_level);

            // Fire evolution event
            evolve_events.send(VesselEvolved {
                new_tier: vessel.evolution_tier,
                new_power: vessel.power_level,
                new_mutation,
            });

            info!(
                "Prime Vessel evolved to tier {} ({}) - shed Lesser Self Gen {}",
                vessel.evolution_tier,
                vessel.title(),
                vessel_state.total_lesser_selves
            );
        }
    }
}

/// Handle the recovery state after shedding
pub fn process_shedding_recovery(mut query: Query<&mut PrimeVessel>, time: Res<Time>) {
    // Track recovery time (5 seconds of recovery after shedding)
    const RECOVERY_TIME: f32 = 5.0;

    for mut vessel in query.iter_mut() {
        if vessel.roaming_state == VesselRoamingState::Shedding {
            // Transition to recovery (immediate in this simplified version)
            vessel.roaming_state = VesselRoamingState::Recovering;
        } else if vessel.roaming_state == VesselRoamingState::Recovering {
            // After recovery, return to wandering
            // In a full implementation, this would use a timer
            vessel.roaming_state = VesselRoamingState::Wandering;
        }
    }
}

/// Roll for a new mutation based on evolution tier
fn roll_mutation(tier: u8, spirit_count: usize) -> Option<VesselMutation> {
    let mut rng = rand::thread_rng();

    // Higher tiers and more absorbed spirits increase mutation chance
    let mutation_chance = (tier as f32 * 0.1) + (spirit_count as f32 * 0.001);
    if rng.gen::<f32>() > mutation_chance.min(0.8) {
        return None;
    }

    // Roll for mutation type based on tier
    let roll: u8 = rng.gen_range(0..=15);
    Some(match roll {
        0 => VesselMutation::VenomousStrike,
        1 => VesselMutation::CorrosiveTouch,
        2 => VesselMutation::SpiritDrain,
        3 => VesselMutation::ChaosBurst,
        4 => VesselMutation::SoulRend,
        5 => VesselMutation::ChitinousArmor,
        6 => VesselMutation::RegenerativeFlesh,
        7 => VesselMutation::SpiritBarrier,
        8 => VesselMutation::ChaosShield,
        9 => VesselMutation::VoidSkin,
        10 => VesselMutation::BlinkDash,
        11 => VesselMutation::ShadowMeld,
        12 => VesselMutation::TerrestrialPhase,
        13 => VesselMutation::SwiftMutation,
        14 => VesselMutation::SpiritSense,
        15 => VesselMutation::FrenzyAura,
        _ => VesselMutation::CorruptionWake,
    })
}

/// Collect mutations for a Lesser Self based on current tier
fn collect_mutations(tier: u8, new_mutation: Option<VesselMutation>) -> Vec<VesselMutation> {
    let mut mutations = Vec::new();
    let mut rng = rand::thread_rng();

    // Each tier adds a chance for a mutation
    for _ in 0..tier {
        if let Some(m) = roll_mutation(tier, rng.gen_range(10..100)) {
            if !mutations.contains(&m) {
                mutations.push(m);
            }
        }
    }

    // Add the new mutation if present
    if let Some(m) = new_mutation {
        if !mutations.contains(&m) {
            mutations.push(m);
        }
    }

    mutations
}

/// Spawn a Lesser Self entity
fn spawn_lesser_self(
    commands: &mut Commands,
    lesser_self: LesserSelf,
    position: &GridPosition,
) -> Entity {
    // Calculate stats based on power level and mutations
    let base_attack = 10.0 + (lesser_self.power_level * 0.1);
    let base_defense = 5.0 + (lesser_self.power_level * 0.05);

    // Apply mutation modifiers
    let mut attack = base_attack;
    let mut defense = base_defense;
    let mut speed = 5.0;

    for mutation in &lesser_self.mutations {
        match mutation {
            VesselMutation::ChitinousArmor => defense *= 1.3,
            VesselMutation::VenomousStrike => attack *= 1.2,
            VesselMutation::SwiftMutation => speed *= 1.5,
            VesselMutation::RegenerativeFlesh => defense *= 1.2,
            _ => {}
        }
    }

    commands
        .spawn((
            lesser_self,
            GridPosition {
                x: position.x + 1, // Spawn slightly offset
                y: position.y,
            },
            MonsterStats {
                attack,
                defense,
                speed,
                spirit_affinity: 0.8,
            },
            MonsterState {
                state: StateType::Chaos,
                stability_meter: 0.2,
                corruption_meter: 0.7,
                obedience_meter: 0.0,
                chaos_output: 1.5,
            },
            AiBehavior {
                behavior_tree_id: "lesser_self_patrol".to_string(),
                aggression: 0.8,
                flee_threshold: 0.0, // Never flee
            },
            AiState::Patrol,
            SoulAlignment {
                alignment: 0.7,
                aggression_modifier: 1.5,
                chaos_locked: false,
            },
        ))
        .id()
}

/// Update Lesser Self patrol behavior
pub fn update_lesser_self_patrol(
    mut query: Query<(&mut LesserSelf, &mut GridPosition), Without<PrimeVessel>>,
    time: Res<Time>,
) {
    for (mut lesser_self, mut position) in query.iter_mut() {
        if lesser_self.patrol_route.is_empty() {
            continue;
        }

        // Get target position
        let target = lesser_self.patrol_route[lesser_self.patrol_index];

        // Move towards target (simplified movement)
        if position.x < target.0 {
            position.x += 1;
        } else if position.x > target.0 {
            position.x -= 1;
        }

        if position.y < target.1 {
            position.y += 1;
        } else if position.y > target.1 {
            position.y -= 1;
        }

        // Check if reached target
        if position.x == target.0 && position.y == target.1 {
            lesser_self.patrol_index =
                (lesser_self.patrol_index + 1) % lesser_self.patrol_route.len();
        }
    }
}
