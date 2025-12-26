use bevy::prelude::*;
use crate::resources::TimingQuality;

#[derive(Event)]
pub struct BeatHit {
    pub beat_number: u32,
}

#[derive(Event)]
pub struct RhythmInputEvaluated {
    pub quality: TimingQuality,
    pub combo: u32,
}
