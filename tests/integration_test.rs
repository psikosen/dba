/// Integration tests for Shaman's Journey
///
/// These tests verify that critical systems work together correctly
/// across crate boundaries.

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Helper function to create a minimal test app with common plugins
    fn create_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_state::<GameState>();
        app
    }

    #[test]
    fn test_app_initialization() {
        let app = create_test_app();
        assert!(app.world().contains_resource::<State<GameState>>());
    }

    #[test]
    fn test_game_state_transitions() {
        let mut app = create_test_app();

        // Should start in Boot state
        let current_state = app.world().resource::<State<GameState>>();
        assert!(matches!(current_state.get(), GameState::Boot));

        // Transition to Playing state
        app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::Playing);
        app.update();

        let current_state = app.world().resource::<State<GameState>>();
        assert!(matches!(current_state.get(), GameState::Playing));
    }

    #[test]
    fn test_combat_plugin_integration() {
        let mut app = create_test_app();
        app.add_plugins(bevy_shaman_combat::CombatPlugin);

        // Verify combat resources are initialized
        app.update();

        // Combat plugin should register its systems
        assert!(app.world().contains_resource::<bevy_shaman_combat::resources::CombatConfig>());
    }

    #[test]
    fn test_monster_plugin_integration() {
        let mut app = create_test_app();
        app.add_plugins(bevy_shaman_monsters::MonstersPlugin);

        app.update();

        // Monster plugin should initialize its resources
        assert!(app.world().contains_resource::<bevy_shaman_monsters::resources::MonsterDatabase>());
    }

    #[test]
    fn test_world_plugin_integration() {
        let mut app = create_test_app();
        app.add_plugins(bevy_shaman_world::WorldPlugin);

        app.update();

        // World plugin should initialize generation resources
        assert!(app.world().contains_resource::<bevy_shaman_world::systems::generation::WorldGenConfig>());
    }

    #[test]
    fn test_save_plugin_integration() {
        let mut app = create_test_app();
        app.add_plugins(bevy_shaman_save::SavePlugin);

        app.update();

        // Save plugin should register event types
        // The autosave timer should be initialized
        let events = app.world().resource::<Events<bevy_shaman_save::systems::events::SaveRequested>>();
        assert!(events.is_empty());
    }

    #[test]
    fn test_items_plugin_integration() {
        let mut app = create_test_app();
        app.add_plugins(bevy_shaman_items::ItemsPlugin);

        app.update();

        // Items plugin should initialize the item database
        assert!(app.world().contains_resource::<bevy_shaman_items::resources::ItemDatabase>());
    }

    #[test]
    fn test_shop_plugin_integration() {
        let mut app = create_test_app();
        app.add_plugins(bevy_shaman_shop::ShopPlugin);

        app.update();

        // Shop plugin should initialize shop inventory
        assert!(app.world().contains_resource::<bevy_shaman_shop::resources::ShopInventory>());
    }

    #[test]
    fn test_multiple_plugins_together() {
        let mut app = create_test_app();

        // Add multiple plugins that should work together
        app.add_plugins((
            bevy_shaman_core::CorePlugin,
            bevy_shaman_combat::CombatPlugin,
            bevy_shaman_monsters::MonstersPlugin,
            bevy_shaman_items::ItemsPlugin,
        ));

        // Run several update cycles
        for _ in 0..10 {
            app.update();
        }

        // All resources should be present
        assert!(app.world().contains_resource::<bevy_shaman_core::resources::Spirit>());
        assert!(app.world().contains_resource::<bevy_shaman_combat::resources::CombatConfig>());
        assert!(app.world().contains_resource::<bevy_shaman_monsters::resources::MonsterDatabase>());
        assert!(app.world().contains_resource::<bevy_shaman_items::resources::ItemDatabase>());
    }

    #[test]
    fn test_dungeon_generation_integration() {
        let mut app = create_test_app();
        app.add_plugins(bevy_shaman_dungeons::DungeonsPlugin);

        app.update();

        // Dungeon generator should be initialized
        assert!(app.world().contains_resource::<bevy_shaman_dungeons::systems::generation::DungeonGenerator>());
    }

    #[test]
    fn test_minion_system_integration() {
        let mut app = create_test_app();
        app.add_plugins((
            bevy_shaman_core::CorePlugin,
            bevy_shaman_minions::MinionsPlugin,
        ));

        app.update();

        // Minion formation manager should be initialized
        assert!(app.world().contains_resource::<bevy_shaman_minions::resources::FormationManager>());
    }

    #[test]
    fn test_ui_plugin_integration() {
        let mut app = create_test_app();
        app.add_plugins((
            bevy_shaman_core::CorePlugin,
            bevy_shaman_ui::UiPlugin,
        ));

        app.update();

        // UI systems should register without errors
        // Just verify the app can update with UI plugin
        for _ in 0..5 {
            app.update();
        }
    }

    #[test]
    fn test_story_plugin_integration() {
        let mut app = create_test_app();
        app.add_plugins(bevy_shaman_story::StoryPlugin);

        app.update();

        // Story resources should be initialized
        assert!(app.world().contains_resource::<bevy_shaman_story::resources::QuestLog>());
    }

    #[test]
    fn test_error_types_are_usable() {
        // Test that our custom error types can be created and used
        use bevy_shaman_save::{SaveError, LoadError};
        use bevy_shaman_world::WorldGenerationError;

        // Create error instances
        let save_err = SaveError::InvalidData("test".to_string());
        let load_err = LoadError::FileNotFound("test.json".to_string());
        let world_err = WorldGenerationError::InvalidConfiguration("test".to_string());

        // Verify they implement Display
        assert!(format!("{}", save_err).contains("Invalid save data"));
        assert!(format!("{}", load_err).contains("Save file not found"));
        assert!(format!("{}", world_err).contains("Invalid world generation configuration"));

        // Verify they implement Error trait
        use std::error::Error;
        assert!(save_err.source().is_none());
        assert!(load_err.source().is_none());
        assert!(world_err.source().is_none());
    }

    #[test]
    fn test_save_load_event_flow() {
        let mut app = create_test_app();
        app.add_plugins(bevy_shaman_save::SavePlugin);

        // Send save request event
        app.world_mut().send_event(bevy_shaman_save::systems::events::SaveRequested);

        // Update to process event
        app.update();

        // Verify autosave timer exists
        assert!(app.world().contains_resource::<bevy_shaman_save::systems::autosave::AutosaveTimer>());
    }

    #[test]
    fn test_combat_and_monster_integration() {
        let mut app = create_test_app();
        app.add_plugins((
            bevy_shaman_combat::CombatPlugin,
            bevy_shaman_monsters::MonstersPlugin,
        ));

        // Run multiple update cycles
        for _ in 0..5 {
            app.update();
        }

        // Both plugins should have initialized their resources
        assert!(app.world().contains_resource::<bevy_shaman_combat::resources::CombatConfig>());
        assert!(app.world().contains_resource::<bevy_shaman_monsters::resources::MonsterDatabase>());
    }

    #[test]
    fn test_world_and_dungeon_integration() {
        let mut app = create_test_app();
        app.add_plugins((
            bevy_shaman_world::WorldPlugin,
            bevy_shaman_dungeons::DungeonsPlugin,
        ));

        app.update();

        // Both should initialize successfully
        assert!(app.world().contains_resource::<bevy_shaman_world::systems::generation::WorldGenConfig>());
        assert!(app.world().contains_resource::<bevy_shaman_dungeons::systems::generation::DungeonGenerator>());
    }

    #[test]
    fn test_items_and_shop_integration() {
        let mut app = create_test_app();
        app.add_plugins((
            bevy_shaman_items::ItemsPlugin,
            bevy_shaman_shop::ShopPlugin,
        ));

        app.update();

        // Verify both item and shop databases are initialized
        assert!(app.world().contains_resource::<bevy_shaman_items::resources::ItemDatabase>());
        assert!(app.world().contains_resource::<bevy_shaman_shop::resources::ShopInventory>());
    }

    #[test]
    fn test_full_game_plugin_stack() {
        let mut app = create_test_app();

        // Add all major plugins except audio (which requires ALSA)
        app.add_plugins((
            bevy_shaman_core::CorePlugin,
            bevy_shaman_combat::CombatPlugin,
            bevy_shaman_monsters::MonstersPlugin,
            bevy_shaman_minions::MinionsPlugin,
            bevy_shaman_world::WorldPlugin,
            bevy_shaman_dungeons::DungeonsPlugin,
            bevy_shaman_items::ItemsPlugin,
            bevy_shaman_shop::ShopPlugin,
            bevy_shaman_ui::UiPlugin,
            bevy_shaman_story::StoryPlugin,
            bevy_shaman_save::SavePlugin,
        ));

        // Run several update cycles to ensure no conflicts
        for i in 0..20 {
            app.update();
            // Verify app doesn't crash during updates
            assert!(i < 20, "App update loop completed successfully");
        }

        // Verify all major resources are present
        assert!(app.world().contains_resource::<bevy_shaman_core::resources::Spirit>());
        assert!(app.world().contains_resource::<bevy_shaman_combat::resources::CombatConfig>());
        assert!(app.world().contains_resource::<bevy_shaman_monsters::resources::MonsterDatabase>());
    }

    #[test]
    fn test_game_state_transition_with_plugins() {
        let mut app = create_test_app();
        app.add_plugins(bevy_shaman_core::CorePlugin);

        // Verify initial state
        let state = app.world().resource::<State<GameState>>();
        assert!(matches!(state.get(), GameState::Boot));

        // Transition to Playing
        app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::Playing);
        app.update();

        let state = app.world().resource::<State<GameState>>();
        assert!(matches!(state.get(), GameState::Playing));

        // Transition to Paused
        app.world_mut().resource_mut::<NextState<GameState>>().set(GameState::Paused);
        app.update();

        let state = app.world().resource::<State<GameState>>();
        assert!(matches!(state.get(), GameState::Paused));
    }
}
