use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// GRID & MOVEMENT COMPONENTS
// ============================================================================

/// Grid position (data-oriented: single Vec2i for cache efficiency)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &GridPosition) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

/// Movement command queue (stateless: systems consume and clear)
#[derive(Component, Default)]
pub struct MovementQueue {
    pub commands: Vec<MovementCommand>,
}

#[derive(Debug, Clone, Copy)]
pub enum MovementCommand {
    Move(IVec2),
    Dash(IVec2),
    Teleport(GridPosition),
}

/// Marks entities that block grid tiles
#[derive(Component)]
pub struct BlocksMovement;

// ============================================================================
// PLAYER RESOURCES
// ============================================================================

/// Spirit energy for purification, rituals, special songs
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Spirit {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
}

impl Default for Spirit {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
            regen_rate: 5.0, // per second
        }
    }
}

impl Spirit {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            regen_rate: 5.0,
        }
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }
}

/// Physical endurance for movement, dodges, sustained rhythm actions
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
}

impl Default for Stamina {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
            regen_rate: 10.0, // per second
        }
    }
}

impl Stamina {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
            regen_rate: 10.0,
        }
    }
}

// ============================================================================
// ANIMATION
// ============================================================================

/// Sprite animation state (data-only)
#[derive(Component)]
pub struct SpriteAnimation {
    pub timer: Timer,
    pub frame_count: usize,
    pub current_frame: usize,
    pub looping: bool,
}

impl SpriteAnimation {
    pub fn new(frame_duration: f32, frame_count: usize, looping: bool) -> Self {
        Self {
            timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            frame_count,
            current_frame: 0,
            looping,
        }
    }
}

// ============================================================================
// MARKERS & TAGS
// ============================================================================

/// Player entity marker
#[derive(Component)]
pub struct Player;

/// Camera follow target marker
#[derive(Component)]
pub struct CameraTarget;

/// Health component (generic, used by player and monsters)
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
}

// ============================================================================
// COMBAT RESOURCES
// ============================================================================

/// Combat difficulty levels (affects BloodLust gain)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatDifficulty {
    Easy,
    Normal,
    Hard,
    Boss,
}

/// Blood lust mechanic - rises from violence, reduced by music/plants
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BloodLust {
    pub current: f32,      // 0.0 to 100.0
    pub threshold: f32,    // When it triggers corruption
    pub decay_rate: f32,   // How fast it decays out of combat
}

impl Default for BloodLust {
    fn default() -> Self {
        Self {
            current: 0.0,
            threshold: 70.0,
            decay_rate: 5.0,  // Per second
        }
    }
}

impl BloodLust {
    pub fn is_corrupting(&self) -> bool {
        self.current >= self.threshold
    }

    pub fn add_from_combat(&mut self, _enemy_health: f32, was_overkill: bool, difficulty: CombatDifficulty) {
        let base_gain = match difficulty {
            CombatDifficulty::Easy => 2.0,
            CombatDifficulty::Normal => 5.0,
            CombatDifficulty::Hard => 10.0,
            CombatDifficulty::Boss => 15.0,
        };

        let overkill_multiplier = if was_overkill { 2.0 } else { 1.0 };
        self.current = (self.current + base_gain * overkill_multiplier).min(100.0);
    }

    pub fn reduce_with_music(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn reduce_with_plant(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }

    pub fn reduce_with_food(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
}
