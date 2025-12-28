use bevy::prelude::*;
use bevy_shaman_monsters::components::{AiBehavior, MonsterState, StateType};
use bevy_shaman_story::resources::BrotherCleansingProgress;

use crate::components::{ChaoticShift, CorruptionImmune, PrimeVessel, SoulAlignment};
use crate::events::{
    CorruptionIndexChanged, CorruptionIndexRevealed, EntityChaosLocked, SoulAlignmentShifted,
    TotalCollapseTriggered,
};
use crate::resources::GlobalCorruptionIndex;

/// Update all entity soul alignments based on the Global Corruption Index
pub fn update_soul_alignments(
    corruption_index: Res<GlobalCorruptionIndex>,
    mut query: Query<
        (Entity, &mut SoulAlignment, Option<&mut AiBehavior>),
        (Without<CorruptionImmune>, Without<ChaoticShift>),
    >,
    mut alignment_events: EventWriter<SoulAlignmentShifted>,
) {
    if !corruption_index.is_changed() {
        return;
    }

    let corruption = corruption_index.corruption_percentage;

    for (entity, mut alignment, ai_behavior) in query.iter_mut() {
        let old_alignment = alignment.alignment;
        alignment.update_from_corruption(corruption);

        // Check if entity became hostile due to alignment shift
        let became_hostile = old_alignment < 0.5 && alignment.alignment >= 0.5;

        // Update AI aggression if present
        if let Some(mut ai) = ai_behavior {
            ai.aggression = (ai.aggression * alignment.aggression_modifier).min(1.0);
        }

        // Fire event for significant shifts
        if (alignment.alignment - old_alignment).abs() > 0.05 {
            alignment_events.send(SoulAlignmentShifted {
                entity,
                old_alignment,
                new_alignment: alignment.alignment,
                became_hostile,
            });
        }
    }
}

/// Monitor corruption index and fire danger level change events
pub fn monitor_corruption_changes(
    corruption_index: Res<GlobalCorruptionIndex>,
    mut last_danger_level: Local<u8>,
    mut last_percentage: Local<f32>,
    mut change_events: EventWriter<CorruptionIndexChanged>,
) {
    if !corruption_index.is_changed() {
        return;
    }

    let new_danger = corruption_index.danger_level();
    let new_percentage = corruption_index.corruption_percentage;

    // Fire event if danger level changed
    if new_danger != *last_danger_level {
        change_events.send(CorruptionIndexChanged {
            old_percentage: *last_percentage,
            new_percentage,
            old_danger_level: *last_danger_level,
            new_danger_level: new_danger,
        });

        warn!(
            "World danger level changed: {} -> {} ({}% corruption)",
            *last_danger_level,
            new_danger,
            (new_percentage * 100.0) as u32
        );
    }

    *last_danger_level = new_danger;
    *last_percentage = new_percentage;
}

/// Handle Total Collapse trigger (Zero-Point)
pub fn check_total_collapse(
    mut corruption_index: ResMut<GlobalCorruptionIndex>,
    mut query: Query<(Entity, &mut SoulAlignment, &mut MonsterState), Without<ChaoticShift>>,
    mut collapse_events: EventWriter<TotalCollapseTriggered>,
    mut chaos_events: EventWriter<EntityChaosLocked>,
    mut commands: Commands,
) {
    // Only trigger once
    if !corruption_index.total_collapse_triggered {
        return;
    }

    // Check if we've already processed
    static mut COLLAPSE_PROCESSED: bool = false;
    unsafe {
        if COLLAPSE_PROCESSED {
            return;
        }
        COLLAPSE_PROCESSED = true;
    }

    // Fire the collapse event
    collapse_events.send(TotalCollapseTriggered {
        spirits_consumed_by_vessel: corruption_index.vessel_consumed,
        spirits_consumed_by_player: corruption_index.player_consumed,
        player_contribution_percentage: corruption_index.player_corruption_contribution(),
    });

    error!(
        "TOTAL COLLAPSE TRIGGERED! Player contribution: {:.1}%",
        corruption_index.player_corruption_contribution() * 100.0
    );

    // Lock all entities to chaos
    for (entity, mut alignment, mut monster_state) in query.iter_mut() {
        alignment.lock_to_chaos();
        monster_state.state = StateType::Chaos;
        monster_state.stability_meter = 0.0;
        monster_state.chaos_output = 3.0;

        // Mark as chaos-locked
        commands.entity(entity).insert(ChaoticShift);

        chaos_events.send(EntityChaosLocked {
            entity,
            entity_type: "entity".to_string(),
        });
    }
}

/// Check if the Corruption Index UI should be revealed
/// (After completing "Purify Your Brother" quest)
pub fn check_ui_reveal(
    mut corruption_index: ResMut<GlobalCorruptionIndex>,
    brother_progress: Res<BrotherCleansingProgress>,
    mut reveal_events: EventWriter<CorruptionIndexRevealed>,
) {
    // Already revealed
    if corruption_index.ui_revealed {
        return;
    }

    // Reveal after brother is fully cleansed
    if brother_progress.is_fully_cleansed() {
        corruption_index.reveal_ui();

        reveal_events.send(CorruptionIndexRevealed {
            current_percentage: corruption_index.corruption_percentage,
            spirits_remaining: corruption_index.free_spirit_count,
        });

        info!(
            "REVELATION: The Corruption Index is now visible. {} spirits remain. \
             World corruption: {:.1}%",
            corruption_index.free_spirit_count,
            corruption_index.corruption_percentage * 100.0
        );
    }
}

/// Apply world effects based on corruption level
pub fn apply_corruption_world_effects(
    corruption_index: Res<GlobalCorruptionIndex>,
    mut monster_query: Query<(&mut MonsterState, &SoulAlignment), Without<PrimeVessel>>,
) {
    if !corruption_index.is_changed() {
        return;
    }

    let danger_level = corruption_index.danger_level();

    for (mut monster, alignment) in monster_query.iter_mut() {
        // Skip chaos-locked entities
        if alignment.chaos_locked {
            continue;
        }

        // Apply gradual chaos based on danger level
        match danger_level {
            0 => {
                // Peaceful - monsters tend towards stability
                monster.stability_meter = (monster.stability_meter + 0.01).min(1.0);
            }
            1 => {
                // Uneasy - slight instability
                monster.stability_meter = (monster.stability_meter - 0.005).max(0.3);
            }
            2 => {
                // Dangerous - moderate chaos
                monster.stability_meter = (monster.stability_meter - 0.01).max(0.2);
                monster.chaos_output = 1.2;
            }
            3 => {
                // Hostile - high chaos
                monster.stability_meter = (monster.stability_meter - 0.02).max(0.1);
                monster.chaos_output = 1.5;
            }
            4 => {
                // Critical - near-chaos
                monster.stability_meter = 0.1;
                monster.chaos_output = 2.0;
            }
            5 => {
                // Total Collapse - pure chaos
                monster.stability_meter = 0.0;
                monster.state = StateType::Chaos;
                monster.chaos_output = 3.0;
            }
            _ => {}
        }
    }
}
