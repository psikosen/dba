use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// LLM AI COMPONENTS
// ============================================================================

/// Marks an entity as using LLM-driven AI (bosses, important NPCs)
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct LlmAi {
    /// Character name for context
    pub character_name: String,
    /// Character role/type for prompt engineering
    pub role: AiRole,
    /// Personality traits that influence responses
    pub personality: PersonalityTraits,
    /// Current emotional state (affects dialogue tone)
    pub emotional_state: EmotionalState,
    /// Combat stance or strategy
    pub combat_stance: CombatStance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiRole {
    Boss,           // Major boss encounters
    MiniBoss,       // Mini-boss encounters
    Brother,        // Player's brother NPCs
    SpiritGuide,    // Spirit world guides
    Antagonist,     // Story antagonists
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraits {
    /// Aggressive vs Peaceful (0.0 to 1.0)
    pub aggression: f32,
    /// Wise vs Reckless (0.0 to 1.0)
    pub wisdom: f32,
    /// Talkative vs Silent (0.0 to 1.0)
    pub chattiness: f32,
    /// Honorable vs Deceitful (0.0 to 1.0)
    pub honor: f32,
    /// Spiritual affinity (0.0 to 1.0)
    pub spirituality: f32,
}

impl Default for PersonalityTraits {
    fn default() -> Self {
        Self {
            aggression: 0.5,
            wisdom: 0.5,
            chattiness: 0.5,
            honor: 0.7,
            spirituality: 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EmotionalState {
    Calm,
    Angry,
    Fearful,
    Confident,
    Desperate,
    Mocking,
    Respectful,
    Corrupted,    // For corrupted entities
}

impl Default for EmotionalState {
    fn default() -> Self {
        EmotionalState::Calm
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CombatStance {
    Aggressive,   // Rush down, high damage
    Defensive,    // Block, counter
    Tactical,     // Use environment, special moves
    Evasive,      // Dodge-focused
    Summoner,     // Call minions
    Corrupting,   // Spread corruption
}

impl Default for CombatStance {
    fn default() -> Self {
        CombatStance::Tactical
    }
}

// ============================================================================
// QUERY QUEUE SYSTEM
// ============================================================================

/// Queue of pre-generated LLM queries for this entity
#[derive(Component, Default)]
pub struct LlmQueryQueue {
    /// Queued dialogue responses (pre-generated)
    pub dialogue_responses: Vec<QueuedResponse>,
    /// Queued combat decisions (pre-generated)
    pub combat_decisions: Vec<QueuedCombatDecision>,
    /// Maximum queue size before regeneration
    pub max_queue_size: usize,
}

impl LlmQueryQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            dialogue_responses: Vec::new(),
            combat_decisions: Vec::new(),
            max_queue_size: max_size,
        }
    }

    pub fn needs_dialogue_refill(&self) -> bool {
        self.dialogue_responses.len() < self.max_queue_size / 2
    }

    pub fn needs_combat_refill(&self) -> bool {
        self.combat_decisions.len() < self.max_queue_size / 2
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedResponse {
    /// The dialogue text
    pub text: String,
    /// Context this was generated for
    pub context: DialogueContext,
    /// When this was generated (for freshness)
    pub generated_at: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueContext {
    Greeting,
    Combat,
    Victory,
    Defeat,
    Taunt,
    PhaseTransition,
    PlayerLowHealth,
    PlayerHighCorruption,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedCombatDecision {
    /// The action to take
    pub action: CombatAction,
    /// Reasoning (for debugging/tuning)
    pub reasoning: String,
    /// Conditions for using this action
    pub conditions: Vec<CombatCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatAction {
    BasicAttack,
    SpecialMove(String),
    SummonMinion(String),
    Retreat,
    ChangeStance(CombatStance),
    UseAbility(String),
    SpreadCorruption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatCondition {
    HealthBelow(f32),
    HealthAbove(f32),
    PlayerHealthBelow(f32),
    CorruptionAbove(f32),
    DistanceToPlayer(DistanceCheck),
    PhaseNumber(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistanceCheck {
    LessThan(f32),
    GreaterThan(f32),
    InRange(f32, f32), // min, max
}

// ============================================================================
// CONVERSATION CONTEXT
// ============================================================================

/// Tracks conversation history for coherent dialogue
#[derive(Component, Default)]
pub struct ConversationHistory {
    pub messages: Vec<ConversationMessage>,
    pub max_history: usize,
}

impl ConversationHistory {
    pub fn new(max_history: usize) -> Self {
        Self {
            messages: Vec::new(),
            max_history,
        }
    }

    pub fn add_player_message(&mut self, message: String, timestamp: f64) {
        self.messages.push(ConversationMessage {
            speaker: Speaker::Player,
            text: message,
            timestamp,
        });
        self.trim_history();
    }

    pub fn add_npc_message(&mut self, message: String, timestamp: f64) {
        self.messages.push(ConversationMessage {
            speaker: Speaker::Npc,
            text: message,
            timestamp,
        });
        self.trim_history();
    }

    fn trim_history(&mut self) {
        if self.messages.len() > self.max_history {
            self.messages.remove(0);
        }
    }

    pub fn get_context_string(&self) -> String {
        self.messages
            .iter()
            .map(|m| format!("{}: {}", m.speaker, m.text))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Clone)]
pub struct ConversationMessage {
    pub speaker: Speaker,
    pub text: String,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speaker {
    Player,
    Npc,
}

impl std::fmt::Display for Speaker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Speaker::Player => write!(f, "Player"),
            Speaker::Npc => write!(f, "NPC"),
        }
    }
}

// ============================================================================
// LLM GENERATION REQUEST
// ============================================================================

/// Component for entities requesting LLM generation
#[derive(Component)]
pub struct LlmGenerationRequest {
    pub request_type: GenerationType,
    pub priority: RequestPriority,
    pub requested_at: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationType {
    Dialogue,
    CombatDecision,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RequestPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,  // Boss encounters
}
