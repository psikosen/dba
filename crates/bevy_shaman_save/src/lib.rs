pub mod systems;

use bevy::prelude::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                systems::autosave::autosave_system,
                systems::save_load::handle_save_requests,
            ))
            .add_event::<systems::events::SaveRequested>()
            .add_event::<systems::events::LoadRequested>();
    }
}

pub mod systems {
    pub mod events {
        use bevy::prelude::*;

        #[derive(Event)]
        pub struct SaveRequested;

        #[derive(Event)]
        pub struct LoadRequested {
            pub save_slot: u8,
        }
    }

    pub mod autosave {
        use bevy::prelude::*;

        pub fn autosave_system() {
            // Placeholder: periodic autosave
        }
    }

    pub mod save_load {
        use bevy::prelude::*;

        pub fn handle_save_requests() {
            // Placeholder: serialize game state to JSON
        }
    }
}
