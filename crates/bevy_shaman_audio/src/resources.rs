use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// BEAT CLOCK
// ============================================================================

/// Global beat clock for rhythm evaluation
#[derive(Resource, Debug, Clone)]
pub struct BeatClock {
    pub bpm: f32,
    pub beat_duration: f32, // seconds per beat
    pub current_beat: u32,
    pub time_in_beat: f32,  // 0.0 to beat_duration
    pub is_playing: bool,
}

impl Default for BeatClock {
    fn default() -> Self {
        let bpm = 120.0;
        Self {
            bpm,
            beat_duration: 60.0 / bpm,
            current_beat: 0,
            time_in_beat: 0.0,
            is_playing: false,
        }
    }
}

impl BeatClock {
    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
        self.beat_duration = 60.0 / bpm;
    }

    pub fn tick(&mut self, delta: f32) {
        if !self.is_playing {
            return;
        }

        self.time_in_beat += delta;
        while self.time_in_beat >= self.beat_duration {
            self.current_beat += 1;
            self.time_in_beat -= self.beat_duration;
        }
    }

    /// Returns timing quality: Perfect/Great/Good/Miss
    pub fn evaluate_timing(&self) -> TimingQuality {
        // Normalized time within beat (0.0 to 1.0)
        let normalized_time = self.time_in_beat / self.beat_duration;
        // Distance from nearest beat (start at 0.0 or end at 1.0)
        let distance_from_beat = normalized_time.min(1.0 - normalized_time);

        // Thresholds are fractions of a beat
        if distance_from_beat < 0.05 {
            TimingQuality::Perfect
        } else if distance_from_beat < 0.15 {
            TimingQuality::Great
        } else if distance_from_beat < 0.3 {
            TimingQuality::Good
        } else {
            TimingQuality::Miss
        }
    }
}

// Re-export TimingQuality from core to maintain compatibility
pub use bevy_shaman_core::resources::TimingQuality;

// ============================================================================
// SONG DATABASE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: String,
    pub display_name: String,
    pub bpm: f32,
    pub style: MusicStyle,
    pub unlock_level: u8,
    pub spirit_cost: f32,
    pub stamina_cost: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MusicStyle {
    Calm,
    Aggressive,
    Purifying,
    Harmonizing,
}

#[derive(Resource, Default)]
pub struct SongDB {
    pub songs: HashMap<String, Song>,
}

impl SongDB {
    pub fn register(&mut self, song: Song) {
        self.songs.insert(song.id.clone(), song);
    }

    pub fn get(&self, id: &str) -> Option<&Song> {
        self.songs.get(id)
    }

    pub fn populate_defaults(&mut self) {
        self.register(Song {
            id: "dawn_hymn".to_string(),
            display_name: "Dawn Hymn".to_string(),
            bpm: 90.0,
            style: MusicStyle::Calm,
            unlock_level: 1,
            spirit_cost: 10.0,
            stamina_cost: 5.0,
        });

        self.register(Song {
            id: "war_chant".to_string(),
            display_name: "War Chant".to_string(),
            bpm: 140.0,
            style: MusicStyle::Aggressive,
            unlock_level: 3,
            spirit_cost: 15.0,
            stamina_cost: 20.0,
        });

        self.register(Song {
            id: "purification_rite".to_string(),
            display_name: "Purification Rite".to_string(),
            bpm: 80.0,
            style: MusicStyle::Purifying,
            unlock_level: 2,
            spirit_cost: 25.0,
            stamina_cost: 10.0,
        });
    }
}

// ============================================================================
// ACTIVE SONG
// ============================================================================

#[derive(Resource, Default)]
pub struct ActiveSong {
    pub current_song_id: Option<String>,
    pub combo: u32,
}

impl ActiveSong {
    pub fn start_song(&mut self, song_id: String) {
        self.current_song_id = Some(song_id);
        self.combo = 0;
    }

    pub fn stop_song(&mut self) {
        self.current_song_id = None;
        self.combo = 0;
    }

    pub fn increment_combo(&mut self) {
        self.combo += 1;
    }

    pub fn break_combo(&mut self) {
        self.combo = 0;
    }
}
