use bevy::prelude::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<systems::autosave::AutosaveTimer>()
            .init_resource::<systems::save_load::PendingLoadData>()
            .add_systems(Update, (
                systems::autosave::autosave_system,
                systems::save_load::handle_save_requests,
                systems::save_load::apply_loaded_data,
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
        use std::collections::HashMap;

        #[derive(Serialize, Deserialize, Clone)]
        pub struct TutorialProgressData {
            pub tutorial_started: bool,
            pub tutorial_completed: bool,
            pub current_mission: Option<String>,
            pub current_step: u8,
            pub completed_missions: Vec<String>,
            pub mission_flags: HashMap<String, bool>,
            pub cutscene_viewed: HashMap<String, bool>,
        }

        impl Default for TutorialProgressData {
            fn default() -> Self {
                Self {
                    tutorial_started: false,
                    tutorial_completed: false,
                    current_mission: None,
                    current_step: 0,
                    completed_missions: Vec::new(),
                    mission_flags: HashMap::new(),
                    cutscene_viewed: HashMap::new(),
                }
            }
        }

        #[derive(Serialize, Deserialize, Clone)]
        pub struct SaveData {
            pub player_position: (i32, i32),
            pub player_health: (f32, f32),
            pub player_spirit: (f32, f32),
            pub player_stamina: (f32, f32),
            pub corrupted_tiles: Vec<((i32, i32), f32)>,
            pub inventory_items: Vec<(String, String, u32)>, // (id, display_name, quantity)
            #[serde(default)]
            pub tutorial_progress: TutorialProgressData,
            pub timestamp: f64,
            pub save_version: u32,
        }

        // Resource to store loaded save data for applying in next frame
        #[derive(Resource, Default)]
        pub struct PendingLoadData {
            pub data: Option<SaveData>,
        }

        pub fn handle_save_requests(
            mut save_events: EventReader<super::events::SaveRequested>,
            mut load_events: EventReader<super::events::LoadRequested>,
            player: Query<(&GridPosition, &Health, &Spirit, &Stamina), With<Player>>,
            corrupted: Query<(&GridPosition, &TileCorruption)>,
            time: Res<Time>,
            mut pending_load: ResMut<PendingLoadData>,
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
                        inventory_items: vec![], // TODO: Extract from inventory component when available
                        tutorial_progress: TutorialProgressData::default(), // TODO: Extract from TutorialProgress resource
                        timestamp: time.elapsed_secs_f64(),
                        save_version: 1,
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

            // Handle load requests - store data in resource for next frame
            for event in load_events.read() {
                let filename = format!("saves/save_{}.json", event.save_slot);
                match fs::read_to_string(&filename) {
                    Ok(json) => {
                        match serde_json::from_str::<SaveData>(&json) {
                            Ok(save_data) => {
                                info!("Save data loaded from {}, will apply next frame", filename);
                                pending_load.data = Some(save_data);
                            }
                            Err(e) => error!("Failed to deserialize save data: {}", e),
                        }
                    }
                    Err(e) => error!("Failed to load game from {}: {}", filename, e),
                }
            }
        }

        // System to apply loaded save data to the world
        pub fn apply_loaded_data(
            mut pending_load: ResMut<PendingLoadData>,
            mut player_query: Query<(&mut GridPosition, &mut Health, &mut Spirit, &mut Stamina), With<Player>>,
            mut corrupted_query: Query<(&GridPosition, &mut TileCorruption)>,
        ) {
            if let Some(save_data) = pending_load.data.take() {
                info!("Applying loaded save data to world");

                // Restore player data
                if let Ok((mut pos, mut health, mut spirit, mut stamina)) = player_query.get_single_mut() {
                    pos.x = save_data.player_position.0;
                    pos.y = save_data.player_position.1;
                    health.current = save_data.player_health.0;
                    health.max = save_data.player_health.1;
                    spirit.current = save_data.player_spirit.0;
                    spirit.max = save_data.player_spirit.1;
                    stamina.current = save_data.player_stamina.0;
                    stamina.max = save_data.player_stamina.1;
                    info!("Player data restored to position ({}, {})", pos.x, pos.y);
                }

                // Restore corruption data
                for ((tile_x, tile_y), corruption_level) in &save_data.corrupted_tiles {
                    for (tile_pos, mut corruption) in corrupted_query.iter_mut() {
                        if tile_pos.x == *tile_x && tile_pos.y == *tile_y {
                            corruption.level = *corruption_level;
                        }
                    }
                }
                info!("Restored {} corrupted tiles", save_data.corrupted_tiles.len());
            }
        }
    }
}
