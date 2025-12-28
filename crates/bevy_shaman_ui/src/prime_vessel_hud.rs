use bevy::prelude::*;
use bevy_shaman_prime_vessel::components::{PrimeVessel, LesserSelf, ResurrectionRitual};
use bevy_shaman_prime_vessel::events::{
    VesselEvolved, VesselDefeated, LesserSelfEncountered, CorruptionIndexRevealed,
};
use bevy_shaman_prime_vessel::resources::GlobalCorruptionIndex;

use crate::ancestral_theme::*;

// ============================================================================
// CORRUPTION INDEX WIDGET
// ============================================================================

/// Root container for Corruption Index UI (top-right corner)
#[derive(Component)]
pub struct CorruptionIndexWidget;

#[derive(Component)]
pub struct CorruptionIndexTitle;

#[derive(Component)]
pub struct CorruptionPercentageText;

#[derive(Component)]
pub struct CorruptionBarTrough;

#[derive(Component)]
pub struct CorruptionBarFill;

#[derive(Component)]
pub struct CorruptionSpiritCountText;

/// Spawn the Corruption Index widget (initially hidden)
pub fn spawn_corruption_index_widget(mut commands: Commands) {
    commands
        .spawn((
            CorruptionIndexWidget,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                right: Val::Px(20.0),
                width: Val::Px(280.0),
                height: Val::Px(140.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            BackgroundColor(EBONY),
            BorderColor(BRONZE),
            Visibility::Hidden,  // Hidden until quest complete
            ZIndex(100),
        ))
        .with_children(|parent| {
            // Title: "CORRUPTION INDEX"
            parent.spawn((
                CorruptionIndexTitle,
                Text::new("CORRUPTION INDEX"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(GOLD_SHINE),
                Node {
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
            ));

            // Corruption percentage text
            parent.spawn((
                CorruptionPercentageText,
                Text::new("0.0%"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(FOREST_GREEN),
                Node {
                    margin: UiRect::bottom(Val::Px(6.0)),
                    ..default()
                },
            ));

            // Corruption bar container
            parent
                .spawn((
                    CorruptionBarTrough,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(20.0),
                        margin: UiRect::bottom(Val::Px(8.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(MAHOGANY),
                    BorderColor(BRONZE),
                ))
                .with_children(|trough| {
                    // Corruption fill (grows red as corruption increases)
                    trough.spawn((
                        CorruptionBarFill,
                        Node {
                            width: Val::Percent(0.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(FOREST_GREEN),
                    ));
                });

            // Spirit count text
            parent.spawn((
                CorruptionSpiritCountText,
                Text::new("Spirits: 15,000 / 15,000"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(TERRACOTTA),
            ));
        });
}

/// Update Corruption Index widget with current data
pub fn update_corruption_index_widget(
    corruption_index: Res<GlobalCorruptionIndex>,
    mut widget_query: Query<&mut Visibility, With<CorruptionIndexWidget>>,
    mut percentage_query: Query<&mut Text, (With<CorruptionPercentageText>, Without<CorruptionSpiritCountText>)>,
    mut fill_query: Query<(&mut Node, &mut BackgroundColor), With<CorruptionBarFill>>,
    mut spirit_count_query: Query<&mut Text, (With<CorruptionSpiritCountText>, Without<CorruptionPercentageText>)>,
) {
    // Show/hide widget based on UI reveal status
    if let Ok(mut visibility) = widget_query.get_single_mut() {
        *visibility = if corruption_index.ui_revealed {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if !corruption_index.ui_revealed {
        return;
    }

    // Update percentage text
    if let Ok(mut text) = percentage_query.get_single_mut() {
        **text = format!("{:.1}%", corruption_index.corruption_percentage * 100.0);
    }

    // Update corruption bar fill and color
    if let Ok((mut node, mut bg_color)) = fill_query.get_single_mut() {
        node.width = Val::Percent(corruption_index.corruption_percentage * 100.0);

        // Color transitions: Green -> Yellow -> Orange -> Red
        *bg_color = if corruption_index.corruption_percentage < 0.25 {
            BackgroundColor(FOREST_GREEN)
        } else if corruption_index.corruption_percentage < 0.50 {
            BackgroundColor(GOLD)
        } else if corruption_index.corruption_percentage < 0.75 {
            BackgroundColor(OCHRE_RED)
        } else {
            BackgroundColor(BLOOD_RED)
        };
    }

    // Update spirit count text
    if let Ok(mut text) = spirit_count_query.get_single_mut() {
        **text = format!(
            "Spirits: {} / {}",
            corruption_index.free_spirit_count,
            corruption_index.initial_spirit_count
        );
    }
}

/// Reveal Corruption Index UI when quest completes
pub fn handle_corruption_index_revealed(
    mut revealed_events: EventReader<CorruptionIndexRevealed>,
    mut widget_query: Query<&mut Visibility, With<CorruptionIndexWidget>>,
) {
    for _event in revealed_events.read() {
        if let Ok(mut visibility) = widget_query.get_single_mut() {
            *visibility = Visibility::Visible;
            info!("Corruption Index UI revealed!");
        }
    }
}

// ============================================================================
// DANGER LEVEL INDICATOR
// ============================================================================

/// Danger level indicator (top-left corner, always visible)
#[derive(Component)]
pub struct DangerLevelWidget;

#[derive(Component)]
pub struct DangerLevelIconText;

#[derive(Component)]
pub struct DangerLevelText;

#[derive(Component)]
pub struct VesselCountText;

/// Spawn the Danger Level widget
pub fn spawn_danger_level_widget(mut commands: Commands) {
    commands
        .spawn((
            DangerLevelWidget,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),
                width: Val::Px(220.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(EBONY.with_alpha(0.85)),
            BorderColor(BRONZE),
            ZIndex(100),
        ))
        .with_children(|parent| {
            // Danger icon + level
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(6.0)),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    row.spawn((
                        DangerLevelIconText,
                        Text::new("⚠"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(GOLD),
                        Node {
                            margin: UiRect::right(Val::Px(8.0)),
                            ..default()
                        },
                    ));

                    row.spawn((
                        DangerLevelText,
                        Text::new("DANGER: LOW"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(FOREST_GREEN),
                    ));
                });

            // Vessel count
            parent.spawn((
                VesselCountText,
                Text::new("Prime Vessel: Active\nLesser Selves: 0"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(TERRACOTTA),
            ));
        });
}

/// Update Danger Level indicator based on corruption and vessel state
pub fn update_danger_level_widget(
    corruption_index: Res<GlobalCorruptionIndex>,
    vessel_query: Query<&PrimeVessel>,
    lesser_self_query: Query<&LesserSelf>,
    mut danger_text_query: Query<(&mut Text, &mut TextColor), (With<DangerLevelText>, Without<VesselCountText>)>,
    mut vessel_count_query: Query<&mut Text, (With<VesselCountText>, Without<DangerLevelText>)>,
) {
    // Calculate danger level from corruption percentage
    let danger_level = (corruption_index.corruption_percentage * 10.0) as u8;
    let (danger_text, danger_color) = match danger_level {
        0..=2 => ("DANGER: LOW", FOREST_GREEN),
        3..=4 => ("DANGER: MODERATE", GOLD),
        5..=6 => ("DANGER: HIGH", OCHRE_RED),
        7..=8 => ("DANGER: SEVERE", Color::srgb(0.90, 0.15, 0.10)),
        9..=10 => ("DANGER: CRITICAL", BLOOD_RED),
        _ => ("DANGER: CATASTROPHIC", BLOOD_RED),
    };

    // Update danger text and color
    if let Ok((mut text, mut text_color)) = danger_text_query.get_single_mut() {
        **text = danger_text.to_string();
        *text_color = TextColor(danger_color);
    }

    // Update vessel count
    if let Ok(mut text) = vessel_count_query.get_single_mut() {
        let vessel_status = if let Ok(vessel) = vessel_query.get_single() {
            if vessel.is_defeated {
                "Defeated"
            } else {
                "Active"
            }
        } else {
            "None"
        };

        let lesser_self_count = lesser_self_query.iter().count();

        **text = format!(
            "Prime Vessel: {}\nLesser Selves: {}",
            vessel_status, lesser_self_count
        );
    }
}

// ============================================================================
// VESSEL ENCOUNTER NOTIFICATIONS
// ============================================================================

/// Notification banner for vessel encounters (center-top)
#[derive(Component)]
pub struct VesselEncounterNotification {
    pub lifetime: Timer,
}

/// Spawn a vessel encounter notification
pub fn spawn_vessel_encounter_notification(
    mut commands: Commands,
    mut encounter_events: EventReader<LesserSelfEncountered>,
    vessel_query: Query<&PrimeVessel>,
) {
    // Handle Lesser Self encounters
    for event in encounter_events.read() {
        commands
            .spawn((
                VesselEncounterNotification {
                    lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                },
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(100.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(400.0),
                    height: Val::Px(80.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                BackgroundColor(EBONY.with_alpha(0.95)),
                BorderColor(BLOOD_RED),
                ZIndex(200),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("⚠ VESSEL ENCOUNTER ⚠"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(BLOOD_RED),
                    Node {
                        margin: UiRect::bottom(Val::Px(4.0)),
                        ..default()
                    },
                ));

                parent.spawn((
                    Text::new(format!("Lesser Self (Gen {}) - Power: {:.0}", event.generation, event.power_level)),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(GOLD),
                ));
            });
    }

    // Handle Prime Vessel encounters (when entering Combat state)
    for vessel in vessel_query.iter() {
        if vessel.roaming_state == bevy_shaman_prime_vessel::components::VesselRoamingState::Combat {
            // This would need additional event handling
            // For now, we'll hook into the state change event
        }
    }
}

/// Update and despawn expired notifications
pub fn update_vessel_encounter_notifications(
    mut commands: Commands,
    mut notification_query: Query<(Entity, &mut VesselEncounterNotification)>,
    time: Res<Time>,
) {
    for (entity, mut notification) in notification_query.iter_mut() {
        notification.lifetime.tick(time.delta());
        if notification.lifetime.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// ============================================================================
// RESURRECTION RITUAL PROGRESS BAR
// ============================================================================

/// Resurrection ritual UI (center-bottom)
#[derive(Component)]
pub struct ResurrectionRitualWidget;

#[derive(Component)]
pub struct ResurrectionProgressBar;

#[derive(Component)]
pub struct ResurrectionProgressText;

/// Spawn the Resurrection Ritual widget
pub fn spawn_resurrection_ritual_widget(
    mut commands: Commands,
    ritual_query: Query<&ResurrectionRitual, Added<ResurrectionRitual>>,
) {
    for ritual in ritual_query.iter() {
        commands
            .spawn((
                ResurrectionRitualWidget,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(100.0),
                    left: Val::Percent(50.0),
                    width: Val::Px(500.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(15.0)),
                    ..default()
                },
                BackgroundColor(EBONY.with_alpha(0.95)),
                BorderColor(INDIGO),
                ZIndex(150),
            ))
            .with_children(|parent| {
                // Title
                parent.spawn((
                    Text::new("RESURRECTION RITUAL"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(INDIGO),
                    Node {
                        margin: UiRect::bottom(Val::Px(8.0)),
                        ..default()
                    },
                ));

                // Progress bar container
                parent
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Px(30.0),
                            margin: UiRect::bottom(Val::Px(6.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(MAHOGANY),
                        BorderColor(BRONZE),
                    ))
                    .with_children(|trough| {
                        trough.spawn((
                            ResurrectionProgressBar,
                            Node {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            BackgroundColor(INDIGO),
                        ));
                    });

                // Progress text
                parent.spawn((
                    ResurrectionProgressText,
                    Text::new(format!("0 / {} spirits", ritual.spirits_required)),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(GOLD),
                ));
            });
    }
}

/// Update resurrection ritual progress bar
pub fn update_resurrection_ritual_widget(
    ritual_query: Query<&ResurrectionRitual>,
    mut progress_bar_query: Query<&mut Node, With<ResurrectionProgressBar>>,
    mut progress_text_query: Query<&mut Text, With<ResurrectionProgressText>>,
    mut widget_query: Query<&mut Visibility, With<ResurrectionRitualWidget>>,
) {
    if let Ok(ritual) = ritual_query.get_single() {
        // Show widget
        if let Ok(mut visibility) = widget_query.get_single_mut() {
            *visibility = Visibility::Visible;
        }

        // Update progress bar fill
        if let Ok(mut node) = progress_bar_query.get_single_mut() {
            node.width = Val::Percent(ritual.progress_percentage() * 100.0);
        }

        // Update progress text
        if let Ok(mut text) = progress_text_query.get_single_mut() {
            **text = format!(
                "{} / {} spirits ({:.0}%)",
                ritual.spirits_offered,
                ritual.spirits_required,
                ritual.progress_percentage() * 100.0
            );
        }
    } else {
        // Hide widget if no active ritual
        if let Ok(mut visibility) = widget_query.get_single_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

// ============================================================================
// PLUGIN REGISTRATION
// ============================================================================

pub struct PrimeVesselHudPlugin;

impl Plugin for PrimeVesselHudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_corruption_index_widget,
            spawn_danger_level_widget,
        ))
        .add_systems(Update, (
            update_corruption_index_widget,
            update_danger_level_widget,
            spawn_vessel_encounter_notification,
            update_vessel_encounter_notifications,
            spawn_resurrection_ritual_widget,
            update_resurrection_ritual_widget,
            handle_corruption_index_revealed,
        ));
    }
}
