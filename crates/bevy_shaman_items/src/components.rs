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
        let mut remaining = quantity;

        // Try to stack with existing items
        for stack in &mut self.items {
            if stack.item.id == item.id && stack.quantity < stack.item.max_stack && remaining > 0 {
                let space = stack.item.max_stack - stack.quantity;
                let to_add = remaining.min(space);
                stack.quantity += to_add;
                remaining -= to_add;

                if remaining == 0 {
                    return true;
                }
            }
        }

        // Create new stack if there's remaining quantity and space available
        if remaining > 0 && self.items.len() < self.max_slots {
            self.items.push(ItemStack {
                item,
                quantity: remaining,
            });
            true
        } else {
            remaining == 0
        }
    }

    pub fn remove_item(&mut self, item_id: &str, quantity: u32) -> bool {
        // First check if we have enough items
        let total = self.count_item(item_id);
        if total < quantity {
            return false; // Not enough items, don't modify anything
        }

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
    SpiritBlossom, // Temporary calming effect during battle
    MoonPetal,     // Permanent calming bonus until next rest
    StarRoot,      // Temporary, strong calming

    // Poison plants - for Mambele weapon crafting
    VenomVine,      // Basic spiritual poison
    ShadowMushroom, // Strong spiritual poison
    DeathBloom,     // Rare, very potent poison

    // Healing plants
    LifeLeaf,    // Temporary health regen during battle
    EternalBark, // Permanent health bonus until death

    // Spirit plants
    AetherGrass, // Temporary spirit regen boost
    CrystalMoss, // Permanent spirit capacity increase

    // Blood plants - require player blood, powerful effects
    // Named with traditional African words for blood/life
    Mogodu,   // (Setswana: blood) - Increases max health permanently
    Damu,     // (Swahili: blood) - Grants powerful regeneration potion
    Ingazi,   // (Zulu: blood) - Increases strength stat for humans
    Mwazi,    // (Chichewa: blood) - Rare damage resistance potion
    Jini,     // (Yoruba: blood) - Increases vitality for spirits
    Ropa,     // (Shona: blood) - Grants blood fury potion (attack boost)
    Samaki,   // (blood flower) - Increases both human and spirit stats
    Umthombo, // (Zulu: life source) - Legendary healing elixir
    Umdhlebi, // (Zulu: legendary tree) - Passive HP regen, generates rare items daily

    // Spirit energy plants - require spirit energy, mystical effects
    // Named with traditional African words for spirit/soul
    Roho,    // (Swahili: spirit) - Increases max spirit permanently
    Moya,    // (Zulu: spirit) - Grants spirit sight potion
    Emi,     // (Yoruba: breath/spirit) - Increases wisdom for humans
    Moyo,    // (Shona: spirit/heart) - Rare spirit shield potion
    Elima,   // (Lingala: soul) - Increases spiritual power
    Pepo,    // (Swahili: spirit/wind) - Grants ethereal movement potion
    Sankofa, // (Akan: spiritual wisdom) - Increases both human and spirit wisdom
    Nommo,   // (Dogon: life force) - Legendary spirit ascension elixir
    Baobab,  // (African tree of life) - Passive spirit regen, produces status cures
}

impl PlantType {
    /// Returns true if this plant's effect persists permanently (until rest/death)
    pub fn is_permanent(&self) -> bool {
        matches!(
            self,
            PlantType::MoonPetal
                | PlantType::EternalBark
                | PlantType::CrystalMoss
                | PlantType::Mogodu
                | PlantType::Roho
        )
    }

    /// Returns true if this plant is used for weapon poison crafting
    pub fn is_poison_ingredient(&self) -> bool {
        matches!(
            self,
            PlantType::VenomVine | PlantType::ShadowMushroom | PlantType::DeathBloom
        )
    }

    /// Returns true if this is a blood plant (requires blood sacrifice)
    pub fn is_blood_plant(&self) -> bool {
        matches!(
            self,
            PlantType::Mogodu
                | PlantType::Damu
                | PlantType::Ingazi
                | PlantType::Mwazi
                | PlantType::Jini
                | PlantType::Ropa
                | PlantType::Samaki
                | PlantType::Umthombo
                | PlantType::Umdhlebi
        )
    }

    /// Returns true if this is a spirit energy plant (requires spirit sacrifice)
    pub fn is_spirit_plant(&self) -> bool {
        matches!(
            self,
            PlantType::Roho
                | PlantType::Moya
                | PlantType::Emi
                | PlantType::Moyo
                | PlantType::Elima
                | PlantType::Pepo
                | PlantType::Sankofa
                | PlantType::Nommo
                | PlantType::Baobab
        )
    }

    /// Blood cost (HP reduction) when feeding blood plants
    pub fn blood_cost(&self) -> f32 {
        match self {
            PlantType::Mogodu => 15.0, // Low cost, common
            PlantType::Damu => 20.0,
            PlantType::Ingazi => 25.0, // Medium cost
            PlantType::Mwazi => 30.0,
            PlantType::Jini => 25.0,
            PlantType::Ropa => 20.0,
            PlantType::Samaki => 35.0,   // High cost, rare
            PlantType::Umthombo => 40.0, // Legendary, highest cost
            PlantType::Umdhlebi => 45.0, // Ultimate legendary, passive regen
            _ => 0.0,
        }
    }

    /// Spirit cost when feeding spirit plants
    pub fn spirit_cost(&self) -> f32 {
        match self {
            PlantType::Roho => 30.0, // Low cost
            PlantType::Moya => 35.0,
            PlantType::Emi => 40.0, // Medium cost
            PlantType::Moyo => 45.0,
            PlantType::Elima => 40.0,
            PlantType::Pepo => 35.0,
            PlantType::Sankofa => 50.0, // High cost
            PlantType::Nommo => 60.0,   // Legendary, highest cost
            PlantType::Baobab => 65.0,  // Ultimate legendary, passive regen
            _ => 0.0,
        }
    }

    /// Feeding interval in days (how often the plant needs blood/spirit)
    pub fn feeding_interval_days(&self) -> u32 {
        match self {
            // Blood plants - feed every 3-9 days
            PlantType::Mogodu => 3,
            PlantType::Damu => 5,
            PlantType::Ingazi => 4,
            PlantType::Mwazi => 7,
            PlantType::Jini => 6,
            PlantType::Ropa => 4,
            PlantType::Samaki => 8,
            PlantType::Umthombo => 9,
            PlantType::Umdhlebi => 7, // Legendary, feeds every week
            // Spirit plants - feed every 3-9 days
            PlantType::Roho => 3,
            PlantType::Moya => 6,
            PlantType::Emi => 5,
            PlantType::Moyo => 8,
            PlantType::Elima => 7,
            PlantType::Pepo => 4,
            PlantType::Sankofa => 9,
            PlantType::Nommo => 9,
            PlantType::Baobab => 7, // Legendary, feeds every week
            _ => 0,
        }
    }

    /// Returns true if this plant provides passive benefits when planted (not consumed)
    pub fn is_passive_plant(&self) -> bool {
        matches!(self, PlantType::Umdhlebi | PlantType::Baobab)
    }

    /// Passive HP regeneration rate (HP per second when planted, not consumed)
    pub fn passive_hp_regen(&self) -> f32 {
        match self {
            PlantType::Umdhlebi => 0.5, // 1 HP per 2 seconds
            _ => 0.0,
        }
    }

    /// Passive spirit regeneration rate (spirit per second when planted, not consumed)
    pub fn passive_spirit_regen(&self) -> f32 {
        match self {
            PlantType::Baobab => 1.0, // 1 spirit per second
            _ => 0.0,
        }
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
            0.0 // Permanent until rest/death
        } else {
            match self {
                PlantType::SpiritBlossom => 30.0,
                PlantType::StarRoot => 20.0,
                PlantType::LifeLeaf => 45.0,
                PlantType::AetherGrass => 60.0,
                // Blood plant effects (from harvested items)
                PlantType::Damu => 300.0,      // 5 min regen potion
                PlantType::Ingazi => 600.0,    // 10 min strength
                PlantType::Mwazi => 180.0,     // 3 min resistance
                PlantType::Jini => 600.0,      // 10 min vitality
                PlantType::Ropa => 120.0,      // 2 min fury
                PlantType::Samaki => 900.0,    // 15 min dual boost
                PlantType::Umthombo => 1800.0, // 30 min legendary heal
                // Spirit plant effects
                PlantType::Moya => 240.0,    // 4 min spirit sight
                PlantType::Emi => 600.0,     // 10 min wisdom
                PlantType::Moyo => 300.0,    // 5 min shield
                PlantType::Elima => 600.0,   // 10 min power
                PlantType::Pepo => 180.0,    // 3 min ethereal
                PlantType::Sankofa => 900.0, // 15 min dual wisdom
                PlantType::Nommo => 1800.0,  // 30 min ascension
                _ => 0.0,
            }
        }
    }

    /// Stat boost for humans (0.0 if not applicable)
    pub fn human_stat_boost(&self) -> f32 {
        match self {
            PlantType::Ingazi => 5.0,  // +5 strength
            PlantType::Samaki => 3.0,  // +3 all stats
            PlantType::Emi => 5.0,     // +5 wisdom
            PlantType::Sankofa => 4.0, // +4 wisdom
            _ => 0.0,
        }
    }

    /// Stat boost for spirits (0.0 if not applicable)
    pub fn spirit_stat_boost(&self) -> f32 {
        match self {
            PlantType::Jini => 5.0,    // +5 vitality
            PlantType::Samaki => 3.0,  // +3 all stats
            PlantType::Elima => 5.0,   // +5 spiritual power
            PlantType::Sankofa => 4.0, // +4 wisdom
            _ => 0.0,
        }
    }
}

// ============================================================================
// FOOD SYSTEM
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FoodType {
    // Blood lust reduction foods
    SweetBerry, // Small reduction
    HoneyBread, // Medium reduction
    SacredMeal, // Large reduction

    // Stat boost foods
    StrengthMeat, // Temporary attack boost
    SwiftFish,    // Temporary speed boost
    WisdomStew,   // Temporary spirit regen boost
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
            FoodType::StrengthMeat => 120.0, // 2 minutes
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
    pub duration_remaining: f32, // 0.0 for permanent effects
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
    // Blood plant effects
    HealthCapacityBoost, // Permanent max HP increase (Mogodu)
    StrengthBoost,       // Strength increase (Ingazi, Samaki)
    DamageResistance,    // Damage resistance (Mwazi)
    VitalityBoost,       // Vitality for spirits (Jini, Samaki)
    BloodFury,           // Attack speed and damage (Ropa)
    LegendaryHealing,    // Powerful healing over time (Umthombo)
    // Spirit plant effects
    SpiritSight,      // See hidden spirits/items (Moya)
    WisdomBoost,      // Wisdom increase (Emi, Sankofa)
    SpiritShield,     // Spirit damage resistance (Moyo)
    SpiritualPower,   // Spiritual power increase (Elima)
    EtherealMovement, // Phase through enemies (Pepo)
    SpiritAscension,  // Legendary spirit boost (Nommo)
}

impl ActiveEffects {
    pub fn add_effect(&mut self, effect: ActiveEffect) {
        // Don't stack permanent effects of the same type
        if effect.is_permanent {
            self.effects
                .retain(|e| !(e.effect_type == effect.effect_type && e.is_permanent));
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
                true // Keep permanent effects
            }
        });
    }
}

// ============================================================================
// BLOOD SACRIFICE & SPIRIT COST TRACKING
// ============================================================================

/// Tracks temporary HP penalty from blood sacrifices (cleared on rest)
#[derive(Component, Default)]
pub struct BloodSacrificePenalty {
    pub hp_reduction: f32, // Total HP reduced from blood sacrifices today
}

impl BloodSacrificePenalty {
    pub fn add_penalty(&mut self, amount: f32) {
        self.hp_reduction += amount;
    }

    pub fn clear_on_rest(&mut self) {
        self.hp_reduction = 0.0;
    }
}

// ============================================================================
// PLANT CARE SYSTEM (for blood/spirit plants in gardens)
// ============================================================================

/// Component for a planted blood/spirit plant that needs regular care
#[derive(Component, Clone)]
pub struct PlantCare {
    pub plant_type: PlantType,
    pub days_since_feeding: u32,
    pub feeding_interval: u32,   // Days between feedings
    pub is_withering: bool,      // True if overdue for feeding
    pub days_since_harvest: u32, // Days since last item generation (for passive plants)
}

impl PlantCare {
    pub fn new(plant_type: PlantType) -> Self {
        Self {
            plant_type,
            days_since_feeding: 0,
            feeding_interval: plant_type.feeding_interval_days(),
            is_withering: false,
            days_since_harvest: 0,
        }
    }

    pub fn needs_feeding(&self) -> bool {
        self.days_since_feeding >= self.feeding_interval
    }

    pub fn feed(&mut self) {
        self.days_since_feeding = 0;
        self.is_withering = false;
    }

    pub fn advance_day(&mut self) {
        self.days_since_feeding += 1;
        self.days_since_harvest += 1;
        if self.needs_feeding() {
            self.is_withering = true;
        }
    }

    pub fn can_harvest(&self) -> bool {
        self.days_since_harvest >= 1 && !self.is_withering
    }

    pub fn harvest(&mut self) {
        self.days_since_harvest = 0;
    }
}

// ============================================================================
// PASSIVE PLANT BENEFITS
// ============================================================================

/// Component for plants that provide passive HP regeneration (like Umdhlebi)
#[derive(Component)]
pub struct PassiveHpRegen {
    pub regen_rate: f32, // HP per second
}

/// Component for plants that provide passive spirit regeneration (like Baobab)
#[derive(Component)]
pub struct PassiveSpiritRegen {
    pub regen_rate: f32, // Spirit per second
}

// ============================================================================
// PLANT ITEM GENERATION
// ============================================================================

/// Rare items that can be generated by Umdhlebi
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UmdhlebiRareItem {
    BloodCrystal,  // Powerful crafting material
    LifeEssence,   // Legendary potion ingredient
    AncestralBone, // Spirit communication item
    VitalSeed,     // Plant a new blood plant
    Nothing,       // Sometimes generates nothing
}

/// Status effects that Baobab cures can remove
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCureType {
    FullPartyAlignment, // Cures all status effects for full party
}
