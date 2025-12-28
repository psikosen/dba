/// Tests for UI systems
#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy_shaman_core::components::{Health, Spirit, Stamina, Player};
    use bevy_shaman_items::components::{Inventory, Item, ItemStack};
    use bevy_shaman_story::systems::quest_system::*;
    use crate::ancestral_hud::*;
    use crate::ancestral_inventory::*;
    use crate::ancestral_quest_tracker::*;

    // ========================================================================
    // HUD TESTS
    // ========================================================================

    #[test]
    fn test_vitality_bars_update_with_player_stats() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn player with stats
        let player = app.world_mut().spawn((
            Player,
            Health { current: 75.0, max: 100.0 },
            Spirit { current: 50.0, max: 100.0 },
            Stamina { current: 30.0, max: 100.0 },
        )).id();

        // Spawn HUD bars
        let health_bar = app.world_mut().spawn((
            HealthBarFill,
            Node {
                width: Val::Percent(100.0),
                ..default()
            },
        )).id();

        let spirit_bar = app.world_mut().spawn((
            SpiritBarFill,
            Node {
                width: Val::Percent(100.0),
                ..default()
            },
        )).id();

        let stamina_bar = app.world_mut().spawn((
            StaminaBarFill,
            Node {
                width: Val::Percent(100.0),
                ..default()
            },
        )).id();

        // Add update system
        app.add_systems(Update, update_vitality_bars);
        app.update();

        // Check health bar width (75%)
        let health_node = app.world().entity(health_bar).get::<Node>().unwrap();
        if let Val::Percent(width) = health_node.width {
            assert_eq!(width, 75.0, "Health bar should be 75% wide");
        } else {
            panic!("Health bar width should be in percent");
        }

        // Check spirit bar width (50%)
        let spirit_node = app.world().entity(spirit_bar).get::<Node>().unwrap();
        if let Val::Percent(width) = spirit_node.width {
            assert_eq!(width, 50.0, "Spirit bar should be 50% wide");
        } else {
            panic!("Spirit bar width should be in percent");
        }

        // Check stamina bar width (30%)
        let stamina_node = app.world().entity(stamina_bar).get::<Node>().unwrap();
        if let Val::Percent(width) = stamina_node.width {
            assert_eq!(width, 30.0, "Stamina bar should be 30% wide");
        } else {
            panic!("Stamina bar width should be in percent");
        }
    }

    #[test]
    fn test_hud_state_toggles() {
        let mut state = AncestralHudState::default();

        assert!(state.show_minimap, "Minimap should be visible by default");
        assert!(state.show_abilities, "Abilities should be visible by default");
        assert!(state.show_quest_tracker, "Quest tracker should be visible by default");

        state.show_minimap = false;
        assert!(!state.show_minimap, "Minimap should be toggleable");
    }

    // ========================================================================
    // INVENTORY TESTS
    // ========================================================================

    #[test]
    fn test_inventory_state_initialization() {
        let state = AncestralInventoryState::default();

        assert!(!state.visible, "Inventory should be hidden by default");
        assert_eq!(state.current_tab, InventoryTab::Inventory);
        assert_eq!(state.selected_compartment, None);
        assert_eq!(state.hovered_compartment, None);
    }

    #[test]
    fn test_inventory_tab_switching() {
        let mut state = AncestralInventoryState::default();

        assert_eq!(state.current_tab, InventoryTab::Inventory);

        state.current_tab = InventoryTab::Character;
        assert_eq!(state.current_tab, InventoryTab::Character);

        state.current_tab = InventoryTab::Map;
        assert_eq!(state.current_tab, InventoryTab::Map);
    }

    #[test]
    fn test_item_icon_mapping() {
        // Test that item IDs map to appropriate icons
        use crate::ancestral_inventory::get_tactile_icon;

        assert_eq!(get_tactile_icon("health_potion"), "🧪");
        assert_eq!(get_tactile_icon("spirit_orb"), "🔮");
        assert_eq!(get_tactile_icon("iron_sword"), "⚔️");
        assert_eq!(get_tactile_icon("wooden_staff"), "🪄");
        assert_eq!(get_tactile_icon("leather_armor"), "🛡️");
        assert_eq!(get_tactile_icon("sacred_drum"), "🪘");
        assert_eq!(get_tactile_icon("unknown_item"), "📦"); // Default
    }

    // ========================================================================
    // QUEST TRACKER TESTS
    // ========================================================================

    #[test]
    fn test_quest_tracker_shows_active_quest() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create quest log with active quest
        let mut quest_log = QuestLog::default();
        quest_log.quests.push(
            Quest::new(
                "test_quest".to_string(),
                "Test Quest".to_string(),
                "A test quest for validation".to_string(),
                "Test Giver".to_string(),
            )
            .with_objective(QuestObjective::new(
                "Collect 5 herbs".to_string(),
                ObjectiveType::Collect("herb".to_string()),
                5,
            ))
        );
        quest_log.quests[0].status = QuestStatus::Active;

        app.insert_resource(quest_log);

        // Verify quest log has active quest
        let quest_log = app.world().resource::<QuestLog>();
        let active_quests: Vec<_> = quest_log.quests.iter()
            .filter(|q| q.status == QuestStatus::Active)
            .collect();

        assert_eq!(active_quests.len(), 1, "Should have one active quest");
        assert_eq!(active_quests[0].title, "Test Quest");
    }

    #[test]
    fn test_quest_objective_progress() {
        let mut quest = Quest::new(
            "progress_test".to_string(),
            "Progress Test".to_string(),
            "Test objective progress".to_string(),
            "Tester".to_string(),
        )
        .with_objective(QuestObjective::new(
            "Kill 10 monsters".to_string(),
            ObjectiveType::Kill("monster".to_string()),
            10,
        ));

        assert!(!quest.is_complete(), "Quest should not be complete initially");

        // Add progress
        quest.objectives[0].add_progress(5);
        assert_eq!(quest.objectives[0].progress, 5);
        assert!(!quest.is_complete(), "Quest should not be complete at 5/10");

        // Complete objective
        quest.objectives[0].add_progress(5);
        assert_eq!(quest.objectives[0].progress, 10);
        assert!(quest.is_complete(), "Quest should be complete at 10/10");
    }

    #[test]
    fn test_quest_progress_summary() {
        let quest = Quest::new(
            "summary_test".to_string(),
            "Summary Test".to_string(),
            "Test progress summary".to_string(),
            "Tester".to_string(),
        )
        .with_objective(QuestObjective::new(
            "Objective 1".to_string(),
            ObjectiveType::Custom,
            1,
        ))
        .with_objective(QuestObjective::new(
            "Objective 2".to_string(),
            ObjectiveType::Custom,
            1,
        ))
        .with_objective(QuestObjective::new(
            "Objective 3".to_string(),
            ObjectiveType::Custom,
            1,
        ));

        let summary = quest.progress_summary();
        assert_eq!(summary, "0/3 objectives");

        // Complete one objective
        let mut quest_modified = quest.clone();
        quest_modified.objectives[0].add_progress(1);
        let summary = quest_modified.progress_summary();
        assert_eq!(summary, "1/3 objectives");
    }

    #[test]
    fn test_quest_rewards_text() {
        let rewards = QuestRewards::default()
            .with_gold(100)
            .with_xp(500)
            .with_item("sword".to_string(), 1)
            .with_reputation("village".to_string(), 10);

        let text = rewards.rewards_text();

        assert!(text.contains("100 gold"), "Should show gold reward");
        assert!(text.contains("500 XP"), "Should show XP reward");
        assert!(text.contains("sword"), "Should show item reward");
        assert!(text.contains("+10 village"), "Should show reputation reward");
    }

    // ========================================================================
    // INTEGRATION TESTS
    // ========================================================================

    #[test]
    fn test_hud_player_stats_integration() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn complete player
        let player = app.world_mut().spawn((
            Player,
            Health { current: 100.0, max: 100.0 },
            Spirit { current: 100.0, max: 100.0 },
            Stamina { current: 100.0, max: 100.0 },
        )).id();

        // Spawn HUD
        let health_bar = app.world_mut().spawn((
            HealthBarFill,
            Node { width: Val::Percent(0.0), ..default() },
        )).id();

        app.add_systems(Update, update_vitality_bars);
        app.update();

        // Damage player
        let mut health = app.world_mut().entity_mut(player).get_mut::<Health>().unwrap();
        health.current = 25.0;
        drop(health);

        app.update();

        // Verify bar updated
        let health_node = app.world().entity(health_bar).get::<Node>().unwrap();
        if let Val::Percent(width) = health_node.width {
            assert_eq!(width, 25.0, "Health bar should update to 25% after damage");
        }
    }

    #[test]
    fn test_inventory_with_real_items() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create inventory with items
        let mut inventory = Inventory::default();
        inventory.items.push(Some(ItemStack {
            item: Item {
                id: "health_potion".to_string(),
                name: "Health Potion".to_string(),
                description: "Restores health".to_string(),
                item_type: bevy_shaman_items::components::ItemType::Consumable,
                rarity: bevy_shaman_items::components::Rarity::Common,
            },
            quantity: 3,
        }));

        // Spawn player with inventory
        let player = app.world_mut().spawn((
            Player,
            inventory,
        )).id();

        // Verify inventory has items
        let player_inv = app.world().entity(player).get::<Inventory>().unwrap();
        assert_eq!(player_inv.items.len(), 1);
        assert_eq!(player_inv.items[0].as_ref().unwrap().quantity, 3);
        assert_eq!(player_inv.items[0].as_ref().unwrap().item.id, "health_potion");
    }

    #[test]
    fn test_quest_log_ui_state() {
        let mut ui_state = QuestLogUIState::default();
        assert!(!ui_state.visible, "Quest log should be hidden by default");

        ui_state.visible = true;
        assert!(ui_state.visible, "Quest log should be toggleable");
    }
}
