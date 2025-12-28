use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Game settings resource
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameSettings {
    pub audio: AudioSettings,
    pub gameplay: GameplaySettings,
    pub controls: ControlSettings,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            audio: AudioSettings::default(),
            gameplay: GameplaySettings::default(),
            controls: ControlSettings::default(),
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

/// Control scheme settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlSettings {
    pub scheme: ControlScheme,
    pub gamepad_enabled: bool,
    pub gamepad_deadzone: u8,  // 0-100 (percentage)
}

impl Default for ControlSettings {
    fn default() -> Self {
        Self {
            scheme: ControlScheme::WASD,
            gamepad_enabled: true,
            gamepad_deadzone: 15,  // 15% deadzone
        }
    }
}

/// Available control schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlScheme {
    WASD,
    Arrows,
    ESDF,
    Gamepad,
}

impl ControlScheme {
    pub fn name(&self) -> &'static str {
        match self {
            ControlScheme::WASD => "WASD",
            ControlScheme::Arrows => "Arrow Keys",
            ControlScheme::ESDF => "ESDF",
            ControlScheme::Gamepad => "Gamepad Only",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            ControlScheme::WASD => ControlScheme::Arrows,
            ControlScheme::Arrows => ControlScheme::ESDF,
            ControlScheme::ESDF => ControlScheme::Gamepad,
            ControlScheme::Gamepad => ControlScheme::WASD,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            ControlScheme::WASD => ControlScheme::Gamepad,
            ControlScheme::Arrows => ControlScheme::WASD,
            ControlScheme::ESDF => ControlScheme::Arrows,
            ControlScheme::Gamepad => ControlScheme::ESDF,
        }
    }

    /// Get the movement keys for this scheme
    pub fn movement_keys(&self) -> MovementKeys {
        match self {
            ControlScheme::WASD => MovementKeys {
                up: KeyCode::KeyW,
                down: KeyCode::KeyS,
                left: KeyCode::KeyA,
                right: KeyCode::KeyD,
            },
            ControlScheme::Arrows => MovementKeys {
                up: KeyCode::ArrowUp,
                down: KeyCode::ArrowDown,
                left: KeyCode::ArrowLeft,
                right: KeyCode::ArrowRight,
            },
            ControlScheme::ESDF => MovementKeys {
                up: KeyCode::KeyE,
                down: KeyCode::KeyD,
                left: KeyCode::KeyS,
                right: KeyCode::KeyF,
            },
            ControlScheme::Gamepad => MovementKeys {
                // Fallback to WASD when gamepad scheme is selected
                up: KeyCode::KeyW,
                down: KeyCode::KeyS,
                left: KeyCode::KeyA,
                right: KeyCode::KeyD,
            },
        }
    }
}

/// Movement key configuration
#[derive(Debug, Clone, Copy)]
pub struct MovementKeys {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
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
        info!("Saving settings: master_volume={}, music_volume={}, sfx_volume={}, difficulty={:?}, control_scheme={:?}",
            self.audio.master_volume,
            self.audio.music_volume,
            self.audio.sfx_volume,
            self.gameplay.difficulty,
            self.controls.scheme
        );
    }
}
