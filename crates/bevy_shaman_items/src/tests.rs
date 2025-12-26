#[cfg(test)]
mod item_tests {
    use super::super::components::*;

    // Helper to create test item
    fn create_test_item(id: &str, max_stack: u32) -> Item {
        Item {
            id: id.to_string(),
            display_name: format!("Test {}", id),
            item_type: ItemType::CraftingMaterial,
            max_stack,
        }
    }

    // ============================================================================
    // INVENTORY TESTS
    // ============================================================================

    #[test]
    fn test_inventory_default() {
        let inv = Inventory::default();
        assert_eq!(inv.items.len(), 0);
        assert_eq!(inv.max_slots, 0);
    }

    #[test]
    fn test_inventory_new() {
        let inv = Inventory::new(20);
        assert_eq!(inv.items.len(), 0);
        assert_eq!(inv.max_slots, 20);
    }

    #[test]
    fn test_inventory_add_item_single() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("wood", 99);

        let result = inv.add_item(item, 10);
        assert!(result);
        assert_eq!(inv.items.len(), 1);
        assert_eq!(inv.items[0].quantity, 10);
    }

    #[test]
    fn test_inventory_add_item_stacking() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("wood", 99);

        inv.add_item(item.clone(), 10);
        inv.add_item(item.clone(), 20);

        assert_eq!(inv.items.len(), 1);
        assert_eq!(inv.items[0].quantity, 30);
    }

    #[test]
    fn test_inventory_add_item_stack_overflow() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("wood", 50);

        inv.add_item(item.clone(), 40);
        let result = inv.add_item(item.clone(), 20);

        // Should create a new stack
        assert!(result);
        assert_eq!(inv.items.len(), 2);
        assert_eq!(inv.items[0].quantity, 50); // First stack maxed
        assert_eq!(inv.items[1].quantity, 10); // Remaining in second stack
    }

    #[test]
    fn test_inventory_add_item_full_slots() {
        let mut inv = Inventory::new(2);

        let item1 = create_test_item("wood", 1);
        let item2 = create_test_item("stone", 1);
        let item3 = create_test_item("iron", 1);

        assert!(inv.add_item(item1, 1));
        assert!(inv.add_item(item2, 1));
        assert!(!inv.add_item(item3, 1)); // No space
    }

    #[test]
    fn test_inventory_remove_item() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("wood", 99);
        inv.add_item(item.clone(), 30);

        let result = inv.remove_item("wood", 10);
        assert!(result);
        assert_eq!(inv.items[0].quantity, 20);
    }

    #[test]
    fn test_inventory_remove_item_exact_amount() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("wood", 99);
        inv.add_item(item, 10);

        let result = inv.remove_item("wood", 10);
        assert!(result);
        assert_eq!(inv.items.len(), 0); // Stack removed
    }

    #[test]
    fn test_inventory_remove_item_multiple_stacks() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("wood", 50);

        inv.add_item(item.clone(), 50);
        inv.add_item(item.clone(), 30);

        let result = inv.remove_item("wood", 60);
        assert!(result);
        assert_eq!(inv.items.len(), 1);
        assert_eq!(inv.items[0].quantity, 20);
    }

    #[test]
    fn test_inventory_remove_item_insufficient() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("wood", 99);
        inv.add_item(item, 10);

        let result = inv.remove_item("wood", 20);
        assert!(!result);
        assert_eq!(inv.items[0].quantity, 10); // Unchanged
    }

    #[test]
    fn test_inventory_remove_item_nonexistent() {
        let mut inv = Inventory::new(20);
        let result = inv.remove_item("nonexistent", 1);
        assert!(!result);
    }

    #[test]
    fn test_inventory_count_item() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("wood", 99);

        inv.add_item(item.clone(), 30);
        inv.add_item(item.clone(), 20);

        let count = inv.count_item("wood");
        assert_eq!(count, 50);
    }

    #[test]
    fn test_inventory_count_item_zero() {
        let inv = Inventory::new(20);
        let count = inv.count_item("wood");
        assert_eq!(count, 0);
    }

    // ============================================================================
    // SPIRIT ORB SIZE TESTS
    // ============================================================================

    #[test]
    fn test_spirit_orb_restore_values() {
        assert_eq!(SpiritOrbSize::Small.spirit_restore(), 20.0);
        assert_eq!(SpiritOrbSize::Medium.spirit_restore(), 50.0);
        assert_eq!(SpiritOrbSize::Large.spirit_restore(), 100.0);

        assert_eq!(SpiritOrbSize::Small.stamina_restore(), 15.0);
        assert_eq!(SpiritOrbSize::Medium.stamina_restore(), 40.0);
        assert_eq!(SpiritOrbSize::Large.stamina_restore(), 80.0);
    }

    // ============================================================================
    // ITEM TYPE TESTS
    // ============================================================================

    #[test]
    fn test_item_types_exist() {
        let _ = ItemType::SpiritOrb(SpiritOrbSize::Small);
        let _ = ItemType::Herb;
        let _ = ItemType::Remedy;
        let _ = ItemType::CraftingMaterial;
        let _ = ItemType::KeyItem;
    }

    // ============================================================================
    // EDGE CASE TESTS - Finding Bugs!
    // ============================================================================

    #[test]
    fn test_inventory_zero_slots() {
        let mut inv = Inventory::new(0);
        let item = create_test_item("wood", 99);

        let result = inv.add_item(item, 1);
        assert!(!result);
    }

    #[test]
    fn test_inventory_add_zero_quantity() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("wood", 99);

        let result = inv.add_item(item, 0);
        assert!(result); // Implementation may vary
    }

    #[test]
    fn test_inventory_remove_zero_quantity() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("wood", 99);
        inv.add_item(item, 10);

        let result = inv.remove_item("wood", 0);
        assert!(result); // Removing 0 should succeed trivially
        assert_eq!(inv.items[0].quantity, 10);
    }

    #[test]
    fn test_inventory_stack_exactly_at_max() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("wood", 50);

        inv.add_item(item.clone(), 50);
        let result = inv.add_item(item.clone(), 1);

        // Should create new stack
        assert!(result);
        assert_eq!(inv.items.len(), 2);
    }

    #[test]
    fn test_inventory_add_item_max_stack_1() {
        let mut inv = Inventory::new(20);
        let item = create_test_item("key", 1);

        inv.add_item(item.clone(), 1);
        inv.add_item(item.clone(), 1);

        // Should have 2 separate stacks
        assert_eq!(inv.items.len(), 2);
    }

    #[test]
    fn test_inventory_remove_from_middle_stack() {
        let mut inv = Inventory::new(20);
        let item1 = create_test_item("wood", 50);
        let item2 = create_test_item("stone", 50);
        let item3 = create_test_item("wood", 50);

        inv.add_item(item1, 50);
        inv.add_item(item2, 50);
        inv.add_item(item3, 50);

        // Remove 75 wood (should remove first stack + 25 from third)
        let result = inv.remove_item("wood", 75);
        assert!(result);

        // Should have stone and remaining wood
        assert_eq!(inv.items.len(), 2);
        let wood_count = inv.count_item("wood");
        assert_eq!(wood_count, 25);
    }

    #[test]
    fn test_inventory_large_quantities() {
        let mut inv = Inventory::new(100);
        let item = create_test_item("bulk", 99999);

        for _ in 0..50 {
            inv.add_item(item.clone(), 1000);
        }

        let count = inv.count_item("bulk");
        assert_eq!(count, 50000);
    }

    #[test]
    fn test_inventory_many_different_items() {
        let mut inv = Inventory::new(100);

        for i in 0..100 {
            let item = create_test_item(&format!("item_{}", i), 1);
            assert!(inv.add_item(item, 1));
        }

        assert_eq!(inv.items.len(), 100);

        // Try to add one more
        let item = create_test_item("extra", 1);
        assert!(!inv.add_item(item, 1));
    }

    #[test]
    fn test_inventory_empty_item_id() {
        let mut inv = Inventory::new(20);
        let item = Item {
            id: "".to_string(),
            display_name: "Empty".to_string(),
            item_type: ItemType::CraftingMaterial,
            max_stack: 10,
        };

        inv.add_item(item, 5);
        assert_eq!(inv.count_item(""), 5);
    }

    #[test]
    fn test_inventory_stacking_logic_bug_hunt() {
        // This test hunts for a specific bug: adding items that partially stack
        let mut inv = Inventory::new(10);
        let item = create_test_item("wood", 50);

        // Add 40, then add 30 more (should fill first stack to 50, put 20 in new stack)
        inv.add_item(item.clone(), 40);
        inv.add_item(item.clone(), 30);

        assert_eq!(inv.items.len(), 2);
        assert_eq!(inv.items[0].quantity, 50);
        assert_eq!(inv.items[1].quantity, 20);

        // Add 35 more (should fill second stack to 50, put 5 in third stack)
        inv.add_item(item.clone(), 35);

        assert_eq!(inv.items.len(), 3);
        assert_eq!(inv.items[0].quantity, 50);
        assert_eq!(inv.items[1].quantity, 50);
        assert_eq!(inv.items[2].quantity, 5);
    }

    #[test]
    fn test_inventory_remove_partial_stacks() {
        let mut inv = Inventory::new(10);
        let item = create_test_item("wood", 50);

        // Create 3 stacks: 50, 50, 30
        inv.add_item(item.clone(), 50);
        inv.add_item(item.clone(), 50);
        inv.add_item(item.clone(), 30);

        // Remove 120 (should remove first 2 stacks, leave 10 in third)
        inv.remove_item("wood", 120);

        assert_eq!(inv.items.len(), 1);
        assert_eq!(inv.items[0].quantity, 10);
    }

    #[test]
    fn test_pickupable_component() {
        let item = create_test_item("test", 10);
        let pickupable = Pickupable {
            item: item.clone(),
            quantity: 5,
            auto_pickup: true,
        };

        assert_eq!(pickupable.quantity, 5);
        assert!(pickupable.auto_pickup);
    }

    #[test]
    fn test_crafting_station_types() {
        use CraftingStationType::*;
        let _ = HerbBench;
        let _ = SpiritAltar;
        let _ = InstrumentWorkshop;
    }
}
