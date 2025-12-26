use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// INVENTORY
// ============================================================================

#[derive(Component, Default)]
pub struct Inventory {
    pub items: Vec<ItemStack>,
    pub max_slots: usize,
}

impl Inventory {
    pub fn new(max_slots: usize) -> Self {
        Self {
            items: Vec::new(),
            max_slots,
        }
    }

    pub fn add_item(&mut self, item: Item, quantity: u32) -> bool {
        // Try to stack with existing item
        for stack in &mut self.items {
            if stack.item.id == item.id && stack.quantity < stack.item.max_stack {
                let space = stack.item.max_stack - stack.quantity;
                let to_add = quantity.min(space);
                stack.quantity += to_add;
                if quantity <= to_add {
                    return true;
                }
            }
        }

        // Create new stack if space available
        if self.items.len() < self.max_slots {
            self.items.push(ItemStack { item, quantity });
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, item_id: &str, quantity: u32) -> bool {
        let mut remaining = quantity;

        self.items.retain_mut(|stack| {
            if stack.item.id == item_id && remaining > 0 {
                if stack.quantity <= remaining {
                    remaining -= stack.quantity;
                    return false; // Remove this stack
                } else {
                    stack.quantity -= remaining;
                    remaining = 0;
                }
            }
            true
        });

        remaining == 0
    }

    pub fn count_item(&self, item_id: &str) -> u32 {
        self.items
            .iter()
            .filter(|s| s.item.id == item_id)
            .map(|s| s.quantity)
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct ItemStack {
    pub item: Item,
    pub quantity: u32,
}

// ============================================================================
// ITEMS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub display_name: String,
    pub item_type: ItemType,
    pub max_stack: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemType {
    SpiritOrb(SpiritOrbSize),
    Herb,
    Remedy,
    CraftingMaterial,
    KeyItem,
    Plant(PlantType),
    Food(FoodType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpiritOrbSize {
    Small,
    Medium,
    Large,
}

impl SpiritOrbSize {
    pub fn spirit_restore(&self) -> f32 {
        match self {
            SpiritOrbSize::Small => 20.0,
            SpiritOrbSize::Medium => 50.0,
            SpiritOrbSize::Large => 100.0,
        }
    }

    pub fn stamina_restore(&self) -> f32 {
        match self {
            SpiritOrbSize::Small => 15.0,
            SpiritOrbSize::Medium => 40.0,
            SpiritOrbSize::Large => 80.0,
        }
    }
}

// ============================================================================
// PICKUPABLE ITEMS (WORLD ENTITIES)
// ============================================================================

#[derive(Component)]
pub struct Pickupable {
    pub item: Item,
    pub quantity: u32,
    pub auto_pickup: bool,
}

// ============================================================================
// CRAFTING
// ============================================================================

#[derive(Component)]
pub struct CraftingStation {
    pub station_type: CraftingStationType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CraftingStationType {
    HerbBench,
    SpiritAltar,
    InstrumentWorkshop,
}

// ============================================================================
// PLANT SYSTEM
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlantType {
    // Calming plants - reduce blood lust
    SpiritBlossom,      // Temporary calming effect during battle
    MoonPetal,          // Permanent calming bonus until next rest
    StarRoot,           // Temporary, strong calming

    // Poison plants - for Mambele weapon crafting
    VenomVine,          // Basic spiritual poison
    ShadowMushroom,     // Strong spiritual poison
    DeathBloom,         // Rare, very potent poison

    // Healing plants
    LifeLeaf,           // Temporary health regen during battle
    EternalBark,        // Permanent health bonus until death

    // Spirit plants
    AetherGrass,        // Temporary spirit regen boost
    CrystalMoss,        // Permanent spirit capacity increase
}

impl PlantType {
    /// Returns true if this plant's effect persists permanently (until rest/death)
    pub fn is_permanent(&self) -> bool {
        matches!(
            self,
            PlantType::MoonPetal | PlantType::EternalBark | PlantType::CrystalMoss
        )
    }

    /// Returns true if this plant is used for weapon poison crafting
    pub fn is_poison_ingredient(&self) -> bool {
        matches!(
            self,
            PlantType::VenomVine | PlantType::ShadowMushroom | PlantType::DeathBloom
        )
    }

    /// Blood lust reduction amount
    pub fn blood_lust_reduction(&self) -> f32 {
        match self {
            PlantType::SpiritBlossom => 15.0,
            PlantType::MoonPetal => 25.0,
            PlantType::StarRoot => 30.0,
            _ => 0.0,
        }
    }

    /// Poison strength for weapon crafting
    pub fn poison_strength(&self) -> f32 {
        match self {
            PlantType::VenomVine => 5.0,
            PlantType::ShadowMushroom => 10.0,
            PlantType::DeathBloom => 20.0,
            _ => 0.0,
        }
    }

    /// Duration in seconds (0.0 means permanent)
    pub fn effect_duration(&self) -> f32 {
        if self.is_permanent() {
            0.0  // Permanent until rest/death
        } else {
            match self {
                PlantType::SpiritBlossom => 30.0,
                PlantType::StarRoot => 20.0,
                PlantType::LifeLeaf => 45.0,
                PlantType::AetherGrass => 60.0,
                _ => 0.0,
            }
        }
    }
}

// ============================================================================
// FOOD SYSTEM
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FoodType {
    // Blood lust reduction foods
    SweetBerry,         // Small reduction
    HoneyBread,         // Medium reduction
    SacredMeal,         // Large reduction

    // Stat boost foods
    StrengthMeat,       // Temporary attack boost
    SwiftFish,          // Temporary speed boost
    WisdomStew,         // Temporary spirit regen boost
}

impl FoodType {
    pub fn blood_lust_reduction(&self) -> f32 {
        match self {
            FoodType::SweetBerry => 10.0,
            FoodType::HoneyBread => 20.0,
            FoodType::SacredMeal => 35.0,
            _ => 0.0,
        }
    }

    pub fn effect_duration(&self) -> f32 {
        match self {
            FoodType::StrengthMeat => 120.0,  // 2 minutes
            FoodType::SwiftFish => 90.0,
            FoodType::WisdomStew => 150.0,
            _ => 0.0,
        }
    }
}

// ============================================================================
// ACTIVE EFFECTS TRACKING
// ============================================================================

/// Component tracking active plant/food effects on an entity
#[derive(Component, Default)]
pub struct ActiveEffects {
    pub effects: Vec<ActiveEffect>,
}

#[derive(Debug, Clone)]
pub struct ActiveEffect {
    pub effect_type: EffectType,
    pub duration_remaining: f32,  // 0.0 for permanent effects
    pub strength: f32,
    pub is_permanent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    BloodLustReduction,
    HealthRegen,
    SpiritRegen,
    AttackBoost,
    SpeedBoost,
    SpiritCapacityBoost,
}

impl ActiveEffects {
    pub fn add_effect(&mut self, effect: ActiveEffect) {
        // Don't stack permanent effects of the same type
        if effect.is_permanent {
            self.effects.retain(|e| {
                !(e.effect_type == effect.effect_type && e.is_permanent)
            });
        }
        self.effects.push(effect);
    }

    pub fn remove_permanent_effects(&mut self) {
        self.effects.retain(|e| !e.is_permanent);
    }

    pub fn update(&mut self, delta: f32) {
        self.effects.retain_mut(|effect| {
            if !effect.is_permanent {
                effect.duration_remaining -= delta;
                effect.duration_remaining > 0.0
            } else {
                true  // Keep permanent effects
            }
        });
    }
}
