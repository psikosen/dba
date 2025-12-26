pub mod components;
pub mod systems;

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct DungeonsPlugin;

impl Plugin for DungeonsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                systems::generation::generate_dungeon_rooms,
                systems::encounters::spawn_encounters,
                systems::bosses::manage_boss_fights,
            ).run_if(in_state(GameState::Playing)))
            .add_event::<systems::events::DungeonEntered>()
            .add_event::<systems::events::BossDefeated>();
    }
}

pub mod components {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Component)]
    pub struct DungeonRoom {
        pub room_type: RoomType,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum RoomType {
        Empty,
        Encounter,
        Treasure,
        Boss,
    }

    #[derive(Component)]
    pub struct BossArena {
        pub boss_id: String,
        pub phase: u8,
    }
}

pub mod systems {
    pub mod events {
        use bevy::prelude::*;

        #[derive(Event)]
        pub struct DungeonEntered {
            pub dungeon_id: String,
        }

        #[derive(Event)]
        pub struct BossDefeated {
            pub boss_id: String,
        }
    }

    pub mod generation {
        use bevy::prelude::*;

        pub fn generate_dungeon_rooms() {
            // Placeholder: procedural room generation
        }
    }

    pub mod encounters {
        use bevy::prelude::*;

        pub fn spawn_encounters() {
            // Placeholder: spawn monsters in dungeon rooms
        }
    }

    pub mod bosses {
        use bevy::prelude::*;

        pub fn manage_boss_fights() {
            // Placeholder: boss phase transitions
        }
    }
}
