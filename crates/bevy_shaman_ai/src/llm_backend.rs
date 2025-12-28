use crate::components::*;
use crate::resources::*;
/// LLM Backend Abstraction Layer
/// Provides a unified interface for different LLM backends:
/// - Placeholder (rule-based responses for development/fallback)
/// - GGUF Local (via llama.cpp when model is available)
/// - HTTP API (for remote LLM services)
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// BACKEND TRAIT
// ============================================================================

/// Trait for LLM backend implementations
pub trait LlmBackend: Send + Sync {
    /// Generate text from a prompt
    fn generate(&mut self, prompt: &str, config: &ModelConfig) -> Result<String, LlmError>;

    /// Check if the backend is ready/loaded
    fn is_ready(&self) -> bool;

    /// Get backend name for logging
    fn name(&self) -> &str;
}

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone)]
pub enum LlmError {
    ModelNotLoaded,
    GenerationFailed(String),
    Timeout,
    InvalidPrompt,
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmError::ModelNotLoaded => write!(f, "Model not loaded"),
            LlmError::GenerationFailed(msg) => write!(f, "Generation failed: {}", msg),
            LlmError::Timeout => write!(f, "Generation timeout"),
            LlmError::InvalidPrompt => write!(f, "Invalid prompt"),
        }
    }
}

impl std::error::Error for LlmError {}

// ============================================================================
// PLACEHOLDER BACKEND (Intelligent Rule-Based)
// ============================================================================

pub struct PlaceholderBackend {
    personality_db: PersonalityDatabase,
    response_templates: ResponseTemplates,
}

impl PlaceholderBackend {
    pub fn new() -> Self {
        Self {
            personality_db: PersonalityDatabase::default(),
            response_templates: ResponseTemplates::default(),
        }
    }

    /// Generate dialogue based on personality traits and context
    fn generate_dialogue_response(
        &self,
        personality: &PersonalityTraits,
        context: &str,
        role: &str,
    ) -> String {
        let templates = match role.to_lowercase().as_str() {
            "boss" => &self.response_templates.boss_dialogue,
            "brother" => &self.response_templates.brother_dialogue,
            "spirit" => &self.response_templates.spirit_dialogue,
            _ => &self.response_templates.generic_dialogue,
        };

        // Pick template based on personality
        let template_index = if personality.wisdom > 0.7 {
            0 // Wise response
        } else if personality.aggression > 0.7 {
            1 // Aggressive response
        } else if personality.chattiness > 0.7 {
            2 // Chatty response
        } else {
            3 // Balanced response
        };

        let default_template = "...".to_string();
        let template = templates
            .get(template_index % templates.len())
            .unwrap_or(&default_template);

        // Simple context-aware modifications
        let response = if context.contains("health") || context.contains("hurt") {
            template.replace("[topic]", "your wounds")
        } else if context.contains("spirit") || context.contains("corruption") {
            template.replace("[topic]", "the corruption spreading")
        } else if context.contains("combat") || context.contains("fight") {
            template.replace("[topic]", "the battle ahead")
        } else {
            template.replace("[topic]", "our path")
        };

        response
    }

    /// Generate combat decision based on personality and situation
    fn generate_combat_decision(
        &self,
        personality: &PersonalityTraits,
        health_pct: f32,
    ) -> CombatDecision {
        // Aggressive personalities attack more
        if personality.aggression > 0.7 && health_pct > 30.0 {
            if rand::random::<f32>() > 0.5 {
                CombatDecision::SpecialAttack("Fury Strike".to_string())
            } else {
                CombatDecision::BasicAttack
            }
        }
        // Low health - defensive or desperate
        else if health_pct < 30.0 {
            if personality.wisdom > 0.6 {
                CombatDecision::ChangeStance("Defensive".to_string())
            } else {
                CombatDecision::SpecialAttack("Desperate Assault".to_string())
            }
        }
        // Spiritual characters summon
        else if personality.spirituality > 0.7 && rand::random::<f32>() > 0.7 {
            CombatDecision::SummonMinion("Spirit".to_string())
        }
        // Tactical default
        else {
            if rand::random::<f32>() > 0.6 {
                CombatDecision::UseAbility("Power Strike".to_string())
            } else {
                CombatDecision::BasicAttack
            }
        }
    }
}

impl LlmBackend for PlaceholderBackend {
    fn generate(&mut self, prompt: &str, _config: &ModelConfig) -> Result<String, LlmError> {
        // Parse prompt to extract key information
        // This is a simplified parser - real implementation would be more robust

        if prompt.contains("Combat") || prompt.contains("action") {
            // Combat decision generation
            let personality = PersonalityTraits {
                aggression: 0.7,
                wisdom: 0.5,
                chattiness: 0.3,
                honor: 0.6,
                spirituality: 0.4,
            };

            let decision = self.generate_combat_decision(&personality, 75.0);
            let json = serde_json::json!({
                "action": format!("{:?}", decision),
                "details": "tactical_decision",
                "reasoning": "based on current situation",
                "taunt": "Face my ancestral power!"
            });

            serde_json::to_string_pretty(&json).map_err(|e| {
                LlmError::GenerationFailed(format!("JSON serialization failed: {}", e))
            })
        } else {
            // Dialogue generation
            let personality = PersonalityTraits {
                aggression: 0.4,
                wisdom: 0.7,
                chattiness: 0.6,
                honor: 0.8,
                spirituality: 0.7,
            };

            let role = if prompt.contains("boss") || prompt.contains("Boss") {
                "boss"
            } else if prompt.contains("brother") || prompt.contains("Brother") {
                "brother"
            } else {
                "spirit"
            };

            Ok(self.generate_dialogue_response(&personality, prompt, role))
        }
    }

    fn is_ready(&self) -> bool {
        true // Always ready
    }

    fn name(&self) -> &str {
        "Placeholder (Rule-Based)"
    }
}

// ============================================================================
// PERSONALITY DATABASE
// ============================================================================

#[derive(Default)]
struct PersonalityDatabase {
    // Store pre-generated personality profiles for common character types
}

// ============================================================================
// RESPONSE TEMPLATES
// ============================================================================

#[derive(Default)]
struct ResponseTemplates {
    boss_dialogue: Vec<String>,
    brother_dialogue: Vec<String>,
    spirit_dialogue: Vec<String>,
    generic_dialogue: Vec<String>,
}

impl ResponseTemplates {
    fn default() -> Self {
        Self {
            boss_dialogue: vec![
                "The spirits whisper of [topic]... you cannot escape destiny.".to_string(),
                "Your strength means nothing! [topic] will be your downfall!".to_string(),
                "Interesting... [topic] reveals much about you, young shaman.".to_string(),
                "Enough talk of [topic]. Let our power speak!".to_string(),
            ],
            brother_dialogue: vec![
                "Brother, consider [topic] carefully. The ancestors guide us.".to_string(),
                "Ha! Remember when we trained? [topic] reminds me of those days.".to_string(),
                "Let's discuss [topic]. Your journey has weight, brother.".to_string(),
                "[topic]... yes, this concerns us both. Stay vigilant.".to_string(),
            ],
            spirit_dialogue: vec![
                "The winds carry truths about [topic]... listen closely.".to_string(),
                "Seek balance in [topic], child of earth.".to_string(),
                "Ancient wisdom speaks: [topic] is but one path of many.".to_string(),
                "Your spirit questions [topic]... good. Question everything.".to_string(),
            ],
            generic_dialogue: vec![
                "I sense [topic] weighs on your mind.".to_string(),
                "Tell me more about [topic].".to_string(),
                "Interesting perspective on [topic].".to_string(),
                "Let us explore [topic] together.".to_string(),
            ],
        }
    }
}

// ============================================================================
// COMBAT DECISION TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatDecision {
    BasicAttack,
    SpecialAttack(String),
    SummonMinion(String),
    ChangeStance(String),
    UseAbility(String),
    SpreadCorruption,
}

// ============================================================================
// GGUF BACKEND (Actual implementation with llama-cpp-2)
// ============================================================================

#[cfg(feature = "llm")]
use llama_cpp_2::{
    context::params::LlamaContextParams,
    llama_backend::LlamaBackend as LlamaCppBackend,
    llama_batch::LlamaBatch,
    model::{params::LlamaModelParams, AddBos, LlamaModel as LlamaCppModel},
    token::data_array::LlamaTokenDataArray,
};

#[cfg(feature = "llm")]
pub struct GgufBackend {
    model_path: String,
    is_loaded: bool,
    model: Option<LlamaCppModel>,
    backend: Option<LlamaCppBackend>,
    n_ctx: u32,
    n_batch: u32,
}

#[cfg(feature = "llm")]
impl GgufBackend {
    pub fn new(model_path: String) -> Self {
        Self {
            model_path,
            is_loaded: false,
            model: None,
            backend: None,
            n_ctx: 2048,  // Context window size
            n_batch: 512, // Batch size for processing
        }
    }

    pub fn load_model(&mut self) -> Result<(), LlmError> {
        info!("Loading GGUF model from: {}", self.model_path);

        // Check if file exists
        if !std::path::Path::new(&self.model_path).exists() {
            return Err(LlmError::GenerationFailed(format!(
                "Model file not found: {}",
                self.model_path
            )));
        }

        // Initialize llama.cpp backend
        let backend = LlamaCppBackend::init()
            .map_err(|e| LlmError::GenerationFailed(format!("Backend init failed: {}", e)))?;

        // Set up model parameters
        let model_params = LlamaModelParams::default();

        // Load the model
        let model = LlamaCppModel::load_from_file(&backend, &self.model_path, &model_params)
            .map_err(|e| LlmError::GenerationFailed(format!("Model load failed: {}", e)))?;

        self.model = Some(model);
        self.backend = Some(backend);
        self.is_loaded = true;

        info!("GGUF model loaded successfully");
        Ok(())
    }

    #[allow(dead_code)]
    fn generate_impl(&mut self, prompt: &str, config: &ModelConfig) -> Result<String, LlmError> {
        let model = self.model.as_ref().ok_or(LlmError::ModelNotLoaded)?;

        let backend = self.backend.as_ref().ok_or(LlmError::ModelNotLoaded)?;

        // Create context parameters
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(std::num::NonZeroU32::new(self.n_ctx))
            .with_n_batch(self.n_batch)
            .with_seed(config.seed.unwrap_or(42));

        // Create context
        let mut ctx = model
            .new_context(backend, ctx_params)
            .map_err(|e| LlmError::GenerationFailed(format!("Context creation failed: {}", e)))?;

        // Tokenize the prompt
        let tokens = model
            .str_to_token(prompt, AddBos::Always)
            .map_err(|e| LlmError::GenerationFailed(format!("Tokenization failed: {}", e)))?;

        if tokens.is_empty() {
            return Err(LlmError::InvalidPrompt);
        }

        info!("Tokenized prompt: {} tokens", tokens.len());

        // Create batch for processing
        let mut batch = LlamaBatch::new(self.n_batch as usize, 1);

        // Add tokens to batch
        let last_index = (tokens.len() - 1) as i32;
        for (i, token) in tokens.into_iter().enumerate() {
            let is_last = i as i32 == last_index;
            batch
                .add(token, i as i32, &[0], is_last)
                .map_err(|e| LlmError::GenerationFailed(format!("Batch add failed: {}", e)))?;
        }

        // Decode the batch
        ctx.decode(&mut batch)
            .map_err(|e| LlmError::GenerationFailed(format!("Decode failed: {}", e)))?;

        // Generate response tokens
        let mut response_tokens = Vec::new();
        let max_tokens = config.max_tokens.unwrap_or(256);

        for _ in 0..max_tokens {
            // Get logits for the last token
            let candidates = ctx.candidates_ith(batch.n_tokens() - 1);

            let mut candidates_array = LlamaTokenDataArray::from_iter(candidates, false);

            // Sample next token using temperature
            let next_token = if config.temperature > 0.0 {
                candidates_array.sample_token_greedy(&mut ctx)
            } else {
                candidates_array.sample_token_greedy(&mut ctx)
            };

            // Check for end of sequence
            if model.is_eog_token(next_token) {
                break;
            }

            response_tokens.push(next_token);

            // Clear batch and add new token
            batch.clear();
            batch
                .add(next_token, response_tokens.len() as i32, &[0], true)
                .map_err(|e| LlmError::GenerationFailed(format!("Batch add failed: {}", e)))?;

            // Decode for next iteration
            ctx.decode(&mut batch)
                .map_err(|e| LlmError::GenerationFailed(format!("Decode failed: {}", e)))?;
        }

        // Convert tokens back to string
        let response = model
            .token_to_str(response_tokens.as_slice())
            .map_err(|e| LlmError::GenerationFailed(format!("Detokenization failed: {}", e)))?;

        info!("Generated response: {} tokens", response_tokens.len());
        Ok(response)
    }
}

#[cfg(feature = "llm")]
impl LlmBackend for GgufBackend {
    fn generate(&mut self, prompt: &str, config: &ModelConfig) -> Result<String, LlmError> {
        if !self.is_loaded {
            return Err(LlmError::ModelNotLoaded);
        }

        self.generate_impl(prompt, config)
    }

    fn is_ready(&self) -> bool {
        self.is_loaded
    }

    fn name(&self) -> &str {
        "GGUF Local Model"
    }
}

// ============================================================================
// BACKEND FACTORY
// ============================================================================

pub fn create_backend(model: &LlmModel) -> Box<dyn LlmBackend> {
    #[cfg(feature = "llm")]
    {
        if model.is_loaded && std::path::Path::new(&model.model_path).exists() {
            info!("Attempting to load GGUF model from: {}", model.model_path);
            let mut backend = GgufBackend::new(model.model_path.clone());

            if backend.load_model().is_ok() {
                info!("GGUF model loaded successfully");
                return Box::new(backend);
            }
        }
    }

    // Fallback to placeholder
    info!("Using placeholder LLM backend (rule-based responses)");
    Box::new(PlaceholderBackend::new())
}
