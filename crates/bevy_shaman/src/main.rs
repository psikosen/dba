use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Shaman's Journey".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        // Core systems - movement, camera, grid, animation
        .add_plugins(bevy_shaman_core::CorePlugin)
        // Combat - rhythm evaluation, hit resolution, status effects
        .add_plugins(bevy_shaman_combat::CombatPlugin)
        // Audio - beat clock, song manager, spatial SFX
        .add_plugins(bevy_shaman_audio::AudioPlugin)
        // Monsters - state machine, corruption, sprite swapping, AI
        .add_plugins(bevy_shaman_monsters::MonstersPlugin)
        // Minions - taming, formation, commands
        .add_plugins(bevy_shaman_minions::MinionsPlugin)
        // World - tiles, corruption, purification, day-night
        .add_plugins(bevy_shaman_world::WorldPlugin)
        // Dungeons - generation, room graph, encounters, bosses
        .add_plugins(bevy_shaman_dungeons::DungeonsPlugin)
        // Items - loot, inventory, spirit orbs, crafting
        .add_plugins(bevy_shaman_items::ItemsPlugin)
        // Shop - store, currency, buying/selling
        .add_plugins(bevy_shaman_shop::ShopPlugin)
        // UI - HUD, bestiary, skill tree, crafting UI
        .add_plugins(bevy_shaman_ui::UiPlugin)
        // Story - dialogue, quests, reputation, cutscenes
        .add_plugins(bevy_shaman_story::StoryPlugin)
        // Save - snapshot, migrations, autosave
        .add_plugins(bevy_shaman_save::SavePlugin)
        // AI - LLM-driven boss/NPC behavior and dialogue
        .add_plugins(bevy_shaman_ai::AiPlugin)
        .run();
}
