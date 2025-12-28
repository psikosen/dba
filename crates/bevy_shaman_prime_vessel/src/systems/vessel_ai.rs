use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, Player};
use rand::Rng;

use crate::components::{PrimeVessel, VesselMutation, VesselRoamingState, WorldSpirit};
use crate::resources::PrimeVesselState;

/// Primary AI behavior system for the Prime Vessel
pub fn update_vessel_behavior(
    mut vessel_query: Query<(Entity, &mut PrimeVessel, &GridPosition)>,
    spirit_query: Query<(Entity, &WorldSpirit, &GridPosition), Without<PrimeVessel>>,
    player_query: Query<&GridPosition, With<Player>>,
    time: Res<Time>,
) {
    for (vessel_entity, mut vessel, vessel_pos) in vessel_query.iter_mut() {
        if !vessel.is_active || vessel.is_defeated {
            continue;
        }

        match vessel.roaming_state {
            VesselRoamingState::Wandering => {
                // Look for nearby spirits to hunt
                if let Some((target, _, _)) =
                    find_nearest_spirit(vessel_pos, &spirit_query, &vessel)
                {
                    vessel.current_target = Some(target);
                    vessel.roaming_state = VesselRoamingState::Hunting;
                }
            }
            VesselRoamingState::Hunting => {
                // Verify target still exists
                if let Some(target) = vessel.current_target {
                    if spirit_query.get(target).is_err() {
                        vessel.current_target = None;
                        vessel.roaming_state = VesselRoamingState::Wandering;
                    }
                } else {
                    vessel.roaming_state = VesselRoamingState::Wandering;
                }
            }
            VesselRoamingState::Absorbing => {
                // Handled by absorption system
            }
            VesselRoamingState::Shedding | VesselRoamingState::Recovering => {
                // Handled by shedding system
            }
            VesselRoamingState::Combat => {
                // Check if player still nearby
                if let Ok(player_pos) = player_query.get_single() {
                    let dx = (vessel_pos.x - player_pos.x).abs();
                    let dy = (vessel_pos.y - player_pos.y).abs();
                    if dx > 10 || dy > 10 {
                        // Player fled, return to hunting
                        vessel.roaming_state = VesselRoamingState::Wandering;
                    }
                }
            }
            VesselRoamingState::Dormant => {
                // Do nothing - awaiting resurrection
            }
        }
    }
}

/// Find the nearest absorbable spirit
fn find_nearest_spirit(
    vessel_pos: &GridPosition,
    spirits: &Query<(Entity, &WorldSpirit, &GridPosition), Without<PrimeVessel>>,
    vessel: &PrimeVessel,
) -> Option<(Entity, f32, f32)> {
    let mut nearest: Option<(Entity, f32, f32)> = None;

    // Base detection range, increased by SpiritSense mutation
    let mut detection_range = 50;
    // Note: In full implementation, we'd check vessel mutations here

    for (entity, spirit, pos) in spirits.iter() {
        // Skip purified spirits
        if spirit.is_purified || spirit.being_absorbed {
            continue;
        }

        let dx = (vessel_pos.x - pos.x).abs();
        let dy = (vessel_pos.y - pos.y).abs();

        if dx <= detection_range && dy <= detection_range {
            let dist = ((dx * dx + dy * dy) as f32).sqrt();
            if nearest.is_none() || dist < nearest.unwrap().1 {
                nearest = Some((entity, dist, spirit.power));
            }
        }
    }

    nearest
}

/// Move the Prime Vessel towards its target
pub fn move_vessel(
    mut vessel_query: Query<(&mut PrimeVessel, &mut GridPosition)>,
    spirit_query: Query<&GridPosition, (With<WorldSpirit>, Without<PrimeVessel>)>,
    time: Res<Time>,
    mut move_timer: Local<f32>,
) {
    // Movement cooldown tracking
    const MOVE_INTERVAL: f32 = 0.2; // Move every 0.2 seconds

    *move_timer += time.delta_secs();
    if *move_timer < MOVE_INTERVAL {
        return;
    }
    *move_timer = 0.0;

    for (mut vessel, mut vessel_pos) in vessel_query.iter_mut() {
        if !vessel.is_active {
            continue;
        }

        match vessel.roaming_state {
            VesselRoamingState::Hunting => {
                if let Some(target) = vessel.current_target {
                    if let Ok(target_pos) = spirit_query.get(target) {
                        // Move towards target
                        move_towards(&mut vessel_pos, target_pos);

                        // Check if close enough to absorb
                        let dx = (vessel_pos.x - target_pos.x).abs();
                        let dy = (vessel_pos.y - target_pos.y).abs();
                        if dx <= 2 && dy <= 2 {
                            vessel.roaming_state = VesselRoamingState::Absorbing;
                        }
                    }
                }
            }
            VesselRoamingState::Wandering => {
                // Random wander
                wander_randomly(&mut vessel_pos);
            }
            _ => {}
        }
    }
}

/// Move position one step towards target
fn move_towards(pos: &mut GridPosition, target: &GridPosition) {
    // Move one tile towards target
    if pos.x < target.x {
        pos.x += 1;
    } else if pos.x > target.x {
        pos.x -= 1;
    }

    if pos.y < target.y {
        pos.y += 1;
    } else if pos.y > target.y {
        pos.y -= 1;
    }
}

/// Random wander movement
fn wander_randomly(pos: &mut GridPosition) {
    let mut rng = rand::thread_rng();

    // 50% chance to move each axis
    if rng.gen_bool(0.5) {
        pos.x += if rng.gen_bool(0.5) { 1 } else { -1 };
    }
    if rng.gen_bool(0.5) {
        pos.y += if rng.gen_bool(0.5) { 1 } else { -1 };
    }
}

/// Check for player encounter (combat trigger)
pub fn check_player_encounter(
    mut vessel_query: Query<(&mut PrimeVessel, &GridPosition)>,
    player_query: Query<&GridPosition, With<Player>>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        for (mut vessel, vessel_pos) in vessel_query.iter_mut() {
            if !vessel.is_active || vessel.roaming_state == VesselRoamingState::Combat {
                continue;
            }

            let dx = (vessel_pos.x - player_pos.x).abs();
            let dy = (vessel_pos.y - player_pos.y).abs();

            // Aggro range of 8 tiles
            if dx <= 8 && dy <= 8 {
                vessel.roaming_state = VesselRoamingState::Combat;
                warn!("Prime Vessel {} has engaged the player!", vessel.title());
            }
        }
    }
}

/// Spawn the Prime Vessel into the world
pub fn spawn_prime_vessel(
    mut commands: Commands,
    mut vessel_state: ResMut<PrimeVesselState>,
    query: Query<Entity, With<PrimeVessel>>,
) {
    // Only spawn if not already spawned
    if vessel_state.has_spawned || !query.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();

    // Spawn at a random position in the world
    let spawn_x = rng.gen_range(-200..200);
    let spawn_y = rng.gen_range(-200..200);

    let vessel_entity = commands
        .spawn((
            PrimeVessel::default(),
            crate::components::MetabolicHistory::default(),
            GridPosition {
                x: spawn_x,
                y: spawn_y,
            },
            bevy_shaman_monsters::components::MonsterStats {
                attack: 50.0,
                defense: 30.0,
                speed: 8.0,
                spirit_affinity: 1.0,
            },
            bevy_shaman_monsters::components::MonsterState {
                state: bevy_shaman_monsters::components::StateType::Chaos,
                stability_meter: 0.1,
                corruption_meter: 0.8,
                obedience_meter: 0.0,
                chaos_output: 2.0,
            },
            crate::components::SoulAlignment {
                alignment: 0.8,
                aggression_modifier: 2.0,
                chaos_locked: false,
            },
        ))
        .id();

    vessel_state.spawn_vessel(vessel_entity);

    info!(
        "The Prime Vessel has manifested at ({}, {})!",
        spawn_x, spawn_y
    );
}
