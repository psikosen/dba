use crate::components::{MonsterId, StateType};
use crate::resources::MonsterSpriteDB;
use crate::systems::events::MonsterStateChanged;
use bevy::prelude::*;
use bevy_shaman_core::systems::assets::MonsterSpriteHandles;

/// Marker resource to track if sprites have been registered
#[derive(Resource, Default)]
pub struct MonsterSpritesRegistered(pub bool);

/// Populate the MonsterSpriteDB from loaded assets (runs once after assets load)
pub fn populate_monster_sprite_db(
    mut sprite_db: ResMut<MonsterSpriteDB>,
    sprite_handles: Option<Res<MonsterSpriteHandles>>,
    mut registered: ResMut<MonsterSpritesRegistered>,
) {
    if registered.0 {
        return;
    }

    let Some(sprite_handles) = sprite_handles else {
        return;
    };

    info!("Populating MonsterSpriteDB with sprite handles...");

    // Register all monster sprites for each state
    // Note: Currently using same sprite for all states - can differentiate later with color tints or separate assets
    let states = [
        StateType::Stable,
        StateType::Chaos,
        StateType::Corrupt,
        StateType::Harmony,
        StateType::Decay,
        StateType::Rage,
        StateType::Void,
        StateType::Ancestral,
    ];

    // Register forest_spirit
    for state in &states {
        sprite_db.register(
            "forest_spirit".to_string(),
            *state,
            sprite_handles.forest_spirit.clone(),
        );
    }

    // Register chaos_hound
    for state in &states {
        sprite_db.register(
            "chaos_hound".to_string(),
            *state,
            sprite_handles.chaos_hound.clone(),
        );
    }

    // Register corrupt_shade
    for state in &states {
        sprite_db.register(
            "corrupt_shade".to_string(),
            *state,
            sprite_handles.corrupt_shade.clone(),
        );
    }

    // Register shadow_beast
    for state in &states {
        sprite_db.register(
            "shadow_beast".to_string(),
            *state,
            sprite_handles.shadow_beast.clone(),
        );
    }

    // Register spirit_wisp
    for state in &states {
        sprite_db.register(
            "spirit_wisp".to_string(),
            *state,
            sprite_handles.spirit_wisp.clone(),
        );
    }

    // Register rock_golem
    for state in &states {
        sprite_db.register(
            "rock_golem".to_string(),
            *state,
            sprite_handles.rock_golem.clone(),
        );
    }

    // Register flame_wraith
    for state in &states {
        sprite_db.register(
            "flame_wraith".to_string(),
            *state,
            sprite_handles.flame_wraith.clone(),
        );
    }

    // Register void_stalker
    for state in &states {
        sprite_db.register(
            "void_stalker".to_string(),
            *state,
            sprite_handles.void_stalker.clone(),
        );
    }

    registered.0 = true;
    info!(
        "MonsterSpriteDB populated with {} entries",
        sprite_db.sprites.len()
    );
}

/// Get color tint for a given state (provides visual feedback even without sprite assets)
fn get_state_color_tint(state: StateType) -> Color {
    match state {
        StateType::Stable => Color::WHITE,
        StateType::Chaos => Color::srgb(1.0, 0.6, 0.2), // Orange
        StateType::Corrupt => Color::srgb(0.6, 0.2, 0.6), // Purple
        StateType::Harmony => Color::srgb(0.4, 1.0, 0.8), // Cyan
        StateType::Decay => Color::srgb(0.5, 0.5, 0.3), // Brown
        StateType::Rage => Color::srgb(1.0, 0.2, 0.2),  // Red
        StateType::Void => Color::srgb(0.2, 0.2, 0.4),  // Dark blue
        StateType::Ancestral => Color::srgb(1.0, 1.0, 0.6), // Golden
    }
}

/// Swaps monster sprites when state changes and applies color tint
pub fn swap_sprites_on_state_change(
    mut events: EventReader<MonsterStateChanged>,
    sprite_db: Res<MonsterSpriteDB>,
    mut monsters: Query<(&MonsterId, &mut Sprite)>,
) {
    for event in events.read() {
        let Ok((monster_id, mut sprite)) = monsters.get_mut(event.entity) else {
            continue;
        };

        // Apply color tint for visual differentiation
        sprite.color = get_state_color_tint(event.new_state);

        // Swap sprite asset if available
        if let Some(new_sprite_handle) = sprite_db.get(&monster_id.0, event.new_state) {
            info!(
                "Swapping sprite for {} from {:?} to {:?}",
                monster_id.0, event.old_state, event.new_state
            );
            sprite.image = new_sprite_handle.clone();
        } else {
            // No sprite asset, but color tint provides visual feedback
            info!(
                "Applied color tint for {} state change {:?} -> {:?} (no sprite asset)",
                monster_id.0, event.old_state, event.new_state
            );
        }
    }
}
