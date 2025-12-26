use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Top-level game flow state machine
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Serialize, Deserialize)]
pub enum GameState {
    #[default]
    Boot,
    MainMenu,
    Playing,
    Paused,
    Dialogue,
    Cutscene,
}

/// World location state machine
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Serialize, Deserialize)]
pub enum WorldState {
    #[default]
    Overworld,
    SpiritWorld(SpiritWorldId),
    Dungeon,
    BossArena,
}

/// Five Spirit Worlds aligned to world-states
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SpiritWorldId {
    Harmony,
    Chaos,
    Decay,
    Void,
    Ancestral,
}

/// Combat encounter state
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Serialize, Deserialize)]
pub enum CombatState {
    #[default]
    None,
    Encounter,
    Boss,
}
