use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Re-export from core to avoid circular dependency
pub use bevy_shaman_core::components::{BloodLust, CombatDifficulty};

#[derive(Component, Default)]
pub struct StatusEffects {
    pub effects: Vec<StatusEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: f32,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusEffectType {
    Burn,
    Poison,
    SpiritualPoison,  // From Mambele plant-based weapons
    Stun,
    Slow,
    Purifying,
}

#[derive(Component)]
pub struct Attack {
    pub attacker: Entity,
    pub damage: f32,
    pub target: Entity,
    pub rhythm_quality: bevy_shaman_core::resources::TimingQuality,
}

// ============================================================================
// WEAPON SYSTEM
// ============================================================================

/// Player's equipped weapon
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct EquippedWeapon {
    pub weapon_type: WeaponType,
    pub durability: f32,
    pub max_durability: f32,
    pub damage_multiplier: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    Crossbow,
    Mambele,  // Spiritual poison-tipped throwing weapon
    Staff,    // Magic staff
}

impl WeaponType {
    pub fn base_damage(&self) -> f32 {
        match self {
            WeaponType::Crossbow => 15.0,
            WeaponType::Mambele => 12.0,
            WeaponType::Staff => 10.0,
        }
    }

    pub fn spirit_cost(&self) -> f32 {
        match self {
            WeaponType::Crossbow => 0.0,   // Physical weapon, no spirit cost
            WeaponType::Mambele => 5.0,    // Uses spiritual poison
            WeaponType::Staff => 10.0,     // Pure magic
        }
    }

    pub fn can_apply_poison(&self) -> bool {
        matches!(self, WeaponType::Mambele)
    }

    pub fn can_cast_spells(&self) -> bool {
        matches!(self, WeaponType::Staff)
    }
}

impl Default for EquippedWeapon {
    fn default() -> Self {
        Self {
            weapon_type: WeaponType::Staff,
            durability: 100.0,
            max_durability: 100.0,
            damage_multiplier: 1.0,
        }
    }
}

// ============================================================================
// BLOOD LUST & CORRUPTION
// ============================================================================

// BloodLust and CombatDifficulty moved to bevy_shaman_core to break circular dependency
// They are re-exported at the top of this file

// ============================================================================
// RANDOM WHEEL MECHANIC
// ============================================================================

/// Triggers random combat events with 18% chance
#[derive(Component)]
pub struct CombatWheel {
    pub trigger_chance: f32,  // 0.18 for 18%
    pub last_trigger: f64,    // Timestamp to prevent spam
    pub cooldown: f32,        // Minimum time between triggers
}

impl Default for CombatWheel {
    fn default() -> Self {
        Self {
            trigger_chance: 0.18,
            last_trigger: 0.0,
            cooldown: 2.0,  // 2 seconds minimum between wheel spins
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelOutcome {
    CriticalHit,         // Double physical damage
    DoubleSpellDamage,   // Double magic damage
    SelfCorruption,      // Add corruption to player
    SpiritCorruption,    // Add corruption to player's spirits/minions
}

impl WheelOutcome {
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => WheelOutcome::CriticalHit,
            1 => WheelOutcome::DoubleSpellDamage,
            2 => WheelOutcome::SelfCorruption,
            _ => WheelOutcome::SpiritCorruption,
        }
    }
}

// ============================================================================
// MONSTER CONTROL
// ============================================================================

/// Enhanced monster control - allows player to take over monsters
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct MonsterControl {
    pub controlled_monster: Option<Entity>,
    pub control_duration: f32,      // How long control lasts
    pub control_strength: f32,      // 0.0 to 1.0, affects success
    pub can_use_abilities: bool,    // Can use monster's special abilities
}

impl Default for MonsterControl {
    fn default() -> Self {
        Self {
            controlled_monster: None,
            control_duration: 0.0,
            control_strength: 0.5,
            can_use_abilities: false,
        }
    }
}

/// Marker component for a monster being controlled
#[derive(Component)]
pub struct UnderPlayerControl {
    pub controller: Entity,
    pub started_at: f64,
}

// ============================================================================
// RHYTHM COMBOS
// ============================================================================

/// Tracks rhythm-based combo chains
#[derive(Component, Default)]
pub struct RhythmCombo {
    pub current_combo: Vec<ComboInput>,
    pub max_combo_length: usize,
    pub last_input_time: f64,
    pub combo_window: f32,  // Time window to continue combo
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComboInput {
    Light,   // Light attack
    Heavy,   // Heavy attack
    Magic,   // Spell cast
    Perfect, // Perfect timing hit
}

impl RhythmCombo {
    pub fn new(max_length: usize, window: f32) -> Self {
        Self {
            current_combo: Vec::new(),
            max_combo_length: max_length,
            last_input_time: 0.0,
            combo_window: window,
        }
    }

    pub fn add_input(&mut self, input: ComboInput, current_time: f64) {
        // Reset if outside combo window
        if current_time - self.last_input_time > self.combo_window as f64 {
            self.current_combo.clear();
        }

        self.current_combo.push(input);
        self.last_input_time = current_time;

        // Trim to max length
        if self.current_combo.len() > self.max_combo_length {
            self.current_combo.remove(0);
        }
    }

    pub fn check_special(&self) -> Option<SpecialMove> {
        // Check for specific combo patterns
        if self.current_combo.len() >= 3 {
            let last_three = &self.current_combo[self.current_combo.len() - 3..];

            match last_three {
                [ComboInput::Light, ComboInput::Light, ComboInput::Heavy] => {
                    return Some(SpecialMove::FlurryFinisher);
                }
                [ComboInput::Perfect, ComboInput::Perfect, ComboInput::Magic] => {
                    return Some(SpecialMove::PerfectCast);
                }
                [ComboInput::Heavy, ComboInput::Magic, ComboInput::Heavy] => {
                    return Some(SpecialMove::SpiritStrike);
                }
                _ => {}
            }
        }
        None
    }

    pub fn reset(&mut self) {
        self.current_combo.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialMove {
    FlurryFinisher,   // Light, Light, Heavy - Extra damage burst
    PerfectCast,      // Perfect, Perfect, Magic - Zero spirit cost cast
    SpiritStrike,     // Heavy, Magic, Heavy - Damage + heal spirit
}
