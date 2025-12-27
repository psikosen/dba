use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Game settings resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    pub audio: AudioSettings,
    pub gameplay: GameplaySettings,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            audio: AudioSettings::default(),
            gameplay: GameplaySettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub master_volume: f32,      // 0.0 to 1.0
    pub music_volume: f32,        // 0.0 to 1.0
    pub sfx_volume: f32,          // 0.0 to 1.0
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.7,
            music_volume: 0.7,
            sfx_volume: 0.5,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameplaySettings {
    pub difficulty: Difficulty,
    pub show_damage_numbers: bool,
    pub screen_shake: bool,
    pub auto_save: bool,
}

impl Default for GameplaySettings {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Normal,
            show_damage_numbers: true,
            screen_shake: true,
            auto_save: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub fn damage_modifier(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.7,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.5,
        }
    }

    pub fn enemy_health_modifier(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.8,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.3,
        }
    }
}

impl GameSettings {
    /// Load settings from file or create default
    pub fn load_or_default() -> Self {
        // In a real implementation, this would load from a file
        // For now, just return defaults
        Self::default()
    }

    /// Save settings to file
    pub fn save(&self) {
        // In a real implementation, this would save to a file
        // For now, just log
        info!("Saving settings: master_volume={}, music_volume={}, sfx_volume={}, difficulty={:?}",
            self.audio.master_volume,
            self.audio.music_volume,
            self.audio.sfx_volume,
            self.gameplay.difficulty
        );
    }
}
