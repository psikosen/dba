use bevy::prelude::*;
use bevy_shaman_core::components::GridPosition;
use bevy_shaman_dungeons::components::{DungeonRoom, RoomType};
use bevy_shaman_dungeons::systems::events::DungeonEntered;
use bevy_shaman_monsters::components::{
    AiBehavior, AiState, MonsterState, MonsterStats, StateType,
};
use rand::Rng;

use crate::components::{
    LesserSelf, MetabolicHistory, PrimeVessel, SoulAlignment, VesselRoamingState,
};
use crate::resources::PrimeVesselState;

// ============================================================================
// DUNGEON ENCOUNTER CONSTANTS
// ============================================================================

/// Chance for Prime Vessel to appear in a dungeon (1%)
pub const PRIME_VESSEL_DUNGEON_CHANCE: f32 = 0.01;

/// Chance for Lesser Self spawns in dungeon rooms (12%)
pub const LESSER_SELF_DUNGEON_CHANCE: f32 = 0.12;

/// Maximum Lesser Selves per dungeon
pub const MAX_LESSER_SELVES_PER_DUNGEON: u32 = 3;

// ============================================================================
// DUNGEON ENCOUNTER MARKERS
// ============================================================================

/// Marker for dungeons that have been processed for Prime Vessel encounters
#[derive(Component)]
pub struct DungeonVesselChecked;

/// Marker for rooms that have been processed for Lesser Self encounters
#[derive(Component)]
pub struct LesserSelfSpawnChecked;

/// Tracks Prime Vessel dungeon encounters per dungeon
#[derive(Resource, Default)]
pub struct DungeonVesselEncounters {
    /// Dungeons where Prime Vessel has appeared
    pub vessel_appearances: Vec<String>,
    /// Lesser Selves spawned per dungeon
    pub lesser_self_counts: std::collections::HashMap<String, u32>,
}

// ============================================================================
// DUNGEON ENCOUNTER SYSTEMS
// ============================================================================

/// Check if Prime Vessel should appear when entering a dungeon
pub fn check_prime_vessel_dungeon_spawn(
    mut dungeon_events: EventReader<DungeonEntered>,
    vessel_state: Res<PrimeVesselState>,
    mut encounter_tracker: ResMut<DungeonVesselEncounters>,
    mut commands: Commands,
) {
    // Skip if vessel doesn't exist or is defeated
    if !vessel_state.has_spawned || vessel_state.vessel_defeated {
        return;
    }

    for event in dungeon_events.read() {
        let mut rng = rand::thread_rng();

        // 1% chance for Prime Vessel to appear
        if rng.gen::<f32>() < PRIME_VESSEL_DUNGEON_CHANCE {
            // Mark this dungeon as having a vessel appearance
            encounter_tracker
                .vessel_appearances
                .push(event.dungeon_id.clone());

            warn!(
                "THE PRIME VESSEL HAS BEEN SIGHTED IN {}! \
                 It hunts in the darkness...",
                event.dungeon_id
            );

            // The actual vessel movement to dungeon would be handled by the AI
            // For now, we just flag it. The vessel will "teleport" to a dungeon room.
        }

        // Initialize Lesser Self counter for this dungeon
        encounter_tracker
            .lesser_self_counts
            .insert(event.dungeon_id.clone(), 0);
    }
}

/// Spawn Lesser Selves in dungeon encounter rooms
pub fn spawn_lesser_selves_in_dungeon(
    mut commands: Commands,
    rooms: Query<
        (Entity, &DungeonRoom, &GridPosition),
        (Without<LesserSelfSpawnChecked>, Without<LesserSelf>),
    >,
    vessel_state: Res<PrimeVesselState>,
    mut encounter_tracker: ResMut<DungeonVesselEncounters>,
) {
    // Only spawn if there are Lesser Selves in the world
    if vessel_state.lesser_selves.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();

    for (room_entity, room, room_pos) in rooms.iter() {
        // Only spawn in encounter rooms
        if !matches!(room.room_type, RoomType::Encounter) {
            // Mark as checked even if not an encounter room
            commands.entity(room_entity).insert(LesserSelfSpawnChecked);
            continue;
        }

        // 12% chance for Lesser Self spawn
        if rng.gen::<f32>() >= LESSER_SELF_DUNGEON_CHANCE {
            commands.entity(room_entity).insert(LesserSelfSpawnChecked);
            continue;
        }

        // Check if we've hit the max per dungeon
        // (Using a simple approach - in practice you'd track the current dungeon ID)
        let total_spawned: u32 = encounter_tracker.lesser_self_counts.values().sum();
        if total_spawned >= MAX_LESSER_SELVES_PER_DUNGEON * 10 {
            // Global cap
            commands.entity(room_entity).insert(LesserSelfSpawnChecked);
            continue;
        }

        // Spawn a Lesser Self!
        let lesser_self = create_dungeon_lesser_self(&mut rng, room_pos);
        let lesser_self_power = lesser_self.power_level;
        let generation = lesser_self.generation;

        commands.spawn((
            lesser_self,
            GridPosition {
                x: room_pos.x + rng.gen_range(-1..=1),
                y: room_pos.y + rng.gen_range(-1..=1),
            },
            MonsterStats {
                attack: 15.0 + (lesser_self_power * 0.1),
                defense: 8.0 + (lesser_self_power * 0.05),
                speed: 6.0,
                spirit_affinity: 0.7,
            },
            MonsterState {
                state: StateType::Chaos,
                stability_meter: 0.15,
                corruption_meter: 0.8,
                obedience_meter: 0.0,
                chaos_output: 1.8,
            },
            AiBehavior {
                behavior_tree_id: "lesser_self_dungeon".to_string(),
                aggression: 0.9,
                flee_threshold: 0.0,
            },
            AiState::Aggressive,
            SoulAlignment {
                alignment: 0.8,
                aggression_modifier: 2.0,
                chaos_locked: false,
            },
        ));

        // Mark room as checked
        commands.entity(room_entity).insert(LesserSelfSpawnChecked);

        warn!(
            "A LESSER SELF (Gen {}) lurks in the dungeon depths at ({}, {})!",
            generation, room_pos.x, room_pos.y
        );
    }
}

/// Create a Lesser Self for dungeon spawning
fn create_dungeon_lesser_self(rng: &mut impl Rng, _position: &GridPosition) -> LesserSelf {
    use crate::components::VesselMutation;

    // Random tier based on dungeon depth/difficulty
    let tier = rng.gen_range(1..=5) as u8;
    let power = (tier as f32 * 100.0).min(LesserSelf::MAX_POWER);
    let generation = rng.gen_range(1..=100);

    // Random mutations
    let mut mutations = Vec::new();
    let mutation_count = rng.gen_range(0..=tier as usize);

    let possible_mutations = [
        VesselMutation::VenomousStrike,
        VesselMutation::ChitinousArmor,
        VesselMutation::RegenerativeFlesh,
        VesselMutation::SpiritDrain,
        VesselMutation::ChaosBurst,
        VesselMutation::BlinkDash,
    ];

    for _ in 0..mutation_count {
        let mutation = possible_mutations[rng.gen_range(0..possible_mutations.len())];
        if !mutations.contains(&mutation) {
            mutations.push(mutation);
        }
    }

    LesserSelf {
        origin_tier: tier,
        power_level: power,
        generation,
        creation_day: 0, // Unknown - dungeon spawn
        mutations,
        patrol_route: Vec::new(), // No patrol in dungeons
        patrol_index: 0,
    }
}

/// Teleport Prime Vessel to dungeon if it has appeared there
pub fn move_vessel_to_dungeon(
    mut vessel_query: Query<(&mut PrimeVessel, &mut GridPosition)>,
    encounter_tracker: Res<DungeonVesselEncounters>,
    boss_rooms: Query<&GridPosition, With<bevy_shaman_dungeons::components::BossArena>>,
) {
    // If vessel has appeared in a dungeon, move it near the boss room
    if encounter_tracker.vessel_appearances.is_empty() {
        return;
    }

    for (mut vessel, mut vessel_pos) in vessel_query.iter_mut() {
        // Only move if vessel is wandering (not in combat or other states)
        if vessel.roaming_state != VesselRoamingState::Wandering {
            continue;
        }

        // Find a boss room to lurk near
        if let Some(boss_pos) = boss_rooms.iter().next() {
            vessel_pos.x = boss_pos.x - 5;
            vessel_pos.y = boss_pos.y - 5;
            vessel.roaming_state = VesselRoamingState::Hunting;

            info!(
                "Prime Vessel materializes near the dungeon depths at ({}, {})",
                vessel_pos.x, vessel_pos.y
            );
        }
    }
}
