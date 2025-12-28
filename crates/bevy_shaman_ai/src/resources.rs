use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::llm_backend::{LlmBackend, create_backend};

// ============================================================================
// LLM MODEL RESOURCE
// ============================================================================

/// Global resource managing the LLM model
#[derive(Resource)]
pub struct LlmModel {
    /// Path to the GGUF model file
    pub model_path: String,
    /// Whether the model is currently loaded
    pub is_loaded: bool,
    /// Model configuration
    pub config: ModelConfig,
    /// Generation statistics
    pub stats: GenerationStats,
    /// Active backend (boxed to allow different implementations)
    #[allow(dead_code)]
    backend: Option<Box<dyn LlmBackend>>,
}

impl Default for LlmModel {
    fn default() -> Self {
        let mut model = Self {
            model_path: "models/gemma3-270m.gguf".to_string(),
            is_loaded: false,
            config: ModelConfig::default(),
            stats: GenerationStats::default(),
            backend: None,
        };

        // Initialize backend
        let backend = create_backend(&model);
        model.backend = Some(backend);
        if let Some(backend) = &model.backend {
            info!("LLM backend initialized: {}", backend.name());
        }

        model
    }
}

impl LlmModel {
    /// Generate text using the active backend
    pub fn generate(&mut self, prompt: &str) -> Result<String, String> {
        if let Some(backend) = &mut self.backend {
            self.stats.total_requests += 1;

            match backend.generate(prompt, &self.config) {
                Ok(response) => {
                    self.stats.successful_generations += 1;
                    Ok(response)
                }
                Err(e) => {
                    self.stats.failed_generations += 1;
                    Err(format!("{}", e))
                }
            }
        } else {
            Err("No backend available".to_string())
        }
    }

    /// Check if backend is ready
    pub fn is_backend_ready(&self) -> bool {
        self.backend.as_ref().map(|b| b.is_ready()).unwrap_or(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Maximum tokens to generate
    pub max_tokens: usize,
    /// Temperature for sampling (0.0 = deterministic, 1.0 = creative)
    pub temperature: f32,
    /// Top-p sampling threshold
    pub top_p: f32,
    /// Top-k sampling limit
    pub top_k: usize,
    /// Repeat penalty
    pub repeat_penalty: f32,
    /// Number of threads for inference
    pub n_threads: usize,
    /// Context window size
    pub context_size: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            max_tokens: 150,      // Short responses for real-time gameplay
            temperature: 0.8,     // Balanced creativity
            top_p: 0.9,
            top_k: 40,
            repeat_penalty: 1.1,
            n_threads: 4,
            context_size: 2048,
        }
    }
}

#[derive(Debug, Default)]
pub struct GenerationStats {
    pub total_requests: usize,
    pub successful_generations: usize,
    pub failed_generations: usize,
    pub average_generation_time_ms: f32,
    pub cache_hits: usize,
}

// ============================================================================
// PROMPT TEMPLATES
// ============================================================================

/// Resource containing prompt templates for different scenarios
#[derive(Resource)]
pub struct PromptTemplates {
    pub boss_combat: String,
    pub boss_dialogue: String,
    pub brother_dialogue: String,
    pub spirit_guide: String,
}

impl Default for PromptTemplates {
    fn default() -> Self {
        Self {
            boss_combat: Self::boss_combat_template(),
            boss_dialogue: Self::boss_dialogue_template(),
            brother_dialogue: Self::brother_dialogue_template(),
            spirit_guide: Self::spirit_guide_template(),
        }
    }
}

impl PromptTemplates {
    fn boss_combat_template() -> String {
        r#"You are {name}, a {role} in an African-inspired fantasy game.

Personality:
- Aggression: {aggression}
- Wisdom: {wisdom}
- Honor: {honor}
- Spirituality: {spirituality}

Current State:
- Health: {health}%
- Emotional State: {emotion}
- Combat Phase: {phase}
- Player Health: {player_health}%
- Player Corruption: {player_corruption}%

Combat Stance: {stance}

Recent actions: {recent_actions}

Based on this situation, decide your next combat action. Choose from:
- Basic Attack
- Special Move: [describe]
- Summon Minion: [type]
- Change Stance: [new stance]
- Spread Corruption
- Use Ability: [describe]

Provide your decision in JSON format:
{
  "action": "action_type",
  "details": "specific details",
  "reasoning": "why you chose this",
  "taunt": "optional battle cry or taunt (max 20 words)"
}"#.to_string()
    }

    fn boss_dialogue_template() -> String {
        r#"You are {name}, a {role} in an African-inspired fantasy game set in a world where shamans commune with spirits.

Personality:
- Aggression: {aggression}
- Wisdom: {wisdom}
- Chattiness: {chattiness}
- Honor: {honor}
- Spirituality: {spirituality}

Current Situation: {context}
Emotional State: {emotion}

Conversation History:
{history}

Generate a response that:
1. Fits your personality and role
2. References African folklore/spirituality themes
3. Is concise (max 30 words)
4. Matches your emotional state
5. Advances the narrative

Response:"#.to_string()
    }

    fn brother_dialogue_template() -> String {
        r#"You are {name}, one of the player's brothers in an African-inspired shaman adventure.

Your relationship with the player: {relationship}
Your spiritual affinity: {affinity}

Personality:
- Wisdom: {wisdom}
- Chattiness: {chattiness}
- Honor: {honor}

Current Situation: {context}
Recent events: {recent_events}

Conversation History:
{history}

Generate dialogue that:
1. Reflects your brotherly bond
2. Provides guidance or support
3. References shared cultural/spiritual heritage
4. Is natural and conversational (max 40 words)
5. May include advice about spirits, combat, or the journey

Dialogue:"#.to_string()
    }

    fn spirit_guide_template() -> String {
        r#"You are {name}, a spirit guide from the ancestral realm.

Spirit Type: {spirit_type}
Alignment: {alignment}

Personality:
- Wisdom: {wisdom}
- Spirituality: {spirituality}

Current Situation: {context}
Player's spiritual state: {player_state}

Generate mystical guidance that:
1. Speaks in riddles or metaphors
2. References nature, ancestors, and spirits
3. Is brief but profound (max 25 words)
4. Provides hints without direct answers

Guidance:"#.to_string()
    }

    pub fn fill_boss_combat_prompt(&self, data: &PromptData) -> String {
        self.boss_combat
            .replace("{name}", &data.name)
            .replace("{role}", &data.role)
            .replace("{aggression}", &data.aggression.to_string())
            .replace("{wisdom}", &data.wisdom.to_string())
            .replace("{honor}", &data.honor.to_string())
            .replace("{spirituality}", &data.spirituality.to_string())
            .replace("{health}", &data.health.to_string())
            .replace("{emotion}", &data.emotion)
            .replace("{phase}", &data.phase.to_string())
            .replace("{player_health}", &data.player_health.to_string())
            .replace("{player_corruption}", &data.player_corruption.to_string())
            .replace("{stance}", &data.stance)
            .replace("{recent_actions}", &data.recent_actions)
    }

    pub fn fill_boss_dialogue_prompt(&self, data: &PromptData) -> String {
        self.boss_dialogue
            .replace("{name}", &data.name)
            .replace("{role}", &data.role)
            .replace("{aggression}", &data.aggression.to_string())
            .replace("{wisdom}", &data.wisdom.to_string())
            .replace("{chattiness}", &data.chattiness.to_string())
            .replace("{honor}", &data.honor.to_string())
            .replace("{spirituality}", &data.spirituality.to_string())
            .replace("{context}", &data.context)
            .replace("{emotion}", &data.emotion)
            .replace("{history}", &data.history)
    }

    pub fn fill_brother_dialogue_prompt(&self, data: &PromptData) -> String {
        self.brother_dialogue
            .replace("{name}", &data.name)
            .replace("{relationship}", &data.relationship)
            .replace("{affinity}", &data.affinity)
            .replace("{wisdom}", &data.wisdom.to_string())
            .replace("{chattiness}", &data.chattiness.to_string())
            .replace("{honor}", &data.honor.to_string())
            .replace("{context}", &data.context)
            .replace("{recent_events}", &data.recent_events)
            .replace("{history}", &data.history)
    }
}

/// Data structure for filling prompt templates
#[derive(Default)]
pub struct PromptData {
    pub name: String,
    pub role: String,
    pub aggression: f32,
    pub wisdom: f32,
    pub chattiness: f32,
    pub honor: f32,
    pub spirituality: f32,
    pub health: f32,
    pub emotion: String,
    pub phase: u32,
    pub player_health: f32,
    pub player_corruption: f32,
    pub stance: String,
    pub recent_actions: String,
    pub context: String,
    pub history: String,
    pub relationship: String,
    pub affinity: String,
    pub recent_events: String,
}

// ============================================================================
// RESPONSE CACHE
// ============================================================================

/// Cache for LLM responses to avoid regenerating common queries
#[derive(Resource, Default)]
pub struct ResponseCache {
    pub entries: std::collections::HashMap<u64, CachedResponse>,
    pub max_entries: usize,
}

impl ResponseCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_entries,
        }
    }

    pub fn get(&self, prompt_hash: u64) -> Option<&CachedResponse> {
        self.entries.get(&prompt_hash)
    }

    pub fn insert(&mut self, prompt_hash: u64, response: CachedResponse) {
        if self.entries.len() >= self.max_entries {
            // Remove oldest entry
            if let Some(&oldest_key) = self.entries.keys().next() {
                self.entries.remove(&oldest_key);
            }
        }
        self.entries.insert(prompt_hash, response);
    }
}

#[derive(Clone)]
pub struct CachedResponse {
    pub text: String,
    pub generated_at: f64,
    pub use_count: usize,
}
