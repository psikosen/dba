pub mod systems;

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                systems::hud::update_hud,
                systems::rhythm_ui::display_rhythm_visualizer,
                systems::bestiary::display_bestiary,
            ).run_if(in_state(GameState::Playing)));
    }
}

pub mod systems {
    pub mod hud {
        use bevy::prelude::*;

        pub fn update_hud() {
            // Placeholder: update health, spirit, stamina bars
        }
    }

    pub mod rhythm_ui {
        use bevy::prelude::*;

        pub fn display_rhythm_visualizer() {
            // Placeholder: visual beat clock and timing windows
        }
    }

    pub mod bestiary {
        use bevy::prelude::*;

        pub fn display_bestiary() {
            // Placeholder: monster collection UI
        }
    }
}
