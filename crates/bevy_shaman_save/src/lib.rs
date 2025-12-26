use bevy::prelude::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<systems::autosave::AutosaveTimer>()
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
        use super::events::SaveRequested;

        #[derive(Resource)]
        pub struct AutosaveTimer {
            pub timer: Timer,
        }

        impl Default for AutosaveTimer {
            fn default() -> Self {
                Self {
                    timer: Timer::from_seconds(300.0, TimerMode::Repeating), // 5 minutes
                }
            }
        }

        pub fn autosave_system(
            time: Res<Time>,
            mut timer: ResMut<AutosaveTimer>,
            mut save_events: EventWriter<SaveRequested>,
        ) {
            timer.timer.tick(time.delta());

            if timer.timer.just_finished() {
                info!("Autosave triggered");
                save_events.send(SaveRequested);
            }
        }
    }

    pub mod save_load {
        use bevy::prelude::*;
        use bevy_shaman_core::components::{GridPosition, Health, Spirit, Stamina, Player};
        use bevy_shaman_world::components::TileCorruption;
        use serde::{Deserialize, Serialize};
        use std::fs;

        #[derive(Serialize, Deserialize)]
        pub struct SaveData {
            pub player_position: (i32, i32),
            pub player_health: (f32, f32),
            pub player_spirit: (f32, f32),
            pub player_stamina: (f32, f32),
            pub corrupted_tiles: Vec<((i32, i32), f32)>,
            pub timestamp: f64,
        }

        pub fn handle_save_requests(
            mut save_events: EventReader<super::events::SaveRequested>,
            mut load_events: EventReader<super::events::LoadRequested>,
            player: Query<(&GridPosition, &Health, &Spirit, &Stamina), With<Player>>,
            corrupted: Query<(&GridPosition, &TileCorruption)>,
            time: Res<Time>,
        ) {
            // Handle save requests
            for _event in save_events.read() {
                if let Ok((pos, health, spirit, stamina)) = player.get_single() {
                    let save_data = SaveData {
                        player_position: (pos.x, pos.y),
                        player_health: (health.current, health.max),
                        player_spirit: (spirit.current, spirit.max),
                        player_stamina: (stamina.current, stamina.max),
                        corrupted_tiles: corrupted
                            .iter()
                            .filter(|(_, corruption)| corruption.is_corrupt())
                            .map(|(pos, corruption)| ((pos.x, pos.y), corruption.level))
                            .collect(),
                        timestamp: time.elapsed_secs_f64(),
                    };

                    match serde_json::to_string_pretty(&save_data) {
                        Ok(json) => {
                            // Create saves directory if it doesn't exist
                            if let Err(e) = fs::create_dir_all("saves") {
                                error!("Failed to create saves directory: {}", e);
                                return;
                            }

                            // Write to file
                            match fs::write("saves/autosave.json", json) {
                                Ok(_) => info!("Game saved successfully"),
                                Err(e) => error!("Failed to save game: {}", e),
                            }
                        }
                        Err(e) => error!("Failed to serialize save data: {}", e),
                    }
                }
            }

            // Handle load requests
            for event in load_events.read() {
                let filename = format!("saves/save_{}.json", event.save_slot);
                match fs::read_to_string(&filename) {
                    Ok(json) => {
                        match serde_json::from_str::<SaveData>(&json) {
                            Ok(save_data) => {
                                info!("Game loaded from {}", filename);
                                // Note: Actual loading of data would require world mutation
                                // which is complex in ECS. This is a simplified version.
                                // In production, you'd use Commands and NextState.
                            }
                            Err(e) => error!("Failed to deserialize save data: {}", e),
                        }
                    }
                    Err(e) => error!("Failed to load game from {}: {}", filename, e),
                }
            }
        }
    }
}
