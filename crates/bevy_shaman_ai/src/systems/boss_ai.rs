use bevy::prelude::*;
use crate::components::*;
use bevy_shaman_core::components::{Health, Player};
use bevy_shaman_monsters::components::{MonsterState, AiState};

/// System to apply LLM-driven combat decisions for bosses
pub fn boss_combat_ai(
    mut boss_query: Query<(
        &mut LlmAi,
        &mut LlmQueryQueue,
        &Health,
        &MonsterState,
        &mut AiState,
    )>,
    player_query: Query<&Health, With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let player_health = player_query.get_single().map(|h| h.current / h.max * 100.0).unwrap_or(100.0);

    for (mut ai, mut queue, health, monster_state, mut ai_state) in boss_query.iter_mut() {
        // Update emotional state based on health
        update_emotional_state(&mut ai, health, monster_state);

        // Try to use a queued combat decision
        if let Some(decision) = find_suitable_decision(&mut queue, health, player_health, monster_state) {
            execute_combat_decision(&decision, &mut ai_state, &ai);

            info!(
                "Boss {} executes: {:?} - {}",
                ai.character_name, decision.action, decision.reasoning
            );

            // Remove used decision from queue
            queue.combat_decisions.retain(|d| !std::ptr::eq(d, &decision));
        } else {
            // Fallback to basic behavior if no queued decision
            *ai_state = AiState::Aggressive;
        }
    }
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
                CombatCondition::DistanceToPlayer(_) => true, // TODO: Calculate actual distance
                CombatCondition::PhaseNumber(_) => true,      // TODO: Track phases
            }
        })
    }).cloned()
}

/// Execute a combat decision
fn execute_combat_decision(decision: &QueuedCombatDecision, ai_state: &mut AiState, ai: &LlmAi) {
    match &decision.action {
        CombatAction::BasicAttack => {
            *ai_state = AiState::Aggressive;
        }
        CombatAction::SpecialMove(_move_name) => {
            // TODO: Trigger special move animation/effect
            *ai_state = AiState::Aggressive;
            info!("Boss uses special move: {}", _move_name);
        }
        CombatAction::SummonMinion(_minion_type) => {
            // TODO: Spawn minion entity
            info!("Boss summons: {}", _minion_type);
        }
        CombatAction::Retreat => {
            *ai_state = AiState::Fleeing;
        }
        CombatAction::ChangeStance(new_stance) => {
            // Change combat stance
            info!("Boss changes stance to: {:?}", new_stance);
        }
        CombatAction::UseAbility(_ability) => {
            // TODO: Trigger ability
            info!("Boss uses ability: {}", _ability);
        }
        CombatAction::SpreadCorruption => {
            // TODO: Apply corruption spread effect
            info!("Boss spreads corruption!");
        }
    }
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
            .iter().position
            |r| matches!(r.context, context) || matches!(r.context, DialogueContext::Combat))
        {
            let response = queue.dialogue_responses.remove(idx);
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

                let response = queue.dialogue_responses.remove(idx);
            // Trigger phase transition dialogue
            if let Some(response_idx) = queue
                .dialogue_responses
                .iter().position
                |r| matches!(r.context, DialogueContext::PhaseTransition))
            {
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
