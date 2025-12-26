use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// INSTRUMENT CHOICE
// ============================================================================

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstrumentChoice {
    #[default]
    NotChosen,
    Kora,
    Ngoni,
}

impl InstrumentChoice {
    /// Kora: stronger stability/purification, easier control
    pub fn stability_modifier(&self) -> f32 {
        match self {
            InstrumentChoice::Kora => 1.3,
            InstrumentChoice::Ngoni => 0.9,
            InstrumentChoice::NotChosen => 1.0,
        }
    }

    /// Ngoni: higher damage, stronger state pushing
    pub fn damage_modifier(&self) -> f32 {
        match self {
            InstrumentChoice::Kora => 0.9,
            InstrumentChoice::Ngoni => 1.3,
            InstrumentChoice::NotChosen => 1.0,
        }
    }

    /// Kora: better control, Ngoni: higher stamina costs
    pub fn stamina_cost_modifier(&self) -> f32 {
        match self {
            InstrumentChoice::Kora => 0.8,
            InstrumentChoice::Ngoni => 1.2,
            InstrumentChoice::NotChosen => 1.0,
        }
    }

    /// Kora: easier to control minions
    pub fn obedience_modifier(&self) -> f32 {
        match self {
            InstrumentChoice::Kora => 1.2,
            InstrumentChoice::Ngoni => 0.8,
            InstrumentChoice::NotChosen => 1.0,
        }
    }
}

// ============================================================================
// BROTHER CLEANSING PROGRESS
// ============================================================================

#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize)]
pub struct BrotherCleansingProgress {
    pub fights_completed: u8,
    pub total_fights: u8,
}

impl BrotherCleansingProgress {
    pub fn new() -> Self {
        Self {
            fights_completed: 0,
            total_fights: 4,
        }
    }

    pub fn complete_fight(&mut self) {
        if self.fights_completed < self.total_fights {
            self.fights_completed += 1;
        }
    }

    pub fn is_fully_cleansed(&self) -> bool {
        self.fights_completed >= self.total_fights
    }

    pub fn corruption_percentage(&self) -> f32 {
        1.0 - (self.fights_completed as f32 / self.total_fights as f32)
    }
}
