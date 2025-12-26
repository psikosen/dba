#[cfg(test)]
mod shop_tests {
    use super::super::resources::*;
    use bevy_shaman_items::components::{Item, ItemType, SpiritOrbSize};

    // Helper to create a test item
    fn create_test_item(id: &str, max_stack: u32) -> Item {
        Item {
            id: id.to_string(),
            display_name: format!("Test {}", id),
            item_type: ItemType::CraftingMaterial,
            max_stack,
        }
    }

    // ============================================================================
    // CURRENCY TESTS
    // ============================================================================

    #[test]
    fn test_currency_default() {
        let currency = Currency::default();
        assert_eq!(currency.gold, 0);
    }

    #[test]
    fn test_currency_custom() {
        let currency = Currency { gold: 1000 };
        assert_eq!(currency.gold, 1000);
    }

    // ============================================================================
    // SHOP INVENTORY TESTS
    // ============================================================================

    #[test]
    fn test_shop_inventory_default() {
        let shop = ShopInventory::default();
        assert_eq!(shop.items.len(), 0);
        assert_eq!(shop.max_items, 20);
    }

    #[test]
    fn test_shop_add_item() {
        let mut shop = ShopInventory::default();
        let item = create_test_item("wood", 99);

        let result = shop.add_item(item, 10, 50);
        assert!(result);
        assert_eq!(shop.items.len(), 1);
    }

    #[test]
    fn test_shop_add_item_at_max_capacity() {
        let mut shop = ShopInventory::default();

        // Fill shop to max capacity
        for i in 0..20 {
            let item = create_test_item(&format!("item_{}", i), 99);
            assert!(shop.add_item(item, 10, 10));
        }

        // Try to add one more
        let item = create_test_item("extra", 99);
        assert!(!shop.add_item(item, 10, 10));
    }

    #[test]
    fn test_shop_remove_stock() {
        let mut shop = ShopInventory::default();
        let item = create_test_item("wood", 99);
        shop.add_item(item, 10, 50);

        let result = shop.remove_stock("wood", 10);
        assert!(result);
        assert_eq!(shop.items[0].stock, 40);
    }

    #[test]
    fn test_shop_remove_stock_insufficient() {
        let mut shop = ShopInventory::default();
        let item = create_test_item("wood", 99);
        shop.add_item(item, 10, 5);

        let result = shop.remove_stock("wood", 10);
        assert!(!result);
        assert_eq!(shop.items[0].stock, 5); // Stock unchanged
    }

    #[test]
    fn test_shop_remove_stock_exact_amount() {
        let mut shop = ShopInventory::default();
        let item = create_test_item("wood", 99);
        shop.add_item(item, 10, 10);

        let result = shop.remove_stock("wood", 10);
        assert!(result);
        assert_eq!(shop.items[0].stock, 0);
    }

    #[test]
    fn test_shop_remove_stock_nonexistent_item() {
        let mut shop = ShopInventory::default();
        let result = shop.remove_stock("nonexistent", 1);
        assert!(!result);
    }

    #[test]
    fn test_shop_add_stock() {
        let mut shop = ShopInventory::default();
        let item = create_test_item("wood", 99);
        shop.add_item(item, 10, 10);

        shop.add_stock("wood", 5);
        assert_eq!(shop.items[0].stock, 15);
    }

    #[test]
    fn test_shop_add_stock_nonexistent_item() {
        let mut shop = ShopInventory::default();
        shop.add_stock("nonexistent", 5);
        assert_eq!(shop.items.len(), 0); // No change
    }

    #[test]
    fn test_shop_get_price() {
        let mut shop = ShopInventory::default();
        let item = create_test_item("wood", 99);
        shop.add_item(item, 25, 10);

        let price = shop.get_price("wood");
        assert_eq!(price, Some(25));
    }

    #[test]
    fn test_shop_get_price_nonexistent() {
        let shop = ShopInventory::default();
        let price = shop.get_price("nonexistent");
        assert_eq!(price, None);
    }

    #[test]
    fn test_shop_populate_default_items() {
        let mut shop = ShopInventory::default();
        shop.populate_default_items();

        // Should have populated items
        assert!(shop.items.len() > 0);

        // Check for specific items
        assert!(shop.get_price("moonpetal_seed").is_some());
        assert!(shop.get_price("shovel").is_some());
        assert!(shop.get_price("small_spirit_orb").is_some());
    }

    #[test]
    fn test_shop_item_prices() {
        let mut shop = ShopInventory::default();
        shop.populate_default_items();

        // Verify specific prices
        assert_eq!(shop.get_price("moonpetal_seed"), Some(10));
        assert_eq!(shop.get_price("starroot_seed"), Some(15));
        assert_eq!(shop.get_price("lifeleaf_seed"), Some(20));
        assert_eq!(shop.get_price("shovel"), Some(100));
        assert_eq!(shop.get_price("small_spirit_orb"), Some(25));
        assert_eq!(shop.get_price("medium_spirit_orb"), Some(60));
    }

    #[test]
    fn test_shop_item_initial_stock() {
        let mut shop = ShopInventory::default();
        shop.populate_default_items();

        // Verify initial stock levels
        let moonpetal = shop.items.iter().find(|i| i.item.id == "moonpetal_seed");
        assert!(moonpetal.is_some());
        assert_eq!(moonpetal.unwrap().stock, 50);

        let shovel = shop.items.iter().find(|i| i.item.id == "shovel");
        assert!(shovel.is_some());
        assert_eq!(shovel.unwrap().stock, 5);
    }

    // ============================================================================
    // SHOP ITEM TESTS
    // ============================================================================

    #[test]
    fn test_shop_item_creation() {
        let item = create_test_item("test", 10);
        let shop_item = ShopItem {
            item: item.clone(),
            price: 50,
            stock: 100,
        };

        assert_eq!(shop_item.item.id, "test");
        assert_eq!(shop_item.price, 50);
        assert_eq!(shop_item.stock, 100);
    }

    #[test]
    fn test_shop_item_clone() {
        let item = create_test_item("test", 10);
        let shop_item1 = ShopItem {
            item,
            price: 50,
            stock: 100,
        };

        let shop_item2 = shop_item1.clone();
        assert_eq!(shop_item1.price, shop_item2.price);
        assert_eq!(shop_item1.stock, shop_item2.stock);
    }

    // ============================================================================
    // EDGE CASE TESTS - Finding Bugs!
    // ============================================================================

    #[test]
    fn test_shop_zero_stock() {
        let mut shop = ShopInventory::default();
        let item = create_test_item("wood", 99);
        shop.add_item(item, 10, 0);

        let result = shop.remove_stock("wood", 1);
        assert!(!result);
    }

    #[test]
    fn test_shop_zero_price() {
        let mut shop = ShopInventory::default();
        let item = create_test_item("free_item", 99);
        shop.add_item(item, 0, 10);

        let price = shop.get_price("free_item");
        assert_eq!(price, Some(0));
    }

    #[test]
    fn test_shop_large_quantities() {
        let mut shop = ShopInventory::default();
        let item = create_test_item("bulk", 99999);
        shop.add_item(item, 1, 10000);

        let result = shop.remove_stock("bulk", 5000);
        assert!(result);
        assert_eq!(shop.items[0].stock, 5000);
    }

    #[test]
    fn test_shop_add_stock_overflow() {
        let mut shop = ShopInventory::default();
        let item = create_test_item("wood", 99);
        shop.add_item(item, 10, u32::MAX - 10);

        // Adding stock that would overflow
        shop.add_stock("wood", 20);
        // Should overflow or saturate depending on implementation
        // Just ensure it doesn't panic
    }

    #[test]
    fn test_shop_multiple_same_items() {
        // Bug test: What if we try to add the same item twice?
        let mut shop = ShopInventory {
            items: Vec::new(),
            max_items: 20,
        };

        let item1 = create_test_item("wood", 99);
        let item2 = create_test_item("wood", 99);

        shop.add_item(item1, 10, 50);
        shop.add_item(item2, 15, 30);

        // Should have 2 separate entries (potential bug!)
        assert_eq!(shop.items.len(), 2);

        // Both should be findable, but get_price will return the first one
        assert_eq!(shop.get_price("wood"), Some(10));
    }

    #[test]
    fn test_shop_remove_then_add_back() {
        let mut shop = ShopInventory::default();
        let item = create_test_item("wood", 99);
        shop.add_item(item, 10, 50);

        shop.remove_stock("wood", 30);
        shop.add_stock("wood", 15);

        assert_eq!(shop.items[0].stock, 35);
    }

    #[test]
    fn test_shop_empty_item_id() {
        let mut shop = ShopInventory::default();
        let item = Item {
            id: "".to_string(),
            display_name: "Empty".to_string(),
            item_type: ItemType::CraftingMaterial,
            max_stack: 1,
        };

        shop.add_item(item, 10, 10);
        assert!(shop.get_price("").is_some());
    }

    #[test]
    fn test_shop_unicode_item_ids() {
        let mut shop = ShopInventory::default();
        let item = Item {
            id: "月花種".to_string(),
            display_name: "Moon Flower".to_string(),
            item_type: ItemType::CraftingMaterial,
            max_stack: 99,
        };

        shop.add_item(item, 10, 10);
        assert!(shop.get_price("月花種").is_some());
    }

    #[test]
    fn test_shop_item_types_variety() {
        let mut shop = ShopInventory::default();
        shop.populate_default_items();

        let mut has_plant = false;
        let mut has_spirit_orb = false;
        let mut has_crafting = false;
        let mut has_key_item = false;

        for shop_item in &shop.items {
            match shop_item.item.item_type {
                ItemType::Plant(_) => has_plant = true,
                ItemType::SpiritOrb(_) => has_spirit_orb = true,
                ItemType::CraftingMaterial => has_crafting = true,
                ItemType::KeyItem => has_key_item = true,
                _ => {}
            }
        }

        assert!(has_plant);
        assert!(has_spirit_orb);
        assert!(has_crafting);
        assert!(has_key_item);
    }

    #[test]
    fn test_spirit_orb_sizes() {
        assert_eq!(SpiritOrbSize::Small.spirit_restore(), 20.0);
        assert_eq!(SpiritOrbSize::Medium.spirit_restore(), 50.0);
        assert_eq!(SpiritOrbSize::Large.spirit_restore(), 100.0);

        assert_eq!(SpiritOrbSize::Small.stamina_restore(), 15.0);
        assert_eq!(SpiritOrbSize::Medium.stamina_restore(), 40.0);
        assert_eq!(SpiritOrbSize::Large.stamina_restore(), 80.0);
    }
}
