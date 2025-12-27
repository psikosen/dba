use bevy::prelude::*;
use crate::components::*;

/// Event for player initiating dialogue with an NPC
#[derive(Event)]
pub struct PlayerDialogueRequest {
    pub player: Entity,
    pub npc: Entity,
    pub player_message: Option<String>,
}

/// Event for NPC responding to player
#[derive(Event)]
pub struct NpcDialogueResponse {
    pub npc: Entity,
    pub response: String,
    pub context: DialogueContext,
}

/// System to handle brother NPC dialogue requests
pub fn brother_dialogue_system(
    mut dialogue_events: EventReader<PlayerDialogueRequest>,
    mut response_events: EventWriter<NpcDialogueResponse>,
    mut brother_query: Query<(
        &LlmAi,
        &mut LlmQueryQueue,
        &mut ConversationHistory,
    )>,
    time: Res<Time>,
) {
    for event in dialogue_events.read() {
        if let Ok((ai, mut queue, mut history)) = brother_query.get_mut(event.npc) {
            // Add player message to history if provided
            if let Some(ref message) = event.player_message {
                history.add_player_message(message.clone(), time.elapsed_secs_f64());
            }

            // Get appropriate response from queue
            let context = determine_dialogue_context(&ai, &history);

            if let Some(response_idx) = queue
                .dialogue_responses
                .iter()
                .position(|r| matches!(&r.context, ctx if ctx == &context) || matches!(r.context, DialogueContext::Greeting))
            {
                let response = queue.dialogue_responses.remove(response_idx);

                // Add NPC response to history
                history.add_npc_message(response.text.clone(), time.elapsed_secs_f64());

                // Send response event
                response_events.send(NpcDialogueResponse {
                    npc: event.npc,
                    response: response.text.clone(),
                    context: response.context.clone(),
                });

                info!("Brother {}: {}", ai.character_name, response.text);
            } else {
                // Fallback response if queue is empty
                let fallback = generate_fallback_brother_response(&ai);
                history.add_npc_message(fallback.clone(), time.elapsed_secs_f64());

                response_events.send(NpcDialogueResponse {
                    npc: event.npc,
                    response: fallback,
                    context: DialogueContext::Custom("fallback".to_string()),
                });
            }
        }
    }
}

/// Determine dialogue context based on AI state and history
fn determine_dialogue_context(
    // Placeholder for LLM integration: Will analyze AI personality and emotional state
    // to determine appropriate dialogue context (e.g., worried brother vs. confident mentor)
    _ai: &LlmAi,
    history: &ConversationHistory,
) -> DialogueContext {
    if history.messages.is_empty() {
        DialogueContext::Greeting
    } else {
        // Analyze recent conversation to determine context
        // For now, use a simple rule-based approach
        DialogueContext::Custom("conversation".to_string())
    }
}

/// Generate fallback response when queue is empty
fn generate_fallback_brother_response(ai: &LlmAi) -> String {
    let responses = if ai.personality.wisdom > 0.7 {
        vec![
            "The ancestors speak through you, brother.",
            "Trust in your spirit, as I trust in mine.",
            "Our bond is stronger than any corruption.",
        ]
    } else if ai.personality.chattiness > 0.7 {
        vec![
            "Brother! Tell me of your journeys!",
            "The spirits are restless today, can you feel it?",
            "Remember the songs our mother taught us?",
        ]
    } else {
        vec![
            "I'm here when you need me, brother.",
            "Stay strong.",
            "The path ahead is clear.",
        ]
    };

    let index = (ai.personality.spirituality * responses.len() as f32) as usize;
    responses[index.min(responses.len() - 1)].to_string()
}

/// System to provide contextual advice based on player state
pub fn brother_advice_system(
    mut advice_events: EventWriter<NpcDialogueResponse>,
    brother_query: Query<(Entity, &LlmAi, &LlmQueryQueue)>,
    player_query: Query<&bevy_shaman_core::components::Spirit, With<bevy_shaman_core::components::Player>>,
    combat_query: Query<&bevy_shaman_combat::components::BloodLust>,
    time: Res<Time>,
    mut last_advice: Local<f64>,
) {
    let current_time = time.elapsed_secs_f64();

    // Advice cooldown (every 60 seconds)
    if current_time - *last_advice < 60.0 {
        return;
    }

    if let Ok(spirit) = player_query.get_single() {
        let spirit_percent = spirit.current / spirit.max;

        // Check if player needs advice
        let needs_advice = if spirit_percent < 0.3 {
            Some("Your spirit energy is low. Rest and meditate.")
        } else if let Ok(blood_lust) = combat_query.get_single() {
            if blood_lust.is_corrupting() {
                Some("Brother, your blood lust rises! Use the calming plants or play your instrument.")
            } else {
                None
            }
        } else {
            None
        };

        if let Some(advice) = needs_advice {
            // Send advice from a random brother
            if let Some((entity, ai, _)) = brother_query.iter().next() {
                advice_events.send(NpcDialogueResponse {
                    npc: entity,
                    response: format!("{}: {}", ai.character_name, advice),
                    context: DialogueContext::Custom("advice".to_string()),
                });

                *last_advice = current_time;
                info!("Brother {} provides advice", ai.character_name);
            }
        }
    }
}

/// System to generate dynamic greetings based on time of day/game state
pub fn dynamic_greeting_system(
    mut commands: Commands,
    mut brother_query: Query<(Entity, &LlmAi, &mut LlmQueryQueue), Added<LlmAi>>,
) {
    for (entity, ai, mut queue) in brother_query.iter_mut() {
        // Generate initial greeting when brother is first encountered
        if queue.dialogue_responses.is_empty() {
            let greeting = generate_initial_greeting(ai);

            queue.dialogue_responses.push(QueuedResponse {
                text: greeting.clone(),
                context: DialogueContext::Greeting,
                generated_at: 0.0,
            });

            // Attach greeting animations based on personality
            // Spawn visual/audio effects for greetings
            if ai.personality.spirituality > 0.7 {
                // Spiritual brothers get mystical particle effects
                info!("Spawning mystical particle effects for {}", ai.character_name);
                commands.trigger_targets(
                    GreetingAnimationEvent {
                        animation_type: GreetingAnimation::MysticalParticles,
                        intensity: ai.personality.spirituality,
                    },
                    entity,
                );
            } else if ai.personality.chattiness > 0.7 {
                // Chatty brothers get expressive gestures
                info!("Spawning expressive gesture animation for {}", ai.character_name);
                commands.trigger_targets(
                    GreetingAnimationEvent {
                        animation_type: GreetingAnimation::ExpressiveGesture,
                        intensity: ai.personality.chattiness,
                    },
                    entity,
                );
            } else {
                // Default subtle greeting animation
                commands.trigger_targets(
                    GreetingAnimationEvent {
                        animation_type: GreetingAnimation::SubtleNod,
                        intensity: 0.5,
                    },
                    entity,
                );
            }

            // Trigger voice synthesis based on emotional state
            commands.trigger_targets(
                VoiceSynthesisEvent {
                    text: greeting,
                    emotion: ai.emotional_state,
                    pitch: 1.0 - (ai.personality.wisdom * 0.3), // Wise characters speak lower
                },
                entity,
            );

            info!("Generated initial greeting with animations for brother {}", ai.character_name);
        }
    }
}

// Greeting animation events
#[derive(Event)]
pub struct GreetingAnimationEvent {
    pub animation_type: GreetingAnimation,
    pub intensity: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum GreetingAnimation {
    MysticalParticles,
    ExpressiveGesture,
    SubtleNod,
}

#[derive(Event)]
pub struct VoiceSynthesisEvent {
    pub text: String,
    pub emotion: EmotionalState,
    pub pitch: f32,
}

fn generate_initial_greeting(ai: &LlmAi) -> String {
    match ai.personality.honor {
        h if h > 0.8 => format!("{}: May the ancestors guide your steps, brother.", ai.character_name),
        h if h > 0.5 => format!("{}: Good to see you! The spirits are strong today.", ai.character_name),
        _ => format!("{}: Hey! What brings you here?", ai.character_name),
    }
}

/// System to update conversation context based on game events
pub fn update_conversation_context(
    mut brother_query: Query<(&LlmAi, &mut ConversationHistory)>,
    combat_events: EventReader<bevy_shaman_combat::systems::events::HitLanded>,
    time: Res<Time>,
) {
    // Update conversation history with context from game events
    // so that brothers can reference what's happening in the game
    let current_time = time.elapsed_secs_f64();

    // Track major combat events
    let combat_count = combat_events.len();
    if combat_count > 0 {
        for (ai, mut history) in brother_query.iter_mut() {
            // Add context about combat based on personality
            let combat_message = if ai.personality.spirituality > 0.7 {
                format!("The spirits are unsettled... I sense {} clashes of energy nearby.", combat_count)
            } else if ai.personality.wisdom > 0.7 {
                format!("Brother, I've been observing {} combat exchanges. Your technique improves.", combat_count)
            } else {
                format!("Sounds like you've been busy! {} hits landed!", combat_count)
            };

            // Update conversation history with context
            history.add_npc_message(combat_message, current_time);

            // Update emotional state based on combat intensity
            // (This would be implemented in the actual game events system)
        }

        info!(
            "Updated conversation history for {} brothers based on {} combat events",
            brother_query.iter().count(),
            combat_count
        );
    }

    // TODO: Add more event readers for:
    // - Quest progress
    // - Corruption spreading
    // - Spirit encounters
    // - Player status changes
}
