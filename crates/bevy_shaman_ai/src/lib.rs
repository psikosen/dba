pub mod components;
pub mod resources;
pub mod systems;
pub mod llm_backend;

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<resources::LlmModel>()
            .init_resource::<resources::PromptTemplates>()
            .insert_resource(resources::ResponseCache::new(100))
            // Systems - Query Queue Management
            .add_systems(Update, (
                systems::query_queue::monitor_query_queues,
                systems::query_queue::process_generation_requests,
            ).run_if(in_state(GameState::Playing)))
            // Systems - Boss AI
            .add_systems(Update, (
                systems::boss_ai::boss_combat_ai,
                systems::boss_ai::boss_combat_dialogue,
                systems::boss_ai::boss_phase_transitions,
            ).run_if(in_state(GameState::Playing)))
            // Systems - NPC Dialogue
            .add_systems(Update, (
                systems::npc_dialogue::brother_dialogue_system,
                systems::npc_dialogue::brother_advice_system,
                systems::npc_dialogue::dynamic_greeting_system,
                systems::npc_dialogue::update_conversation_context,
            ).run_if(in_state(GameState::Playing)))
            // Events - NPC Dialogue
            .add_event::<systems::npc_dialogue::PlayerDialogueRequest>()
            .add_event::<systems::npc_dialogue::NpcDialogueResponse>()
            // Events - Boss Combat
            .add_event::<systems::boss_ai::SpecialMoveTriggered>()
            .add_event::<systems::boss_ai::SummonMinionEvent>()
            .add_event::<systems::boss_ai::StanceChangeEvent>()
            .add_event::<systems::boss_ai::AbilityTriggered>()
            .add_event::<systems::boss_ai::CorruptionSpreadEvent>();
    }
}

// ============================================================================
// HELPER FUNCTIONS FOR SETTING UP LLM AI ENTITIES
// ============================================================================

/// Helper to spawn a boss with LLM AI
pub fn spawn_boss_with_ai(
    commands: &mut Commands,
    name: String,
    personality: components::PersonalityTraits,
) -> Entity {
    commands.spawn((
        Name::new(name.clone()),
        components::LlmAi {
            character_name: name,
            role: components::AiRole::Boss,
            personality,
            emotional_state: components::EmotionalState::Confident,
            combat_stance: components::CombatStance::Tactical,
        },
        components::LlmQueryQueue::new(10),
        components::ConversationHistory::new(20),
        // Request initial generation
        components::LlmGenerationRequest {
            request_type: components::GenerationType::Both,
            priority: components::RequestPriority::Critical,
            requested_at: 0.0,
        },
    )).id()
}

/// Helper to spawn a brother NPC with LLM AI
pub fn spawn_brother_with_ai(
    commands: &mut Commands,
    name: String,
    personality: components::PersonalityTraits,
) -> Entity {
    commands.spawn((
        Name::new(name.clone()),
        components::LlmAi {
            character_name: name,
            role: components::AiRole::Brother,
            personality,
            emotional_state: components::EmotionalState::Calm,
            combat_stance: components::CombatStance::Defensive,
        },
        components::LlmQueryQueue::new(8),
        components::ConversationHistory::new(30),
        components::LlmGenerationRequest {
            request_type: components::GenerationType::Dialogue,
            priority: components::RequestPriority::Normal,
            requested_at: 0.0,
        },
    )).id()
}
