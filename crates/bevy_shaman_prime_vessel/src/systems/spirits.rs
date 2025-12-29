use bevy::prelude::*;
use bevy_shaman_core::components::GridPosition;
use bevy_shaman_core::resources::GameCalendar;
use bevy_shaman_world::components::BiomeType;
use rand::Rng;

use crate::components::{MetabolicHistory, PrimeVessel, VisibleSpirit, WorldSpirit, WorldSpiritType};
use crate::events::{
    SpiritAbsorbed, SpiritFreed, SpiritPurified, VesselAbsorbedSpirit, WorldSpiritSpawned,
};
use crate::resources::{GlobalCorruptionIndex, SpiritSpawnConfig};

/// Spawn the initial population of world spirits (15,000)
pub fn spawn_initial_spirits(
    mut commands: Commands,
    mut spawn_config: ResMut<SpiritSpawnConfig>,
    corruption_index: Res<GlobalCorruptionIndex>,
    mut spawn_events: EventWriter<WorldSpiritSpawned>,
) {
    // Only spawn once
    if spawn_config.initial_population_spawned {
        return;
    }
    spawn_config.initial_population_spawned = true;

    let initial_count = corruption_index.initial_spirit_count;
    let mut rng = rand::thread_rng();

    info!("Spawning {} world spirits...", initial_count);

    // Define spawn regions across biomes
    let biome_regions = [
        (BiomeType::Jungle, (-500, -500), (0, 0)),
        (BiomeType::Desert, (0, -500), (500, 0)),
        (BiomeType::Forest, (-500, 0), (0, 500)),
        (BiomeType::Safari, (0, 0), (500, 500)),
        (BiomeType::DeadRealm, (-250, -250), (250, 250)),
    ];

    let spirits_per_biome = initial_count / biome_regions.len() as u32;

    for (biome, (min_x, min_y), (max_x, max_y)) in biome_regions.iter() {
        for _ in 0..spirits_per_biome {
            let x = rng.gen_range(*min_x..*max_x);
            let y = rng.gen_range(*min_y..*max_y);

            // Roll spirit type
            let spirit_type = roll_spirit_type(&spawn_config, &mut rng);

            // Calculate power based on type
            let base_power = rng
                .gen_range(spawn_config.neutral_power_range.0..spawn_config.neutral_power_range.1);
            let power = base_power * spirit_type.power_multiplier();

            let entity = commands
                .spawn((WorldSpirit::new(power, spirit_type), GridPosition { x, y }))
                .id();

            spawn_events.send(WorldSpiritSpawned {
                entity,
                spirit_type,
                power,
                position: (x, y),
            });
        }
    }

    info!("World spirit population initialized.");
}

/// Roll for spirit type based on spawn weights
fn roll_spirit_type(config: &SpiritSpawnConfig, rng: &mut impl Rng) -> WorldSpiritType {
    let weights = &config.spawn_weights;
    let total = weights.total();
    let roll = rng.gen_range(0..total);

    let mut cumulative = 0;
    cumulative += weights.neutral;
    if roll < cumulative {
        return WorldSpiritType::Neutral;
    }
    cumulative += weights.ancestral;
    if roll < cumulative {
        return WorldSpiritType::Ancestral;
    }
    cumulative += weights.chaos;
    if roll < cumulative {
        return WorldSpiritType::Chaos;
    }
    cumulative += weights.harmony;
    if roll < cumulative {
        return WorldSpiritType::Harmony;
    }

    WorldSpiritType::Void
}

/// Process spirit absorption by the Prime Vessel
pub fn process_vessel_absorption(
    mut vessel_query: Query<(
        Entity,
        &mut PrimeVessel,
        &mut MetabolicHistory,
        &GridPosition,
    )>,
    mut spirit_query: Query<(Entity, &mut WorldSpirit, &GridPosition)>,
    mut corruption_index: ResMut<GlobalCorruptionIndex>,
    calendar: Res<GameCalendar>,
    mut absorb_events: EventWriter<SpiritAbsorbed>,
    mut vessel_absorb_events: EventWriter<VesselAbsorbedSpirit>,
    mut commands: Commands,
) {
    for (vessel_entity, mut vessel, mut history, vessel_pos) in vessel_query.iter_mut() {
        // Skip if not actively absorbing
        if vessel.roaming_state != crate::components::VesselRoamingState::Absorbing {
            continue;
        }

        // Find the target spirit
        if let Some(target_entity) = vessel.current_target {
            if let Ok((spirit_entity, mut spirit, spirit_pos)) = spirit_query.get_mut(target_entity)
            {
                // Check if spirit is already being absorbed or purified
                if spirit.being_absorbed || spirit.is_purified {
                    vessel.current_target = None;
                    vessel.roaming_state = crate::components::VesselRoamingState::Wandering;
                    continue;
                }

                // Check if vessel is close enough (within 2 tiles)
                let dx = (vessel_pos.x - spirit_pos.x).abs();
                let dy = (vessel_pos.y - spirit_pos.y).abs();

                if dx <= 2 && dy <= 2 {
                    // Absorb the spirit!
                    let power = spirit.power;
                    let spirit_type = spirit.spirit_type;

                    // Add to metabolic history
                    history.absorb(power, calendar.current_day());
                    vessel.power_level += power;
                    vessel.max_power_reached = vessel.max_power_reached.max(vessel.power_level);

                    // Update corruption index
                    corruption_index.consume_spirit(true);

                    // Fire events
                    absorb_events.send(SpiritAbsorbed {
                        spirit_entity,
                        absorber_entity: vessel_entity,
                        by_vessel: true,
                        power,
                    });

                    vessel_absorb_events.send(VesselAbsorbedSpirit {
                        spirit_entity,
                        spirit_type,
                        power_gained: power,
                        vessel_new_power: vessel.power_level,
                    });

                    // Remove the spirit entity
                    commands.entity(spirit_entity).despawn();

                    // Clear target and return to hunting
                    vessel.current_target = None;
                    vessel.roaming_state = crate::components::VesselRoamingState::Hunting;

                    info!(
                        "Prime Vessel absorbed {} spirit (+{:.0} power, total: {:.0})",
                        format!("{:?}", spirit_type).to_lowercase(),
                        power,
                        vessel.power_level
                    );
                }
            } else {
                // Target no longer exists
                vessel.current_target = None;
                vessel.roaming_state = crate::components::VesselRoamingState::Wandering;
            }
        }
    }
}

/// Handle player spirit absorption
pub fn process_player_absorption(
    player_query: Query<(Entity, &GridPosition), With<bevy_shaman_core::components::Player>>,
    mut spirit_query: Query<(Entity, &mut WorldSpirit, &GridPosition)>,
    mut corruption_index: ResMut<GlobalCorruptionIndex>,
    mut absorb_events: EventWriter<SpiritAbsorbed>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    // Player presses E to absorb nearby spirits
    if !input.just_pressed(KeyCode::KeyE) {
        return;
    }

    if let Ok((player_entity, player_pos)) = player_query.get_single() {
        // Find nearest absorbable spirit within range
        let mut nearest: Option<(Entity, f32, f32)> = None;
        let absorption_range = 3;

        for (spirit_entity, spirit, spirit_pos) in spirit_query.iter() {
            if spirit.being_absorbed || spirit.is_purified {
                continue;
            }

            let dx = (player_pos.x - spirit_pos.x).abs();
            let dy = (player_pos.y - spirit_pos.y).abs();

            if dx <= absorption_range && dy <= absorption_range {
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                let should_update = match nearest {
                    None => true,
                    Some((_, nearest_dist, _)) => dist < nearest_dist,
                };
                if should_update {
                    nearest = Some((spirit_entity, dist, spirit.power));
                }
            }
        }

        if let Some((spirit_entity, _, power)) = nearest {
            // Update corruption index
            corruption_index.consume_spirit(false);

            // Fire event
            absorb_events.send(SpiritAbsorbed {
                spirit_entity,
                absorber_entity: player_entity,
                by_vessel: false,
                power,
            });

            // Remove spirit
            commands.entity(spirit_entity).despawn();

            info!("Player absorbed spirit (+{:.0} power)", power);
        }
    }
}

/// Handle spirit purification by player
pub fn process_spirit_purification(
    player_query: Query<&GridPosition, With<bevy_shaman_core::components::Player>>,
    mut spirit_query: Query<(Entity, &mut WorldSpirit, &GridPosition)>,
    mut corruption_index: ResMut<GlobalCorruptionIndex>,
    mut purify_events: EventWriter<SpiritPurified>,
    input: Res<ButtonInput<KeyCode>>,
) {
    // Player presses P to purify nearby spirits
    if !input.just_pressed(KeyCode::KeyP) {
        return;
    }

    if let Ok(player_pos) = player_query.get_single() {
        let purify_range = 3;

        for (entity, mut spirit, spirit_pos) in spirit_query.iter_mut() {
            if spirit.is_purified {
                continue;
            }

            let dx = (player_pos.x - spirit_pos.x).abs();
            let dy = (player_pos.y - spirit_pos.y).abs();

            if dx <= purify_range && dy <= purify_range {
                spirit.is_purified = true;
                corruption_index.purify_spirit();

                purify_events.send(SpiritPurified {
                    spirit_entity: entity,
                    power: spirit.power,
                });

                info!("Spirit purified! Corruption slowed.");
            }
        }
    }
}

/// Free trapped spirits from defeated entities
pub fn free_trapped_spirits(
    mut freed_events: EventReader<bevy_shaman_core::events::EntityDied>,
    mut corruption_index: ResMut<GlobalCorruptionIndex>,
    mut spirit_free_events: EventWriter<SpiritFreed>,
    mut commands: Commands,
) {
    for event in freed_events.read() {
        // When entities die, they may release trapped spirits
        // Roll for spirit release (50% chance per entity)
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            let power = rng.gen_range(5.0..20.0);

            // Create a freed spirit
            let spirit = WorldSpirit {
                power,
                spirit_type: WorldSpiritType::Trapped,
                is_purified: false,
                being_absorbed: false,
                absorber: None,
            };

            let entity = commands.spawn(spirit).id();

            // Add back to world count
            corruption_index.free_spirit();

            spirit_free_events.send(SpiritFreed {
                spirit_entity: entity,
                freed_from: event.entity,
                power,
            });
        }
    }
}

/// PERFORMANCE OPTIMIZATION: Limit number of visible spirits on screen
/// This prevents lag while keeping background processing active
/// - Normal: Max 10 visible spirits
/// - With Prime Vessel boss: Max 15 visible spirits
pub fn manage_visible_spirits(
    mut commands: Commands,
    player_query: Query<&GridPosition, With<bevy_shaman_core::components::Player>>,
    spirit_query: Query<(Entity, &GridPosition, &WorldSpirit), Without<VisibleSpirit>>,
    visible_spirits: Query<(Entity, &GridPosition), With<VisibleSpirit>>,
    prime_vessel_query: Query<Entity, With<PrimeVessel>>,
) {
    let Ok(player_pos) = player_query.get_single() else {
        return;
    };

    // Determine max visible spirits based on Prime Vessel presence
    let max_visible = if !prime_vessel_query.is_empty() {
        // Prime Vessel boss present - allow 15 spirits
        15
    } else {
        // No boss - normal limit of 10 spirits
        10
    };

    let current_visible = visible_spirits.iter().count();

    // If we have too many visible spirits, hide the farthest ones
    if current_visible > max_visible {
        let mut visible_with_distance: Vec<_> = visible_spirits
            .iter()
            .map(|(entity, pos)| {
                let dx = (player_pos.x - pos.x).abs();
                let dy = (player_pos.y - pos.y).abs();
                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                (entity, distance)
            })
            .collect();

        // Sort by distance (farthest first)
        visible_with_distance.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Hide spirits beyond the limit
        for (entity, _) in visible_with_distance.iter().take(current_visible - max_visible) {
            commands.entity(*entity).remove::<VisibleSpirit>();
        }
    }
    // If we have room for more visible spirits, show closest hidden ones
    else if current_visible < max_visible {
        let slots_available = max_visible - current_visible;

        // Find closest hidden spirits to player
        let mut hidden_spirits: Vec<_> = spirit_query
            .iter()
            .map(|(entity, pos, _)| {
                let dx = (player_pos.x - pos.x).abs();
                let dy = (player_pos.y - pos.y).abs();
                let distance = ((dx * dx + dy * dy) as f32).sqrt();
                (entity, distance)
            })
            .collect();

        // Sort by distance (closest first)
        hidden_spirits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Make closest spirits visible
        for (entity, _) in hidden_spirits.iter().take(slots_available) {
            commands.entity(*entity).insert(VisibleSpirit);
        }
    }
}
