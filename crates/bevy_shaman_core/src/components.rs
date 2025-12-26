use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// GRID & MOVEMENT COMPONENTS
// ============================================================================

/// Grid position (data-oriented: single Vec2i for cache efficiency)
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
