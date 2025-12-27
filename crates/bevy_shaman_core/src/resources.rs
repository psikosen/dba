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

/// UI visibility toggles
#[derive(Resource, Default)]
pub struct InventoryVisible(pub bool);

#[derive(Resource, Default)]
pub struct DialogueVisible(pub bool);

#[derive(Resource, Default)]
pub struct CalendarVisible(pub bool);

/// Season types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    pub fn name(&self) -> &str {
        match self {
            Season::Spring => "Spring",
            Season::Summer => "Summer",
            Season::Autumn => "Autumn",
            Season::Winter => "Winter",
        }
    }
}

/// Festival type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Festival {
    pub name: String,
    pub day: u32,
    pub season: Season,
    pub description: String,
}

/// In-game calendar with time tracking and festivals
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GameCalendar {
    /// Current year (starts at 1)
    pub year: u32,
    /// Current season
    pub season: Season,
    /// Current day within season (1-30)
    pub day: u32,
    /// Current hour (0-23)
    pub hour: u32,
    /// Current minute (0-59)
    pub minute: u32,
    /// Time accumulator for hour progression
    pub time_accumulator: f32,
    /// Real seconds per in-game hour (default: 60s = 1 hour)
    pub seconds_per_hour: f32,
    /// Upcoming festivals
    pub festivals: Vec<Festival>,
}

impl Default for GameCalendar {
    fn default() -> Self {
        Self {
            year: 1,
            season: Season::Spring,
            day: 1,
            hour: 6, // Start at dawn
            minute: 0,
            time_accumulator: 0.0,
            seconds_per_hour: 60.0, // 1 real minute = 1 game hour
            festivals: Self::default_festivals(),
        }
    }
}

impl GameCalendar {
    /// Update calendar with delta time
    pub fn update(&mut self, delta_seconds: f32) {
        self.time_accumulator += delta_seconds;

        while self.time_accumulator >= self.seconds_per_hour {
            self.time_accumulator -= self.seconds_per_hour;
            self.advance_hour();
        }

        // Update minute display (for UI smoothness)
        self.minute = ((self.time_accumulator / self.seconds_per_hour) * 60.0) as u32;
    }

    /// Advance by one hour
    fn advance_hour(&mut self) {
        self.hour += 1;
        if self.hour >= 24 {
            self.hour = 0;
            self.advance_day();
        }
    }

    /// Advance by one day
    fn advance_day(&mut self) {
        self.day += 1;
        if self.day > 30 {
            self.day = 1;
            self.advance_season();
        }
    }

    /// Advance by one season
    fn advance_season(&mut self) {
        self.season = match self.season {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Autumn,
            Season::Autumn => Season::Winter,
            Season::Winter => {
                self.year += 1;
                Season::Spring
            }
        };
    }

    /// Get current time as formatted string (HH:MM)
    pub fn time_string(&self) -> String {
        format!("{:02}:{:02}", self.hour, self.minute)
    }

    /// Get current date as formatted string
    pub fn date_string(&self) -> String {
        format!("{} {}, Year {}", self.season.name(), self.day, self.year)
    }

    /// Check if today is a festival day
    pub fn get_active_festival(&self) -> Option<&Festival> {
        self.festivals.iter()
            .find(|f| f.season == self.season && f.day == self.day)
    }

    /// Get default festivals
    fn default_festivals() -> Vec<Festival> {
        vec![
            Festival {
                name: "Festival of Renewal".to_string(),
                day: 1,
                season: Season::Spring,
                description: "Celebrating new beginnings and the awakening of spirits".to_string(),
            },
            Festival {
                name: "Midsummer Celebration".to_string(),
                day: 15,
                season: Season::Summer,
                description: "Honor the sun and the balance of light and shadow".to_string(),
            },
            Festival {
                name: "Harvest Moon Festival".to_string(),
                day: 20,
                season: Season::Autumn,
                description: "Give thanks for the bounty and prepare for winter".to_string(),
            },
            Festival {
                name: "Winter Solstice Vigil".to_string(),
                day: 10,
                season: Season::Winter,
                description: "Light fires to guide spirits through the longest night".to_string(),
            },
        ]
    }
}

// ============================================================================
// RHYTHM & TIMING
// ============================================================================

/// Timing quality for rhythm-based combat
/// Moved from audio crate to avoid circular dependencies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimingQuality {
    Perfect,
    Great,
    Good,
    Miss,
}

impl TimingQuality {
    pub fn damage_multiplier(&self) -> f32 {
        match self {
            TimingQuality::Perfect => 1.5,
            TimingQuality::Great => 1.2,
            TimingQuality::Good => 1.0,
            TimingQuality::Miss => 0.5,
        }
    }

    pub fn control_modifier(&self) -> f32 {
        match self {
            TimingQuality::Perfect => 1.3,
            TimingQuality::Great => 1.1,
            TimingQuality::Good => 1.0,
            TimingQuality::Miss => 0.7,
        }
    }
}

// ============================================================================
// SAVE/LOAD
// ============================================================================

/// Flag to indicate we're loading from a save file
/// This prevents duplicate world generation and player spawning
#[derive(Resource, Default)]
pub struct LoadingFromSave {
    pub is_loading: bool,
}
