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
