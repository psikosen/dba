/// End-to-End Gameplay Integration Tests
///
/// These tests simulate full gameplay scenarios to ensure all systems
/// work together correctly. Unlike unit tests, these tests:
/// - Load all game plugins
/// - Simulate player actions over multiple frames
/// - Verify complex state transitions
/// - Test integration between multiple crates

use bevy::prelude::*;
use bevy_shaman_core::{
    components::{GridPosition, Health, Player, Spirit, Stamina},
    CorePlugin,
};
use bevy_shaman_items::{
    components::{Inventory, Item, ItemType, Pickupable, SpiritOrbSize},
    systems::inventory::{drop_items, pickup_items, use_items, ItemDropped, ItemUsed},
    ItemsPlugin,
};

// ============================================================================
// TEST UTILITIES
// ============================================================================

/// Create a minimal test app with necessary plugins for gameplay testing
fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(CorePlugin)
        .add_plugins(ItemsPlugin)
        .add_event::<ItemUsed>()
        .add_event::<ItemDropped>();
    app
}

/// Spawn a test player with starting stats
fn spawn_test_player(app: &mut App) -> Entity {
    app.world_mut()
        .spawn((
            Player,
            Health::new(100.0),
            Spirit::new(100.0),
            Stamina::new(100.0),
            GridPosition { x: 0, y: 0 },
            Inventory::new(20),
        ))
        .id()
}

/// Spawn a test item in the world
fn spawn_test_item(app: &mut App, item: Item, position: GridPosition, quantity: u32) -> Entity {
    app.world_mut()
        .spawn((
            Pickupable {
                item,
                quantity,
                auto_pickup: false,
            },
            position,
        ))
        .id()
}

/// Create a Spirit Orb item for testing
fn create_spirit_orb(size: SpiritOrbSize) -> Item {
    Item {
        id: format!("spirit_orb_{:?}", size),
        display_name: format!("{:?} Spirit Orb", size),
        item_type: ItemType::SpiritOrb(size),
        max_stack: 10,
    }
}

// ============================================================================
// E2E GAMEPLAY TESTS
// ============================================================================

#[test]
fn test_e2e_item_pickup_and_use_flow() {
    // Scenario: Player walks up to a Spirit Orb, picks it up, and uses it
    let mut app = create_test_app();
    let player = spawn_test_player(&mut app);

    // Place a Small Spirit Orb next to player
    let orb = create_spirit_orb(SpiritOrbSize::Small);
    let _item_entity = spawn_test_item(
        &mut app,
        orb.clone(),
        GridPosition { x: 0, y: 1 }, // 1 tile away
        1,
    );

    // Verify initial state
    {
        let player_ref = app.world().entity(player);
        let inventory = player_ref.get::<Inventory>().unwrap();
        assert_eq!(inventory.items.len(), 0, "Inventory should start empty");

        let spirit = player_ref.get::<Spirit>().unwrap();
        assert_eq!(spirit.current, 100.0, "Spirit should start at max");
    }

    // Simulate player pressing E to pick up item
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyE);

    // Run pickup system
    app.add_systems(Update, pickup_items);
    app.update();

    // Verify item was picked up
    {
        let player_ref = app.world().entity(player);
        let inventory = player_ref.get::<Inventory>().unwrap();
        assert_eq!(inventory.items.len(), 1, "Item should be in inventory");
        assert_eq!(
            inventory.items[0].item.id,
            orb.id,
            "Correct item should be picked up"
        );
        assert_eq!(inventory.items[0].quantity, 1);
    }

    // Reduce player's spirit to test consumption
    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut spirit = player_ref.get_mut::<Spirit>().unwrap();
        spirit.current = 50.0;
    }

    // Use the spirit orb
    app.world_mut()
        .resource_mut::<Events<ItemUsed>>()
        .send(ItemUsed {
            player,
            item_id: orb.id.clone(),
        });

    app.add_systems(Update, use_items);
    app.update();

    // Verify spirit was restored
    {
        let player_ref = app.world().entity(player);
        let spirit = player_ref.get::<Spirit>().unwrap();
        assert_eq!(
            spirit.current, 70.0,
            "Spirit should be restored by 20 (Small orb)"
        );

        let inventory = player_ref.get::<Inventory>().unwrap();
        assert_eq!(inventory.items.len(), 0, "Item should be consumed");
    }
}

#[test]
fn test_e2e_inventory_full_scenario() {
    // Scenario: Fill inventory, try to pick up more, drop items to make space
    let mut app = create_test_app();
    let player = spawn_test_player(&mut app);

    // Create inventory with only 2 slots
    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut inventory = player_ref.get_mut::<Inventory>().unwrap();
        inventory.max_slots = 2;
    }

    // Place 3 items in the world
    let item1 = create_spirit_orb(SpiritOrbSize::Small);
    let item2 = create_spirit_orb(SpiritOrbSize::Medium);
    let item3 = create_spirit_orb(SpiritOrbSize::Large);

    spawn_test_item(&mut app, item1.clone(), GridPosition { x: 0, y: 1 }, 1);
    spawn_test_item(&mut app, item2.clone(), GridPosition { x: 1, y: 0 }, 1);
    spawn_test_item(&mut app, item3.clone(), GridPosition { x: 0, y: -1 }, 1);

    // Set up pickup system
    app.add_systems(Update, pickup_items);

    // Pick up first item
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyE);
    app.world_mut()
        .entity_mut(player)
        .get_mut::<GridPosition>()
        .unwrap()
        .y = 1;
    app.update();

    // Pick up second item
    app.world_mut()
        .entity_mut(player)
        .get_mut::<GridPosition>()
        .unwrap()
        .x = 1;
    app.world_mut()
        .entity_mut(player)
        .get_mut::<GridPosition>()
        .unwrap()
        .y = 0;
    app.update();

    // Verify inventory is full
    {
        let player_ref = app.world().entity(player);
        let inventory = player_ref.get::<Inventory>().unwrap();
        assert_eq!(inventory.items.len(), 2, "Inventory should be full");
    }

    // Try to pick up third item (should fail)
    app.world_mut()
        .entity_mut(player)
        .get_mut::<GridPosition>()
        .unwrap()
        .y = -1;
    app.update();

    {
        let player_ref = app.world().entity(player);
        let inventory = player_ref.get::<Inventory>().unwrap();
        assert_eq!(
            inventory.items.len(),
            2,
            "Inventory should still be 2 (can't pick up more)"
        );
    }

    // Drop one item to make space
    app.add_systems(Update, drop_items);
    app.world_mut()
        .resource_mut::<Events<ItemDropped>>()
        .send(ItemDropped {
            player,
            item_id: item1.id.clone(),
            quantity: 1,
        });
    app.update();

    {
        let player_ref = app.world().entity(player);
        let inventory = player_ref.get::<Inventory>().unwrap();
        assert_eq!(inventory.items.len(), 1, "One item should remain");
    }

    // Now pick up the third item
    app.update();

    {
        let player_ref = app.world().entity(player);
        let inventory = player_ref.get::<Inventory>().unwrap();
        assert_eq!(inventory.items.len(), 2, "Should have 2 items again");
    }
}

#[test]
fn test_e2e_health_and_spirit_management() {
    // Scenario: Player takes damage, heals with items, manages spirit
    let mut app = create_test_app();
    let player = spawn_test_player(&mut app);

    // Reduce health
    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut health = player_ref.get_mut::<Health>().unwrap();
        health.take_damage(50.0);
    }

    // Give player a remedy
    let remedy = Item {
        id: "healing_remedy".to_string(),
        display_name: "Healing Remedy".to_string(),
        item_type: ItemType::Remedy,
        max_stack: 10,
    };

    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut inventory = player_ref.get_mut::<Inventory>().unwrap();
        inventory.add_item(remedy.clone(), 1);
    }

    // Use remedy
    app.add_systems(Update, use_items);
    app.world_mut()
        .resource_mut::<Events<ItemUsed>>()
        .send(ItemUsed {
            player,
            item_id: remedy.id.clone(),
        });
    app.update();

    // Verify health was restored
    {
        let player_ref = app.world().entity(player);
        let health = player_ref.get::<Health>().unwrap();
        assert_eq!(
            health.current, 80.0,
            "Health should be restored by 30 (remedy heals 30)"
        );
    }
}

#[test]
fn test_e2e_item_stacking_in_gameplay() {
    // Scenario: Player picks up multiple of the same item, they stack properly
    let mut app = create_test_app();
    let player = spawn_test_player(&mut app);

    let orb = create_spirit_orb(SpiritOrbSize::Small);

    // Spawn 3 separate orb pickups
    spawn_test_item(&mut app, orb.clone(), GridPosition { x: 0, y: 1 }, 2);
    spawn_test_item(&mut app, orb.clone(), GridPosition { x: 1, y: 0 }, 3);
    spawn_test_item(&mut app, orb.clone(), GridPosition { x: 0, y: -1 }, 1);

    app.add_systems(Update, pickup_items);

    // Pick up all items
    for pos in &[(0, 1), (1, 0), (0, -1)] {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);

        let mut player_pos = app
            .world_mut()
            .entity_mut(player)
            .get_mut::<GridPosition>()
            .unwrap();
        player_pos.x = pos.0;
        player_pos.y = pos.1;

        app.update();
    }

    // Verify all items stacked into single slot
    {
        let player_ref = app.world().entity(player);
        let inventory = player_ref.get::<Inventory>().unwrap();
        assert_eq!(
            inventory.items.len(),
            1,
            "All orbs should stack into one slot"
        );
        assert_eq!(
            inventory.items[0].quantity, 6,
            "Total quantity should be 2+3+1=6"
        );
    }
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[test]
#[ignore] // Run with: cargo test --test e2e_gameplay -- --ignored
fn test_e2e_performance_many_items() {
    // Test that the system can handle many items efficiently
    let mut app = create_test_app();
    let player = spawn_test_player(&mut app);

    // Create 100 item stacks in inventory
    {
        let mut player_ref = app.world_mut().entity_mut(player);
        let mut inventory = player_ref.get_mut::<Inventory>().unwrap();
        inventory.max_slots = 100;

        for i in 0..100 {
            let item = Item {
                id: format!("item_{}", i),
                display_name: format!("Item {}", i),
                item_type: ItemType::CraftingMaterial,
                max_stack: 99,
            };
            inventory.add_item(item, 50);
        }
    }

    // Time how long it takes to iterate and count all items
    use std::time::Instant;
    let start = Instant::now();

    for _ in 0..1000 {
        let player_ref = app.world().entity(player);
        let inventory = player_ref.get::<Inventory>().unwrap();
        let _total: u32 = inventory.items.iter().map(|s| s.quantity).sum();
    }

    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 100,
        "1000 iterations should complete in <100ms, took {:?}",
        duration
    );
}
