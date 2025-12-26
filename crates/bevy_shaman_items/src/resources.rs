use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::components::{Item, ItemType, SpiritOrbSize};

// ============================================================================
// LOOT TABLES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTable {
    pub id: String,
    pub drops: Vec<LootDrop>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootDrop {
    pub item_id: String,
    pub weight: f32,
    pub min_quantity: u32,
    pub max_quantity: u32,
}

#[derive(Resource, Default)]
pub struct LootTableDB {
    pub tables: HashMap<String, LootTable>,
}

impl LootTableDB {
    pub fn register(&mut self, table: LootTable) {
        self.tables.insert(table.id.clone(), table);
    }

    pub fn get(&self, id: &str) -> Option<&LootTable> {
        self.tables.get(id)
    }

    pub fn populate_defaults(&mut self) {
        // Spirit World monster loot table (includes Spirit Orbs)
        self.register(LootTable {
            id: "spirit_world_monster".to_string(),
            drops: vec![
                LootDrop {
                    item_id: "spirit_orb_small".to_string(),
                    weight: 0.5,
                    min_quantity: 1,
                    max_quantity: 2,
                },
                LootDrop {
                    item_id: "spirit_orb_medium".to_string(),
                    weight: 0.25,
                    min_quantity: 1,
                    max_quantity: 1,
                },
                LootDrop {
                    item_id: "spirit_orb_large".to_string(),
                    weight: 0.05,
                    min_quantity: 1,
                    max_quantity: 1,
                },
            ],
        });

        // Corrupted monster loot table
        self.register(LootTable {
            id: "corrupt_monster".to_string(),
            drops: vec![
                LootDrop {
                    item_id: "corrupt_essence".to_string(),
                    weight: 0.6,
                    min_quantity: 1,
                    max_quantity: 3,
                },
                LootDrop {
                    item_id: "spirit_orb_small".to_string(),
                    weight: 0.3,
                    min_quantity: 1,
                    max_quantity: 1,
                },
            ],
        });

        // Plant monster loot table (drops seeds)
        self.register(LootTable {
            id: "plant_monster".to_string(),
            drops: vec![
                LootDrop {
                    item_id: "moonpetal_seed".to_string(),
                    weight: 0.3,
                    min_quantity: 1,
                    max_quantity: 3,
                },
                LootDrop {
                    item_id: "shadowroot_seed".to_string(),
                    weight: 0.25,
                    min_quantity: 1,
                    max_quantity: 2,
                },
                LootDrop {
                    item_id: "crystalmoss_seed".to_string(),
                    weight: 0.2,
                    min_quantity: 1,
                    max_quantity: 2,
                },
                LootDrop {
                    item_id: "voidflower_seed".to_string(),
                    weight: 0.15,
                    min_quantity: 1,
                    max_quantity: 1,
                },
                LootDrop {
                    item_id: "eternalbark_seed".to_string(),
                    weight: 0.1,
                    min_quantity: 1,
                    max_quantity: 1,
                },
            ],
        });
    }
}

// ============================================================================
// ITEM DATABASE
// ============================================================================

#[derive(Resource, Default)]
pub struct ItemDB {
    pub items: HashMap<String, Item>,
}

impl ItemDB {
    pub fn register(&mut self, item: Item) {
        self.items.insert(item.id.clone(), item);
    }

    pub fn get(&self, id: &str) -> Option<&Item> {
        self.items.get(id)
    }

    pub fn populate_defaults(&mut self) {
        // Spirit Orbs
        self.register(Item {
            id: "spirit_orb_small".to_string(),
            display_name: "Small Spirit Orb".to_string(),
            item_type: ItemType::SpiritOrb(SpiritOrbSize::Small),
            max_stack: 99,
        });

        self.register(Item {
            id: "spirit_orb_medium".to_string(),
            display_name: "Medium Spirit Orb".to_string(),
            item_type: ItemType::SpiritOrb(SpiritOrbSize::Medium),
            max_stack: 99,
        });

        self.register(Item {
            id: "spirit_orb_large".to_string(),
            display_name: "Large Spirit Orb".to_string(),
            item_type: ItemType::SpiritOrb(SpiritOrbSize::Large),
            max_stack: 99,
        });

        // Crafting materials
        self.register(Item {
            id: "healing_herb".to_string(),
            display_name: "Healing Herb".to_string(),
            item_type: ItemType::Herb,
            max_stack: 50,
        });

        self.register(Item {
            id: "corrupt_essence".to_string(),
            display_name: "Corrupt Essence".to_string(),
            item_type: ItemType::CraftingMaterial,
            max_stack: 20,
        });
    }
}

// ============================================================================
// CRAFTING RECIPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub display_name: String,
    pub ingredients: Vec<Ingredient>,
    pub output_item_id: String,
    pub output_quantity: u32,
    pub required_station: Option<crate::components::CraftingStationType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub item_id: String,
    pub quantity: u32,
}

#[derive(Resource, Default)]
pub struct RecipeDB {
    pub recipes: HashMap<String, Recipe>,
}

impl RecipeDB {
    pub fn register(&mut self, recipe: Recipe) {
        self.recipes.insert(recipe.id.clone(), recipe);
    }

    pub fn get(&self, id: &str) -> Option<&Recipe> {
        self.recipes.get(id)
    }

    pub fn populate_defaults(&mut self) {
        self.register(Recipe {
            id: "healing_remedy".to_string(),
            display_name: "Healing Remedy".to_string(),
            ingredients: vec![Ingredient {
                item_id: "healing_herb".to_string(),
                quantity: 3,
            }],
            output_item_id: "healing_remedy".to_string(),
            output_quantity: 1,
            required_station: Some(crate::components::CraftingStationType::HerbBench),
        });
    }
}
