use bevy::prelude::*;
use bevy_shaman_core::components::{Health, Player, GridPosition};
use bevy_shaman_combat::components::StatusEffectType;
use rand::Rng;

use crate::components::{PrimeVessel, LesserSelf, VesselMutation, VesselRoamingState};
use crate::events::{VesselDefeated, LesserSelfDefeated};
use crate::resources::PrimeVesselState;

// ============================================================================
// VESSEL HEALTH & DAMAGE TRACKING
// ============================================================================

/// Initialize Health component for newly spawned Prime Vessels
pub fn initialize_vessel_health(
    mut commands: Commands,
    vessel_query: Query<(Entity, &PrimeVessel), Added<PrimeVessel>>,
) {
    for (entity, vessel) in vessel_query.iter() {
        // Health scales with evolution tier: Base 500 + (tier * 200)
        let max_health = 500.0 + (vessel.evolution_tier as f32 * 200.0);

        commands.entity(entity).insert(Health {
            current: max_health,
            max: max_health,
        });
    }
}

/// Initialize Health component for newly spawned Lesser Selves
pub fn initialize_lesser_self_health(
    mut commands: Commands,
    lesser_self_query: Query<(Entity, &LesserSelf), Added<LesserSelf>>,
) {
    for (entity, lesser_self) in lesser_self_query.iter() {
        // Lesser Self health based on origin tier: Base 300 + (tier * 100)
        let max_health = 300.0 + (lesser_self.origin_tier as f32 * 100.0);

        commands.entity(entity).insert(Health {
            current: max_health,
            max: max_health,
        });
    }
}

/// Handle vessel defeat when health reaches 0
pub fn check_vessel_defeat(
    mut commands: Commands,
    mut vessel_query: Query<(Entity, &mut PrimeVessel, &Health)>,
    mut vessel_state: ResMut<PrimeVesselState>,
    mut defeated_events: EventWriter<VesselDefeated>,
) {
    for (entity, mut vessel, health) in vessel_query.iter_mut() {
        if health.current <= 0.0 && !vessel.is_defeated {
            // Mark vessel as defeated
            vessel.is_defeated = true;
            vessel.is_active = false;
            vessel.roaming_state = VesselRoamingState::Dormant;

            // Fire defeat event
            defeated_events.send(VesselDefeated {
                defeated_by_player: true,
                final_tier: vessel.evolution_tier,
                final_power: vessel.power_level,
                lesser_selves_remaining: vessel.lesser_selves_shed,
            });

            info!(
                "Prime Vessel defeated! Tier: {}, Final Power: {:.0}",
                vessel.evolution_tier, vessel.power_level
            );
        }
    }
}

/// Handle Lesser Self defeat when health reaches 0
pub fn check_lesser_self_defeat(
    mut commands: Commands,
    mut lesser_self_query: Query<(Entity, &LesserSelf, &Health)>,
    mut defeated_events: EventWriter<LesserSelfDefeated>,
) {
    for (entity, lesser_self, health) in lesser_self_query.iter_mut() {
        if health.current <= 0.0 {
            // Calculate spirits freed based on power level
            let spirits_freed = (lesser_self.power_level / 10.0) as u32;

            // Fire defeat event (will trigger spirit spawning)
            defeated_events.send(LesserSelfDefeated {
                lesser_self_entity: entity,
                generation: lesser_self.generation,
                spirits_freed,
            });

            // Despawn the Lesser Self
            commands.entity(entity).despawn_recursive();

            info!(
                "Lesser Self (Gen {}) defeated! {} spirits freed.",
                lesser_self.generation, spirits_freed
            );
        }
    }
}

// ============================================================================
// VESSEL MUTATION COMBAT EFFECTS
// ============================================================================

/// Component marking active mutation effects on an entity
#[derive(Component, Debug, Clone)]
pub struct ActiveMutationEffects {
    pub effects: Vec<MutationEffect>,
}

#[derive(Debug, Clone)]
pub struct MutationEffect {
    pub mutation: VesselMutation,
    pub duration_remaining: f32,
    pub strength: f32,
}

/// Apply ongoing mutation effects during combat
pub fn apply_mutation_effects(
    mut vessel_query: Query<(&PrimeVessel, &mut Health, &GridPosition), Without<Player>>,
    mut lesser_self_query: Query<(&LesserSelf, &mut Health, &GridPosition), Without<Player>>,
    mut player_query: Query<(&mut Health, &GridPosition), With<Player>>,
    time: Res<Time>,
) {
    // Process Prime Vessel mutations
    for (vessel, mut health, vessel_pos) in vessel_query.iter_mut() {
        for mutation in get_tier_mutations(vessel.evolution_tier) {
            apply_mutation_passive_effect(
                &mutation,
                &mut health,
                vessel_pos,
                &mut player_query,
                time.delta_secs(),
            );
        }
    }

    // Process Lesser Self mutations
    for (lesser_self, mut health, pos) in lesser_self_query.iter_mut() {
        for mutation in &lesser_self.mutations {
            apply_mutation_passive_effect(
                mutation,
                &mut health,
                pos,
                &mut player_query,
                time.delta_secs(),
            );
        }
    }
}

/// Get mutations available at a given evolution tier
fn get_tier_mutations(tier: u8) -> Vec<VesselMutation> {
    let mut mutations = Vec::new();

    // Unlock mutations progressively with tier
    match tier {
        0 => {
            mutations.push(VesselMutation::SwiftMutation);
        }
        1..=2 => {
            mutations.push(VesselMutation::SwiftMutation);
            mutations.push(VesselMutation::VenomousStrike);
        }
        3..=4 => {
            mutations.push(VesselMutation::SwiftMutation);
            mutations.push(VesselMutation::VenomousStrike);
            mutations.push(VesselMutation::ChitinousArmor);
            mutations.push(VesselMutation::SpiritSense);
        }
        5..=6 => {
            mutations.push(VesselMutation::SwiftMutation);
            mutations.push(VesselMutation::VenomousStrike);
            mutations.push(VesselMutation::ChitinousArmor);
            mutations.push(VesselMutation::SpiritSense);
            mutations.push(VesselMutation::SpiritDrain);
            mutations.push(VesselMutation::RegenerativeFlesh);
        }
        7..=8 => {
            mutations.push(VesselMutation::SwiftMutation);
            mutations.push(VesselMutation::VenomousStrike);
            mutations.push(VesselMutation::ChitinousArmor);
            mutations.push(VesselMutation::SpiritSense);
            mutations.push(VesselMutation::SpiritDrain);
            mutations.push(VesselMutation::RegenerativeFlesh);
            mutations.push(VesselMutation::BlinkDash);
            mutations.push(VesselMutation::ChaosBurst);
            mutations.push(VesselMutation::FrenzyAura);
        }
        9..=10 => {
            // Prime Chaos - All mutations
            mutations.push(VesselMutation::VenomousStrike);
            mutations.push(VesselMutation::CorrosiveTouch);
            mutations.push(VesselMutation::SpiritDrain);
            mutations.push(VesselMutation::ChaosBurst);
            mutations.push(VesselMutation::SoulRend);
            mutations.push(VesselMutation::ChitinousArmor);
            mutations.push(VesselMutation::RegenerativeFlesh);
            mutations.push(VesselMutation::SpiritBarrier);
            mutations.push(VesselMutation::ChaosShield);
            mutations.push(VesselMutation::VoidSkin);
            mutations.push(VesselMutation::BlinkDash);
            mutations.push(VesselMutation::ShadowMeld);
            mutations.push(VesselMutation::TerrestrialPhase);
            mutations.push(VesselMutation::SwiftMutation);
            mutations.push(VesselMutation::SpiritSense);
            mutations.push(VesselMutation::FrenzyAura);
            mutations.push(VesselMutation::CorruptionWake);
            mutations.push(VesselMutation::MassAbsorption);
        }
        _ => {}
    }

    mutations
}

/// Apply passive effects from a mutation
fn apply_mutation_passive_effect(
    mutation: &VesselMutation,
    health: &mut Health,
    vessel_pos: &GridPosition,
    player_query: &mut Query<(&mut Health, &GridPosition), With<Player>>,
    delta: f32,
) {
    match mutation {
        VesselMutation::RegenerativeFlesh => {
            // Regenerate 1% max health per second
            let regen = health.max * 0.01 * delta;
            health.current = (health.current + regen).min(health.max);
        }

        VesselMutation::FrenzyAura => {
            // Damage nearby player (5 damage/sec within 3 tile radius)
            if let Ok((mut player_health, player_pos)) = player_query.get_single_mut() {
                let dx = (vessel_pos.x - player_pos.x).abs();
                let dy = (vessel_pos.y - player_pos.y).abs();
                if dx <= 3 && dy <= 3 {
                    let damage = 5.0 * delta;
                    player_health.current = (player_health.current - damage).max(0.0);
                }
            }
        }

        VesselMutation::SpiritBarrier => {
            // Passive damage reduction handled in damage calculation
        }

        VesselMutation::ChaosShield => {
            // Reflects damage - handled in hit resolution
        }

        // Other mutations are active abilities, not passive effects
        _ => {}
    }
}

/// Calculate damage reduction from defensive mutations
pub fn calculate_mutation_damage_reduction(mutations: &[VesselMutation]) -> f32 {
    let mut reduction = 0.0;

    for mutation in mutations {
        match mutation {
            VesselMutation::ChitinousArmor => reduction += 0.15,  // 15% reduction
            VesselMutation::SpiritBarrier => reduction += 0.20,   // 20% reduction
            VesselMutation::ChaosShield => reduction += 0.10,     // 10% reduction
            VesselMutation::VoidSkin => reduction += 0.25,        // 25% reduction
            _ => {}
        }
    }

    // Cap at 70% damage reduction
    reduction.min(0.70)
}

/// Calculate damage bonus from offensive mutations
pub fn calculate_mutation_damage_bonus(mutations: &[VesselMutation]) -> f32 {
    let mut bonus = 1.0;

    for mutation in mutations {
        match mutation {
            VesselMutation::VenomousStrike => bonus += 0.15,      // +15% damage
            VesselMutation::CorrosiveTouch => bonus += 0.20,      // +20% damage
            VesselMutation::SoulRend => bonus += 0.30,            // +30% damage
            VesselMutation::ChaosBurst => bonus += 0.25,          // +25% damage
            _ => {}
        }
    }

    bonus
}
