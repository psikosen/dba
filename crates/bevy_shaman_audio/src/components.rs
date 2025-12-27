use bevy::prelude::*;

/// Marks an entity as responding to rhythm input
#[derive(Component)]
pub struct RhythmResponder {
    pub enabled: bool,
}

impl Default for RhythmResponder {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Visual beat prompt that shows when to press
#[derive(Component)]
pub struct BeatPrompt {
    /// Which beat this prompt is for
    pub target_beat: u32,
    /// Time remaining until this beat hits (in seconds)
    pub time_until_hit: f32,
    /// Whether this prompt has been consumed
    pub consumed: bool,
    /// Starting y offset from player
    pub start_y_offset: f32,
}

/// Resource to track active beat prompts
#[derive(Resource, Default)]
pub struct BeatPromptManager {
    pub active_prompts: Vec<u32>,  // Track which beats have prompts
    pub max_prompts: usize,        // Max simultaneous prompts (2)
    pub lookahead_beats: u32,      // How many beats ahead to spawn prompts
}
