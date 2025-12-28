pub mod components;
pub mod resources;
pub mod systems;

#[cfg(test)]
mod tests;

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register resources
            .init_resource::<resources::MonsterSpriteDB>()
            .init_resource::<resources::MonsterTemplateDB>()
            .init_resource::<systems::sprite_swap::MonsterSpritesRegistered>()
            // Sprite DB population (runs every frame until complete)
            .add_systems(Update, systems::sprite_swap::populate_monster_sprite_db)
            // Monster state machine systems
            .add_systems(
                Update,
                (
                    systems::state_machine::update_monster_state_meters,
                    systems::state_machine::evaluate_state_transitions,
                    systems::sprite_swap::swap_sprites_on_state_change,
                    systems::personality::apply_music_influence,
                    systems::corruption::propagate_corruption_influence,
                    systems::ai::process_monster_ai,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Events
            .add_event::<systems::events::MonsterStateChanged>()
            .add_event::<systems::events::MonsterCorrupted>();
    }
}
