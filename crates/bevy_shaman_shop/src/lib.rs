use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

#[cfg(test)]
mod tests;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<resources::Currency>()
            .init_resource::<resources::ShopInventory>()
            .add_systems(Update, systems::populate_shop_inventory.run_if(in_state(GameState::Playing)))
            .add_event::<systems::events::PurchaseEvent>()
            .add_event::<systems::events::SellEvent>();
    }
}

pub mod components {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct Shopkeeper {
        pub name: String,
        pub greeting: String,
    }

    #[derive(Component)]
    pub struct ShopMarker;
}

pub mod resources {
    use bevy::prelude::*;
    use bevy_shaman_items::components::{Item, ItemType, SpiritOrbSize};

    #[derive(Resource, Default)]
    pub struct Currency {
        pub gold: u32,
    }

    #[derive(Resource)]
    pub struct ShopInventory {
        pub items: Vec<ShopItem>,
        pub max_items: usize,
    }

    impl Default for ShopInventory {
        fn default() -> Self {
            Self {
                items: Vec::new(),
                max_items: 20,
            }
        }
    }

    #[derive(Clone)]
    pub struct ShopItem {
        pub item: Item,
        pub price: u32,
        pub stock: u32,
    }

    impl ShopInventory {
        pub fn add_item(&mut self, item: Item, price: u32, stock: u32) -> bool {
            if self.items.len() >= self.max_items {
                return false;
            }
            self.items.push(ShopItem { item, price, stock });
            true
        }

        pub fn remove_stock(&mut self, item_id: &str, quantity: u32) -> bool {
            if let Some(shop_item) = self.items.iter_mut().find(|i| i.item.id == item_id) {
                if shop_item.stock >= quantity {
                    shop_item.stock -= quantity;
                    return true;
                }
            }
            false
        }

        pub fn add_stock(&mut self, item_id: &str, quantity: u32) {
            if let Some(shop_item) = self.items.iter_mut().find(|i| i.item.id == item_id) {
                shop_item.stock = shop_item.stock.saturating_add(quantity);
            }
        }

        pub fn get_price(&self, item_id: &str) -> Option<u32> {
            self.items.iter()
                .find(|i| i.item.id == item_id)
                .map(|i| i.price)
        }

        pub fn populate_default_items(&mut self) {
            // Seeds
            self.add_item(
                Item {
                    id: "moonpetal_seed".to_string(),
                    display_name: "Moon Petal Seed".to_string(),
                    item_type: ItemType::Plant(bevy_shaman_items::components::PlantType::MoonPetal),
                    max_stack: 99,
                },
                10,
                50,
            );

            self.add_item(
                Item {
                    id: "starroot_seed".to_string(),
                    display_name: "Star Root Seed".to_string(),
                    item_type: ItemType::Plant(bevy_shaman_items::components::PlantType::StarRoot),
                    max_stack: 99,
                },
                15,
                50,
            );

            self.add_item(
                Item {
                    id: "lifeleaf_seed".to_string(),
                    display_name: "Life Leaf Seed".to_string(),
                    item_type: ItemType::Plant(bevy_shaman_items::components::PlantType::LifeLeaf),
                    max_stack: 99,
                },
                20,
                30,
            );

            // Tools and items
            self.add_item(
                Item {
                    id: "shovel".to_string(),
                    display_name: "Shovel".to_string(),
                    item_type: ItemType::KeyItem,
                    max_stack: 1,
                },
                100,
                5,
            );

            self.add_item(
                Item {
                    id: "remedy".to_string(),
                    display_name: "Remedy".to_string(),
                    item_type: ItemType::Remedy,
                    max_stack: 10,
                },
                50,
                20,
            );

            // Spirit orbs
            self.add_item(
                Item {
                    id: "small_spirit_orb".to_string(),
                    display_name: "Small Spirit Orb".to_string(),
                    item_type: ItemType::SpiritOrb(SpiritOrbSize::Small),
                    max_stack: 10,
                },
                25,
                30,
            );

            self.add_item(
                Item {
                    id: "medium_spirit_orb".to_string(),
                    display_name: "Medium Spirit Orb".to_string(),
                    item_type: ItemType::SpiritOrb(SpiritOrbSize::Medium),
                    max_stack: 10,
                },
                60,
                15,
            );

            // Crafting materials
            self.add_item(
                Item {
                    id: "wood".to_string(),
                    display_name: "Wood".to_string(),
                    item_type: ItemType::CraftingMaterial,
                    max_stack: 99,
                },
                5,
                100,
            );

            self.add_item(
                Item {
                    id: "stone".to_string(),
                    display_name: "Stone".to_string(),
                    item_type: ItemType::CraftingMaterial,
                    max_stack: 99,
                },
                8,
                80,
            );

            self.add_item(
                Item {
                    id: "iron_ore".to_string(),
                    display_name: "Iron Ore".to_string(),
                    item_type: ItemType::CraftingMaterial,
                    max_stack: 50,
                },
                30,
                20,
            );
        }
    }
}

pub mod systems {
    use bevy::prelude::*;
    use bevy_shaman_items::components::Inventory;
    use crate::resources::{Currency, ShopInventory};

    pub mod events {
        use bevy::prelude::*;

        #[derive(Event)]
        pub struct PurchaseEvent {
            pub buyer: Entity,
            pub item_id: String,
            pub quantity: u32,
        }

        #[derive(Event)]
        pub struct SellEvent {
            pub seller: Entity,
            pub item_id: String,
            pub quantity: u32,
        }
    }

    pub fn populate_shop_inventory(
        mut shop: ResMut<ShopInventory>,
    ) {
        // Only populate once
        if shop.items.is_empty() {
            shop.populate_default_items();
        }
    }

    pub fn process_purchases(
        mut purchase_events: EventReader<events::PurchaseEvent>,
        mut currency: ResMut<Currency>,
        mut shop: ResMut<ShopInventory>,
        mut inventory_query: Query<&mut Inventory>,
    ) {
        for event in purchase_events.read() {
            if let Some(price) = shop.get_price(&event.item_id) {
                let total_cost = price * event.quantity;

                // Check if player has enough gold
                if currency.gold >= total_cost {
                    // Check if shop has enough stock
                    if shop.remove_stock(&event.item_id, event.quantity) {
                        // Deduct gold
                        currency.gold -= total_cost;

                        // Add item to player inventory
                        if let Ok(mut inventory) = inventory_query.get_mut(event.buyer) {
                            if let Some(shop_item) = shop.items.iter().find(|i| i.item.id == event.item_id) {
                                inventory.add_item(shop_item.item.clone(), event.quantity);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn process_sales(
        mut sell_events: EventReader<events::SellEvent>,
        mut currency: ResMut<Currency>,
        mut shop: ResMut<ShopInventory>,
        mut inventory_query: Query<&mut Inventory>,
    ) {
        for event in sell_events.read() {
            if let Ok(mut inventory) = inventory_query.get_mut(event.seller) {
                // Check if player has the item
                if inventory.count_item(&event.item_id) >= event.quantity {
                    // Remove item from inventory
                    if inventory.remove_item(&event.item_id, event.quantity) {
                        // Get price (sell at 50% of shop price)
                        if let Some(price) = shop.get_price(&event.item_id) {
                            let sell_price = (price / 2) * event.quantity;
                            currency.gold += sell_price;

                            // Add stock back to shop
                            shop.add_stock(&event.item_id, event.quantity);
                        }
                    }
                }
            }
        }
    }
}
