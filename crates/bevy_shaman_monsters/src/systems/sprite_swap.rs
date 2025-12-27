use bevy::prelude::*;
use crate::components::{MonsterId, StateType};
use crate::resources::MonsterSpriteDB;
use crate::systems::events::MonsterStateChanged;
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
        sprite_db.register("forest_spirit".to_string(), *state, sprite_handles.forest_spirit.clone());
    }

    // Register chaos_hound
    for state in &states {
        sprite_db.register("chaos_hound".to_string(), *state, sprite_handles.chaos_hound.clone());
    }

    // Register corrupt_shade
    for state in &states {
        sprite_db.register("corrupt_shade".to_string(), *state, sprite_handles.corrupt_shade.clone());
    }

    // Register shadow_beast
    for state in &states {
        sprite_db.register("shadow_beast".to_string(), *state, sprite_handles.shadow_beast.clone());
    }

    // Register spirit_wisp
    for state in &states {
        sprite_db.register("spirit_wisp".to_string(), *state, sprite_handles.spirit_wisp.clone());
    }

    // Register rock_golem
    for state in &states {
        sprite_db.register("rock_golem".to_string(), *state, sprite_handles.rock_golem.clone());
    }

    // Register flame_wraith
    for state in &states {
        sprite_db.register("flame_wraith".to_string(), *state, sprite_handles.flame_wraith.clone());
    }

    // Register void_stalker
    for state in &states {
        sprite_db.register("void_stalker".to_string(), *state, sprite_handles.void_stalker.clone());
    }

    registered.0 = true;
    info!("MonsterSpriteDB populated with {} entries", sprite_db.sprites.len());
}

/// Swaps monster sprites when state changes
pub fn swap_sprites_on_state_change(
    mut events: EventReader<MonsterStateChanged>,
    sprite_db: Res<MonsterSpriteDB>,
    mut monsters: Query<(&MonsterId, &mut Sprite)>,
) {
    for event in events.read() {
        let Ok((monster_id, mut sprite)) = monsters.get_mut(event.entity) else {
            continue;
        };

        if let Some(new_sprite_handle) = sprite_db.get(&monster_id.0, event.new_state) {
            info!(
                "Swapping sprite for {} from {:?} to {:?}",
                monster_id.0, event.old_state, event.new_state
            );
            sprite.image = new_sprite_handle.clone();
        } else {
            warn!(
                "No sprite found for monster {} in state {:?}",
                monster_id.0, event.new_state
            );
        }
    }
}
