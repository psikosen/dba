use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// SHAMAN FOCUS ABILITIES - Support & Control Powers
// ============================================================================

/// Shaman's focus resource for casting abilities
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ShamanFocus {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,  // Per second
}

impl Default for ShamanFocus {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
            regen_rate: 5.0,
        }
    }
}

/// Active focus ability being channeled
#[derive(Component, Debug, Clone)]
pub struct ActiveFocusAbility {
    pub ability_type: FocusAbilityType,
    pub target: Option<Entity>,
    pub cast_timer: Timer,
    pub duration_timer: Option<Timer>,
}

/// Types of focus abilities the shaman can use
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FocusAbilityType {
    // Support abilities
    Heal,                 // Restore health to target
    Purify,              // Remove status effects / purify spirits

    // Control abilities
    Stun,                // Disable target temporarily
    Disable,             // Reduce target's capabilities
    Pacify,              // Calm aggressive entities

    // Movement abilities
    PushBack,            // Force push target away
    PullIn,              // Pull target closer
    Lift,                // Levitate target (disable movement)

    // Spirit infusion
    SpiritInfusion,      // Merge with spirits for combat power
}

impl FocusAbilityType {
    /// Focus cost to cast this ability
    pub fn focus_cost(&self) -> f32 {
        match self {
            FocusAbilityType::Heal => 20.0,
            FocusAbilityType::Purify => 25.0,
            FocusAbilityType::Stun => 30.0,
            FocusAbilityType::Disable => 25.0,
            FocusAbilityType::Pacify => 20.0,
            FocusAbilityType::PushBack => 15.0,
            FocusAbilityType::PullIn => 15.0,
            FocusAbilityType::Lift => 35.0,
            FocusAbilityType::SpiritInfusion => 50.0,
        }
    }

    /// Cast time in seconds
    pub fn cast_time(&self) -> f32 {
        match self {
            FocusAbilityType::Heal => 1.0,
            FocusAbilityType::Purify => 1.5,
            FocusAbilityType::Stun => 0.5,
            FocusAbilityType::Disable => 0.8,
            FocusAbilityType::Pacify => 2.0,
            FocusAbilityType::PushBack => 0.3,
            FocusAbilityType::PullIn => 0.3,
            FocusAbilityType::Lift => 1.2,
            FocusAbilityType::SpiritInfusion => 2.0,  // Fixed 2s cast
        }
    }

    /// Duration of effect (if applicable)
    pub fn duration(&self) -> Option<f32> {
        match self {
            FocusAbilityType::Stun => Some(3.0),
            FocusAbilityType::Disable => Some(5.0),
            FocusAbilityType::Pacify => Some(10.0),
            FocusAbilityType::Lift => Some(4.0),
            FocusAbilityType::SpiritInfusion => None,  // Random duration handled separately
            _ => None,
        }
    }

    /// Cooldown in seconds
    pub fn cooldown(&self) -> f32 {
        match self {
            FocusAbilityType::Heal => 5.0,
            FocusAbilityType::Purify => 8.0,
            FocusAbilityType::Stun => 10.0,
            FocusAbilityType::Disable => 7.0,
            FocusAbilityType::Pacify => 15.0,
            FocusAbilityType::PushBack => 3.0,
            FocusAbilityType::PullIn => 3.0,
            FocusAbilityType::Lift => 12.0,
            FocusAbilityType::SpiritInfusion => 0.0,  // Random cooldown handled separately
        }
    }

    /// Range in grid tiles
    pub fn range(&self) -> i32 {
        match self {
            FocusAbilityType::Heal => 8,
            FocusAbilityType::Purify => 10,
            FocusAbilityType::Stun => 6,
            FocusAbilityType::Disable => 7,
            FocusAbilityType::Pacify => 12,
            FocusAbilityType::PushBack => 5,
            FocusAbilityType::PullIn => 5,
            FocusAbilityType::Lift => 6,
            FocusAbilityType::SpiritInfusion => 0,  // Self-cast
        }
    }
}

/// Tracks ability cooldowns
#[derive(Component, Debug, Clone, Default)]
pub struct FocusAbilityCooldowns {
    pub cooldowns: Vec<AbilityCooldown>,
}

#[derive(Debug, Clone)]
pub struct AbilityCooldown {
    pub ability: FocusAbilityType,
    pub remaining: f32,
}

impl FocusAbilityCooldowns {
    pub fn add_cooldown(&mut self, ability: FocusAbilityType, duration: f32) {
        self.cooldowns.push(AbilityCooldown {
            ability,
            remaining: duration,
        });
    }

    pub fn is_on_cooldown(&self, ability: FocusAbilityType) -> bool {
        self.cooldowns.iter().any(|cd| cd.ability == ability && cd.remaining > 0.0)
    }

    pub fn get_remaining(&self, ability: FocusAbilityType) -> f32 {
        self.cooldowns
            .iter()
            .find(|cd| cd.ability == ability)
            .map(|cd| cd.remaining)
            .unwrap_or(0.0)
    }
}

// ============================================================================
// SPIRIT INFUSION MECHANIC
// ============================================================================

/// Component marking an entity infused with spirit power
#[derive(Component, Debug, Clone)]
pub struct SpiritInfused {
    pub duration_remaining: f32,
    pub power_multiplier: f32,
    pub speed_multiplier: f32,
    pub damage_multiplier: f32,
    pub cooldown_override: f32,  // Random cooldown for next use
}

impl SpiritInfused {
    /// Create a new spirit infusion with random duration and cooldown
    pub fn new_random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Random duration: 5-15 seconds
        let duration = rng.gen_range(5.0..=15.0);

        // Random cooldown: 10-30 seconds
        let cooldown = rng.gen_range(10.0..=30.0);

        // Power scales with duration (shorter = more powerful)
        let power_multiplier = 1.5 + ((15.0 - duration) / 10.0);

        Self {
            duration_remaining: duration,
            power_multiplier,
            speed_multiplier: 1.5,
            damage_multiplier: power_multiplier,
            cooldown_override: cooldown,
        }
    }
}

// ============================================================================
// LAYER 3 OBJECT MANIPULATION
// ============================================================================

/// Component for objects that can be manipulated by focus powers
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ManipulableObject {
    pub object_type: ObjectType,
    pub weight: f32,            // Affects push/pull strength needed
    pub durability: f32,        // For breaking objects
    pub is_breakable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectType {
    Box,
    Rock,
    Debris,
    Monster,  // Yes, monsters can be pushed!
}

impl ManipulableObject {
    /// Check if this object can be pushed by a given force
    pub fn can_be_pushed(&self, force: f32) -> bool {
        force >= self.weight * 0.5
    }

    /// Check if this object can be broken by a given force
    pub fn can_be_broken(&self, force: f32) -> bool {
        self.is_breakable && force >= self.durability
    }
}

/// Component marking an object currently being manipulated
#[derive(Component, Debug, Clone)]
pub struct BeingManipulated {
    pub force_direction: Vec2,
    pub force_strength: f32,
    pub manipulator: Entity,
}

// ============================================================================
// FOCUS ABILITY EVENTS
// ============================================================================

#[derive(Event, Debug, Clone)]
pub struct FocusAbilityCast {
    pub caster: Entity,
    pub ability: FocusAbilityType,
    pub target: Option<Entity>,
}

#[derive(Event, Debug, Clone)]
pub struct FocusAbilityCompleted {
    pub caster: Entity,
    pub ability: FocusAbilityType,
    pub target: Option<Entity>,
    pub success: bool,
}

#[derive(Event, Debug, Clone)]
pub struct ObjectManipulated {
    pub object: Entity,
    pub manipulator: Entity,
    pub manipulation_type: ManipulationType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManipulationType {
    Pushed,
    Pulled,
    Lifted,
    Broken,
}
