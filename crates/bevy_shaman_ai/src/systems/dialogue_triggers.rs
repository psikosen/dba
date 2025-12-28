use crate::components::{AiRole, ConversationHistory, LlmAi};
use crate::systems::npc_dialogue::{NpcDialogueResponse, PlayerDialogueRequest};
/// Dialogue Trigger Systems
/// Manages when and how dialogue is triggered based on game events and proximity
use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, Player};

// ============================================================================
// COMPONENTS
// ============================================================================

/// Marks an NPC as having a dialogue interaction zone
#[derive(Component)]
pub struct DialogueZone {
    pub range: f32,
    pub auto_trigger: bool, // Trigger automatically when entering range
}

impl Default for DialogueZone {
    fn default() -> Self {
        Self {
            range: 2.0,
            auto_trigger: false,
        }
    }
}

/// Tracks whether player is currently in an NPC's dialogue zone
#[derive(Component)]
pub struct InDialogueZone {
    pub player_nearby: bool,
    pub last_interaction_time: f64,
}

impl Default for InDialogueZone {
    fn default() -> Self {
        Self {
            player_nearby: false,
            last_interaction_time: 0.0,
        }
    }
}

/// Marks an NPC as having recently greeted the player
#[derive(Component)]
pub struct HasGreeted;

// ============================================================================
// PROXIMITY DETECTION SYSTEM
// ============================================================================

/// Detect when player enters/exits NPC dialogue zones
pub fn detect_dialogue_proximity(
    mut npc_query: Query<
        (
            Entity,
            &GridPosition,
            &DialogueZone,
            &mut InDialogueZone,
            Option<&HasGreeted>,
        ),
        With<LlmAi>,
    >,
    player_query: Query<(Entity, &GridPosition), With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let Ok((player_entity, player_pos)) = player_query.get_single() else {
        return;
    };

    let current_time = time.elapsed_secs_f64();

    for (npc_entity, npc_pos, zone, mut in_zone, has_greeted) in npc_query.iter_mut() {
        let distance = calculate_distance(npc_pos, player_pos);
        let was_nearby = in_zone.player_nearby;
        let is_nearby = distance <= zone.range;

        in_zone.player_nearby = is_nearby;

        // Player just entered dialogue zone
        if is_nearby && !was_nearby {
            info!("Player entered dialogue zone of NPC at {:?}", npc_pos);

            // Auto-trigger greeting if enabled and not greeted before
            if zone.auto_trigger && has_greeted.is_none() {
                commands.trigger(PlayerDialogueRequest {
                    player: player_entity,
                    npc: npc_entity,
                    player_message: None, // Auto-greeting, no player message
                });

                // Mark as greeted
                commands.entity(npc_entity).insert(HasGreeted);
                in_zone.last_interaction_time = current_time;
            }
        }

        // Player left dialogue zone
        if !is_nearby && was_nearby {
            info!("Player left dialogue zone of NPC at {:?}", npc_pos);
        }
    }
}

fn calculate_distance(pos1: &GridPosition, pos2: &GridPosition) -> f32 {
    let dx = (pos1.x - pos2.x) as f32;
    let dy = (pos1.y - pos2.y) as f32;
    (dx * dx + dy * dy).sqrt()
}

// ============================================================================
// MANUAL DIALOGUE TRIGGER SYSTEM
// ============================================================================

/// Allow player to manually trigger dialogue with 'E' key when near NPCs
pub fn manual_dialogue_trigger(
    keyboard: Res<ButtonInput<KeyCode>>,
    npc_query: Query<(Entity, &InDialogueZone, &LlmAi)>,
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    // Find nearest NPC in dialogue zone
    let nearest_npc = npc_query
        .iter()
        .filter(|(_, in_zone, _)| in_zone.player_nearby)
        .next();

    if let Some((npc_entity, _, ai)) = nearest_npc {
        info!("Player initiated dialogue with {}", ai.character_name);

        commands.trigger(PlayerDialogueRequest {
            player: player_entity,
            npc: npc_entity,
            player_message: None,
        });

        // Show dialogue prompt UI would go here
    }
}

// ============================================================================
// STORY EVENT DIALOGUE TRIGGERS
// ============================================================================

/// Event to trigger story-based dialogue
#[derive(Event)]
pub struct StoryDialogueTrigger {
    pub npc_name: String,
    pub context: String,
    pub force_trigger: bool, // Trigger even if player is far away
}

/// Handle story event dialogue triggers
pub fn handle_story_dialogue_triggers(
    mut trigger_events: EventReader<StoryDialogueTrigger>,
    mut npc_query: Query<(Entity, &LlmAi, &mut ConversationHistory)>,
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    let current_time = time.elapsed_secs_f64();

    for trigger in trigger_events.read() {
        // Find NPC by name
        let npc = npc_query
            .iter_mut()
            .find(|(_, ai, _)| ai.character_name == trigger.npc_name);

        if let Some((npc_entity, ai, mut history)) = npc {
            info!(
                "Story trigger: {} speaking to player about '{}'",
                ai.character_name, trigger.context
            );

            // Add story context to conversation history
            history.add_npc_message(
                format!("[Story Context: {}]", trigger.context),
                current_time,
            );

            // Trigger dialogue
            commands.trigger(PlayerDialogueRequest {
                player: player_entity,
                npc: npc_entity,
                player_message: Some(trigger.context.clone()),
            });
        } else {
            warn!(
                "Story dialogue trigger for unknown NPC: {}",
                trigger.npc_name
            );
        }
    }
}

// ============================================================================
// COMBAT START DIALOGUE TRIGGER
// ============================================================================

/// Trigger dialogue when combat with boss starts
pub fn boss_combat_start_dialogue(
    mut boss_query: Query<
        (Entity, &LlmAi, &mut ConversationHistory),
        Added<bevy_shaman_monsters::components::AiState>,
    >,
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    for (boss_entity, ai, mut history) in boss_query.iter_mut() {
        // Only trigger for bosses
        if !matches!(ai.role, AiRole::Boss | AiRole::MiniBoss) {
            continue;
        }

        info!(
            "Boss {} entered combat - triggering intro dialogue",
            ai.character_name
        );

        // Add combat context
        history.add_npc_message("[Combat Started]".to_string(), time.elapsed_secs_f64());

        // Trigger combat intro dialogue
        commands.trigger(PlayerDialogueRequest {
            player: player_entity,
            npc: boss_entity,
            player_message: Some("combat_start".to_string()),
        });
    }
}

// ============================================================================
// BROTHER QUEST DIALOGUE TRIGGERS
// ============================================================================

/// Trigger special dialogue when brother quest milestones are reached
pub fn brother_quest_milestone_dialogue(
    quest_events: EventReader<bevy_shaman_story::systems::quest_system::QuestObjectiveUpdated>,
    brother_query: Query<(Entity, &LlmAi)>,
    player_query: Query<Entity, With<Player>>,
    mut commands: Commands,
) {
    if quest_events.is_empty() {
        return;
    }

    let Ok(player_entity) = player_query.get_single() else {
        return;
    };

    // Find a brother to comment on quest progress
    let brother = brother_query
        .iter()
        .find(|(_, ai)| matches!(ai.role, AiRole::Brother));

    if let Some((brother_entity, ai)) = brother {
        info!("Brother {} commenting on quest progress", ai.character_name);

        commands.trigger(PlayerDialogueRequest {
            player: player_entity,
            npc: brother_entity,
            player_message: Some("quest_progress".to_string()),
        });
    }
}

// ============================================================================
// VISUAL INDICATORS
// ============================================================================

#[derive(Component)]
pub struct DialogueIndicator;

/// Show visual indicator above NPCs with available dialogue
pub fn show_dialogue_indicators(
    mut commands: Commands,
    npc_query: Query<
        (Entity, &InDialogueZone, &GridPosition),
        (With<LlmAi>, Without<DialogueIndicator>),
    >,
    indicator_query: Query<(Entity, &Parent), With<DialogueIndicator>>,
) {
    // Spawn indicators for NPCs with player nearby
    for (npc_entity, in_zone, _npc_pos) in npc_query.iter() {
        if in_zone.player_nearby {
            // Spawn floating indicator above NPC
            let _indicator = commands
                .spawn((
                    DialogueIndicator,
                    Text::new("💬"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(-40.0), // Float above NPC
                        ..default()
                    },
                ))
                .set_parent(npc_entity)
                .id();

            info!("Spawned dialogue indicator for NPC");
        }
    }

    // Remove indicators for NPCs without player nearby
    for (indicator_entity, parent) in indicator_query.iter() {
        if let Ok((_, in_zone, _)) = npc_query.get(parent.get()) {
            if !in_zone.player_nearby {
                commands.entity(indicator_entity).despawn_recursive();
            }
        }
    }
}

// ============================================================================
// DIALOGUE COOLDOWN MANAGEMENT
// ============================================================================

/// Prevent dialogue spam by enforcing cooldowns
pub const DIALOGUE_COOLDOWN: f64 = 5.0; // Seconds between dialogue triggers

pub fn enforce_dialogue_cooldown(
    mut npc_query: Query<&mut InDialogueZone, With<LlmAi>>,
    mut dialogue_requests: EventReader<PlayerDialogueRequest>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs_f64();

    for request in dialogue_requests.read() {
        if let Ok(mut in_zone) = npc_query.get_mut(request.npc) {
            let time_since_last = current_time - in_zone.last_interaction_time;

            if time_since_last < DIALOGUE_COOLDOWN {
                info!(
                    "Dialogue on cooldown ({:.1}s remaining)",
                    DIALOGUE_COOLDOWN - time_since_last
                );
                // Could consume the event or prevent trigger here
            } else {
                in_zone.last_interaction_time = current_time;
            }
        }
    }
}
