use bevy::prelude::*;
use crate::components::*;
use bevy_shaman_core::components::{Health, Player, GridPosition};
use bevy_shaman_monsters::components::{MonsterState, AiState};

/// System to apply LLM-driven combat decisions for bosses
pub fn boss_combat_ai(
    mut boss_query: Query<(
        Entity,
        &mut LlmAi,
        &mut LlmQueryQueue,
        &Health,
        &MonsterState,
        &mut AiState,
        &GridPosition,
    )>,
    player_query: Query<(&Health, &GridPosition), With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
    mut phase_tracker: Local<std::collections::HashMap<Entity, BossPhaseData>>,
) {
    let (player_health_percent, player_pos) = player_query
        .get_single()
        .map(|(h, pos)| (h.current / h.max * 100.0, pos))
        .unwrap_or((100.0, &GridPosition { x: 0, y: 0 }));

    for (entity, mut ai, mut queue, health, monster_state, mut ai_state, boss_pos) in boss_query.iter_mut() {
        // Calculate distance to player
        let distance = calculate_distance(boss_pos, player_pos);

        // Get or initialize phase data for this boss
        let phase_data = phase_tracker.entry(entity).or_insert(BossPhaseData {
            current_phase: 1,
            last_phase_transition: 0.0,
            special_move_cooldown: 0.0,
        });

        // Update phase based on health
        let health_percent = health.current / health.max;
        phase_data.current_phase = calculate_phase_from_health(health_percent);

        // Update cooldowns
        phase_data.special_move_cooldown = (phase_data.special_move_cooldown - time.delta_secs()).max(0.0);

        // Update emotional state based on health
        update_emotional_state(&mut ai, health, monster_state);

        // Try to use a queued combat decision
        if let Some(decision) = find_suitable_decision(
            &mut queue,
            health,
            player_health_percent,
            monster_state,
            distance,
            phase_data.current_phase,
        ) {
            execute_combat_decision(
                &decision,
                &mut ai_state,
                &ai,
                &mut commands,
                entity,
                phase_data,
                distance,
            );

            info!(
                "Boss {} (Phase {}, Distance: {:.1}) executes: {:?} - {}",
                ai.character_name, phase_data.current_phase, distance, decision.action, decision.reasoning
            );

            // Remove used decision from queue
            queue.combat_decisions.retain(|d| !std::ptr::eq(d, &decision));
        } else {
            // Fallback to basic behavior if no queued decision
            *ai_state = AiState::Aggressive;
        }
    }
}

/// Calculate grid distance between two positions
fn calculate_distance(pos1: &GridPosition, pos2: &GridPosition) -> f32 {
    let dx = (pos1.x - pos2.x) as f32;
    let dy = (pos1.y - pos2.y) as f32;
    (dx * dx + dy * dy).sqrt()
}

/// Calculate phase number from health percentage
fn calculate_phase_from_health(health_percent: f32) -> u32 {
    match health_percent {
        p if p > 0.75 => 1,
        p if p > 0.50 => 2,
        p if p > 0.25 => 3,
        _ => 4,
    }
}

/// Boss phase tracking data
#[derive(Clone)]
pub struct BossPhaseData {
    current_phase: u32,
    last_phase_transition: f32,
    special_move_cooldown: f32,
}

/// Update boss emotional state based on combat situation
fn update_emotional_state(ai: &mut LlmAi, health: &Health, monster_state: &MonsterState) {
    let health_percent = health.current / health.max;

    ai.emotional_state = if health_percent < 0.2 {
        EmotionalState::Desperate
    } else if health_percent < 0.5 {
        if monster_state.corruption_meter > 0.7 {
            EmotionalState::Corrupted
        } else {
            EmotionalState::Angry
        }
    } else if health_percent > 0.8 {
        if ai.personality.aggression > 0.7 {
            EmotionalState::Confident
        } else {
            EmotionalState::Calm
        }
    } else {
        EmotionalState::Confident
    };
}

/// Find a suitable combat decision based on current conditions
fn find_suitable_decision(
    queue: &mut LlmQueryQueue,
    health: &Health,
    player_health: f32,
    monster_state: &MonsterState,
    distance_to_player: f32,
    current_phase: u32,
) -> Option<QueuedCombatDecision> {
    let health_percent = health.current / health.max * 100.0;

    queue.combat_decisions.iter().find(|decision| {
        decision.conditions.iter().all(|condition| {
            match condition {
                CombatCondition::HealthBelow(threshold) => health_percent < *threshold,
                CombatCondition::HealthAbove(threshold) => health_percent > *threshold,
                CombatCondition::PlayerHealthBelow(threshold) => player_health < *threshold,
                CombatCondition::CorruptionAbove(threshold) => {
                    monster_state.corruption_meter > *threshold
                }
                CombatCondition::DistanceToPlayer(distance_check) => {
                    match distance_check {
                        DistanceCheck::LessThan(d) => distance_to_player < *d,
                        DistanceCheck::GreaterThan(d) => distance_to_player > *d,
                        DistanceCheck::InRange(min, max) => {
                            distance_to_player >= *min && distance_to_player <= *max
                        }
                    }
                }
                CombatCondition::PhaseNumber(phase) => current_phase == *phase,
            }
        })
    }).cloned()
}

/// Execute a combat decision
fn execute_combat_decision(
    decision: &QueuedCombatDecision,
    ai_state: &mut AiState,
    _ai: &LlmAi,
    commands: &mut Commands,
    boss_entity: Entity,
    phase_data: &mut BossPhaseData,
    distance: f32,
) {
    match &decision.action {
        CombatAction::BasicAttack => {
            *ai_state = if distance > 3.0 {
                AiState::Pursuing
            } else {
                AiState::Aggressive
            };
        }
        CombatAction::SpecialMove(move_name) => {
            // Trigger special move with cooldown
            if phase_data.special_move_cooldown <= 0.0 {
                *ai_state = AiState::Aggressive;

                // Trigger special move event
                commands.trigger_targets(
                    SpecialMoveTriggered {
                        move_name: move_name.clone(),
                        boss_phase: phase_data.current_phase,
                        damage_multiplier: 1.5 + (phase_data.current_phase as f32 * 0.5),
                    },
                    boss_entity,
                );

                // Set cooldown based on phase (more powerful = longer cooldown)
                phase_data.special_move_cooldown = 5.0 + (phase_data.current_phase as f32 * 2.0);

                info!("Boss uses special move: {} (Phase {})", move_name, phase_data.current_phase);
            }
        }
        CombatAction::SummonMinion(minion_type) => {
            // Trigger minion summon event
            commands.trigger_targets(
                SummonMinionEvent {
                    minion_type: minion_type.clone(),
                    count: phase_data.current_phase.min(3), // More minions in later phases
                },
                boss_entity,
            );

            info!("Boss summons {} x{}", minion_type, phase_data.current_phase.min(3));
        }
        CombatAction::Retreat => {
            *ai_state = AiState::Fleeing;
        }
        CombatAction::ChangeStance(new_stance) => {
            // Trigger stance change event
            commands.trigger_targets(
                StanceChangeEvent {
                    new_stance: *new_stance,
                },
                boss_entity,
            );

            info!("Boss changes stance to: {:?}", new_stance);
        }
        CombatAction::UseAbility(ability) => {
            // Trigger ability event
            commands.trigger_targets(
                AbilityTriggered {
                    ability_name: ability.clone(),
                    distance_to_target: distance,
                },
                boss_entity,
            );

            info!("Boss uses ability: {} (distance: {:.1})", ability, distance);
        }
        CombatAction::SpreadCorruption => {
            // Trigger corruption spread event
            commands.trigger_targets(
                CorruptionSpreadEvent {
                    radius: 3.0 + phase_data.current_phase as f32,
                    intensity: 0.3 * phase_data.current_phase as f32,
                },
                boss_entity,
            );

            info!("Boss spreads corruption! (radius: {:.1})", 3.0 + phase_data.current_phase as f32);
        }
    }
}

// ============================================================================
// BOSS COMBAT EVENTS
// ============================================================================

/// Event triggered when a boss uses a special move
#[derive(Event)]
pub struct SpecialMoveTriggered {
    pub move_name: String,
    pub boss_phase: u32,
    pub damage_multiplier: f32,
}

/// Event triggered when a boss summons minions
#[derive(Event)]
pub struct SummonMinionEvent {
    pub minion_type: String,
    pub count: u32,
}

/// Event triggered when a boss changes combat stance
#[derive(Event)]
pub struct StanceChangeEvent {
    pub new_stance: CombatStance,
}

/// Event triggered when a boss uses an ability
#[derive(Event)]
pub struct AbilityTriggered {
    pub ability_name: String,
    pub distance_to_target: f32,
}

/// Event triggered when a boss spreads corruption
#[derive(Event)]
pub struct CorruptionSpreadEvent {
    pub radius: f32,
    pub intensity: f32,
}

/// System to generate boss taunts and dialogue during combat
pub fn boss_combat_dialogue(
    mut boss_query: Query<(&LlmAi, &mut LlmQueryQueue, &mut ConversationHistory)>,
    player_query: Query<&Health, With<Player>>,
    time: Res<Time>,
) {
    let player_health = player_query
        .get_single()
        .map(|h| h.current / h.max * 100.0)
        .unwrap_or(100.0);

    for (ai, mut queue, mut history) in boss_query.iter_mut() {
        // Determine dialogue context based on situation
        let context = if player_health < 30.0 {
            DialogueContext::PlayerLowHealth
        } else if matches!(ai.emotional_state, EmotionalState::Angry | EmotionalState::Desperate) {
            DialogueContext::Taunt
        } else {
            DialogueContext::Combat
        };

        // Find matching dialogue from queue
        if let Some(response_idx) = queue
            .dialogue_responses
            .iter()
            .position(|r| matches!(&r.context, ctx if ctx == &context) || matches!(r.context, DialogueContext::Combat))
        {
            let response = queue.dialogue_responses.remove(response_idx);
            // Add to conversation history
            history.add_npc_message(response.text.clone(), time.elapsed_secs_f64());

            info!("Boss {}: {}", ai.character_name, response.text);

        }
    }
}

/// System to handle boss phase transitions
pub fn boss_phase_transitions(
    mut boss_query: Query<(&mut LlmAi, &Health, &mut LlmQueryQueue)>,
    mut phase_tracker: Local<std::collections::HashMap<String, u32>>,
) {
    for (mut ai, health, mut queue) in boss_query.iter_mut() {
        let health_percent = health.current / health.max;
        let current_phase = phase_tracker.entry(ai.character_name.clone()).or_insert(1);

        // Check for phase transitions (every 25% health)
        let new_phase = match health_percent {
            p if p > 0.75 => 1,
            p if p > 0.50 => 2,
            p if p > 0.25 => 3,
            _ => 4,
        };

        if new_phase > *current_phase {
            *current_phase = new_phase;

            // Trigger phase transition dialogue
            if let Some(response_idx) = queue
                .dialogue_responses
                .iter()
                .position(|r| matches!(r.context, DialogueContext::PhaseTransition))
            {
                let response = queue.dialogue_responses.remove(response_idx);
                info!(
                    "Boss {} enters Phase {}: {}",
                    ai.character_name, new_phase, response.text
                );

                // Update combat stance based on phase
                ai.combat_stance = match new_phase {
                    1 => CombatStance::Tactical,
                    2 => CombatStance::Aggressive,
                    3 => CombatStance::Summoner,
                    4 => CombatStance::Corrupting,
                    _ => CombatStance::Tactical,
                };

                // Update emotional state
                ai.emotional_state = if new_phase >= 4 {
                    EmotionalState::Desperate
                } else if new_phase >= 3 {
                    EmotionalState::Angry
                } else {
                    EmotionalState::Confident
                };

            }
        }
    }
}
