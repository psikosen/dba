use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Time of day progression (affects corruption spread rates, encounter pools)
#[derive(Resource, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeOfDay {
    /// 0.0 = midnight, 0.5 = noon, 1.0 = midnight
    pub normalized: f32,
    pub speed: f32,
}

impl Default for TimeOfDay {
    fn default() -> Self {
        Self {
            normalized: 0.25, // dawn
            speed: 0.01,      // slow progression
        }
    }
}

/// Player level and progression
#[derive(Resource, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerLevel {
    pub current: u8,
    pub experience: u32,
    pub experience_to_next: u32,
}

impl Default for PlayerLevel {
    fn default() -> Self {
        Self {
            current: 1,
            experience: 0,
            experience_to_next: 100,
        }
    }
}

impl PlayerLevel {
    pub fn add_experience(&mut self, amount: u32) -> bool {
        self.experience += amount;
        if self.experience >= self.experience_to_next {
            self.current += 1;
            self.experience -= self.experience_to_next;
            self.experience_to_next = (self.experience_to_next as f32 * 1.5) as u32;
            true // leveled up
        } else {
            false
        }
    }
}

/// Grid occupancy lookup (fast spatial queries)
#[derive(Resource, Default)]
pub struct GridOccupancy {
    pub occupied: std::collections::HashSet<(i32, i32)>,
}

impl GridOccupancy {
    pub fn is_occupied(&self, x: i32, y: i32) -> bool {
        self.occupied.contains(&(x, y))
    }

    pub fn occupy(&mut self, x: i32, y: i32) {
        self.occupied.insert((x, y));
    }

    pub fn vacate(&mut self, x: i32, y: i32) {
        self.occupied.remove(&(x, y));
    }
}
