use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// System to monitor query queues and request refills
pub fn monitor_query_queues(
    mut commands: Commands,
    query: Query<(Entity, &LlmQueryQueue, &LlmAi), Without<LlmGenerationRequest>>,
    time: Res<Time>,
) {
    for (entity, queue, ai) in query.iter() {
        let mut needs_generation = false;
        let mut generation_type = GenerationType::Dialogue;

        // Check if we need dialogue refill
        if queue.needs_dialogue_refill() {
            needs_generation = true;
            generation_type = GenerationType::Dialogue;
        }

        // Check if we need combat decision refill
        if queue.needs_combat_refill() {
            needs_generation = true;
            if matches!(generation_type, GenerationType::Dialogue) {
                generation_type = GenerationType::Both;
            } else {
                generation_type = GenerationType::CombatDecision;
            }
        }

        if needs_generation {
            let priority = match ai.role {
                AiRole::Boss => RequestPriority::Critical,
                AiRole::MiniBoss => RequestPriority::High,
                AiRole::Brother => RequestPriority::Normal,
                _ => RequestPriority::Low,
            };

            commands.entity(entity).insert(LlmGenerationRequest {
                request_type: generation_type,
                priority,
                requested_at: time.elapsed_secs_f64(),
            });

            debug!(
                "Requested {:?} generation for {} (priority: {:?})",
                generation_type, ai.character_name, priority
            );
        }
    }
}

/// System to process LLM generation requests
/// NOTE: This is a stub - actual LLM inference will be implemented when GGUF model is added
pub fn process_generation_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, &LlmAi, &mut LlmQueryQueue, &LlmGenerationRequest)>,
    mut model: ResMut<LlmModel>,
    templates: Res<PromptTemplates>,
    mut cache: ResMut<ResponseCache>,
    time: Res<Time>,
) {
    // Sort by priority
    let mut sorted_requests: Vec<_> = requests.iter_mut().collect();
    sorted_requests.sort_by_key(|(_, _, _, req)| std::cmp::Reverse(req.priority));

    for (entity, ai, mut queue, request) in sorted_requests.into_iter().take(2) {
        // Limit concurrent generations

        match request.request_type {
            GenerationType::Dialogue | GenerationType::Both => {
                generate_dialogue_responses(
                    &ai,
                    &mut queue,
                    &templates,
                    &mut model,
                    &mut cache,
                    time.elapsed_secs_f64(),
                );
            }
            GenerationType::CombatDecision => {
                generate_combat_decisions(
                    &ai,
                    &mut queue,
                    &templates,
                    &mut model,
                    time.elapsed_secs_f64(),
                );
            }
            _ => {}
        }

        // Remove generation request component
        commands.entity(entity).remove::<LlmGenerationRequest>();
    }
}

/// Generate dialogue responses for an AI entity
fn generate_dialogue_responses(
    ai: &LlmAi,
    queue: &mut LlmQueryQueue,
    templates: &PromptTemplates,
    model: &mut LlmModel,
    cache: &mut ResponseCache,
    current_time: f64,
) {
    let contexts = vec![
        DialogueContext::Greeting,
        DialogueContext::Combat,
        DialogueContext::Taunt,
        DialogueContext::PhaseTransition,
    ];

    for context in contexts {
        // Build prompt
        let prompt = build_dialogue_prompt(ai, &context, templates);
        let prompt_hash = calculate_hash(&prompt);

        // Check cache first
        let response_text = if let Some(cached) = cache.get(prompt_hash) {
            model.stats.cache_hits += 1;
            cached.text.clone()
        } else {
            // Generate new response
            // TODO: When GGUF model is loaded, use actual inference here
            // For now, use placeholder responses
            let generated = generate_placeholder_dialogue(ai, &context);

            cache.insert(
                prompt_hash,
                CachedResponse {
                    text: generated.clone(),
                    generated_at: current_time,
                    use_count: 0,
                },
            );

            model.stats.total_requests += 1;
            model.stats.successful_generations += 1;

            generated
        };

        queue.dialogue_responses.push(QueuedResponse {
            text: response_text,
            context,
            generated_at: current_time,
        });
    }

    info!(
        "Generated {} dialogue responses for {}",
        queue.dialogue_responses.len(),
        ai.character_name
    );
}

/// Generate combat decisions for an AI entity
fn generate_combat_decisions(
    ai: &LlmAi,
    queue: &mut LlmQueryQueue,
    templates: &PromptTemplates,
    model: &mut LlmModel,
    current_time: f64,
) {
    // Generate varied combat decisions
    let decision_count = 5;

    for i in 0..decision_count {
        // Build combat prompt
        let prompt = build_combat_prompt(ai, i as u32, templates);

        // TODO: When GGUF model is loaded, use actual inference here
        // For now, use placeholder decisions
        let decision = generate_placeholder_combat_decision(ai, i);

        queue.combat_decisions.push(decision);

        model.stats.total_requests += 1;
        model.stats.successful_generations += 1;
    }

    info!(
        "Generated {} combat decisions for {}",
        queue.combat_decisions.len(),
        ai.character_name
    );
}

/// Build dialogue prompt from AI state and context
fn build_dialogue_prompt(ai: &LlmAi, context: &DialogueContext, templates: &PromptTemplates) -> String {
    let mut data = PromptData {
        name: ai.character_name.clone(),
        role: format!("{:?}", ai.role),
        aggression: ai.personality.aggression,
        wisdom: ai.personality.wisdom,
        chattiness: ai.personality.chattiness,
        honor: ai.personality.honor,
        spirituality: ai.personality.spirituality,
        emotion: format!("{:?}", ai.emotional_state),
        context: format!("{:?}", context),
        ..Default::default()
    };

    match ai.role {
        AiRole::Boss | AiRole::MiniBoss => templates.fill_boss_dialogue_prompt(&data),
        AiRole::Brother => templates.fill_brother_dialogue_prompt(&data),
        _ => templates.fill_boss_dialogue_prompt(&data),
    }
}

/// Build combat prompt from AI state
fn build_combat_prompt(ai: &LlmAi, phase: u32, templates: &PromptTemplates) -> String {
    let data = PromptData {
        name: ai.character_name.clone(),
        role: format!("{:?}", ai.role),
        aggression: ai.personality.aggression,
        wisdom: ai.personality.wisdom,
        honor: ai.personality.honor,
        spirituality: ai.personality.spirituality,
        health: 75.0, // Placeholder
        emotion: format!("{:?}", ai.emotional_state),
        phase,
        stance: format!("{:?}", ai.combat_stance),
        ..Default::default()
    };

    templates.fill_boss_combat_prompt(&data)
}

/// Calculate hash for caching
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

// ============================================================================
// PLACEHOLDER GENERATORS (Until GGUF model is added)
// ============================================================================

fn generate_placeholder_dialogue(ai: &LlmAi, context: &DialogueContext) -> String {
    let base_dialogues = match (ai.role, context) {
        (AiRole::Boss, DialogueContext::Greeting) => {
            vec![
                "The spirits whisper of your arrival, little shaman.",
                "You dare challenge the guardian of this sacred place?",
                "Your ancestors watch... will they weep or rejoice?",
            ]
        }
        (AiRole::Boss, DialogueContext::Combat) => {
            vec![
                "Feel the weight of corruption!",
                "The old ways die with you!",
                "Your spirits betray you!",
            ]
        }
        (AiRole::Boss, DialogueContext::Taunt) => {
            vec![
                "Is this the best the shamans can offer?",
                "Your rhythm falters, child!",
                "The spirits mock your weakness!",
            ]
        }
        (AiRole::Brother, DialogueContext::Greeting) => {
            vec![
                "Brother! The ancestors guide your path.",
                "I sense great trials ahead, but you are ready.",
                "Our bond transcends this world and the next.",
            ]
        }
        _ => vec!["..."],
    };

    // Select based on personality
    let index = (ai.personality.aggression * base_dialogues.len() as f32) as usize;
    base_dialogues[index.min(base_dialogues.len() - 1)].to_string()
}

fn generate_placeholder_combat_decision(ai: &LlmAi, variation: usize) -> QueuedCombatDecision {
    let actions = if ai.personality.aggression > 0.7 {
        vec![
            CombatAction::BasicAttack,
            CombatAction::SpecialMove("Crushing Blow".to_string()),
            CombatAction::SpreadCorruption,
        ]
    } else if ai.personality.wisdom > 0.6 {
        vec![
            CombatAction::ChangeStance(CombatStance::Tactical),
            CombatAction::UseAbility("Spirit Shield".to_string()),
            CombatAction::SummonMinion("Spirit Guardian".to_string()),
        ]
    } else {
        vec![
            CombatAction::BasicAttack,
            CombatAction::UseAbility("Quick Strike".to_string()),
        ]
    };

    let action = actions[variation % actions.len()].clone();

    QueuedCombatDecision {
        action,
        reasoning: "Tactical decision based on current situation".to_string(),
        conditions: vec![
            CombatCondition::HealthAbove(30.0),
            CombatCondition::DistanceToPlayer(DistanceCheck::LessThan(5.0)),
        ],
    }
}
