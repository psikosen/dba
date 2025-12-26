use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<systems::bestiary::BestiaryVisible>()
            .init_resource::<systems::loading_screen::VideoIntroTimer>()
            .init_resource::<systems::shop_ui::ShopVisible>()
            .add_systems(Update, (
                systems::hud::update_hud,
                systems::rhythm_ui::display_rhythm_visualizer,
                systems::bestiary::display_bestiary,
                systems::shop_ui::display_shop,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(Update, systems::loading_screen::display_loading_screen.run_if(in_state(GameState::Boot)))
            .add_systems(Update, systems::main_menu::display_main_menu.run_if(in_state(GameState::MainMenu)));
    }
}

pub mod systems {
    pub mod hud {
        use bevy::prelude::*;
        use bevy_shaman_core::components::{Health, Spirit, Stamina, Player};

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

        pub fn update_hud(
            mut commands: Commands,
            player: Query<(&Health, &Spirit, &Stamina), With<Player>>,
            hud_root: Query<Entity, With<HudRoot>>,
            health_bars: Query<Entity, With<HealthBar>>,
            spirit_bars: Query<Entity, With<SpiritBar>>,
            stamina_bars: Query<Entity, With<StaminaBar>>,
            mut health_text: Query<&mut Text, With<HealthText>>,
            mut spirit_text: Query<&mut Text, (With<SpiritText>, Without<HealthText>, Without<StaminaText>)>,
            mut stamina_text: Query<&mut Text, (With<StaminaText>, Without<HealthText>, Without<SpiritText>)>,
        ) {
            // Initialize HUD if it doesn't exist
            if hud_root.is_empty() {
                commands.spawn((
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
                )).with_children(|parent| {
                    // Health bar
                    parent.spawn((
                        HealthBar,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(25.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.2, 0.2)),
                    )).with_children(|bar| {
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
                    parent.spawn((
                        SpiritBar,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(25.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.5, 0.9)),
                    )).with_children(|bar| {
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
                    parent.spawn((
                        StaminaBar,
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(25.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.7, 0.3)),
                    )).with_children(|bar| {
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
        }
    }

    pub mod rhythm_ui {
        use bevy::prelude::*;
        use bevy_shaman_audio::resources::{BeatClock, TimingQuality};

        #[derive(Component)]
        pub struct RhythmVisualizer;

        #[derive(Component)]
        pub struct BeatIndicator;

        #[derive(Component)]
        pub struct TimingWindow;

        pub fn display_rhythm_visualizer(
            mut commands: Commands,
            rhythm: Option<Res<BeatClock>>,
            visualizer: Query<Entity, With<RhythmVisualizer>>,
            mut beat_indicators: Query<&mut BackgroundColor, With<BeatIndicator>>,
            time: Res<Time>,
        ) {
            // Initialize visualizer if it doesn't exist
            if visualizer.is_empty() {
                commands.spawn((
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
                )).with_children(|parent| {
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
                    let mut tamed_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
                    for monster_id in tamed_monsters.iter() {
                        *tamed_counts.entry(monster_id.0.clone()).or_insert(0) += 1;
                    }

                    commands.spawn((
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
                    )).with_children(|parent| {
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
                commands.spawn((
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
                )).with_children(|parent| {
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
        ) {
            // Initialize main menu if it doesn't exist
            if menu_ui.is_empty() {
                commands.spawn((
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
                )).with_children(|parent| {
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
                    parent.spawn((
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
                    )).with_children(|button| {
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
                    parent.spawn((
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
                    )).with_children(|button| {
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
                    parent.spawn((
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
                    )).with_children(|button| {
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
                    // TODO: Implement load game functionality
                    // For now, just transition to Playing
                    for entity in menu_ui.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    next_state.set(GameState::Playing);
                }
            }

            // Handle Settings button
            for (interaction, _) in settings_interaction.iter() {
                if *interaction == Interaction::Pressed {
                    // TODO: Implement settings menu
                    // For now, do nothing
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
}
