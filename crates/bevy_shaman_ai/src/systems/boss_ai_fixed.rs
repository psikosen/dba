// This file contains fixed versions of the boss AI functions
// Copy these to boss_ai.rs to fix borrowing issues

/// System to generate boss taunts and dialogue during combat - FIXED
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

        // Find matching dialogue response index
        if let Some(idx) = queue
            .dialogue_responses
            .iter()
            .position(|r| matches!(r.context, context) || matches!(r.context, DialogueContext::Combat))
        {
            // Remove and get the response
            let response = queue.dialogue_responses.remove(idx);

            // Add to conversation history
            history.add_npc_message(response.text.clone(), time.elapsed_secs_f64());

            info!("Boss {}: {}", ai.character_name, response.text);
        }
    }
}

/// System to handle boss phase transitions - FIXED
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

            // Find phase transition dialogue index
            if let Some(idx) = queue
                .dialogue_responses
                .iter()
                .position(|r| matches!(r.context, DialogueContext::PhaseTransition))
            {
                // Remove and get the response
                let response = queue.dialogue_responses.remove(idx);

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
