use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Re-export from core to avoid circular dependency
pub use bevy_shaman_core::components::{BloodLust, CombatDifficulty};

// Focus abilities module
pub mod focus_abilities;

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
    SpiritualPoison, // From Mambele plant-based weapons
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
    Mambele, // Spiritual poison-tipped throwing weapon
    Staff,   // Magic staff
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
            WeaponType::Crossbow => 0.0, // Physical weapon, no spirit cost
            WeaponType::Mambele => 5.0,  // Uses spiritual poison
            WeaponType::Staff => 10.0,   // Pure magic
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
    pub trigger_chance: f32, // 0.18 for 18%
    pub last_trigger: f64,   // Timestamp to prevent spam
    pub cooldown: f32,       // Minimum time between triggers
}

impl Default for CombatWheel {
    fn default() -> Self {
        Self {
            trigger_chance: 0.18,
            last_trigger: 0.0,
            cooldown: 2.0, // 2 seconds minimum between wheel spins
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WheelOutcome {
    CriticalHit,       // Double physical damage
    DoubleSpellDamage, // Double magic damage
    SelfCorruption,    // Add corruption to player
    SpiritCorruption,  // Add corruption to player's spirits/minions
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
    pub control_duration: f32,   // How long control lasts
    pub control_strength: f32,   // 0.0 to 1.0, affects success
    pub can_use_abilities: bool, // Can use monster's special abilities
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
// WEAPON ENHANCEMENT SYSTEM
// ============================================================================

/// Weapon enhancement through spirit merging and plant infusion
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WeaponEnhancement {
    pub level: u32,
    pub infused_spirit_energy: f32,
    pub active_enchantments: Vec<Enchantment>,
    pub permanent_bonuses: EnhancementBonuses,
}

impl Default for WeaponEnhancement {
    fn default() -> Self {
        Self {
            level: 0,
            infused_spirit_energy: 0.0,
            active_enchantments: Vec::new(),
            permanent_bonuses: EnhancementBonuses::default(),
        }
    }
}

impl WeaponEnhancement {
    pub const MAX_LEVEL: u32 = 10;

    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate cost for next level enhancement
    pub fn next_level_cost(&self) -> EnhancementCost {
        let base_spirit = 50.0;
        let base_blood = 20.0;

        EnhancementCost {
            spirit_orbs: base_spirit * (self.level as f32 + 1.0) * 1.5,
            blood_plants: (self.level / 2) as u32 + 1,
            spirit_plants: (self.level / 2) as u32 + 1,
        }
    }

    /// Apply permanent upgrade when leveling up
    pub fn apply_upgrade(&mut self, weapon: &mut EquippedWeapon) {
        if self.level >= Self::MAX_LEVEL {
            return;
        }

        self.level += 1;

        // Increase permanent bonuses
        self.permanent_bonuses.damage_percent += 5.0 + (self.level as f32 * 0.5);
        self.permanent_bonuses.spirit_efficiency += 2.0;

        // Restore durability and increase max
        weapon.max_durability += 10.0;
        weapon.durability = weapon.max_durability;

        // Every 3 levels, unlock special ability
        if self.level % 3 == 0 {
            self.unlock_special_ability();
        }
    }

    /// Apply temporary enchantment from plants
    pub fn apply_temporary_enchantment(
        &mut self,
        enchantment_type: EnchantmentType,
        duration: f32,
        strength: f32,
    ) {
        // Remove existing enchantment of same type
        self.active_enchantments
            .retain(|e| e.enchantment_type != enchantment_type);

        self.active_enchantments.push(Enchantment {
            enchantment_type,
            duration_remaining: duration,
            strength,
        });
    }

    /// Update enchantments (remove expired ones)
    pub fn update_enchantments(&mut self, delta: f32) {
        for enchantment in &mut self.active_enchantments {
            enchantment.duration_remaining -= delta;
        }
        self.active_enchantments
            .retain(|e| e.duration_remaining > 0.0);
    }

    /// Get total damage bonus percentage
    pub fn damage_bonus(&self) -> f32 {
        self.permanent_bonuses.damage_percent
    }

    /// Get spirit efficiency (reduces spirit cost)
    pub fn spirit_cost_multiplier(&self) -> f32 {
        let reduction = self.permanent_bonuses.spirit_efficiency;
        (100.0 - reduction).max(20.0) / 100.0 // Minimum 20% cost
    }

    fn unlock_special_ability(&mut self) {
        // Unlocked abilities tracked in permanent bonuses
        match self.level {
            3 => self.permanent_bonuses.critical_chance += 10.0,
            6 => self.permanent_bonuses.lifesteal_percent += 5.0,
            9 => self.permanent_bonuses.ancestral_strike_unlocked = true,
            _ => {}
        }
    }

    /// Check if weapon can use ancestral strike (special attack)
    pub fn has_ancestral_strike(&self) -> bool {
        self.permanent_bonuses.ancestral_strike_unlocked
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnhancementCost {
    pub spirit_orbs: f32,
    pub blood_plants: u32,
    pub spirit_plants: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnhancementBonuses {
    pub damage_percent: f32,
    pub spirit_efficiency: f32,
    pub critical_chance: f32,
    pub lifesteal_percent: f32,
    pub ancestral_strike_unlocked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enchantment {
    pub enchantment_type: EnchantmentType,
    pub duration_remaining: f32,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnchantmentType {
    BloodFury,         // Temporary damage boost from blood plants
    SpiritInfusion,    // Reduced spirit cost from spirit plants
    VenomCoating,      // Poison damage from poison plants
    AncestralBlessing, // Blessing from rare plants
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
    pub combo_window: f32, // Time window to continue combo
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
    FlurryFinisher, // Light, Light, Heavy - Extra damage burst
    PerfectCast,    // Perfect, Perfect, Magic - Zero spirit cost cast
    SpiritStrike,   // Heavy, Magic, Heavy - Damage + heal spirit
}
