use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

// ============================================================================
// PRIME VESSEL - The Biological Chaos Engine
// ============================================================================

/// The Prime Vessel - A roaming AI entity that competes with the player
/// to absorb Spirit resources. It is the catalyst of the Biological Chaos system.
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PrimeVessel {
    /// Current power level (sum of all absorbed spirit power)
    pub power_level: f32,
    /// Maximum power level cap (used for evolution thresholds)
    pub max_power_reached: f32,
    /// Current evolution tier (0-10)
    pub evolution_tier: u8,
    /// Number of Lesser Selves shed
    pub lesser_selves_shed: u32,
    /// Whether the Prime Vessel is currently active in the world
    pub is_active: bool,
    /// Whether the Prime Vessel has been defeated
    pub is_defeated: bool,
    /// Current target spirit entity (if hunting)
    pub current_target: Option<Entity>,
    /// Roaming state
    pub roaming_state: VesselRoamingState,
}

impl Default for PrimeVessel {
    fn default() -> Self {
        Self {
            power_level: 100.0,
            max_power_reached: 100.0,
            evolution_tier: 0,
            lesser_selves_shed: 0,
            is_active: true,
            is_defeated: false,
            current_target: None,
            roaming_state: VesselRoamingState::Wandering,
        }
    }
}

impl PrimeVessel {
    /// Calculate evolution threshold for next tier
    pub fn next_evolution_threshold(&self) -> f32 {
        // Exponential scaling: 500, 1500, 4000, 10000, 25000, 60000...
        500.0 * (2.5_f32).powi(self.evolution_tier as i32)
    }

    /// Check if ready to evolve (shed a Lesser Self)
    pub fn should_evolve(&self) -> bool {
        self.power_level >= self.next_evolution_threshold() && self.evolution_tier < 10
    }

    /// Get display name based on evolution tier
    pub fn title(&self) -> &'static str {
        match self.evolution_tier {
            0 => "The Nascent Vessel",
            1 => "The Awakened Vessel",
            2 => "The Hungry Vessel",
            3 => "The Gorging Vessel",
            4 => "The Ascendant Vessel",
            5 => "The Dominating Vessel",
            6 => "The Apex Vessel",
            7 => "The Transcendent Vessel",
            8 => "The Cosmic Vessel",
            9 => "The Infinite Vessel",
            10 => "The Prime Chaos",
            _ => "The Vessel",
        }
    }
}

/// Roaming state for the Prime Vessel AI
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VesselRoamingState {
    /// Wandering randomly through the world
    Wandering,
    /// Actively hunting a spirit resource
    Hunting,
    /// Absorbing a spirit (stationary)
    Absorbing,
    /// Shedding a Lesser Self (evolution in progress)
    Shedding,
    /// Recovering after shedding
    Recovering,
    /// Engaged in combat with the player
    Combat,
    /// Dormant (defeated, awaiting resurrection ritual)
    Dormant,
}

// ============================================================================
// METABOLIC DECAY SYSTEM (800-day FIFO)
// ============================================================================

/// Tracks spirits absorbed by the Prime Vessel with FIFO decay
#[derive(Component, Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetabolicHistory {
    /// Queue of absorbed spirits (oldest first)
    pub absorbed_spirits: VecDeque<AbsorbedSpirit>,
}

impl MetabolicHistory {
    /// Add a newly absorbed spirit to the queue
    pub fn absorb(&mut self, power: f32, absorbed_day: u32) {
        self.absorbed_spirits.push_back(AbsorbedSpirit {
            power,
            absorbed_day,
        });
    }

    /// Process decay: remove spirits older than 800 days
    /// Returns the total power lost to decay
    pub fn process_decay(&mut self, current_day: u32) -> f32 {
        let mut power_lost = 0.0;
        const DECAY_DAYS: u32 = 800;

        while let Some(oldest) = self.absorbed_spirits.front() {
            if current_day >= oldest.absorbed_day + DECAY_DAYS {
                if let Some(decayed) = self.absorbed_spirits.pop_front() {
                    power_lost += decayed.power;
                }
            } else {
                break;
            }
        }

        power_lost
    }

    /// Calculate total active power from all non-decayed spirits
    pub fn total_power(&self) -> f32 {
        self.absorbed_spirits.iter().map(|s| s.power).sum()
    }

    /// Get count of active spirits
    pub fn spirit_count(&self) -> usize {
        self.absorbed_spirits.len()
    }
}

/// Record of an absorbed spirit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbsorbedSpirit {
    /// Power value of the absorbed spirit
    pub power: f32,
    /// In-game day when absorbed
    pub absorbed_day: u32,
}

// ============================================================================
// LESSER SELVES - Shed Forms of the Prime Vessel
// ============================================================================

/// A Lesser Self - A static snapshot of the Prime Vessel's past form
/// These are permanent patrolling hazards that never decay
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct LesserSelf {
    /// The evolution tier this was shed at
    pub origin_tier: u8,
    /// Power level when shed (capped at 500)
    pub power_level: f32,
    /// Unique identifier for this Lesser Self
    pub generation: u32,
    /// The day this Lesser Self was created
    pub creation_day: u32,
    /// Combat mutations inherited from the Prime Vessel
    pub mutations: Vec<VesselMutation>,
    /// Patrol route points (grid positions)
    pub patrol_route: Vec<(i32, i32)>,
    /// Current patrol index
    pub patrol_index: usize,
}

impl LesserSelf {
    /// Maximum power level for Lesser Selves
    pub const MAX_POWER: f32 = 500.0;

    /// Create a new Lesser Self from the Prime Vessel's current state
    pub fn from_prime_vessel(
        tier: u8,
        power: f32,
        generation: u32,
        creation_day: u32,
        mutations: Vec<VesselMutation>,
        spawn_position: (i32, i32),
    ) -> Self {
        // Generate a simple patrol route around spawn
        let patrol_route = vec![
            spawn_position,
            (spawn_position.0 + 5, spawn_position.1),
            (spawn_position.0 + 5, spawn_position.1 + 5),
            (spawn_position.0, spawn_position.1 + 5),
        ];

        Self {
            origin_tier: tier,
            power_level: power.min(Self::MAX_POWER),
            generation,
            creation_day,
            mutations,
            patrol_route,
            patrol_index: 0,
        }
    }

    /// Get display name based on origin tier
    pub fn title(&self) -> String {
        format!("Lesser Self (Gen {})", self.generation)
    }
}

/// Mutations that the Prime Vessel (and Lesser Selves) can acquire
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VesselMutation {
    // Offensive mutations
    VenomousStrike,
    CorrosiveTouch,
    SpiritDrain,
    ChaosBurst,
    SoulRend,

    // Defensive mutations
    ChitinousArmor,
    RegenerativeFlesh,
    SpiritBarrier,
    ChaosShield,
    VoidSkin,

    // Mobility mutations
    BlinkDash,
    ShadowMeld,
    TerrestrialPhase,
    SwiftMutation,

    // Special mutations
    SpiritSense,    // Can detect spirits from further away
    FrenzyAura,     // Drives nearby creatures chaotic
    CorruptionWake, // Leaves corruption trail
    MassAbsorption, // Can absorb multiple spirits at once
}

// ============================================================================
// WORLD SPIRITS - Resources for Absorption
// ============================================================================

/// A Spirit resource that can be absorbed by the Prime Vessel or Player
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WorldSpirit {
    /// Power value of this spirit
    pub power: f32,
    /// Spirit type/alignment
    pub spirit_type: WorldSpiritType,
    /// Whether this spirit has been purified (cannot be absorbed by Vessel)
    pub is_purified: bool,
    /// Whether this spirit is currently being absorbed
    pub being_absorbed: bool,
    /// Entity absorbing this spirit (if any)
    pub absorber: Option<Entity>,
}

impl Default for WorldSpirit {
    fn default() -> Self {
        Self {
            power: 10.0,
            spirit_type: WorldSpiritType::Neutral,
            is_purified: false,
            being_absorbed: false,
            absorber: None,
        }
    }
}

impl WorldSpirit {
    pub fn new(power: f32, spirit_type: WorldSpiritType) -> Self {
        Self {
            power,
            spirit_type,
            is_purified: false,
            being_absorbed: false,
            absorber: None,
        }
    }
}

/// Types of World Spirits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorldSpiritType {
    /// Basic spirit - provides moderate power
    Neutral,
    /// Ancestral spirit - provides high power, rare
    Ancestral,
    /// Chaos spirit - provides power + mutation chance
    Chaos,
    /// Harmony spirit - provides power + stability
    Harmony,
    /// Void spirit - provides massive power, very rare
    Void,
    /// Trapped spirit - must be freed first (from defeated entities)
    Trapped,
}

impl WorldSpiritType {
    /// Base power multiplier for this spirit type
    pub fn power_multiplier(&self) -> f32 {
        match self {
            WorldSpiritType::Neutral => 1.0,
            WorldSpiritType::Ancestral => 2.0,
            WorldSpiritType::Chaos => 1.5,
            WorldSpiritType::Harmony => 1.5,
            WorldSpiritType::Void => 5.0,
            WorldSpiritType::Trapped => 0.5,
        }
    }
}

// ============================================================================
// CORRUPTION INDEX COMPONENTS
// ============================================================================

/// Marker for entities that have been affected by Total Collapse
#[derive(Component, Debug, Clone, Copy)]
pub struct ChaoticShift;

/// Marker for entities immune to the Corruption Index effects
#[derive(Component, Debug, Clone, Copy)]
pub struct CorruptionImmune;

/// Soul alignment component that tracks an entity's chaos level
/// based on the Global Corruption Index
#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SoulAlignment {
    /// Current alignment (-1.0 = pure order, 1.0 = pure chaos)
    pub alignment: f32,
    /// Base aggression modifier from alignment
    pub aggression_modifier: f32,
    /// Whether this entity has been permanently shifted to chaos
    pub chaos_locked: bool,
}

impl Default for SoulAlignment {
    fn default() -> Self {
        Self {
            alignment: 0.0,
            aggression_modifier: 1.0,
            chaos_locked: false,
        }
    }
}

impl SoulAlignment {
    /// Update alignment based on corruption index percentage
    pub fn update_from_corruption(&mut self, corruption_percentage: f32) {
        if !self.chaos_locked {
            // As corruption rises (spirits decrease), alignment shifts toward chaos
            self.alignment = corruption_percentage;
            // Aggression scales exponentially with corruption
            self.aggression_modifier = 1.0 + (corruption_percentage * corruption_percentage * 2.0);
        }
    }

    /// Lock this entity to chaotic alignment (Total Collapse)
    pub fn lock_to_chaos(&mut self) {
        self.alignment = 1.0;
        self.aggression_modifier = 3.0;
        self.chaos_locked = true;
    }
}

// ============================================================================
// POST-GAME RESURRECTION
// ============================================================================

/// Tracks resurrection ritual progress for the Prime Vessel
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ResurrectionRitual {
    /// Spirits required for resurrection
    pub spirits_required: u32,
    /// Spirits currently offered
    pub spirits_offered: u32,
    /// Whether the ritual is complete
    pub is_complete: bool,
    /// Day the ritual was started
    pub started_day: Option<u32>,
}

impl Default for ResurrectionRitual {
    fn default() -> Self {
        Self {
            spirits_required: 1000,
            spirits_offered: 0,
            is_complete: false,
            started_day: None,
        }
    }
}

impl ResurrectionRitual {
    pub fn offer_spirit(&mut self, power: f32) {
        self.spirits_offered += power as u32;
        if self.spirits_offered >= self.spirits_required {
            self.is_complete = true;
        }
    }

    pub fn progress_percentage(&self) -> f32 {
        (self.spirits_offered as f32 / self.spirits_required as f32).min(1.0)
    }
}
