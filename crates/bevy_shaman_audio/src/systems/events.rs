use crate::resources::TimingQuality;
use bevy::prelude::*;

#[derive(Event)]
pub struct BeatHit {
    pub beat_number: u32,
}

#[derive(Event)]
pub struct RhythmInputEvaluated {
    pub quality: TimingQuality,
    pub combo: u32,
}
