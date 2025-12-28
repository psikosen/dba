use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub mod ancestral_hud;
pub mod ancestral_inventory;
pub mod ancestral_quest_tracker;
pub mod ancestral_theme;
pub mod enhancement_ui;
pub mod prime_vessel_hud;
pub mod skill_tree_ui;

#[cfg(test)]
mod tests;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Ancestral UI resources
            .init_resource::<ancestral_hud::AncestralHudState>()
            .init_resource::<ancestral_inventory::AncestralInventoryState>()
            .init_resource::<ancestral_quest_tracker::QuestLogUIState>()
            .init_resource::<enhancement_ui::EnhancementUIState>()
            .init_resource::<skill_tree_ui::SkillTreeUIState>()
            // Legacy UI resources
            .init_resource::<systems::bestiary::BestiaryVisible>()
            .init_resource::<systems::loading_screen::VideoIntroTimer>()
            .init_resource::<systems::shop_ui::ShopVisible>()
            .init_resource::<systems::dialogue_ui::DialogueUIState>()
            .init_resource::<systems::minimap::MinimapState>()
            .init_resource::<systems::inventory_ui::InventoryUIState>()
            .init_resource::<systems::quest_ui::QuestUIState>()
            .init_resource::<systems::quick_wins::PauseMenuState>()
            .init_resource::<systems::quick_wins::DeathScreenState>()
            .init_resource::<systems::quick_wins::SettingsUIState>()
            .init_resource::<systems::combat_feedback::ScreenShake>()
            // Ancestral HUD systems (run when entering Playing state)
            .add_systems(
                OnEnter(GameState::Playing),
                (
                    ancestral_hud::setup_ancestral_hud,
                    prime_vessel_hud::spawn_corruption_index_widget,
                    prime_vessel_hud::spawn_danger_level_widget,
                ),
            )
            // Ancestral UI update systems
            .add_systems(
                Update,
                (
                    ancestral_hud::update_vitality_bars,
                    ancestral_hud::animate_health_bar,
                    ancestral_hud::animate_spirit_bar,
                    ancestral_hud::handle_dev_mode_toggle,
                    ancestral_hud::update_dev_mode_panel,
                    ancestral_hud::handle_spawn_brother_button,
                    ancestral_hud::handle_spawn_boss_button,
                    ancestral_quest_tracker::update_quest_tracker,
                    ancestral_quest_tracker::display_quest_log_full,
                    ancestral_inventory::display_ancestral_inventory,
                    ancestral_inventory::handle_tab_clicks,
                    ancestral_inventory::handle_compartment_hover,
                    ancestral_inventory::handle_compartment_clicks,
                    ancestral_inventory::display_item_tooltip,
                    ancestral_inventory::display_context_menu,
                    ancestral_inventory::handle_context_menu_clicks,
                    ancestral_inventory::close_context_menu_on_click,
                    enhancement_ui::toggle_enhancement_ui,
                    enhancement_ui::display_enhancement_ui,
                    skill_tree_ui::toggle_skill_tree_ui,
                    skill_tree_ui::display_skill_tree_ui,
                    skill_tree_ui::handle_skill_unlock_clicks,
                    prime_vessel_hud::update_corruption_index_widget,
                    prime_vessel_hud::update_danger_level_widget,
                    prime_vessel_hud::spawn_vessel_encounter_notification,
                    prime_vessel_hud::update_vessel_encounter_notifications,
                    prime_vessel_hud::spawn_resurrection_ritual_widget,
                    prime_vessel_hud::update_resurrection_ritual_widget,
                    prime_vessel_hud::handle_corruption_index_revealed,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Legacy UI systems
            .add_systems(
                Update,
                (
                    systems::hud::update_hud,
                    systems::rhythm_ui::display_rhythm_visualizer,
                    systems::bestiary::display_bestiary,
                    systems::shop_ui::display_shop,
                    systems::inventory_ui::display_inventory,
                    systems::inventory_ui::handle_inventory_interactions,
                    systems::inventory_ui::display_inventory_tooltip,
                    systems::inventory_ui::display_context_menu,
                    systems::minimap::update_minimap,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    systems::dialogue_ui::handle_dialogue_events,
                    systems::dialogue_ui::update_dialogue_ui,
                    systems::dialogue_ui::update_typewriter_text,
                    systems::dialogue_ui::update_dialogue_tree_ui,
                    systems::dialogue_ui::handle_choice_buttons,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    systems::combat_feedback::spawn_damage_numbers,
                    systems::combat_feedback::update_damage_numbers,
                    systems::combat_feedback::spawn_hit_effects,
                    systems::combat_feedback::update_hit_effects,
                    systems::combat_feedback::apply_screen_shake,
                    systems::taming_ui::display_taming_progress,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    systems::quest_ui::display_quest_log,
                    systems::quest_ui::display_quest_tracker,
                    systems::quick_wins::display_pause_menu,
                    systems::quick_wins::display_death_screen,
                    systems::quick_wins::display_combo_counter,
                    systems::quick_wins::display_settings_panel,
                    systems::quick_wins::handle_settings_interactions,
                    systems::calendar_ui::display_calendar,
                    systems::calendar_ui::display_calendar_button,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                systems::loading_screen::display_loading_screen.run_if(in_state(GameState::Boot)),
            )
            .add_systems(
                Update,
                (
                    systems::main_menu::display_main_menu,
                    systems::quick_wins::display_settings_panel,
                    systems::quick_wins::handle_settings_interactions,
                )
                    .run_if(in_state(GameState::MainMenu)),
            );
    }
}

pub mod systems {
    pub mod hud {
        use bevy::prelude::*;
        use bevy_shaman_core::components::{Health, Player, Spirit, Stamina};

        #[derive(Component)]
        pub struct HealthBar;

        #[derive(Component)]
        pub struct HealthText;

        #[derive(Component)]
        pub struct SpiritBar;

        #[derive(Component)]
        pub struct SpiritText;

        #[derive(Component)]
        pub struct StaminaBar;

        #[derive(Component)]
        pub struct StaminaText;

        #[derive(Component)]
        pub struct HudRoot;

        #[derive(Component)]
        pub struct FpsCounter;

        #[derive(Resource)]
        pub struct FpsTracker {
            pub frame_times: Vec<f32>,
            pub last_update: f32,
        }

        impl Default for FpsTracker {
            fn default() -> Self {
                Self {
                    frame_times: Vec::with_capacity(60),
                    last_update: 0.0,
                }
            }
        }

        pub fn update_hud(
            mut commands: Commands,
            player: Query<(&Health, &Spirit, &Stamina), With<Player>>,
            hud_root: Query<Entity, With<HudRoot>>,
            health_bars: Query<Entity, With<HealthBar>>,
            spirit_bars: Query<Entity, With<SpiritBar>>,
            stamina_bars: Query<Entity, With<StaminaBar>>,
            mut health_text: Query<&mut Text, With<HealthText>>,
            mut spirit_text: Query<
                &mut Text,
                (With<SpiritText>, Without<HealthText>, Without<StaminaText>),
            >,
            mut stamina_text: Query<
                &mut Text,
                (With<StaminaText>, Without<HealthText>, Without<SpiritText>),
            >,
            mut fps_text: Query<
                &mut Text,
                (
                    With<FpsCounter>,
                    Without<HealthText>,
                    Without<SpiritText>,
                    Without<StaminaText>,
                ),
            >,
            mut fps_tracker: Local<FpsTracker>,
            time: Res<Time>,
        ) {
            // Initialize HUD if it doesn't exist
            if hud_root.is_empty() {
                commands
                    .spawn((
                        HudRoot,
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(10.0),
                            top: Val::Px(10.0),
                            width: Val::Px(300.0),
                            height: Val::Px(100.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(5.0),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        // Health bar
                        parent
                            .spawn((
                                HealthBar,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(25.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                            ))
                            .with_children(|bar| {
                                bar.spawn((
                                    HealthText,
                                    Text::new("Health: 100 / 100"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });

                        // Spirit bar
                        parent
                            .spawn((
                                SpiritBar,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(25.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.2, 0.5, 0.9)),
                            ))
                            .with_children(|bar| {
                                bar.spawn((
                                    SpiritText,
                                    Text::new("Spirit: 100 / 100"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });

                        // Stamina bar
                        parent
                            .spawn((
                                StaminaBar,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(25.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.3, 0.7, 0.3)),
                            ))
                            .with_children(|bar| {
                                bar.spawn((
                                    StaminaText,
                                    Text::new("Stamina: 100 / 100"),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });

                        // FPS Counter
                        parent
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(20.0),
                                    margin: UiRect::top(Val::Px(10.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.7)),
                            ))
                            .with_children(|fps_container| {
                                fps_container.spawn((
                                    FpsCounter,
                                    Text::new("FPS: 60"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.5, 1.0, 0.5)),
                                ));
                            });
                    });
            }

            // Update bar values and text
            if let Ok((health, spirit, stamina)) = player.get_single() {
                // Update health bar width
                for bar_entity in health_bars.iter() {
                    let percent = (health.current / health.max) * 100.0;
                    commands.entity(bar_entity).insert(Node {
                        width: Val::Percent(percent),
                        height: Val::Px(25.0),
                        ..default()
                    });
                }

                // Update health text
                for mut text in health_text.iter_mut() {
                    **text = format!("Health: {:.0} / {:.0}", health.current, health.max);
                }

                // Update spirit bar width
                for bar_entity in spirit_bars.iter() {
                    let percent = (spirit.current / spirit.max) * 100.0;
                    commands.entity(bar_entity).insert(Node {
                        width: Val::Percent(percent),
                        height: Val::Px(25.0),
                        ..default()
                    });
                }

                // Update spirit text
                for mut text in spirit_text.iter_mut() {
                    **text = format!("Spirit: {:.0} / {:.0}", spirit.current, spirit.max);
                }

                // Update stamina bar width
                for bar_entity in stamina_bars.iter() {
                    let percent = (stamina.current / stamina.max) * 100.0;
                    commands.entity(bar_entity).insert(Node {
                        width: Val::Percent(percent),
                        height: Val::Px(25.0),
                        ..default()
                    });
                }

                // Update stamina text
                for mut text in stamina_text.iter_mut() {
                    **text = format!("Stamina: {:.0} / {:.0}", stamina.current, stamina.max);
                }
            }

            // Update FPS counter
            let delta = time.delta_secs();
            if delta > 0.0 {
                fps_tracker.frame_times.push(delta);
                if fps_tracker.frame_times.len() > 60 {
                    fps_tracker.frame_times.remove(0);
                }
            }

            // Update FPS text every 0.5 seconds
            if time.elapsed_secs() - fps_tracker.last_update > 0.5 {
                fps_tracker.last_update = time.elapsed_secs();

                if !fps_tracker.frame_times.is_empty() {
                    let avg_frame_time: f32 = fps_tracker.frame_times.iter().sum::<f32>()
                        / fps_tracker.frame_times.len() as f32;
                    let fps = if avg_frame_time > 0.0 {
                        1.0 / avg_frame_time
                    } else {
                        60.0
                    };

                    for mut text in fps_text.iter_mut() {
                        **text = format!("FPS: {:.0}", fps);
                    }
                }
            }
        }
    }

    pub mod rhythm_ui {
        use bevy::prelude::*;
        #[cfg(feature = "audio")]
        use bevy_shaman_audio::resources::BeatClock;

        #[derive(Component)]
        pub struct RhythmVisualizer;

        #[derive(Component)]
        pub struct BeatIndicator;

        #[derive(Component)]
        pub struct TimingWindow;

        pub fn display_rhythm_visualizer(
            mut commands: Commands,
            #[cfg(feature = "audio")] rhythm: Option<Res<BeatClock>>,
            visualizer: Query<Entity, With<RhythmVisualizer>>,
            #[cfg_attr(not(feature = "audio"), allow(unused_variables))] mut beat_indicators: Query<
                &mut BackgroundColor,
                With<BeatIndicator>,
            >,
            #[cfg_attr(not(feature = "audio"), allow(unused_variables))] time: Res<Time>,
        ) {
            // Initialize visualizer if it doesn't exist
            if visualizer.is_empty() {
                commands
                    .spawn((
                        RhythmVisualizer,
                        Node {
                            position_type: PositionType::Absolute,
                            right: Val::Px(10.0),
                            top: Val::Px(10.0),
                            width: Val::Px(200.0),
                            height: Val::Px(200.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.7)),
                    ))
                    .with_children(|parent| {
                        // Beat clock circle
                        parent.spawn((
                            BeatIndicator,
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                            BorderRadius::all(Val::Px(50.0)),
                        ));

                        // Timing windows display
                        parent.spawn((
                            TimingWindow,
                            Text::new("Perfect | Good | Miss"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }

            // Update beat indicator color based on rhythm clock
            #[cfg(feature = "audio")]
            if let Some(rhythm_clock) = rhythm {
                for mut bg_color in beat_indicators.iter_mut() {
                    // Calculate beat phase (0.0 to 1.0)
                    let beat_phase = (time.elapsed_secs() * rhythm_clock.bpm / 60.0) % 1.0;

                    // Pulse the indicator on beat
                    let intensity = if beat_phase < 0.1 {
                        1.0 - (beat_phase / 0.1)
                    } else {
                        0.3
                    };

                    *bg_color = BackgroundColor(Color::srgb(
                        0.3 + intensity * 0.7,
                        0.3 + intensity * 0.5,
                        0.3,
                    ));
                }
            }
        }
    }

    pub mod bestiary {
        use bevy::prelude::*;
        use bevy_shaman_monsters::components::{MonsterId, Tamed};

        #[derive(Component)]
        pub struct BestiaryUI;

        #[derive(Resource, Default)]
        pub struct BestiaryVisible(pub bool);

        pub fn display_bestiary(
            mut commands: Commands,
            keyboard: Res<ButtonInput<KeyCode>>,
            mut visible: ResMut<BestiaryVisible>,
            bestiary_ui: Query<Entity, With<BestiaryUI>>,
            tamed_monsters: Query<&MonsterId, With<Tamed>>,
        ) {
            // Toggle bestiary with 'B' key
            if keyboard.just_pressed(KeyCode::KeyB) {
                visible.0 = !visible.0;
            }

            // Show/hide bestiary
            if visible.0 {
                if bestiary_ui.is_empty() {
                    // Count tamed monsters by type
                    let mut tamed_counts: std::collections::HashMap<String, usize> =
                        std::collections::HashMap::new();
                    for monster_id in tamed_monsters.iter() {
                        *tamed_counts.entry(monster_id.0.clone()).or_insert(0) += 1;
                    }

                    commands
                        .spawn((
                            BestiaryUI,
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(25.0),
                                top: Val::Percent(25.0),
                                width: Val::Percent(50.0),
                                height: Val::Percent(50.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(20.0)),
                                row_gap: Val::Px(10.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.1, 0.1, 0.2, 0.9)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("=== BESTIARY ===\nPress B to close\n"),
                                TextFont {
                                    font_size: 20.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));

                            if tamed_counts.is_empty() {
                                parent.spawn((
                                    Text::new("No monsters tamed yet."),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                ));
                            } else {
                                for (monster_type, count) in tamed_counts.iter() {
                                    parent.spawn((
                                        Text::new(format!("{}: {} tamed", monster_type, count)),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                }
                            }
                        });
                }
            } else {
                // Remove bestiary UI when hidden
                for entity in bestiary_ui.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }

    pub mod loading_screen {
        use bevy::prelude::*;
        use bevy_shaman_core::states::GameState;

        #[derive(Component)]
        pub struct LoadingScreenUI;

        #[derive(Component)]
        pub struct VideoIntroImage;

        #[derive(Resource)]
        pub struct VideoIntroTimer {
            pub timer: Timer,
            pub video_loaded: bool,
        }

        impl Default for VideoIntroTimer {
            fn default() -> Self {
                Self {
                    timer: Timer::from_seconds(5.0, TimerMode::Once),
                    video_loaded: false,
                }
            }
        }

        pub fn display_loading_screen(
            mut commands: Commands,
            mut timer: ResMut<VideoIntroTimer>,
            time: Res<Time>,
            loading_ui: Query<Entity, With<LoadingScreenUI>>,
            asset_server: Res<AssetServer>,
            mut next_state: ResMut<NextState<GameState>>,
        ) {
            // Initialize loading screen if it doesn't exist
            if loading_ui.is_empty() {
                commands
                    .spawn((
                        LoadingScreenUI,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                    ))
                    .with_children(|parent| {
                        // Try to load video intro image (user will place at assets/intro_video.png)
                        // For now, show a placeholder
                        parent.spawn((
                            VideoIntroImage,
                            ImageNode {
                                image: asset_server.load("intro_video.png"),
                                ..default()
                            },
                            Node {
                                width: Val::Px(800.0),
                                height: Val::Px(600.0),
                                margin: UiRect::all(Val::Px(20.0)),
                                ..default()
                            },
                        ));

                        // Gritty text overlay
                        parent.spawn((
                            Text::new("SHAMAN"),
                            TextFont {
                                font_size: 80.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.2, 0.2)),
                            Node {
                                position_type: PositionType::Absolute,
                                top: Val::Percent(20.0),
                                ..default()
                            },
                        ));

                        parent.spawn((
                            Text::new("Loading..."),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                            Node {
                                position_type: PositionType::Absolute,
                                bottom: Val::Percent(10.0),
                                ..default()
                            },
                        ));
                    });
            }

            // Tick timer
            timer.timer.tick(time.delta());

            // Transition to main menu after timer expires
            if timer.timer.finished() {
                // Clean up loading screen
                for entity in loading_ui.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                next_state.set(GameState::MainMenu);
            }
        }
    }

    pub mod main_menu {
        use bevy::prelude::*;
        use bevy_shaman_core::states::GameState;

        #[derive(Component)]
        pub struct MainMenuUI;

        #[derive(Component)]
        pub struct NewGameButton;

        #[derive(Component)]
        pub struct LoadGameButton;

        #[derive(Component)]
        pub struct SettingsButton;

        pub fn display_main_menu(
            mut commands: Commands,
            menu_ui: Query<Entity, With<MainMenuUI>>,
            interaction_query: Query<
                (&Interaction, &NewGameButton),
                (Changed<Interaction>, With<Button>),
            >,
            load_interaction: Query<
                (&Interaction, &LoadGameButton),
                (Changed<Interaction>, With<Button>),
            >,
            settings_interaction: Query<
                (&Interaction, &SettingsButton),
                (Changed<Interaction>, With<Button>),
            >,
            mut next_state: ResMut<NextState<GameState>>,
            mut load_events: EventWriter<bevy_shaman_save::systems::events::LoadRequested>,
            mut settings_state: ResMut<super::quick_wins::SettingsUIState>,
        ) {
            // Initialize main menu if it doesn't exist
            if menu_ui.is_empty() {
                commands
                    .spawn((
                        MainMenuUI,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            row_gap: Val::Px(20.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
                    ))
                    .with_children(|parent| {
                        // Title
                        parent.spawn((
                            Text::new("SHAMAN"),
                            TextFont {
                                font_size: 80.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.2, 0.2)),
                            Node {
                                margin: UiRect::bottom(Val::Px(50.0)),
                                ..default()
                            },
                        ));

                        // New Game button
                        parent
                            .spawn((
                                NewGameButton,
                                Button,
                                Node {
                                    width: Val::Px(300.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                                BorderColor(Color::srgb(0.6, 0.2, 0.2)),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new("NEW GAME"),
                                    TextFont {
                                        font_size: 28.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });

                        // Load Game button
                        parent
                            .spawn((
                                LoadGameButton,
                                Button,
                                Node {
                                    width: Val::Px(300.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                                BorderColor(Color::srgb(0.6, 0.2, 0.2)),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new("LOAD GAME"),
                                    TextFont {
                                        font_size: 28.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });

                        // Settings button
                        parent
                            .spawn((
                                SettingsButton,
                                Button,
                                Node {
                                    width: Val::Px(300.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                                BorderColor(Color::srgb(0.6, 0.2, 0.2)),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new("SETTINGS"),
                                    TextFont {
                                        font_size: 28.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                    });
            }

            // Handle New Game button
            for (interaction, _) in interaction_query.iter() {
                if *interaction == Interaction::Pressed {
                    // Clean up menu
                    for entity in menu_ui.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    next_state.set(GameState::Playing);
                }
            }

            // Handle Load Game button
            for (interaction, _) in load_interaction.iter() {
                if *interaction == Interaction::Pressed {
                    // Check if save file exists
                    if std::path::Path::new("saves/autosave.json").exists() {
                        info!("Loading game from autosave (slot 0)");
                        // Send LoadRequested event for autosave (slot 0)
                        load_events.send(bevy_shaman_save::systems::events::LoadRequested {
                            save_slot: 0,
                        });
                    } else {
                        warn!("No save file found, starting new game instead");
                    }

                    // Clean up menu and transition to Playing
                    for entity in menu_ui.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    next_state.set(GameState::Playing);
                }
            }

            // Handle Settings button (main menu)
            for (interaction, _) in settings_interaction.iter() {
                if *interaction == Interaction::Pressed {
                    info!("Opening settings panel");
                    settings_state.visible = true;
                }
            }
        }
    }

    pub mod shop_ui {
        use bevy::prelude::*;

        #[derive(Component)]
        pub struct ShopUI;

        #[derive(Component)]
        pub struct ShopItemButton {
            pub item_id: String,
        }

        #[derive(Resource, Default)]
        pub struct ShopVisible(pub bool);

        pub fn display_shop(
            mut commands: Commands,
            keyboard: Res<ButtonInput<KeyCode>>,
            mut visible: ResMut<ShopVisible>,
            shop_ui: Query<Entity, With<ShopUI>>,
            shop: Option<Res<bevy_shaman_shop::resources::ShopInventory>>,
            currency: Option<Res<bevy_shaman_shop::resources::Currency>>,
        ) {
            // Toggle shop with 'S' key
            if keyboard.just_pressed(KeyCode::KeyS) {
                visible.0 = !visible.0;
            }

            // Show/hide shop
            if visible.0 {
                if shop_ui.is_empty() {
                    commands.spawn((
                        ShopUI,
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(15.0),
                            top: Val::Percent(10.0),
                            width: Val::Percent(70.0),
                            height: Val::Percent(80.0),
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(20.0)),
                            row_gap: Val::Px(10.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
                    )).with_children(|parent| {
                        // Title and gold display
                        let gold_text = if let Some(curr) = currency.as_ref() {
                            format!("=== SHOP === | Gold: {}", curr.gold)
                        } else {
                            "=== SHOP === | Gold: 0".to_string()
                        };

                        parent.spawn((
                            Text::new(format!("{}\nPress S to close\n", gold_text)),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.7, 0.2)),
                        ));

                        // Shop items
                        if let Some(shop_data) = shop.as_ref() {
                            if shop_data.items.is_empty() {
                                parent.spawn((
                                    Text::new("Shop is empty."),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                ));
                            } else {
                                // Create scrollable list
                                parent.spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(90.0),
                                        flex_direction: FlexDirection::Column,
                                        row_gap: Val::Px(5.0),
                                        overflow: Overflow::clip_y(),
                                        ..default()
                                    },
                                )).with_children(|scroll_parent| {
                                    for shop_item in shop_data.items.iter() {
                                        scroll_parent.spawn((
                                            Node {
                                                width: Val::Percent(100.0),
                                                padding: UiRect::all(Val::Px(10.0)),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                                        )).with_children(|item_parent| {
                                            item_parent.spawn((
                                                Text::new(format!(
                                                    "{} - {} gold (Stock: {})",
                                                    shop_item.item.display_name,
                                                    shop_item.price,
                                                    shop_item.stock
                                                )),
                                                TextFont {
                                                    font_size: 14.0,
                                                    ..default()
                                                },
                                                TextColor(Color::WHITE),
                                            ));
                                        });
                                    }
                                });
                            }
                        } else {
                            parent.spawn((
                                Text::new("Shop system not initialized."),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.3, 0.3)),
                            ));
                        }

                        // Instructions
                        parent.spawn((
                            Text::new("\nNote: Shop transactions require currency system.\nUse events to buy/sell items."),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.6, 0.6, 0.6)),
                            Node {
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            },
                        ));
                    });
                }
            } else {
                // Remove shop UI when hidden
                for entity in shop_ui.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }

    // ============================================================================
    // CROSSCODE-STYLE DIALOGUE UI
    // ============================================================================
    pub mod dialogue_ui {
        use bevy::prelude::*;
        use bevy_shaman_core::events::DialogueRequested;
        use bevy_shaman_story::components::{NpcDialogue, NpcName, NpcSicknessState};
        use bevy_shaman_story::resources::{PortraitDB, PortraitEmotion};

        #[derive(Component)]
        pub struct DialogueUIRoot;

        #[derive(Component)]
        pub struct DialoguePortraitLeft;

        #[derive(Component)]
        pub struct DialoguePortraitRight;

        #[derive(Component)]
        pub struct DialogueBox;

        #[derive(Component)]
        pub struct DialogueSpeakerName;

        #[derive(Component)]
        pub struct DialogueText;

        #[derive(Component)]
        pub struct DialogueSpeakerHandle;

        #[derive(Component)]
        pub struct CinematicGradient;

        #[derive(Component)]
        pub struct BarkText {
            pub lifetime: Timer,
        }

        #[derive(Component)]
        pub struct TypewriterText {
            pub full_text: String,
            pub current_index: usize,
            pub timer: Timer,
            pub chars_per_second: f32,
        }

        impl TypewriterText {
            pub fn new(text: String, chars_per_second: f32) -> Self {
                Self {
                    full_text: text,
                    current_index: 0,
                    timer: Timer::from_seconds(1.0 / chars_per_second, TimerMode::Repeating),
                    chars_per_second,
                }
            }

            pub fn is_complete(&self) -> bool {
                self.current_index >= self.full_text.len()
            }

            pub fn skip_to_end(&mut self) {
                self.current_index = self.full_text.len();
            }
        }

        #[derive(Resource)]
        pub struct DialogueUIState {
            pub active: bool,
            pub npc_entity: Option<Entity>,
            pub npc_name: String,
            pub dialogue_text: String,
            pub npc_portrait_side: PortraitSide,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum PortraitSide {
            Left,
            Right,
        }

        impl Default for DialogueUIState {
            fn default() -> Self {
                Self {
                    active: false,
                    npc_entity: None,
                    npc_name: String::new(),
                    dialogue_text: String::new(),
                    npc_portrait_side: PortraitSide::Left,
                }
            }
        }

        pub fn handle_dialogue_events(
            mut dialogue_events: EventReader<DialogueRequested>,
            mut dialogue_state: ResMut<DialogueUIState>,
            npc_query: Query<(&NpcDialogue, &NpcSicknessState, &NpcName)>,
            keyboard: Res<ButtonInput<KeyCode>>,
        ) {
            // Close dialogue on Escape
            if keyboard.just_pressed(KeyCode::Escape) && dialogue_state.active {
                dialogue_state.active = false;
                dialogue_state.npc_entity = None;
                return;
            }

            // Handle new dialogue requests
            for event in dialogue_events.read() {
                if let Ok((dialogue, sickness_state, npc_name)) = npc_query.get(event.npc_entity) {
                    dialogue_state.active = true;
                    dialogue_state.npc_entity = Some(event.npc_entity);
                    dialogue_state.npc_name = npc_name.name.clone();
                    dialogue_state.dialogue_text =
                        dialogue.get_dialogue(*sickness_state).to_string();
                    // Alternate portrait sides for variety
                    dialogue_state.npc_portrait_side = PortraitSide::Left;
                }
            }
        }

        pub fn update_dialogue_ui(
            mut commands: Commands,
            dialogue_state: Res<DialogueUIState>,
            portrait_db: Res<PortraitDB>,
            ui_root_query: Query<Entity, With<DialogueUIRoot>>,
            npc_query: Query<&NpcName>,
            asset_server: Res<AssetServer>,
        ) {
            // Clean up existing UI if dialogue is closed
            if !dialogue_state.active {
                for entity in ui_root_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Only spawn UI once
            if !ui_root_query.is_empty() {
                return;
            }

            // Get NPC info
            let npc_emotion = if let Some(npc_entity) = dialogue_state.npc_entity {
                if let Ok(npc_name) = npc_query.get(npc_entity) {
                    npc_name.current_emotion
                } else {
                    PortraitEmotion::Neutral
                }
            } else {
                PortraitEmotion::Neutral
            };

            // Spawn dialogue UI
            commands
                .spawn((
                    DialogueUIRoot,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // ====== CINEMATIC GRADIENT (Bottom fade) ======
                    parent.spawn((
                        CinematicGradient,
                        Node {
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            width: Val::Percent(100.0),
                            height: Val::Percent(30.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                    ));

                    // ====== NPC PORTRAIT (Left or Right) ======
                    let portrait_handle =
                        portrait_db.get_portrait(&dialogue_state.npc_name, npc_emotion);

                    // Left portrait (NPC speaking)
                    if dialogue_state.npc_portrait_side == PortraitSide::Left {
                        parent.spawn((
                            DialoguePortraitLeft,
                            ImageNode {
                                image: portrait_handle.cloned().unwrap_or_else(|| {
                                    // Placeholder colored square if no portrait exists
                                    asset_server.load("portraits/npcs/placeholder.png")
                                }),
                                ..default()
                            },
                            Node {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(0.0),
                                left: Val::Px(0.0),
                                width: Val::Px(512.0),
                                height: Val::Px(512.0),
                                ..default()
                            },
                        ));
                    }

                    // ====== DIALOGUE BOX (Floating, anchored to speaker) ======
                    let box_left = match dialogue_state.npc_portrait_side {
                        PortraitSide::Left => Val::Px(480.0), // Offset from left portrait
                        PortraitSide::Right => Val::Px(50.0), // Offset from left edge if portrait on right
                    };

                    parent
                        .spawn((
                            DialogueBox,
                            Node {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(120.0),
                                left: box_left,
                                width: Val::Px(700.0),
                                height: Val::Auto,
                                padding: UiRect::all(Val::Px(20.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(10.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.15, 0.15, 0.2, 0.85)), // Dark grey, semi-transparent
                            BorderColor(Color::srgba(0.4, 0.6, 0.8, 1.0)), // Sci-fi blue border
                        ))
                        .with_children(|box_parent| {
                            // Speaker handle (visual bracket on the side)
                            box_parent.spawn((
                                DialogueSpeakerHandle,
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(-10.0),
                                    top: Val::Px(20.0),
                                    width: Val::Px(5.0),
                                    height: Val::Px(60.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.5, 0.7, 0.9, 1.0)), // Light blue handle
                            ));

                            // NPC Name
                            box_parent.spawn((
                                DialogueSpeakerName,
                                Text::new(&dialogue_state.npc_name),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                            ));

                            // Dialogue text with typewriter effect
                            box_parent.spawn((
                                DialogueText,
                                TypewriterText::new(dialogue_state.dialogue_text.clone(), 30.0), // 30 chars per second
                                Text::new(""), // Start with empty text
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Node {
                                    max_width: Val::Px(660.0),
                                    ..default()
                                },
                            ));

                            // Continue prompt
                            box_parent.spawn((
                                Text::new("Press ESC to close"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgba(0.7, 0.7, 0.7, 0.8)),
                            ));
                        });
                });
        }

        // System to spawn bark text (reactions like "[nods]")
        pub fn spawn_bark_text(
            mut commands: Commands,
            _character_name: &str,
            bark_message: &str,
            side: PortraitSide,
        ) {
            let position_left = match side {
                PortraitSide::Left => Val::Px(420.0),
                PortraitSide::Right => Val::Percent(75.0),
            };

            commands.spawn((
                BarkText {
                    lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                },
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(480.0),
                    left: position_left,
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
                Text::new(bark_message),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        }

        // System to clean up bark text after lifetime
        pub fn update_bark_text(
            mut commands: Commands,
            time: Res<Time>,
            mut bark_query: Query<(Entity, &mut BarkText)>,
        ) {
            for (entity, mut bark) in bark_query.iter_mut() {
                bark.lifetime.tick(time.delta());
                if bark.lifetime.finished() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }

        // System to update typewriter text effect
        pub fn update_typewriter_text(
            time: Res<Time>,
            keyboard: Res<ButtonInput<KeyCode>>,
            mut typewriter_query: Query<(&mut TypewriterText, &mut Text)>,
            #[cfg(feature = "audio")] mut sfx_events: EventWriter<
                bevy_shaman_audio::systems::audio_playback::PlaySoundEffect,
            >,
        ) {
            for (mut typewriter, mut text) in typewriter_query.iter_mut() {
                // Skip to end if Space or Enter is pressed
                if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
                    typewriter.skip_to_end();
                }

                // Update timer
                typewriter.timer.tick(time.delta());

                // Reveal characters
                if typewriter.timer.just_finished() && !typewriter.is_complete() {
                    typewriter.current_index += 1;
                    // Play typewriter sound effect every few characters to avoid spam
                    #[cfg(feature = "audio")]
                    if typewriter.current_index % 3 == 0 {
                        sfx_events.send(bevy_shaman_audio::systems::audio_playback::PlaySoundEffect::TypewriterBeep);
                    }
                }

                // Update displayed text
                let displayed_text: String = typewriter
                    .full_text
                    .chars()
                    .take(typewriter.current_index)
                    .collect();
                **text = displayed_text;
            }
        }

        // ============================================================================
        // DIALOGUE TREE UI (Multiple Choice System)
        // ============================================================================

        use bevy_shaman_story::systems::dialogue_tree::{
            ActiveDialogueState, DialogueChoiceSelected, DialogueFlags, DialogueReputation,
            DialogueTreeEnded, DialogueTreeRegistry,
        };

        #[derive(Component)]
        pub struct DialogueTreeUIRoot;

        #[derive(Component)]
        pub struct DialogueChoiceButton {
            pub choice_index: usize,
            pub next_node_id: String,
        }

        #[derive(Component)]
        pub struct DialogueChoicesContainer;

        /// Update dialogue tree UI (multiple choice system)
        pub fn update_dialogue_tree_ui(
            mut commands: Commands,
            dialogue_state: Res<ActiveDialogueState>,
            registry: Res<DialogueTreeRegistry>,
            flags: Res<DialogueFlags>,
            reputation: Res<DialogueReputation>,
            ui_root_query: Query<Entity, With<DialogueTreeUIRoot>>,
            npc_query: Query<&bevy_shaman_story::components::NpcName>,
        ) {
            // Clean up if no active dialogue
            if !dialogue_state.active {
                for entity in ui_root_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Get current tree and node
            let Some(tree_id) = &dialogue_state.tree_id else {
                return;
            };
            let Some(tree) = registry.get(tree_id) else {
                return;
            };

            // Get current node (or start if none)
            let node_id = dialogue_state
                .current_node_id
                .as_deref()
                .unwrap_or(&tree.starting_node_id);
            let Some(node) = tree.get_node(node_id) else {
                return;
            };

            // Only spawn UI once per node
            if !ui_root_query.is_empty() {
                return;
            }

            // Get NPC name
            let npc_name = if let Some(npc_entity) = dialogue_state.npc_entity {
                npc_query
                    .get(npc_entity)
                    .map(|n| n.name.clone())
                    .unwrap_or_else(|_| node.speaker.clone())
            } else {
                node.speaker.clone()
            };

            // Spawn dialogue tree UI
            commands
                .spawn((
                    DialogueTreeUIRoot,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Cinematic gradient
                    parent.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            bottom: Val::Px(0.0),
                            width: Val::Percent(100.0),
                            height: Val::Percent(40.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                    ));

                    // Dialogue box
                    parent
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(200.0),
                                left: Val::Px(100.0),
                                width: Val::Px(800.0),
                                height: Val::Auto,
                                padding: UiRect::all(Val::Px(20.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(15.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.15, 0.15, 0.2, 0.9)),
                            BorderColor(Color::srgb(0.5, 0.7, 0.9)),
                        ))
                        .with_children(|box_parent| {
                            // Speaker name
                            box_parent.spawn((
                                Text::new(&npc_name),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                            ));

                            // Dialogue text
                            box_parent.spawn((
                                Text::new(&node.text),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                Node {
                                    max_width: Val::Px(760.0),
                                    ..default()
                                },
                            ));

                            // Choices container
                            if !node.choices.is_empty() && !node.is_end_node {
                                box_parent
                                    .spawn((
                                        DialogueChoicesContainer,
                                        Node {
                                            width: Val::Percent(100.0),
                                            flex_direction: FlexDirection::Column,
                                            row_gap: Val::Px(10.0),
                                            margin: UiRect::top(Val::Px(15.0)),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|choices_parent| {
                                        for (idx, choice) in node.choices.iter().enumerate() {
                                            let is_available =
                                                choice.is_available(&flags, &reputation);
                                            let button_color = if is_available {
                                                Color::srgb(0.3, 0.4, 0.5)
                                            } else {
                                                Color::srgb(0.2, 0.2, 0.25)
                                            };

                                            let text_color = if is_available {
                                                Color::WHITE
                                            } else {
                                                Color::srgb(0.5, 0.5, 0.5)
                                            };

                                            let display_text = if is_available {
                                                choice.text.clone()
                                            } else {
                                                choice.disabled_text.clone().unwrap_or_else(|| {
                                                    format!("[Locked] {}", choice.text)
                                                })
                                            };

                                            choices_parent
                                                .spawn((
                                                    DialogueChoiceButton {
                                                        choice_index: idx,
                                                        next_node_id: choice.next_node_id.clone(),
                                                    },
                                                    Button,
                                                    Node {
                                                        width: Val::Percent(100.0),
                                                        padding: UiRect::all(Val::Px(12.0)),
                                                        justify_content: JustifyContent::Start,
                                                        align_items: AlignItems::Center,
                                                        ..default()
                                                    },
                                                    BackgroundColor(button_color),
                                                    BorderColor(Color::srgb(0.4, 0.6, 0.8)),
                                                ))
                                                .with_children(|button_parent| {
                                                    button_parent.spawn((
                                                        Text::new(format!(
                                                            "{}. {}",
                                                            idx + 1,
                                                            display_text
                                                        )),
                                                        TextFont {
                                                            font_size: 16.0,
                                                            ..default()
                                                        },
                                                        TextColor(text_color),
                                                    ));
                                                });
                                        }
                                    });
                            } else if node.is_end_node {
                                // End node - show close prompt
                                box_parent.spawn((
                                    Text::new("Press ESC to close"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgba(0.7, 0.7, 0.7, 0.8)),
                                    Node {
                                        margin: UiRect::top(Val::Px(10.0)),
                                        ..default()
                                    },
                                ));
                            }
                        });
                });
        }

        /// Handle dialogue choice button clicks
        pub fn handle_choice_buttons(
            mut commands: Commands,
            mut dialogue_state: ResMut<ActiveDialogueState>,
            mut choice_selected: EventWriter<DialogueChoiceSelected>,
            mut tree_ended: EventWriter<DialogueTreeEnded>,
            keyboard: Res<ButtonInput<KeyCode>>,
            registry: Res<DialogueTreeRegistry>,
            button_query: Query<
                (&Interaction, &DialogueChoiceButton),
                (Changed<Interaction>, With<Button>),
            >,
            ui_root: Query<Entity, With<DialogueTreeUIRoot>>,
        ) {
            // Close dialogue on escape
            if keyboard.just_pressed(KeyCode::Escape) && dialogue_state.active {
                if let Some(tree_id) = dialogue_state.tree_id.clone() {
                    tree_ended.send(DialogueTreeEnded { tree_id });
                }
                dialogue_state.end_dialogue();
                for entity in ui_root.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Handle button clicks
            for (interaction, choice_button) in button_query.iter() {
                if *interaction == Interaction::Pressed {
                    let Some(tree_id) = dialogue_state.tree_id.clone() else {
                        continue;
                    };
                    let Some(tree) = registry.get(&tree_id) else {
                        continue;
                    };

                    let node_id = dialogue_state
                        .current_node_id
                        .clone()
                        .unwrap_or_else(|| tree.starting_node_id.clone());

                    // Send choice selected event
                    choice_selected.send(DialogueChoiceSelected {
                        tree_id: tree_id.clone(),
                        node_id: node_id.clone(),
                        choice_index: choice_button.choice_index,
                        next_node_id: choice_button.next_node_id.clone(),
                    });

                    // Navigate to next node
                    if let Some(next_node) = tree.get_node(&choice_button.next_node_id) {
                        dialogue_state.navigate_to_node(choice_button.next_node_id.clone());

                        // Despawn current UI to re-render with new node
                        for entity in ui_root.iter() {
                            commands.entity(entity).despawn_recursive();
                        }

                        // End dialogue if this is an end node
                        if next_node.is_end_node {
                            // Don't end immediately, let player read the final message
                            // They can press ESC to close
                        }
                    }

                    break;
                }
            }

            // Keyboard shortcuts for choices (1-4 keys)
            if dialogue_state.active {
                let Some(tree_id) = &dialogue_state.tree_id else {
                    return;
                };
                let Some(tree) = registry.get(tree_id) else {
                    return;
                };

                let node_id = dialogue_state
                    .current_node_id
                    .as_deref()
                    .unwrap_or(&tree.starting_node_id);
                let Some(node) = tree.get_node(node_id) else {
                    return;
                };

                for (idx, key) in [
                    KeyCode::Digit1,
                    KeyCode::Digit2,
                    KeyCode::Digit3,
                    KeyCode::Digit4,
                ]
                .iter()
                .enumerate()
                {
                    if keyboard.just_pressed(*key) && idx < node.choices.len() {
                        let choice = &node.choices[idx];

                        // Send choice selected event
                        choice_selected.send(DialogueChoiceSelected {
                            tree_id: tree_id.clone(),
                            node_id: node_id.to_string(),
                            choice_index: idx,
                            next_node_id: choice.next_node_id.clone(),
                        });

                        // Navigate to next node
                        dialogue_state.navigate_to_node(choice.next_node_id.clone());

                        // Despawn current UI to re-render
                        for entity in ui_root.iter() {
                            commands.entity(entity).despawn_recursive();
                        }

                        break;
                    }
                }
            }
        }
    }

    // ============================================================================
    // MINIMAP SYSTEM
    // ============================================================================
    pub mod minimap {
        use bevy::prelude::*;
        use bevy_shaman_core::components::{GridPosition, Player};
        use bevy_shaman_world::components::{BiomeType, WorldTile};
        use std::collections::HashSet;

        #[derive(Component)]
        pub struct MinimapRoot;

        #[derive(Component)]
        pub struct MinimapTile {
            pub grid_x: i32,
            pub grid_y: i32,
        }

        #[derive(Resource)]
        pub struct MinimapState {
            pub visible: bool,
            pub explored_tiles: HashSet<(i32, i32)>, // Fog of war tracking
            pub view_radius: i32,
        }

        impl Default for MinimapState {
            fn default() -> Self {
                Self {
                    visible: true,
                    explored_tiles: HashSet::new(),
                    view_radius: 10,
                }
            }
        }

        pub fn update_minimap(
            mut commands: Commands,
            mut minimap_state: ResMut<MinimapState>,
            player_query: Query<&GridPosition, With<Player>>,
            tile_query: Query<(&GridPosition, &WorldTile)>,
            npc_query: Query<&GridPosition, With<bevy_shaman_story::components::NpcName>>,
            monster_query: Query<&GridPosition, With<bevy_shaman_monsters::components::MonsterId>>,
            minimap_root_query: Query<Entity, With<MinimapRoot>>,
            _keyboard: Res<ButtonInput<KeyCode>>,
        ) {
            if !minimap_state.visible {
                // Clean up minimap if not visible
                for entity in minimap_root_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Get player position
            let player_pos = if let Ok(pos) = player_query.get_single() {
                pos
            } else {
                return;
            };

            // Update explored tiles based on player view radius
            for x in (player_pos.x - minimap_state.view_radius)
                ..=(player_pos.x + minimap_state.view_radius)
            {
                for y in (player_pos.y - minimap_state.view_radius)
                    ..=(player_pos.y + minimap_state.view_radius)
                {
                    // Check if within circular radius
                    let dx = x - player_pos.x;
                    let dy = y - player_pos.y;
                    if dx * dx + dy * dy <= minimap_state.view_radius * minimap_state.view_radius {
                        minimap_state.explored_tiles.insert((x, y));
                    }
                }
            }

            // Only spawn minimap once
            if !minimap_root_query.is_empty() {
                return;
            }

            // Spawn minimap UI
            commands
                .spawn((
                    MinimapRoot,
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        width: Val::Px(250.0),
                        height: Val::Px(250.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
                    BorderColor(Color::srgb(0.3, 0.4, 0.5)),
                    GlobalZIndex(3000), // Higher than inventory (2000) to always show on top
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("Minimap"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.9, 1.0)),
                    ));

                    // Minimap grid container
                    parent
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            display: Display::Grid,
                            grid_template_columns: vec![GridTrack::auto(); 20],
                            grid_template_rows: vec![GridTrack::auto(); 20],
                            ..default()
                        },))
                        .with_children(|grid_parent| {
                            // Render explored tiles
                            let map_range = 10; // Show 20x20 grid
                            for dy in -map_range..=map_range {
                                for dx in -map_range..=map_range {
                                    let world_x = player_pos.x + dx;
                                    let world_y = player_pos.y + dy;

                                    let tile_color = if minimap_state
                                        .explored_tiles
                                        .contains(&(world_x, world_y))
                                    {
                                        // Find tile type at this position
                                        let mut found_color = Color::srgb(0.2, 0.2, 0.2); // Default unexplored
                                        for (tile_pos, world_tile) in tile_query.iter() {
                                            if tile_pos.x == world_x && tile_pos.y == world_y {
                                                found_color = match world_tile.biome {
                                                    BiomeType::Village => {
                                                        Color::srgb(0.8, 0.6, 0.4)
                                                    }
                                                    BiomeType::Jungle => Color::srgb(0.0, 0.5, 0.2),
                                                    BiomeType::Desert => Color::srgb(0.9, 0.8, 0.5),
                                                    BiomeType::Forest => Color::srgb(0.1, 0.4, 0.1),
                                                    BiomeType::Safari => Color::srgb(0.7, 0.7, 0.3),
                                                    BiomeType::DeadRealm => {
                                                        Color::srgb(0.3, 0.1, 0.3)
                                                    }
                                                    BiomeType::Mountains => {
                                                        Color::srgb(0.5, 0.5, 0.5)
                                                    }
                                                    BiomeType::SpiritRealm => {
                                                        Color::srgb(0.4, 0.2, 0.8)
                                                    }
                                                };
                                                break;
                                            }
                                        }
                                        found_color
                                    } else {
                                        Color::srgba(0.1, 0.1, 0.1, 0.5) // Fog of war
                                    };

                                    // Check for entities at this position
                                    let mut final_color = tile_color;

                                    // Player position marker (highest priority)
                                    if dx == 0 && dy == 0 {
                                        final_color = Color::srgb(1.0, 1.0, 0.0);
                                    // Yellow for player
                                    } else {
                                        // Check for NPCs
                                        for npc_pos in npc_query.iter() {
                                            if npc_pos.x == world_x && npc_pos.y == world_y {
                                                final_color = Color::srgb(0.2, 1.0, 0.2); // Green for NPCs
                                                break;
                                            }
                                        }

                                        // Check for monsters (if no NPC found)
                                        if final_color == tile_color {
                                            for monster_pos in monster_query.iter() {
                                                if monster_pos.x == world_x
                                                    && monster_pos.y == world_y
                                                {
                                                    final_color = Color::srgb(1.0, 0.2, 0.2); // Red for monsters
                                                    break;
                                                }
                                            }
                                        }
                                    }

                                    grid_parent.spawn((
                                        MinimapTile {
                                            grid_x: world_x,
                                            grid_y: world_y,
                                        },
                                        Node {
                                            width: Val::Px(10.0),
                                            height: Val::Px(10.0),
                                            border: UiRect::all(Val::Px(0.5)),
                                            ..default()
                                        },
                                        BackgroundColor(final_color),
                                        BorderColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
                                    ));
                                }
                            }
                        });
                });
        }
    }

    // ============================================================================
    // INVENTORY UI SYSTEM
    // ============================================================================
    pub mod inventory_ui {
        use bevy::prelude::*;
        use bevy_shaman_core::components::Player;
        use bevy_shaman_items::components::{Inventory, ItemStack};

        #[derive(Component)]
        pub struct InventoryUIRoot;

        #[derive(Component)]
        pub struct InventorySlot {
            pub slot_index: usize,
        }

        #[derive(Component)]
        pub struct InventoryItemIcon;

        #[derive(Component)]
        pub struct InventoryItemText;

        #[derive(Component)]
        pub struct ItemTooltip {
            pub item_id: String,
            pub item_name: String,
            pub quantity: u32,
        }

        #[derive(Component)]
        pub struct TooltipUI;

        #[derive(Component)]
        pub struct ContextMenuUI;

        #[derive(Component)]
        pub struct ContextMenuItem {
            pub action: ItemAction,
        }

        #[derive(Clone, Copy, PartialEq)]
        pub enum ItemAction {
            Use,
            Drop,
            DropStack,
            Examine,
        }

        #[derive(Resource)]
        pub struct InventoryUIState {
            pub visible: bool,
            pub selected_slot: Option<usize>,
            pub dragging_slot: Option<usize>,
            pub hovered_slot: Option<usize>,
            pub show_context_menu: bool,
            pub context_menu_slot: Option<usize>,
        }

        impl Default for InventoryUIState {
            fn default() -> Self {
                Self {
                    visible: false,
                    selected_slot: None,
                    dragging_slot: None,
                    hovered_slot: None,
                    show_context_menu: false,
                    context_menu_slot: None,
                }
            }
        }

        const GRID_COLS: usize = 8;
        const GRID_ROWS: usize = 6;
        const SLOT_SIZE: f32 = 70.0;
        const SLOT_SPACING: f32 = 5.0;

        /// Get an icon emoji for an item based on its ID
        fn get_item_icon(item_id: &str) -> &'static str {
            match item_id {
                // Potions
                id if id.contains("health_potion") || id.contains("healing") => "🧪",
                id if id.contains("mana_potion") || id.contains("spirit") => "💙",
                id if id.contains("stamina") => "💚",
                // Food
                id if id.contains("bread") || id.contains("food") => "🍞",
                id if id.contains("meat") => "🍖",
                id if id.contains("berry") || id.contains("fruit") => "🍇",
                // Weapons
                id if id.contains("sword") => "⚔️",
                id if id.contains("axe") => "🪓",
                id if id.contains("bow") => "🏹",
                id if id.contains("staff") => "🪄",
                id if id.contains("dagger") => "🗡️",
                // Armor
                id if id.contains("helmet") || id.contains("head") => "⛑️",
                id if id.contains("chest") || id.contains("armor") => "🛡️",
                id if id.contains("boots") || id.contains("feet") => "🥾",
                // Resources
                id if id.contains("wood") || id.contains("log") => "🪵",
                id if id.contains("stone") || id.contains("rock") => "🪨",
                id if id.contains("ore") || id.contains("metal") => "⛏️",
                id if id.contains("herb") || id.contains("plant") => "🌿",
                id if id.contains("gem") || id.contains("crystal") => "💎",
                // Tools
                id if id.contains("key") => "🔑",
                id if id.contains("book") || id.contains("scroll") => "📜",
                id if id.contains("map") => "🗺️",
                // Quest items
                id if id.contains("quest") => "❗",
                // Default
                _ => "📦",
            }
        }

        pub fn display_inventory(
            mut commands: Commands,
            keyboard: Res<ButtonInput<KeyCode>>,
            mut ui_state: ResMut<InventoryUIState>,
            ui_root_query: Query<Entity, With<InventoryUIRoot>>,
            player_inventory: Query<&Inventory, With<Player>>,
        ) {
            // Toggle inventory with 'I' key
            if keyboard.just_pressed(KeyCode::KeyI) {
                ui_state.visible = !ui_state.visible;
            }

            // Hide inventory if not visible
            if !ui_state.visible {
                for entity in ui_root_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Only spawn UI once
            if !ui_root_query.is_empty() {
                return;
            }

            // Get player inventory
            let inventory = if let Ok(inv) = player_inventory.get_single() {
                inv
            } else {
                return;
            };

            let total_slots = GRID_COLS * GRID_ROWS;

            // Spawn inventory UI
            commands
                .spawn((
                    InventoryUIRoot,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(15.0),
                        top: Val::Percent(10.0),
                        width: Val::Auto,
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(25.0)),
                        row_gap: Val::Px(15.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.08, 0.12, 0.95)),
                    BorderColor(Color::srgb(0.5, 0.7, 0.9)),
                ))
                .with_children(|parent| {
                    // ====== Header ======
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.15, 0.2, 0.25, 0.8)),
                            BorderColor(Color::srgb(0.3, 0.5, 0.7)),
                        ))
                        .with_children(|header| {
                            // Title
                            header.spawn((
                                Text::new("⚔ INVENTORY ⚔"),
                                TextFont {
                                    font_size: 28.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                            ));

                            // Slot counter
                            header.spawn((
                                Text::new(format!(
                                    "Slots: {} / {}",
                                    inventory.items.len(),
                                    inventory.max_slots
                                )),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.7, 0.8, 0.9)),
                            ));
                        });

                    // ====== Inventory Grid ======
                    parent
                        .spawn((
                            Node {
                                display: Display::Grid,
                                grid_template_columns: vec![GridTrack::px(SLOT_SIZE); GRID_COLS],
                                grid_template_rows: vec![GridTrack::px(SLOT_SIZE); GRID_ROWS],
                                column_gap: Val::Px(SLOT_SPACING),
                                row_gap: Val::Px(SLOT_SPACING),
                                padding: UiRect::all(Val::Px(15.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.05, 0.05, 0.08, 0.6)),
                        ))
                        .with_children(|grid| {
                            // Render all slots
                            for slot_index in 0..total_slots {
                                let item_stack: Option<&ItemStack> =
                                    inventory.items.get(slot_index);

                                let is_selected = ui_state.selected_slot == Some(slot_index);
                                let is_dragging = ui_state.dragging_slot == Some(slot_index);
                                let is_hovered = ui_state.hovered_slot == Some(slot_index);

                                grid.spawn((
                                    InventorySlot { slot_index },
                                    Button,
                                    Node {
                                        width: Val::Px(SLOT_SIZE),
                                        height: Val::Px(SLOT_SIZE),
                                        border: UiRect::all(Val::Px(
                                            if is_selected || is_dragging { 3.0 } else { 2.0 },
                                        )),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    BackgroundColor(if is_dragging {
                                        Color::srgba(0.4, 0.5, 0.6, 0.7)
                                    } else if is_hovered && item_stack.is_some() {
                                        Color::srgba(0.3, 0.35, 0.4, 0.95)
                                    } else if item_stack.is_some() {
                                        Color::srgba(0.2, 0.25, 0.3, 0.9)
                                    } else {
                                        Color::srgba(0.1, 0.1, 0.15, 0.5)
                                    }),
                                    BorderColor(if is_selected {
                                        Color::srgb(0.9, 0.8, 0.3)
                                    } else if is_dragging {
                                        Color::srgb(0.7, 0.9, 1.0)
                                    } else if item_stack.is_some() {
                                        Color::srgb(0.4, 0.6, 0.8)
                                    } else {
                                        Color::srgb(0.2, 0.2, 0.3)
                                    }),
                                ))
                                .with_children(|slot| {
                                    if let Some(stack) = item_stack {
                                        // Item icon
                                        slot.spawn((
                                            InventoryItemIcon,
                                            Text::new(get_item_icon(&stack.item.id)),
                                            TextFont {
                                                font_size: 28.0,
                                                ..default()
                                            },
                                            Node {
                                                margin: UiRect::bottom(Val::Px(2.0)),
                                                ..default()
                                            },
                                        ));

                                        // Item name (shortened)
                                        let display_name = if stack.item.display_name.len() > 8 {
                                            format!("{}...", &stack.item.display_name[..5])
                                        } else {
                                            stack.item.display_name.clone()
                                        };

                                        slot.spawn((
                                            InventoryItemText,
                                            Text::new(&display_name),
                                            TextFont {
                                                font_size: 9.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.9, 0.9, 1.0)),
                                            Node {
                                                margin: UiRect::bottom(Val::Px(1.0)),
                                                ..default()
                                            },
                                        ));

                                        // Quantity display
                                        slot.spawn((
                                            Text::new(format!("x{}", stack.quantity)),
                                            TextFont {
                                                font_size: 11.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(1.0, 0.9, 0.5)),
                                        ));
                                    } else {
                                        // Empty slot indicator
                                        slot.spawn((
                                            Text::new("─"),
                                            TextFont {
                                                font_size: 20.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgba(0.3, 0.3, 0.3, 0.5)),
                                        ));
                                    }
                                });
                            }
                        });

                    // ====== Footer with controls ======
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                margin: UiRect::top(Val::Px(10.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(5.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.15, 0.2, 0.25, 0.8)),
                        ))
                        .with_children(|footer| {
                            footer.spawn((
                                Text::new("Press [I] or [ESC] to close"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.7, 0.8)),
                            ));

                            footer.spawn((
                                Text::new("Left-click & drag: Move item | Right-click: Item menu"),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 0.5, 0.6)),
                            ));

                            footer.spawn((
                                Text::new("Hover for details | Double-click: Use item"),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.5, 0.5, 0.6)),
                            ));
                        });
                });
        }

        /// Handle click interactions with inventory slots - drag-drop, tooltips, context menu
        pub fn handle_inventory_interactions(
            mut ui_state: ResMut<InventoryUIState>,
            slot_query: Query<(&InventorySlot, &Interaction), Changed<Interaction>>,
            mut player_inventory: Query<&mut Inventory, With<Player>>,
            mouse_button: Res<ButtonInput<MouseButton>>,
            keyboard: Res<ButtonInput<KeyCode>>,
            mut use_events: EventWriter<bevy_shaman_items::systems::inventory::ItemUsed>,
            mut drop_events: EventWriter<bevy_shaman_items::systems::inventory::ItemDropped>,
            player_query: Query<Entity, With<Player>>,
            _time: Res<Time>,
        ) {
            // Close inventory with ESC
            if keyboard.just_pressed(KeyCode::Escape) && ui_state.visible {
                ui_state.visible = false;
                ui_state.dragging_slot = None;
                ui_state.show_context_menu = false;
                return;
            }

            if !ui_state.visible {
                return;
            }

            let Ok(player_entity) = player_query.get_single() else {
                return;
            };

            // Update hover state
            let mut new_hovered_slot = None;
            for (slot, interaction) in slot_query.iter() {
                if *interaction == Interaction::Hovered {
                    new_hovered_slot = Some(slot.slot_index);
                    break;
                }
            }
            ui_state.hovered_slot = new_hovered_slot;

            // Handle mouse button releases
            if mouse_button.just_released(MouseButton::Left) {
                if let Some(dragging_from) = ui_state.dragging_slot {
                    // Drop on hovered slot or back to original
                    if let Some(target_slot) = ui_state.hovered_slot {
                        if target_slot != dragging_from {
                            // Swap items between slots
                            if let Ok(mut inventory) = player_inventory.get_single_mut() {
                                let from_item = inventory.items.get(dragging_from).cloned();
                                let to_item = inventory.items.get(target_slot).cloned();

                                // Simple swap logic
                                if let Some(from) = from_item {
                                    inventory.items.remove(dragging_from);
                                    if let Some(to) = to_item {
                                        inventory.items.insert(dragging_from, to);
                                    }
                                    inventory.items.insert(target_slot, from);
                                    info!(
                                        "Moved item from slot {} to slot {}",
                                        dragging_from, target_slot
                                    );
                                }
                            }
                        }
                    }
                    ui_state.dragging_slot = None;
                }
            }

            // Handle slot interactions
            for (slot, interaction) in slot_query.iter() {
                if *interaction != Interaction::Pressed {
                    continue;
                }

                let Ok(inventory) = player_inventory.get_single() else {
                    continue;
                };

                let has_item = inventory.items.get(slot.slot_index).is_some();

                // Left click - start dragging if has item
                if mouse_button.just_pressed(MouseButton::Left) && has_item {
                    ui_state.dragging_slot = Some(slot.slot_index);
                    ui_state.selected_slot = Some(slot.slot_index);
                    ui_state.show_context_menu = false;
                }

                // Right click - show context menu
                if mouse_button.just_pressed(MouseButton::Right) && has_item {
                    ui_state.show_context_menu = true;
                    ui_state.context_menu_slot = Some(slot.slot_index);
                    ui_state.dragging_slot = None;
                    info!("Opening context menu for slot {}", slot.slot_index);
                }
            }

            // Handle context menu actions
            if ui_state.show_context_menu {
                if let Some(menu_slot) = ui_state.context_menu_slot {
                    if let Ok(inventory) = player_inventory.get_single() {
                        if let Some(item_stack) = inventory.items.get(menu_slot) {
                            // For now, use keyboard shortcuts for actions
                            // U = Use, D = Drop one, X = Drop stack
                            if keyboard.just_pressed(KeyCode::KeyU) {
                                info!("Using item: {}", item_stack.item.display_name);
                                use_events.send(bevy_shaman_items::systems::inventory::ItemUsed {
                                    player: player_entity,
                                    item_id: item_stack.item.id.clone(),
                                });
                                ui_state.show_context_menu = false;
                            } else if keyboard.just_pressed(KeyCode::KeyD) {
                                info!("Dropping 1x {}", item_stack.item.display_name);
                                drop_events.send(
                                    bevy_shaman_items::systems::inventory::ItemDropped {
                                        player: player_entity,
                                        item_id: item_stack.item.id.clone(),
                                        quantity: 1,
                                    },
                                );
                                ui_state.show_context_menu = false;
                            } else if keyboard.just_pressed(KeyCode::KeyX) {
                                info!("Dropping entire stack of {}", item_stack.item.display_name);
                                drop_events.send(
                                    bevy_shaman_items::systems::inventory::ItemDropped {
                                        player: player_entity,
                                        item_id: item_stack.item.id.clone(),
                                        quantity: item_stack.quantity,
                                    },
                                );
                                ui_state.show_context_menu = false;
                            }
                        }
                    }
                }

                // Close menu with ESC or any click outside
                if keyboard.just_pressed(KeyCode::Escape) {
                    ui_state.show_context_menu = false;
                }
            }
        }

        /// Display tooltip when hovering over inventory items
        pub fn display_inventory_tooltip(
            mut commands: Commands,
            ui_state: Res<InventoryUIState>,
            tooltip_query: Query<Entity, With<TooltipUI>>,
            player_inventory: Query<&Inventory, With<Player>>,
        ) {
            // Remove existing tooltips
            for entity in tooltip_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            if !ui_state.visible {
                return;
            }

            // Show tooltip for hovered slot
            if let Some(hovered_slot) = ui_state.hovered_slot {
                if let Ok(inventory) = player_inventory.get_single() {
                    if let Some(item_stack) = inventory.items.get(hovered_slot) {
                        // Spawn tooltip UI
                        commands
                            .spawn((
                                TooltipUI,
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(50.0),
                                    top: Val::Percent(30.0),
                                    padding: UiRect::all(Val::Px(12.0)),
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(5.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.05, 0.05, 0.08, 0.98)),
                                BorderColor(Color::srgb(0.6, 0.7, 0.9)),
                                ZIndex(1000),
                            ))
                            .with_children(|parent| {
                                // Item name
                                parent.spawn((
                                    Text::new(&item_stack.item.display_name),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 1.0)),
                                ));

                                // Item type
                                parent.spawn((
                                    Text::new(format!("Type: {:?}", item_stack.item.item_type)),
                                    TextFont {
                                        font_size: 13.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.7, 0.75, 0.8)),
                                ));

                                // Quantity
                                parent.spawn((
                                    Text::new(format!("Quantity: {}", item_stack.quantity)),
                                    TextFont {
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(1.0, 0.9, 0.5)),
                                ));

                                // Item ID (for debugging)
                                parent.spawn((
                                    Text::new(format!("ID: {}", item_stack.item.id)),
                                    TextFont {
                                        font_size: 10.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.4, 0.4, 0.5)),
                                ));
                            });
                    }
                }
            }
        }

        /// Display context menu for item actions
        pub fn display_context_menu(
            mut commands: Commands,
            ui_state: Res<InventoryUIState>,
            context_menu_query: Query<Entity, With<ContextMenuUI>>,
            player_inventory: Query<&Inventory, With<Player>>,
        ) {
            // Remove existing context menus
            for entity in context_menu_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            if !ui_state.visible || !ui_state.show_context_menu {
                return;
            }

            // Show context menu for selected slot
            if let Some(menu_slot) = ui_state.context_menu_slot {
                if let Ok(inventory) = player_inventory.get_single() {
                    if let Some(item_stack) = inventory.items.get(menu_slot) {
                        // Spawn context menu UI
                        commands
                            .spawn((
                                ContextMenuUI,
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(55.0),
                                    top: Val::Percent(40.0),
                                    padding: UiRect::all(Val::Px(15.0)),
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Px(8.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    min_width: Val::Px(200.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.08, 0.08, 0.12, 0.98)),
                                BorderColor(Color::srgb(0.7, 0.8, 0.9)),
                                ZIndex(1001),
                            ))
                            .with_children(|parent| {
                                // Header
                                parent.spawn((
                                    Text::new(format!("⚙ {}", item_stack.item.display_name)),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 1.0)),
                                    Node {
                                        margin: UiRect::bottom(Val::Px(5.0)),
                                        ..default()
                                    },
                                ));

                                // Action options
                                parent.spawn((
                                    Text::new("[U] Use Item"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.6, 0.9, 0.6)),
                                ));

                                parent.spawn((
                                    Text::new("[D] Drop One"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.7, 0.5)),
                                ));

                                parent.spawn((
                                    Text::new("[X] Drop Stack"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.5, 0.5)),
                                ));

                                parent.spawn((
                                    Text::new("\n[ESC] Close Menu"),
                                    TextFont {
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.5, 0.5, 0.6)),
                                ));
                            });
                    }
                }
            }
        }
    }

    // ============================================================================
    // COMBAT VISUAL FEEDBACK SYSTEM
    // ============================================================================
    pub mod combat_feedback {
        use bevy::prelude::*;

        // Re-export the HitLanded event from combat crate
        // We'll listen to this event to spawn damage numbers
        use bevy_shaman_combat::systems::events::HitLanded;

        #[derive(Component)]
        pub struct DamageNumber {
            pub lifetime: Timer,
            pub initial_y: f32,
        }

        #[derive(Component)]
        pub struct HitEffect {
            pub lifetime: Timer,
        }

        const DAMAGE_NUMBER_LIFETIME: f32 = 1.2;
        const DAMAGE_NUMBER_RISE_SPEED: f32 = 50.0;
        const HIT_EFFECT_LIFETIME: f32 = 0.3;

        /// Spawn floating damage numbers when hits land
        pub fn spawn_damage_numbers(
            mut commands: Commands,
            mut hit_events: EventReader<HitLanded>,
            target_positions: Query<&Transform>,
        ) {
            for event in hit_events.read() {
                // Get target position
                let target_transform = if let Ok(transform) = target_positions.get(event.target) {
                    transform.translation
                } else {
                    continue;
                };

                // Determine color based on damage amount
                let (color, font_size) = if event.damage >= 50.0 {
                    // Critical hit
                    (Color::srgb(1.0, 0.2, 0.2), 32.0)
                } else if event.damage >= 25.0 {
                    // Heavy hit
                    (Color::srgb(1.0, 0.6, 0.2), 28.0)
                } else {
                    // Normal hit
                    (Color::srgb(1.0, 1.0, 0.4), 24.0)
                };

                // Spawn damage number UI element
                commands.spawn((
                    DamageNumber {
                        lifetime: Timer::from_seconds(DAMAGE_NUMBER_LIFETIME, TimerMode::Once),
                        initial_y: target_transform.y,
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(target_transform.x),
                        top: Val::Px(720.0 - target_transform.y), // Convert world to screen Y
                        ..default()
                    },
                    Text::new(format!("{:.0}", event.damage)),
                    TextFont {
                        font_size,
                        ..default()
                    },
                    TextColor(color),
                ));
            }
        }

        /// Update and animate damage numbers
        pub fn update_damage_numbers(
            mut commands: Commands,
            time: Res<Time>,
            mut damage_numbers: Query<(Entity, &mut DamageNumber, &mut Node, &mut TextColor)>,
        ) {
            for (entity, mut damage_num, mut node, mut text_color) in damage_numbers.iter_mut() {
                damage_num.lifetime.tick(time.delta());

                // Rise animation
                let progress = damage_num.lifetime.fraction();

                if let Val::Px(current_top) = node.top {
                    node.top = Val::Px(current_top - time.delta_secs() * DAMAGE_NUMBER_RISE_SPEED);
                }

                // Fade out
                let alpha = 1.0 - progress;
                text_color.0.set_alpha(alpha);

                // Remove when lifetime expires
                if damage_num.lifetime.finished() {
                    commands.entity(entity).despawn();
                }
            }
        }

        /// Spawn hit effects (flashes, screen shake, etc.)
        pub fn spawn_hit_effects(
            mut commands: Commands,
            mut hit_events: EventReader<HitLanded>,
            target_positions: Query<&Transform>,
        ) {
            for event in hit_events.read() {
                // Get target position
                let target_transform = if let Ok(transform) = target_positions.get(event.target) {
                    transform.translation
                } else {
                    continue;
                };

                // Spawn hit flash effect
                commands.spawn((
                    HitEffect {
                        lifetime: Timer::from_seconds(HIT_EFFECT_LIFETIME, TimerMode::Once),
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(target_transform.x - 16.0),
                        top: Val::Px(720.0 - target_transform.y - 16.0),
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                    BorderRadius::all(Val::Px(16.0)),
                ));
            }
        }

        /// Update hit effects (cleanup and fade out)
        pub fn update_hit_effects(
            mut commands: Commands,
            time: Res<Time>,
            mut hit_effects: Query<(Entity, &mut HitEffect, &mut BackgroundColor)>,
        ) {
            for (entity, mut effect, mut bg_color) in hit_effects.iter_mut() {
                effect.lifetime.tick(time.delta());

                // Fade out
                let alpha = 1.0 - effect.lifetime.fraction();
                bg_color.0.set_alpha(alpha * 0.8);

                // Remove when lifetime expires
                if effect.lifetime.finished() {
                    commands.entity(entity).despawn();
                }
            }
        }

        /// Screen shake resource
        #[derive(Resource, Default)]
        pub struct ScreenShake {
            pub intensity: f32,
            pub duration: f32,
            pub current_time: f32,
        }

        impl ScreenShake {
            pub fn trigger(&mut self, intensity: f32, duration: f32) {
                self.intensity = intensity;
                self.duration = duration;
                self.current_time = 0.0;
            }

            pub fn is_active(&self) -> bool {
                self.current_time < self.duration
            }
        }

        /// Apply screen shake effect to camera
        pub fn apply_screen_shake(
            mut camera: Query<&mut Transform, With<Camera>>,
            mut shake: ResMut<ScreenShake>,
            time: Res<Time>,
            mut hit_events: EventReader<HitLanded>,
        ) {
            // Trigger shake on heavy hits
            for event in hit_events.read() {
                if event.damage >= 50.0 {
                    shake.trigger(8.0, 0.3);
                } else if event.damage >= 25.0 {
                    shake.trigger(4.0, 0.2);
                } else if event.damage >= 10.0 {
                    shake.trigger(2.0, 0.15);
                }
            }

            if !shake.is_active() {
                return;
            }

            shake.current_time += time.delta_secs();

            // Calculate shake offset using sine wave
            let progress = shake.current_time / shake.duration;
            let decay = 1.0 - progress;
            let shake_amount = shake.intensity * decay;

            // Apply shake to camera
            for mut transform in camera.iter_mut() {
                use std::f32::consts::PI;
                let offset_x = (shake.current_time * 20.0).sin() * shake_amount;
                let offset_y = (shake.current_time * 25.0 + PI / 2.0).sin() * shake_amount;

                // Apply shake as small translation offsets
                transform.translation.x += offset_x;
                transform.translation.y += offset_y;
            }
        }
    }

    // ============================================================================
    // TAMING PROGRESS UI SYSTEM
    // ============================================================================
    pub mod taming_ui {
        use bevy::prelude::*;
        use bevy_shaman_minions::components::TamingProgress;

        #[derive(Component)]
        pub struct TamingProgressBar;

        pub fn display_taming_progress(
            mut commands: Commands,
            taming_query: Query<&TamingProgress>,
            progress_bar_query: Query<Entity, With<TamingProgressBar>>,
        ) {
            let has_taming_in_progress = !taming_query.is_empty();

            if has_taming_in_progress {
                // Spawn progress bar if it doesn't exist
                if progress_bar_query.is_empty() {
                    commands
                        .spawn((
                            TamingProgressBar,
                            Node {
                                position_type: PositionType::Absolute,
                                left: Val::Percent(35.0),
                                bottom: Val::Percent(20.0),
                                width: Val::Px(400.0),
                                height: Val::Px(40.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(5.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
                            BorderColor(Color::srgb(0.4, 0.6, 0.9)),
                        ))
                        .with_children(|parent| {
                            // Title
                            parent.spawn((
                                Text::new("Taming..."),
                                TextFont {
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(5.0)),
                                    ..default()
                                },
                            ));

                            // Progress bar background
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(20.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                                    BorderColor(Color::srgb(0.3, 0.4, 0.5)),
                                ))
                                .with_children(|progress_bg| {
                                    // Get the first taming progress
                                    if let Some(taming) = taming_query.iter().next() {
                                        // Progress bar fill
                                        progress_bg.spawn((
                                            Node {
                                                width: Val::Percent(taming.progress * 100.0),
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.3, 0.7, 1.0)),
                                        ));
                                    }
                                });
                        });
                } else {
                    // Update existing progress bar
                    for bar_entity in progress_bar_query.iter() {
                        commands.entity(bar_entity).despawn_recursive();
                        // Re-spawn with updated progress
                        if let Some(taming) = taming_query.iter().next() {
                            commands
                                .spawn((
                                    TamingProgressBar,
                                    Node {
                                        position_type: PositionType::Absolute,
                                        left: Val::Percent(35.0),
                                        bottom: Val::Percent(20.0),
                                        width: Val::Px(400.0),
                                        height: Val::Px(40.0),
                                        flex_direction: FlexDirection::Column,
                                        padding: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
                                    BorderColor(Color::srgb(0.4, 0.6, 0.9)),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new(format!(
                                            "Taming... {:.0}%",
                                            taming.progress * 100.0
                                        )),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.9, 0.9, 1.0)),
                                        Node {
                                            margin: UiRect::bottom(Val::Px(5.0)),
                                            ..default()
                                        },
                                    ));

                                    parent
                                        .spawn((
                                            Node {
                                                width: Val::Percent(100.0),
                                                height: Val::Px(20.0),
                                                border: UiRect::all(Val::Px(2.0)),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                                            BorderColor(Color::srgb(0.3, 0.4, 0.5)),
                                        ))
                                        .with_children(|progress_bg| {
                                            progress_bg.spawn((
                                                Node {
                                                    width: Val::Percent(taming.progress * 100.0),
                                                    height: Val::Percent(100.0),
                                                    ..default()
                                                },
                                                BackgroundColor(Color::srgb(0.3, 0.7, 1.0)),
                                            ));
                                        });
                                });
                        }
                    }
                }
            } else {
                // Remove progress bar if no taming in progress
                for entity in progress_bar_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }

    // ============================================================================
    // QUEST UI SYSTEM
    // ============================================================================
    pub mod quest_ui {
        use bevy::prelude::*;
        use bevy_shaman_story::systems::quest_system::{QuestLog, QuestRegistry};

        #[derive(Component)]
        pub struct QuestLogUI;

        #[derive(Component)]
        pub struct QuestTrackerUI;

        #[derive(Resource)]
        pub struct QuestUIState {
            pub log_visible: bool,
        }

        impl Default for QuestUIState {
            fn default() -> Self {
                Self { log_visible: false }
            }
        }

        /// Display quest log panel (toggle with Q key)
        pub fn display_quest_log(
            mut commands: Commands,
            keyboard: Res<ButtonInput<KeyCode>>,
            mut ui_state: ResMut<QuestUIState>,
            quest_log: Res<QuestLog>,
            quest_registry: Res<QuestRegistry>,
            log_ui_query: Query<Entity, With<QuestLogUI>>,
        ) {
            // Toggle with Q key
            if keyboard.just_pressed(KeyCode::KeyQ) {
                ui_state.log_visible = !ui_state.log_visible;
            }

            // Hide if not visible
            if !ui_state.log_visible {
                for entity in log_ui_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Only spawn UI once
            if !log_ui_query.is_empty() {
                return;
            }

            // Spawn quest log UI
            commands
                .spawn((
                    QuestLogUI,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(20.0),
                        top: Val::Percent(10.0),
                        width: Val::Px(600.0),
                        height: Val::Percent(80.0),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::all(Val::Px(20.0)),
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.08, 0.12, 0.95)),
                    BorderColor(Color::srgb(0.6, 0.7, 0.3)),
                ))
                .with_children(|parent| {
                    // Header
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.2, 0.25, 0.15, 0.8)),
                        ))
                        .with_children(|header| {
                            header.spawn((
                                Text::new("QUEST LOG"),
                                TextFont {
                                    font_size: 28.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.5)),
                            ));

                            header.spawn((
                                Text::new(format!(
                                    "\nActive: {} | Completed: {}",
                                    quest_log.active_quests.len(),
                                    quest_log.completed_quests.len()
                                )),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                            ));
                        });

                    // Active quests section
                    if !quest_log.active_quests.is_empty() {
                        parent.spawn((
                            Text::new("=== ACTIVE QUESTS ==="),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.7, 0.3)),
                            Node {
                                margin: UiRect::vertical(Val::Px(10.0)),
                                ..default()
                            },
                        ));

                        for quest_id in &quest_log.active_quests {
                            if let Some(quest) = quest_registry.get(quest_id) {
                                let is_tracked = quest_log.tracked_quest.as_ref() == Some(quest_id);

                                parent
                                    .spawn((
                                        Node {
                                            width: Val::Percent(100.0),
                                            padding: UiRect::all(Val::Px(12.0)),
                                            margin: UiRect::bottom(Val::Px(8.0)),
                                            flex_direction: FlexDirection::Column,
                                            row_gap: Val::Px(5.0),
                                            border: UiRect::all(Val::Px(2.0)),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgba(0.15, 0.2, 0.1, 0.8)),
                                        BorderColor(if is_tracked {
                                            Color::srgb(0.9, 0.7, 0.3)
                                        } else {
                                            Color::srgb(0.3, 0.4, 0.2)
                                        }),
                                    ))
                                    .with_children(|quest_box| {
                                        // Quest title
                                        quest_box.spawn((
                                            Text::new(if is_tracked {
                                                format!("[TRACKED] {}", quest.title)
                                            } else {
                                                quest.title.clone()
                                            }),
                                            TextFont {
                                                font_size: 18.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.9, 0.9, 0.6)),
                                        ));

                                        // Quest description
                                        quest_box.spawn((
                                            Text::new(&quest.description),
                                            TextFont {
                                                font_size: 14.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                            Node {
                                                margin: UiRect::vertical(Val::Px(5.0)),
                                                ..default()
                                            },
                                        ));

                                        // Objectives
                                        for (_idx, objective) in quest.objectives.iter().enumerate()
                                        {
                                            let icon = if objective.is_complete() {
                                                "✓"
                                            } else {
                                                "○"
                                            };
                                            let color = if objective.is_complete() {
                                                Color::srgb(0.3, 0.9, 0.3)
                                            } else {
                                                Color::srgb(0.9, 0.9, 0.9)
                                            };

                                            quest_box.spawn((
                                                Text::new(format!(
                                                    "  {} {} [{}]",
                                                    icon,
                                                    objective.description,
                                                    objective.progress_text()
                                                )),
                                                TextFont {
                                                    font_size: 14.0,
                                                    ..default()
                                                },
                                                TextColor(color),
                                            ));
                                        }

                                        // Rewards
                                        quest_box.spawn((
                                            Text::new(format!(
                                                "\nRewards: {}",
                                                quest.rewards.rewards_text()
                                            )),
                                            TextFont {
                                                font_size: 12.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.9, 0.7, 0.3)),
                                            Node {
                                                margin: UiRect::top(Val::Px(5.0)),
                                                ..default()
                                            },
                                        ));
                                    });
                            }
                        }
                    } else {
                        parent.spawn((
                            Text::new("No active quests"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                        ));
                    }

                    // Footer
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgba(0.15, 0.2, 0.15, 0.8)),
                        ))
                        .with_children(|footer| {
                            footer.spawn((
                                Text::new("Press [Q] to close"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.7, 0.6)),
                            ));
                        });
                });
        }

        /// Display on-screen quest tracker (always visible for tracked quest)
        pub fn display_quest_tracker(
            mut commands: Commands,
            quest_log: Res<QuestLog>,
            quest_registry: Res<QuestRegistry>,
            tracker_ui_query: Query<Entity, With<QuestTrackerUI>>,
        ) {
            // Get tracked quest
            let tracked_quest_id = match &quest_log.tracked_quest {
                Some(id) if quest_log.active_quests.contains(id) => id,
                _ => {
                    // No tracked quest, clean up UI
                    for entity in tracker_ui_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    return;
                }
            };

            let quest = match quest_registry.get(tracked_quest_id) {
                Some(q) => q,
                None => {
                    for entity in tracker_ui_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    return;
                }
            };

            // Clean up old tracker
            for entity in tracker_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            // Spawn quest tracker UI (top-right corner)
            commands
                .spawn((
                    QuestTrackerUI,
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(120.0),
                        right: Val::Px(10.0),
                        width: Val::Px(350.0),
                        height: Val::Auto,
                        padding: UiRect::all(Val::Px(15.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(5.0),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.12, 0.08, 0.85)),
                    BorderColor(Color::srgb(0.7, 0.6, 0.3)),
                ))
                .with_children(|parent| {
                    // Quest title
                    parent.spawn((
                        Text::new(&quest.title),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.8, 0.4)),
                        Node {
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        },
                    ));

                    // Current objective (first incomplete)
                    if let Some(current_obj) = quest.get_current_objective() {
                        parent.spawn((
                            Text::new(format!("○ {}", current_obj.description)),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));

                        parent.spawn((
                            Text::new(format!("   Progress: {}", current_obj.progress_text())),
                            TextFont {
                                font_size: 13.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.9, 0.7)),
                        ));

                        // Location hint if available
                        if let Some(hint) = &quest.location_hint {
                            parent.spawn((
                                Text::new(format!("   Location: {}", hint)),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.7, 0.9)),
                                Node {
                                    margin: UiRect::top(Val::Px(5.0)),
                                    ..default()
                                },
                            ));
                        }
                    } else {
                        // All objectives complete
                        parent.spawn((
                            Text::new("✓ Return to quest giver"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.3, 0.9, 0.3)),
                        ));
                    }

                    // Overall progress
                    parent.spawn((
                        Text::new(format!("\n{}", quest.progress_summary())),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                    ));
                });
        }
    }

    // ============================================================================
    // QUICK WIN FEATURES (Death Screen, Pause Menu, Combo Counter, Settings)
    // ============================================================================
    pub mod quick_wins {
        use bevy::prelude::*;
        use bevy_shaman_combat::components::RhythmCombo;
        use bevy_shaman_core::components::{Health, Player};
        use bevy_shaman_core::states::GameState;

        #[derive(Component)]
        pub struct PauseMenuUI;

        #[derive(Component)]
        pub struct DeathScreenUI;

        #[derive(Component)]
        pub struct ComboCounterUI;

        #[derive(Component)]
        pub struct SettingsPanelUI;

        // Settings UI component markers
        #[derive(Component)]
        pub struct MasterVolumeSlider;

        #[derive(Component)]
        pub struct MusicVolumeSlider;

        #[derive(Component)]
        pub struct SfxVolumeSlider;

        #[derive(Component)]
        pub struct DifficultyButton;

        #[derive(Component)]
        pub struct ControlSchemeButton;

        #[derive(Component)]
        pub struct GamepadToggleButton;

        #[derive(Component)]
        pub struct DeadzoneSlider;

        #[derive(Component)]
        pub struct ScreenShakeToggle;

        #[derive(Component)]
        pub struct DamageNumbersToggle;

        #[derive(Component)]
        pub struct AutoSaveToggle;

        #[derive(Component)]
        pub struct SettingsCloseButton;

        #[derive(Component)]
        pub struct SettingLabel(pub String);

        #[derive(Component)]
        pub struct ResumeButton;

        #[derive(Component)]
        pub struct EnhancementMenuButton;

        #[derive(Component)]
        pub struct SkillTreeMenuButton;

        #[derive(Component)]
        pub struct QuitButton;

        #[derive(Component)]
        pub struct RespawnButton;

        #[derive(Resource)]
        pub struct PauseMenuState {
            pub paused: bool,
        }

        impl Default for PauseMenuState {
            fn default() -> Self {
                Self { paused: false }
            }
        }

        #[derive(Resource)]
        pub struct DeathScreenState {
            pub player_dead: bool,
        }

        impl Default for DeathScreenState {
            fn default() -> Self {
                Self { player_dead: false }
            }
        }

        #[derive(Resource)]
        pub struct SettingsUIState {
            pub visible: bool,
        }

        impl Default for SettingsUIState {
            fn default() -> Self {
                Self { visible: false }
            }
        }

        /// Display pause menu (toggle with ESC key)
        pub fn display_pause_menu(
            mut commands: Commands,
            keyboard: Res<ButtonInput<KeyCode>>,
            mut pause_state: ResMut<PauseMenuState>,
            death_state: Res<DeathScreenState>,
            pause_ui_query: Query<Entity, With<PauseMenuUI>>,
            button_query: Query<
                (
                    &Interaction,
                    Option<&ResumeButton>,
                    Option<&EnhancementMenuButton>,
                    Option<&SkillTreeMenuButton>,
                    Option<&QuitButton>,
                ),
                (Changed<Interaction>, With<Button>),
            >,
            mut enhancement_ui_state: ResMut<crate::enhancement_ui::EnhancementUIState>,
            mut skill_tree_ui_state: ResMut<crate::skill_tree_ui::SkillTreeUIState>,
            mut next_state: ResMut<NextState<GameState>>,
        ) {
            // Don't show pause menu if player is dead
            if death_state.player_dead {
                pause_state.paused = false;
                for entity in pause_ui_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Toggle with ESC key
            if keyboard.just_pressed(KeyCode::Escape) {
                pause_state.paused = !pause_state.paused;
            }

            // Hide if not paused
            if !pause_state.paused {
                for entity in pause_ui_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Only spawn UI once
            if !pause_ui_query.is_empty() {
                // Handle button clicks
                for (
                    interaction,
                    resume_button,
                    enhancement_button,
                    skill_tree_button,
                    quit_button,
                ) in button_query.iter()
                {
                    if *interaction == Interaction::Pressed {
                        if resume_button.is_some() {
                            pause_state.paused = false;
                            for entity in pause_ui_query.iter() {
                                commands.entity(entity).despawn_recursive();
                            }
                        } else if enhancement_button.is_some() {
                            // Open Enhancement UI and close pause menu
                            enhancement_ui_state.visible = true;
                            pause_state.paused = false;
                            for entity in pause_ui_query.iter() {
                                commands.entity(entity).despawn_recursive();
                            }
                        } else if skill_tree_button.is_some() {
                            // Open Skill Tree UI and close pause menu
                            skill_tree_ui_state.visible = true;
                            pause_state.paused = false;
                            for entity in pause_ui_query.iter() {
                                commands.entity(entity).despawn_recursive();
                            }
                        } else if quit_button.is_some() {
                            next_state.set(GameState::MainMenu);
                            pause_state.paused = false;
                            for entity in pause_ui_query.iter() {
                                commands.entity(entity).despawn_recursive();
                            }
                        }
                    }
                }
                return;
            }

            // Spawn pause menu UI
            commands
                .spawn((
                    PauseMenuUI,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(400.0),
                                height: Val::Auto,
                                padding: UiRect::all(Val::Px(40.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(20.0),
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(3.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
                            BorderColor(Color::srgb(0.8, 0.2, 0.2)),
                        ))
                        .with_children(|menu| {
                            // Title
                            menu.spawn((
                                Text::new("PAUSED"),
                                TextFont {
                                    font_size: 48.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.3, 0.3)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(20.0)),
                                    ..default()
                                },
                            ));

                            // Resume button
                            menu.spawn((
                                ResumeButton,
                                Button,
                                Node {
                                    width: Val::Px(300.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                                BorderColor(Color::srgb(0.6, 0.2, 0.2)),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new("RESUME"),
                                    TextFont {
                                        font_size: 24.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });

                            // Enhancement button
                            menu.spawn((
                                EnhancementMenuButton,
                                Button,
                                Node {
                                    width: Val::Px(300.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.4, 0.25, 0.15)),
                                BorderColor(Color::srgb(0.8, 0.5, 0.2)),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new("SPIRIT FORGE (H)"),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });

                            // Skill Tree button
                            menu.spawn((
                                SkillTreeMenuButton,
                                Button,
                                Node {
                                    width: Val::Px(300.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.25, 0.35, 0.25)),
                                BorderColor(Color::srgb(0.4, 0.6, 0.3)),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new("SKILL TREE (K)"),
                                    TextFont {
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });

                            // Quit button
                            menu.spawn((
                                QuitButton,
                                Button,
                                Node {
                                    width: Val::Px(300.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                                BorderColor(Color::srgb(0.6, 0.2, 0.2)),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new("QUIT TO MENU"),
                                    TextFont {
                                        font_size: 24.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                        });
                });
        }

        /// Display death screen
        pub fn display_death_screen(
            mut commands: Commands,
            mut death_state: ResMut<DeathScreenState>,
            player_query: Query<&Health, With<Player>>,
            death_ui_query: Query<Entity, With<DeathScreenUI>>,
            button_query: Query<
                (&Interaction, &RespawnButton),
                (Changed<Interaction>, With<Button>),
            >,
        ) {
            // Check if player is dead
            if let Ok(health) = player_query.get_single() {
                death_state.player_dead = health.current <= 0.0;
            } else {
                death_state.player_dead = false;
            }

            // Hide if not dead
            if !death_state.player_dead {
                for entity in death_ui_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Only spawn UI once
            if !death_ui_query.is_empty() {
                // Handle respawn button
                for (interaction, _) in button_query.iter() {
                    if *interaction == Interaction::Pressed {
                        // TODO: Implement respawn logic
                        info!("Respawn requested");
                        death_state.player_dead = false;
                        for entity in death_ui_query.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                }
                return;
            }

            // Spawn death screen UI
            commands
                .spawn((
                    DeathScreenUI,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.0, 0.0, 0.8)),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(500.0),
                                height: Val::Auto,
                                padding: UiRect::all(Val::Px(50.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(30.0),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.05, 0.05)),
                            BorderColor(Color::srgb(0.8, 0.1, 0.1)),
                        ))
                        .with_children(|menu| {
                            // Title
                            menu.spawn((
                                Text::new("YOU DIED"),
                                TextFont {
                                    font_size: 64.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(1.0, 0.2, 0.2)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(20.0)),
                                    ..default()
                                },
                            ));

                            // Flavor text
                            menu.spawn((
                                Text::new("The spirits mourn your passing..."),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.7, 0.5, 0.5)),
                                Node {
                                    margin: UiRect::bottom(Val::Px(20.0)),
                                    ..default()
                                },
                            ));

                            // Respawn button
                            menu.spawn((
                                RespawnButton,
                                Button,
                                Node {
                                    width: Val::Px(300.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
                                BorderColor(Color::srgb(0.8, 0.3, 0.3)),
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new("RESPAWN"),
                                    TextFont {
                                        font_size: 28.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                        });
                });
        }

        /// Display combo counter on screen
        pub fn display_combo_counter(
            mut commands: Commands,
            player_query: Query<&RhythmCombo, With<Player>>,
            combo_ui_query: Query<Entity, With<ComboCounterUI>>,
        ) {
            // Get player combo
            let combo = if let Ok(combo) = player_query.get_single() {
                combo.current_combo.len()
            } else {
                0
            };

            // Hide if no combo
            if combo == 0 {
                for entity in combo_ui_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Clean up old UI
            for entity in combo_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            // Determine size and color based on combo count
            let (font_size, color) = if combo >= 20 {
                (64.0, Color::srgb(1.0, 0.2, 1.0)) // Huge purple
            } else if combo >= 10 {
                (48.0, Color::srgb(1.0, 0.5, 0.2)) // Large orange
            } else {
                (36.0, Color::srgb(1.0, 1.0, 0.3)) // Normal yellow
            };

            // Spawn combo counter UI
            commands
                .spawn((
                    ComboCounterUI,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(45.0),
                        top: Val::Px(100.0),
                        padding: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(format!("{} COMBO!", combo)),
                        TextFont {
                            font_size,
                            ..default()
                        },
                        TextColor(color),
                    ));
                });
        }

        /// Display settings panel
        pub fn display_settings_panel(
            mut commands: Commands,
            keyboard: Res<ButtonInput<KeyCode>>,
            mut settings_state: ResMut<SettingsUIState>,
            settings_ui_query: Query<Entity, With<SettingsPanelUI>>,
            settings: Res<bevy_shaman_core::settings::GameSettings>,
        ) {
            // Toggle with F1 key
            if keyboard.just_pressed(KeyCode::F1) {
                settings_state.visible = !settings_state.visible;
            }

            // Hide if not visible
            if !settings_state.visible {
                for entity in settings_ui_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Only spawn UI once
            if !settings_ui_query.is_empty() {
                return;
            }

            // Spawn settings panel
            commands
                .spawn((
                    SettingsPanelUI,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(15.0),
                        top: Val::Percent(10.0),
                        width: Val::Px(700.0),
                        height: Val::Auto,
                        max_height: Val::Percent(80.0),
                        padding: UiRect::all(Val::Px(30.0)),
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(12.0),
                        border: UiRect::all(Val::Px(2.0)),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.1, 0.1, 0.12, 0.95)),
                    BorderColor(Color::srgb(0.5, 0.6, 0.7)),
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn((
                        Text::new("SETTINGS"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 1.0)),
                        Node {
                            margin: UiRect::bottom(Val::Px(15.0)),
                            ..default()
                        },
                    ));

                    // === AUDIO SECTION ===
                    parent.spawn((
                        Text::new("=== AUDIO ==="),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.8, 0.9)),
                        Node {
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                    ));

                    // Master Volume
                    spawn_slider_setting(
                        parent,
                        "Master Volume",
                        settings.audio.master_volume,
                        MasterVolumeSlider,
                    );
                    spawn_slider_setting(
                        parent,
                        "Music Volume",
                        settings.audio.music_volume,
                        MusicVolumeSlider,
                    );
                    spawn_slider_setting(
                        parent,
                        "SFX Volume",
                        settings.audio.sfx_volume,
                        SfxVolumeSlider,
                    );

                    // === CONTROLS SECTION ===
                    parent.spawn((
                        Text::new("=== CONTROLS ==="),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.8, 0.9)),
                        Node {
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                    ));

                    // Control Scheme
                    spawn_button_setting(
                        parent,
                        "Control Scheme",
                        settings.controls.scheme.name(),
                        ControlSchemeButton,
                    );

                    // Gamepad Enabled
                    spawn_toggle_setting(
                        parent,
                        "Gamepad Enabled",
                        settings.controls.gamepad_enabled,
                        GamepadToggleButton,
                    );

                    // Deadzone
                    spawn_percentage_slider(
                        parent,
                        "Gamepad Deadzone",
                        settings.controls.gamepad_deadzone,
                        DeadzoneSlider,
                    );

                    // === GAMEPLAY SECTION ===
                    parent.spawn((
                        Text::new("=== GAMEPLAY ==="),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.8, 0.9)),
                        Node {
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                    ));

                    // Difficulty
                    let difficulty_text = match settings.gameplay.difficulty {
                        bevy_shaman_core::settings::Difficulty::Easy => "Easy",
                        bevy_shaman_core::settings::Difficulty::Normal => "Normal",
                        bevy_shaman_core::settings::Difficulty::Hard => "Hard",
                    };
                    spawn_button_setting(parent, "Difficulty", difficulty_text, DifficultyButton);

                    // Gameplay toggles
                    spawn_toggle_setting(
                        parent,
                        "Show Damage Numbers",
                        settings.gameplay.show_damage_numbers,
                        DamageNumbersToggle,
                    );
                    spawn_toggle_setting(
                        parent,
                        "Screen Shake",
                        settings.gameplay.screen_shake,
                        ScreenShakeToggle,
                    );
                    spawn_toggle_setting(
                        parent,
                        "Auto Save",
                        settings.gameplay.auto_save,
                        AutoSaveToggle,
                    );

                    // === CONTROLS INFO ===
                    parent.spawn((
                        Text::new("=== CONTROLS INFO ==="),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.8, 0.9)),
                        Node {
                            margin: UiRect::vertical(Val::Px(10.0)),
                            ..default()
                        },
                    ));

                    parent.spawn((
                        Text::new(format!(
                            "Movement: {} keys",
                            settings.controls.scheme.name()
                        )),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));

                    parent.spawn((
                        Text::new("Gamepad: Left Stick/D-Pad to move, RB/RT to dash"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));

                    // Footer
                    parent.spawn((
                        Text::new("\nPress F1 to close • Use buttons to change settings"),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.6)),
                        Node {
                            margin: UiRect::top(Val::Px(15.0)),
                            ..default()
                        },
                    ));
                });
        }

        /// Helper to spawn a slider setting row
        fn spawn_slider_setting<T: Component>(
            parent: &mut ChildBuilder,
            label: &str,
            value: f32,
            marker: T,
        ) {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    margin: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Label
                    row.spawn((
                        Text::new(format!("{}: {:.0}%", label, value * 100.0)),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        SettingLabel(label.to_string()),
                    ));

                    // Slider buttons
                    row.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(5.0),
                        ..default()
                    })
                    .with_children(|buttons| {
                        // Decrease button
                        buttons
                            .spawn((
                                marker,
                                Button,
                                Node {
                                    width: Val::Px(100.0),
                                    height: Val::Px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("◄"),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                    });
                });
        }

        /// Helper to spawn a percentage slider (0-100)
        fn spawn_percentage_slider<T: Component>(
            parent: &mut ChildBuilder,
            label: &str,
            value: u8,
            marker: T,
        ) {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    margin: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Label
                    row.spawn((
                        Text::new(format!("{}: {}%", label, value)),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        SettingLabel(label.to_string()),
                    ));

                    // Slider buttons
                    row.spawn(Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(5.0),
                        ..default()
                    })
                    .with_children(|buttons| {
                        buttons
                            .spawn((
                                marker,
                                Button,
                                Node {
                                    width: Val::Px(100.0),
                                    height: Val::Px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new("◄ / ►"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                    });
                });
        }

        /// Helper to spawn a button setting row
        fn spawn_button_setting<T: Component>(
            parent: &mut ChildBuilder,
            label: &str,
            value: &str,
            marker: T,
        ) {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    margin: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Label
                    row.spawn((
                        Text::new(format!("{}: {}", label, value)),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        SettingLabel(label.to_string()),
                    ));

                    // Change button
                    row.spawn((
                        marker,
                        Button,
                        Node {
                            width: Val::Px(100.0),
                            height: Val::Px(30.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.3, 0.4)),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("Change"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
                });
        }

        /// Helper to spawn a toggle setting row
        fn spawn_toggle_setting<T: Component>(
            parent: &mut ChildBuilder,
            label: &str,
            value: bool,
            marker: T,
        ) {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    margin: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                })
                .with_children(|row| {
                    // Label
                    row.spawn((
                        Text::new(format!("{}: {}", label, if value { "ON" } else { "OFF" })),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        SettingLabel(label.to_string()),
                    ));

                    // Toggle button
                    row.spawn((
                        marker,
                        Button,
                        Node {
                            width: Val::Px(100.0),
                            height: Val::Px(30.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(if value {
                            Color::srgb(0.2, 0.6, 0.3)
                        } else {
                            Color::srgb(0.6, 0.2, 0.2)
                        }),
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("Toggle"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
                });
        }

        /// Handle settings interactions
        pub fn handle_settings_interactions(
            mut settings: ResMut<bevy_shaman_core::settings::GameSettings>,
            mut interaction_query: Query<
                (
                    &Interaction,
                    Option<&MasterVolumeSlider>,
                    Option<&MusicVolumeSlider>,
                    Option<&SfxVolumeSlider>,
                    Option<&DifficultyButton>,
                    Option<&ControlSchemeButton>,
                    Option<&GamepadToggleButton>,
                    Option<&DeadzoneSlider>,
                    Option<&ScreenShakeToggle>,
                    Option<&DamageNumbersToggle>,
                    Option<&AutoSaveToggle>,
                ),
                (Changed<Interaction>, With<Button>),
            >,
            mut label_query: Query<(&mut Text, &SettingLabel)>,
            mut button_query: Query<(
                &mut BackgroundColor,
                Option<&GamepadToggleButton>,
                Option<&ScreenShakeToggle>,
                Option<&DamageNumbersToggle>,
                Option<&AutoSaveToggle>,
            )>,
        ) {
            for (
                interaction,
                master_vol,
                music_vol,
                sfx_vol,
                difficulty,
                control_scheme,
                gamepad_toggle,
                deadzone,
                screen_shake,
                damage_numbers,
                auto_save,
            ) in interaction_query.iter()
            {
                if *interaction != Interaction::Pressed {
                    continue;
                }

                // Handle volume sliders
                if master_vol.is_some() {
                    settings.audio.master_volume = (settings.audio.master_volume - 0.1).max(0.0);
                    update_label(
                        &mut label_query,
                        "Master Volume",
                        &format!("{:.0}%", settings.audio.master_volume * 100.0),
                    );
                }
                if music_vol.is_some() {
                    settings.audio.music_volume = (settings.audio.music_volume - 0.1).max(0.0);
                    update_label(
                        &mut label_query,
                        "Music Volume",
                        &format!("{:.0}%", settings.audio.music_volume * 100.0),
                    );
                }
                if sfx_vol.is_some() {
                    settings.audio.sfx_volume = (settings.audio.sfx_volume - 0.1).max(0.0);
                    update_label(
                        &mut label_query,
                        "SFX Volume",
                        &format!("{:.0}%", settings.audio.sfx_volume * 100.0),
                    );
                }

                // Handle difficulty
                if difficulty.is_some() {
                    settings.gameplay.difficulty = match settings.gameplay.difficulty {
                        bevy_shaman_core::settings::Difficulty::Easy => {
                            bevy_shaman_core::settings::Difficulty::Normal
                        }
                        bevy_shaman_core::settings::Difficulty::Normal => {
                            bevy_shaman_core::settings::Difficulty::Hard
                        }
                        bevy_shaman_core::settings::Difficulty::Hard => {
                            bevy_shaman_core::settings::Difficulty::Easy
                        }
                    };
                    let difficulty_text = match settings.gameplay.difficulty {
                        bevy_shaman_core::settings::Difficulty::Easy => "Easy",
                        bevy_shaman_core::settings::Difficulty::Normal => "Normal",
                        bevy_shaman_core::settings::Difficulty::Hard => "Hard",
                    };
                    update_label(&mut label_query, "Difficulty", difficulty_text);
                }

                // Handle control scheme
                if control_scheme.is_some() {
                    settings.controls.scheme = settings.controls.scheme.next();
                    update_label(
                        &mut label_query,
                        "Control Scheme",
                        settings.controls.scheme.name(),
                    );
                }

                // Handle gamepad toggle
                if gamepad_toggle.is_some() {
                    settings.controls.gamepad_enabled = !settings.controls.gamepad_enabled;
                    update_label(
                        &mut label_query,
                        "Gamepad Enabled",
                        if settings.controls.gamepad_enabled {
                            "ON"
                        } else {
                            "OFF"
                        },
                    );

                    // Update button color
                    for (mut bg_color, is_gamepad, _, _, _) in button_query.iter_mut() {
                        if is_gamepad.is_some() {
                            *bg_color = if settings.controls.gamepad_enabled {
                                BackgroundColor(Color::srgb(0.2, 0.6, 0.3))
                            } else {
                                BackgroundColor(Color::srgb(0.6, 0.2, 0.2))
                            };
                        }
                    }
                }

                // Handle deadzone
                if deadzone.is_some() {
                    settings.controls.gamepad_deadzone =
                        ((settings.controls.gamepad_deadzone + 5) % 55).max(5);
                    update_label(
                        &mut label_query,
                        "Gamepad Deadzone",
                        &format!("{}%", settings.controls.gamepad_deadzone),
                    );
                }

                // Handle screen shake toggle
                if screen_shake.is_some() {
                    settings.gameplay.screen_shake = !settings.gameplay.screen_shake;
                    update_label(
                        &mut label_query,
                        "Screen Shake",
                        if settings.gameplay.screen_shake {
                            "ON"
                        } else {
                            "OFF"
                        },
                    );

                    for (mut bg_color, _, is_shake, _, _) in button_query.iter_mut() {
                        if is_shake.is_some() {
                            *bg_color = if settings.gameplay.screen_shake {
                                BackgroundColor(Color::srgb(0.2, 0.6, 0.3))
                            } else {
                                BackgroundColor(Color::srgb(0.6, 0.2, 0.2))
                            };
                        }
                    }
                }

                // Handle damage numbers toggle
                if damage_numbers.is_some() {
                    settings.gameplay.show_damage_numbers = !settings.gameplay.show_damage_numbers;
                    update_label(
                        &mut label_query,
                        "Show Damage Numbers",
                        if settings.gameplay.show_damage_numbers {
                            "ON"
                        } else {
                            "OFF"
                        },
                    );

                    for (mut bg_color, _, _, is_damage, _) in button_query.iter_mut() {
                        if is_damage.is_some() {
                            *bg_color = if settings.gameplay.show_damage_numbers {
                                BackgroundColor(Color::srgb(0.2, 0.6, 0.3))
                            } else {
                                BackgroundColor(Color::srgb(0.6, 0.2, 0.2))
                            };
                        }
                    }
                }

                // Handle auto save toggle
                if auto_save.is_some() {
                    settings.gameplay.auto_save = !settings.gameplay.auto_save;
                    update_label(
                        &mut label_query,
                        "Auto Save",
                        if settings.gameplay.auto_save {
                            "ON"
                        } else {
                            "OFF"
                        },
                    );

                    for (mut bg_color, _, _, _, is_auto_save) in button_query.iter_mut() {
                        if is_auto_save.is_some() {
                            *bg_color = if settings.gameplay.auto_save {
                                BackgroundColor(Color::srgb(0.2, 0.6, 0.3))
                            } else {
                                BackgroundColor(Color::srgb(0.6, 0.2, 0.2))
                            };
                        }
                    }
                }

                // Save settings after any change
                settings.save();
            }
        }

        fn update_label(
            label_query: &mut Query<(&mut Text, &SettingLabel)>,
            setting_name: &str,
            new_value: &str,
        ) {
            for (mut text, label) in label_query.iter_mut() {
                if label.0 == setting_name {
                    **text = format!("{}: {}", setting_name, new_value);
                }
            }
        }
    }

    pub mod calendar_ui {
        use bevy::prelude::*;
        use bevy_shaman_core::resources::{CalendarVisible, GameCalendar};

        #[derive(Component)]
        pub struct CalendarUIRoot;

        #[derive(Component)]
        pub struct CalendarTimeText;

        #[derive(Component)]
        pub struct CalendarDateText;

        #[derive(Component)]
        pub struct CalendarFestivalText;

        #[derive(Component)]
        pub struct CalendarToggleButton;

        pub fn display_calendar(
            mut commands: Commands,
            keyboard: Res<ButtonInput<KeyCode>>,
            mut calendar_visible: ResMut<CalendarVisible>,
            ui_root_query: Query<Entity, With<CalendarUIRoot>>,
            calendar: Res<GameCalendar>,
            mut time_text_query: Query<
                &mut Text,
                (
                    With<CalendarTimeText>,
                    Without<CalendarDateText>,
                    Without<CalendarFestivalText>,
                ),
            >,
            mut date_text_query: Query<
                &mut Text,
                (
                    With<CalendarDateText>,
                    Without<CalendarTimeText>,
                    Without<CalendarFestivalText>,
                ),
            >,
            _festival_text_query: Query<
                &mut Text,
                (
                    With<CalendarFestivalText>,
                    Without<CalendarTimeText>,
                    Without<CalendarDateText>,
                ),
            >,
        ) {
            // Toggle visibility with C key
            if keyboard.just_pressed(KeyCode::KeyC) {
                calendar_visible.0 = !calendar_visible.0;
            }

            // Despawn UI if not visible
            if !calendar_visible.0 {
                for entity in ui_root_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                return;
            }

            // Spawn UI if visible and doesn't exist
            if ui_root_query.is_empty() {
                commands
                    .spawn((
                        CalendarUIRoot,
                        Node {
                            position_type: PositionType::Absolute,
                            right: Val::Px(10.0),
                            top: Val::Px(10.0),
                            width: Val::Px(350.0),
                            height: Val::Auto,
                            flex_direction: FlexDirection::Column,
                            padding: UiRect::all(Val::Px(15.0)),
                            row_gap: Val::Px(10.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
                        BorderColor(Color::srgb(0.5, 0.4, 0.3)),
                        BorderRadius::all(Val::Px(8.0)),
                    ))
                    .with_children(|parent| {
                        // Title
                        parent.spawn((
                            Text::new("Calendar & Time"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.8, 0.6)),
                            Node {
                                margin: UiRect::bottom(Val::Px(5.0)),
                                ..default()
                            },
                        ));

                        // Time display
                        parent
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Auto,
                                    padding: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.2, 0.2, 0.3, 0.5)),
                                BorderRadius::all(Val::Px(5.0)),
                            ))
                            .with_children(|time_panel| {
                                time_panel.spawn((
                                    CalendarTimeText,
                                    Text::new(format!("Time: {}", calendar.time_string())),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 1.0)),
                                ));
                            });

                        // Date display
                        parent
                            .spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Auto,
                                    padding: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.2, 0.3, 0.2, 0.5)),
                                BorderRadius::all(Val::Px(5.0)),
                            ))
                            .with_children(|date_panel| {
                                date_panel.spawn((
                                    CalendarDateText,
                                    Text::new(format!("Date: {}", calendar.date_string())),
                                    TextFont {
                                        font_size: 16.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 1.0, 0.9)),
                                ));
                            });

                        // Festival display (if active)
                        if let Some(festival) = calendar.get_active_festival() {
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Auto,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        flex_direction: FlexDirection::Column,
                                        row_gap: Val::Px(5.0),
                                        ..default()
                                    },
                                    BackgroundColor(Color::srgba(0.5, 0.3, 0.1, 0.7)),
                                    BorderRadius::all(Val::Px(5.0)),
                                ))
                                .with_children(|festival_panel| {
                                    festival_panel.spawn((
                                        Text::new(format!("🎉 Festival: {}", festival.name)),
                                        TextFont {
                                            font_size: 16.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(1.0, 0.9, 0.6)),
                                    ));
                                    festival_panel.spawn((
                                        CalendarFestivalText,
                                        Text::new(&festival.description),
                                        TextFont {
                                            font_size: 13.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.9, 0.9, 0.8)),
                                    ));
                                });
                        }

                        // Instructions
                        parent.spawn((
                            Text::new("\nPress C to close"),
                            TextFont {
                                font_size: 12.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.6, 0.6, 0.6)),
                            Node {
                                margin: UiRect::top(Val::Px(5.0)),
                                ..default()
                            },
                        ));
                    });
            } else {
                // Update existing UI text
                for mut text in time_text_query.iter_mut() {
                    **text = format!("Time: {}", calendar.time_string());
                }

                for mut text in date_text_query.iter_mut() {
                    **text = format!("Date: {}", calendar.date_string());
                }
            }
        }

        pub fn display_calendar_button(
            mut commands: Commands,
            button_query: Query<Entity, With<CalendarToggleButton>>,
            mut calendar_visible: ResMut<CalendarVisible>,
            interaction_query: Query<(&Interaction, &CalendarToggleButton), Changed<Interaction>>,
        ) {
            // Spawn button if it doesn't exist
            if button_query.is_empty() {
                commands
                    .spawn((
                        CalendarToggleButton,
                        Button,
                        Node {
                            position_type: PositionType::Absolute,
                            right: Val::Px(10.0),
                            bottom: Val::Px(10.0),
                            width: Val::Px(50.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.3, 0.3, 0.4, 0.9)),
                        BorderColor(Color::srgb(0.5, 0.5, 0.6)),
                        BorderRadius::all(Val::Px(8.0)),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("📅"),
                            TextFont {
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }

            // Handle button clicks
            for (interaction, _) in interaction_query.iter() {
                if *interaction == Interaction::Pressed {
                    calendar_visible.0 = !calendar_visible.0;
                }
            }
        }
    }
}
