use bevy::prelude::*;
use crate::components::MonsterId;
use crate::resources::MonsterSpriteDB;
use crate::systems::events::MonsterStateChanged;

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
            // Swap sprite texture (in actual implementation, update Handle<Image>)
            info!(
                "Swapping sprite for {} from {:?} to {:?}",
                monster_id.0, event.old_state, event.new_state
            );
            // sprite.image = new_sprite_handle.clone();
        }
    }
}
