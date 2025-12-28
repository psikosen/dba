use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// GLOBAL CORRUPTION INDEX (The Doomsday Clock)
// ============================================================================

/// The Global Corruption Index - Tracks the balance of Order in the world
/// Spirits are anchors of Order. As they are consumed, chaos rises.
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct GlobalCorruptionIndex {
    /// Starting number of spirits in the world (15,000)
    pub initial_spirit_count: u32,
    /// Current number of free spirits remaining
    pub free_spirit_count: u32,
    /// Spirits consumed by the Prime Vessel
    pub vessel_consumed: u32,
    /// Spirits consumed by the Player
    pub player_consumed: u32,
    /// Spirits purified (slows corruption)
    pub spirits_purified: u32,
    /// Spirits freed from entities (added back to pool)
    pub spirits_freed: u32,
    /// Whether the Zero-Point has been triggered
    pub total_collapse_triggered: bool,
    /// Whether the UI is visible (unlocked after "Purify Your Brother")
    pub ui_revealed: bool,
    /// Current corruption percentage (0.0 = pure, 1.0 = total collapse)
    pub corruption_percentage: f32,
}

impl Default for GlobalCorruptionIndex {
    fn default() -> Self {
        const INITIAL_SPIRITS: u32 = 15_000;
        Self {
            initial_spirit_count: INITIAL_SPIRITS,
            free_spirit_count: INITIAL_SPIRITS,
            vessel_consumed: 0,
            player_consumed: 0,
            spirits_purified: 0,
            spirits_freed: 0,
            total_collapse_triggered: false,
            ui_revealed: false,
            corruption_percentage: 0.0,
        }
    }
}

impl GlobalCorruptionIndex {
    /// Consume a spirit (remove from world)
    pub fn consume_spirit(&mut self, by_vessel: bool) {
        if self.free_spirit_count > 0 {
            self.free_spirit_count -= 1;
            if by_vessel {
                self.vessel_consumed += 1;
            } else {
                self.player_consumed += 1;
            }
            self.recalculate_corruption();
        }
    }

    /// Consume multiple spirits at once
    pub fn consume_spirits(&mut self, count: u32, by_vessel: bool) {
        let actual_count = count.min(self.free_spirit_count);
        self.free_spirit_count -= actual_count;
        if by_vessel {
            self.vessel_consumed += actual_count;
        } else {
            self.player_consumed += actual_count;
        }
        self.recalculate_corruption();
    }

    /// Purify a spirit (slows corruption without removing)
    /// Purified spirits count as 2x towards Order
    pub fn purify_spirit(&mut self) {
        self.spirits_purified += 1;
        self.recalculate_corruption();
    }

    /// Free a trapped spirit (return to the world pool)
    /// Only way to increase spirit count
    pub fn free_spirit(&mut self) {
        self.free_spirit_count += 1;
        self.spirits_freed += 1;
        self.recalculate_corruption();
    }

    /// Free multiple trapped spirits at once
    pub fn free_spirits(&mut self, count: u32) {
        self.free_spirit_count += count;
        self.spirits_freed += count;
        self.recalculate_corruption();
    }

    /// Recalculate the corruption percentage
    fn recalculate_corruption(&mut self) {
        // Effective spirit count considers purified spirits as bonus
        let effective_spirits = self.free_spirit_count + (self.spirits_purified / 2);

        // Corruption = 1 - (effective_spirits / initial_spirits)
        let ratio = effective_spirits as f32 / self.initial_spirit_count as f32;
        self.corruption_percentage = (1.0 - ratio).clamp(0.0, 1.0);

        // Check for Total Collapse
        if self.free_spirit_count == 0 && !self.total_collapse_triggered {
            self.total_collapse_triggered = true;
        }
    }

    /// Get the current world danger level (0-5)
    pub fn danger_level(&self) -> u8 {
        match self.corruption_percentage {
            x if x < 0.2 => 0, // Peaceful
            x if x < 0.4 => 1, // Uneasy
            x if x < 0.6 => 2, // Dangerous
            x if x < 0.8 => 3, // Hostile
            x if x < 1.0 => 4, // Critical
            _ => 5,            // Total Collapse
        }
    }

    /// Get description of current world state
    pub fn world_state_description(&self) -> &'static str {
        match self.danger_level() {
            0 => "The spirits rest peacefully. Order prevails.",
            1 => "An unease stirs in the spirit realm. Something hungers.",
            2 => "The veil thins. Creatures grow restless and unpredictable.",
            3 => "Chaos seeps into the world. Violence begets violence.",
            4 => "The order crumbles. Only the strong survive.",
            5 => "TOTAL COLLAPSE: All souls have shifted to Chaos. There is no peace.",
            _ => "Unknown state.",
        }
    }

    /// Reveal the UI (after completing "Purify Your Brother" quest)
    pub fn reveal_ui(&mut self) {
        self.ui_revealed = true;
    }

    /// Check if player should see the corruption index
    pub fn is_visible(&self) -> bool {
        self.ui_revealed
    }

    /// Get player's contribution to corruption
    pub fn player_corruption_contribution(&self) -> f32 {
        let total_consumed = self.vessel_consumed + self.player_consumed;
        if total_consumed == 0 {
            0.0
        } else {
            self.player_consumed as f32 / total_consumed as f32
        }
    }
}

// ============================================================================
// PRIME VESSEL GLOBAL STATE
// ============================================================================

/// Tracks the global state of the Prime Vessel system
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct PrimeVesselState {
    /// Whether the Prime Vessel has spawned
    pub has_spawned: bool,
    /// The entity ID of the Prime Vessel (if spawned)
    pub vessel_entity: Option<Entity>,
    /// All Lesser Self entities
    pub lesser_selves: Vec<Entity>,
    /// Total Lesser Selves ever created
    pub total_lesser_selves: u32,
    /// Whether the Prime Vessel has been defeated
    pub vessel_defeated: bool,
    /// Whether post-game resurrection is available
    pub resurrection_available: bool,
    /// Day the vessel was defeated
    pub defeat_day: Option<u32>,
}

impl Default for PrimeVesselState {
    fn default() -> Self {
        Self {
            has_spawned: false,
            vessel_entity: None,
            lesser_selves: Vec::new(),
            total_lesser_selves: 0,
            vessel_defeated: false,
            resurrection_available: false,
            defeat_day: None,
        }
    }
}

impl PrimeVesselState {
    /// Mark the vessel as spawned
    pub fn spawn_vessel(&mut self, entity: Entity) {
        self.has_spawned = true;
        self.vessel_entity = Some(entity);
        self.vessel_defeated = false;
    }

    /// Add a newly shed Lesser Self
    pub fn add_lesser_self(&mut self, entity: Entity) {
        self.lesser_selves.push(entity);
        self.total_lesser_selves += 1;
    }

    /// Mark the vessel as defeated
    pub fn defeat_vessel(&mut self, day: u32) {
        self.vessel_defeated = true;
        self.vessel_entity = None;
        self.defeat_day = Some(day);
        // Resurrection becomes available 100 days after defeat
        self.resurrection_available = false;
    }

    /// Enable resurrection (called 100 days after defeat)
    pub fn enable_resurrection(&mut self) {
        if self.vessel_defeated {
            self.resurrection_available = true;
        }
    }

    /// Resurrect the vessel
    pub fn resurrect_vessel(&mut self, entity: Entity) {
        self.vessel_entity = Some(entity);
        self.vessel_defeated = false;
        self.resurrection_available = false;
        self.defeat_day = None;
    }
}

// ============================================================================
// SPIRIT SPAWN CONFIGURATION
// ============================================================================

/// Configuration for world spirit spawning
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct SpiritSpawnConfig {
    /// Minimum distance between spirit spawns
    pub min_spawn_distance: f32,
    /// Maximum spirits per biome
    pub max_per_biome: u32,
    /// Base power range for neutral spirits
    pub neutral_power_range: (f32, f32),
    /// Spawn weights for each spirit type
    pub spawn_weights: SpiritSpawnWeights,
    /// Whether initial population has spawned
    pub initial_population_spawned: bool,
}

impl Default for SpiritSpawnConfig {
    fn default() -> Self {
        Self {
            min_spawn_distance: 10.0,
            max_per_biome: 3000,
            neutral_power_range: (5.0, 25.0),
            spawn_weights: SpiritSpawnWeights::default(),
            initial_population_spawned: false,
        }
    }
}

/// Spawn weights for different spirit types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiritSpawnWeights {
    pub neutral: u32,
    pub ancestral: u32,
    pub chaos: u32,
    pub harmony: u32,
    pub void: u32,
}

impl Default for SpiritSpawnWeights {
    fn default() -> Self {
        Self {
            neutral: 60,
            ancestral: 15,
            chaos: 10,
            harmony: 10,
            void: 5,
        }
    }
}

impl SpiritSpawnWeights {
    pub fn total(&self) -> u32 {
        self.neutral + self.ancestral + self.chaos + self.harmony + self.void
    }
}
