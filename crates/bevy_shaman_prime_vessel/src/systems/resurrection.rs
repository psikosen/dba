use bevy::prelude::*;
use bevy_shaman_core::components::GridPosition;
use bevy_shaman_core::resources::GameCalendar;
use rand::Rng;

use crate::components::{
    MetabolicHistory, PrimeVessel, ResurrectionRitual, SoulAlignment, VesselRoamingState,
};
use crate::events::{VesselDefeated, VesselResurrected};
use crate::resources::PrimeVesselState;

/// Handle Prime Vessel defeat
pub fn handle_vessel_defeat(
    mut vessel_query: Query<(Entity, &mut PrimeVessel, &GridPosition)>,
    mut vessel_state: ResMut<PrimeVesselState>,
    calendar: Res<GameCalendar>,
    mut defeat_events: EventWriter<VesselDefeated>,
    mut commands: Commands,
) {
    for (entity, mut vessel, position) in vessel_query.iter_mut() {
        // Check if vessel should be defeated (health system would trigger this)
        // For now, this is just the handler when defeat is signaled
        if vessel.is_defeated && vessel.roaming_state != VesselRoamingState::Dormant {
            vessel.roaming_state = VesselRoamingState::Dormant;
            vessel.is_active = false;

            vessel_state.defeat_vessel(calendar.current_day());

            defeat_events.send(VesselDefeated {
                defeated_by_player: true,
                final_tier: vessel.evolution_tier,
                final_power: vessel.power_level,
                lesser_selves_remaining: vessel_state.lesser_selves.len() as u32,
            });

            // Spawn resurrection altar at vessel's death location
            commands.spawn((
                ResurrectionRitual::default(),
                GridPosition {
                    x: position.x,
                    y: position.y,
                },
                ResurrectionAltar,
            ));

            info!(
                "Prime Vessel DEFEATED at tier {} with {:.0} power! \
                 {} Lesser Selves remain in the world. \
                 A dark altar has formed where it fell...",
                vessel.evolution_tier,
                vessel.power_level,
                vessel_state.lesser_selves.len()
            );
        }
    }
}

/// Marker component for the resurrection altar
#[derive(Component)]
pub struct ResurrectionAltar;

/// Check if resurrection becomes available (100 days after defeat)
pub fn check_resurrection_availability(
    mut vessel_state: ResMut<PrimeVesselState>,
    calendar: Res<GameCalendar>,
) {
    if !vessel_state.vessel_defeated || vessel_state.resurrection_available {
        return;
    }

    if let Some(defeat_day) = vessel_state.defeat_day {
        const DAYS_UNTIL_RESURRECTION: u32 = 100;

        if calendar.current_day() >= defeat_day + DAYS_UNTIL_RESURRECTION {
            vessel_state.enable_resurrection();
            warn!(
                "The altar pulses with dark energy... \
                 The Prime Vessel's resurrection ritual is now available."
            );
        }
    }
}

/// Handle resurrection ritual interaction
pub fn process_resurrection_ritual(
    mut altar_query: Query<(&mut ResurrectionRitual, &GridPosition), With<ResurrectionAltar>>,
    player_query: Query<&GridPosition, With<bevy_shaman_core::components::Player>>,
    vessel_state: Res<PrimeVesselState>,
    input: Res<ButtonInput<KeyCode>>,
    mut spirit_offering_state: Local<f32>,
) {
    if !vessel_state.resurrection_available {
        return;
    }

    // Player must be near altar and press R to interact
    if !input.just_pressed(KeyCode::KeyR) {
        return;
    }

    if let Ok(player_pos) = player_query.get_single() {
        for (mut ritual, altar_pos) in altar_query.iter_mut() {
            let dx = (player_pos.x - altar_pos.x).abs();
            let dy = (player_pos.y - altar_pos.y).abs();

            if dx <= 3 && dy <= 3 {
                // Player is offering spirits to the altar
                // In full implementation, this would consume player's spirit orbs
                let offering = 50.0; // Spirit orbs consumed
                ritual.offer_spirit(offering);

                info!(
                    "Offered {:.0} spirit power to the altar. Progress: {:.1}%",
                    offering,
                    ritual.progress_percentage() * 100.0
                );

                if ritual.is_complete {
                    info!("The resurrection ritual is complete! The darkness stirs...");
                }
            }
        }
    }
}

/// Complete resurrection and respawn the Prime Vessel
pub fn complete_resurrection(
    mut commands: Commands,
    mut altar_query: Query<(Entity, &ResurrectionRitual, &GridPosition), With<ResurrectionAltar>>,
    mut vessel_state: ResMut<PrimeVesselState>,
    mut resurrect_events: EventWriter<VesselResurrected>,
) {
    for (altar_entity, ritual, position) in altar_query.iter() {
        if !ritual.is_complete {
            continue;
        }

        // Remove the altar
        commands.entity(altar_entity).despawn();

        // Spawn new Prime Vessel at altar location
        let mut rng = rand::thread_rng();

        // New vessel starts at tier 0 but with some bonus power from the ritual
        let starting_power = 200.0 + (ritual.spirits_offered as f32 * 0.1);

        let vessel_entity = commands
            .spawn((
                PrimeVessel {
                    power_level: starting_power,
                    max_power_reached: starting_power,
                    evolution_tier: 0,
                    lesser_selves_shed: 0,
                    is_active: true,
                    is_defeated: false,
                    current_target: None,
                    roaming_state: VesselRoamingState::Wandering,
                },
                MetabolicHistory::default(),
                GridPosition {
                    x: position.x,
                    y: position.y,
                },
                bevy_shaman_monsters::components::MonsterStats {
                    attack: 60.0, // Slightly stronger on resurrection
                    defense: 35.0,
                    speed: 9.0,
                    spirit_affinity: 1.0,
                },
                bevy_shaman_monsters::components::MonsterState {
                    state: bevy_shaman_monsters::components::StateType::Chaos,
                    stability_meter: 0.05,
                    corruption_meter: 0.9,
                    obedience_meter: 0.0,
                    chaos_output: 2.5,
                },
                SoulAlignment {
                    alignment: 0.9,
                    aggression_modifier: 2.5,
                    chaos_locked: false,
                },
            ))
            .id();

        vessel_state.resurrect_vessel(vessel_entity);

        resurrect_events.send(VesselResurrected {
            starting_power,
            starting_tier: 0,
        });

        error!(
            "THE PRIME VESSEL HAS RISEN! \
             Born anew from the ritual offerings, it hungers once more..."
        );
    }
}

/// Spawn initial vessel with delay after game start
pub fn delayed_vessel_spawn(
    vessel_state: Res<PrimeVesselState>,
    calendar: Res<GameCalendar>,
    mut spawn_triggered: Local<bool>,
) {
    // Wait 30 days before spawning the vessel (give player time to learn)
    const SPAWN_DELAY_DAYS: u32 = 30;

    if *spawn_triggered || vessel_state.has_spawned {
        return;
    }

    if calendar.current_day() >= SPAWN_DELAY_DAYS {
        *spawn_triggered = true;
        // The spawn_prime_vessel system will handle actual spawning
        info!("The stars align... Something ancient awakens in the spirit realm.");
    }
}
