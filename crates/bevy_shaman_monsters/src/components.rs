use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// MONSTER STATE & METERS
// ============================================================================

/// Monster state enum + numeric meters for smooth transitions
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MonsterState {
    pub state: StateType,
    pub stability_meter: f32,  // 0.0 = fully chaotic, 1.0 = fully stable
    pub corruption_meter: f32, // 0.0 = pure, 1.0 = fully corrupt
    pub obedience_meter: f32,  // 0.0 = wild, 1.0 = tame
    pub chaos_output: f32,     // damage multiplier when chaotic
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StateType {
    Stable,
    Chaos,
    Corrupt,
    Harmony,   // Spirit World alignment
    Decay,     // Spirit World alignment
    Rage,      // Spirit World alignment
    Void,      // Spirit World alignment
    Ancestral, // Spirit World alignment
}

impl Default for MonsterState {
    fn default() -> Self {
        Self {
            state: StateType::Stable,
            stability_meter: 0.8,
            corruption_meter: 0.0,
            obedience_meter: 0.5,
            chaos_output: 1.0,
        }
    }
}

impl MonsterState {
    /// Evaluate and return new state based on meters
    pub fn evaluate_state(&self) -> StateType {
        // Priority: Corruption > Chaos > Harmony
        if self.corruption_meter > 0.7 {
            StateType::Corrupt
        } else if self.stability_meter < 0.3 {
            StateType::Chaos
        } else if self.stability_meter > 0.85 && self.corruption_meter < 0.1 {
            StateType::Harmony
        } else {
            StateType::Stable
        }
    }

    pub fn is_controllable(&self) -> bool {
        self.obedience_meter > 0.3 && self.corruption_meter < 0.8
    }
}

// ============================================================================
// MONSTER PERSONALITY & MUSIC AFFINITY
// ============================================================================

/// Defines monster temperament and music preferences
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MusicAffinityProfile {
    /// Likes calm, steady music
    pub prefers_calm: f32,
    /// Likes fast, syncopated music
    pub prefers_aggressive: f32,
    /// Resistance to corruption
    pub corruption_resistance: f32,
    /// Learned trust value with player
    pub trust_level: f32,
}

impl Default for MusicAffinityProfile {
    fn default() -> Self {
        Self {
            prefers_calm: 0.5,
            prefers_aggressive: 0.5,
            corruption_resistance: 0.5,
            trust_level: 0.0,
        }
    }
}

// ============================================================================
// MONSTER IDENTITY & STATS
// ============================================================================

/// Monster template ID for sprite/behavior lookups
#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MonsterId(pub String);

/// Monster stats (data-oriented: single struct for cache locality)
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MonsterStats {
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
    pub spirit_affinity: f32,
}

impl Default for MonsterStats {
    fn default() -> Self {
        Self {
            attack: 10.0,
            defense: 5.0,
            speed: 5.0,
            spirit_affinity: 0.5,
        }
    }
}

// ============================================================================
// CORRUPTION INFLUENCE
// ============================================================================

/// Component marking monsters that can spread corruption
#[derive(Component, Debug, Clone, Copy)]
pub struct CorruptionInfluence {
    pub radius: f32,
    pub strength: f32,
}

/// Component tracking exposure to corruption
#[derive(Component, Default)]
pub struct CorruptionExposure {
    pub accumulated: f32,
    pub sources: Vec<Entity>,
}

// ============================================================================
// AI & BEHAVIOR
// ============================================================================

/// AI behavior tree identifier
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct AiBehavior {
    pub behavior_tree_id: String,
    pub aggression: f32,
    pub flee_threshold: f32, // health % to flee
}

impl Default for AiBehavior {
    fn default() -> Self {
        Self {
            behavior_tree_id: "default".to_string(),
            aggression: 0.5,
            flee_threshold: 0.2,
        }
    }
}

/// Current AI state
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiState {
    Idle,
    Patrol,
    Pursuing,
    Aggressive,
    Fleeing,
    Stunned,
}

impl Default for AiState {
    fn default() -> Self {
        Self::Idle
    }
}

// ============================================================================
// TAMING
// ============================================================================

/// Marks a monster as tamed/controlled by player
#[derive(Component)]
pub struct Tamed {
    pub tamed_at: f64, // timestamp
}

// ============================================================================
// PATHFINDING CACHE
// ============================================================================

/// Caches pathfinding results to avoid recalculating paths every frame
/// Invalidated when target moves or obstacles change
#[derive(Component, Default)]
pub struct PathCache {
    pub target: Option<Entity>,
    pub target_last_position: Option<bevy_shaman_core::components::GridPosition>,
    pub cached_path: Option<Vec<bevy_shaman_core::components::GridPosition>>,
    pub path_index: usize, // Current position in the cached path
}

impl PathCache {
    pub fn invalidate(&mut self) {
        self.cached_path = None;
        self.path_index = 0;
    }

    pub fn is_valid_for(&self, target: Entity, target_pos: &bevy_shaman_core::components::GridPosition) -> bool {
        if let (Some(cached_target), Some(cached_pos)) = (self.target, self.target_last_position) {
            cached_target == target && cached_pos == *target_pos && self.cached_path.is_some()
        } else {
            false
        }
    }

    pub fn cache_path(
        &mut self,
        target: Entity,
        target_pos: bevy_shaman_core::components::GridPosition,
        path: Vec<bevy_shaman_core::components::GridPosition>,
    ) {
        self.target = Some(target);
        self.target_last_position = Some(target_pos);
        self.cached_path = Some(path);
        self.path_index = 0;
    }

    pub fn get_next_step(&mut self) -> Option<bevy_shaman_core::components::GridPosition> {
        if let Some(path) = &self.cached_path {
            if self.path_index < path.len() {
                let step = path[self.path_index];
                self.path_index += 1;
                Some(step)
            } else {
                None
            }
        } else {
            None
        }
    }
}
