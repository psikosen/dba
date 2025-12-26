use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<systems::bestiary::BestiaryVisible>()
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
}
