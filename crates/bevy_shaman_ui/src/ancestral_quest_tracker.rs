/// Ancestral Quest Tracker Integration
/// Updates the Griot's Scroll to display active quests from the QuestLog
use bevy::prelude::*;
use bevy_shaman_story::systems::quest_system::{QuestLog, QuestRegistry, Quest, QuestStatus};
use crate::ancestral_hud::GriotScroll;
use crate::ancestral_theme::*;

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct QuestTitleText;

#[derive(Component)]
pub struct QuestObjectiveText {
    pub objective_index: usize,
}

#[derive(Component)]
pub struct QuestProgressText;

#[derive(Component)]
pub struct QuestRewardsText;

// ============================================================================
// QUEST TRACKER UPDATE SYSTEM
// ============================================================================

/// Update the quest tracker with active quest data
pub fn update_quest_tracker(
    mut commands: Commands,
    quest_log: Res<QuestLog>,
    quest_registry: Res<QuestRegistry>,
    scroll_query: Query<Entity, With<GriotScroll>>,
    existing_content: Query<Entity, Or<(
        With<QuestTitleText>,
        With<QuestObjectiveText>,
        With<QuestProgressText>,
        With<QuestRewardsText>,
    )>>,
) {
    // Only update when quest log changes
    if !quest_log.is_changed() {
        return;
    }

    let Ok(scroll_entity) = scroll_query.get_single() else {
        return;
    };

    // Clear existing quest content
    for entity in existing_content.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Get tracked quest from registry
    if let Some(quest_id) = &quest_log.tracked_quest {
        if let Some(quest) = quest_registry.get(quest_id) {
            // Rebuild quest tracker with current quest
            commands.entity(scroll_entity).with_children(|parent| {
                spawn_quest_content(parent, quest);
            });

            info!("Updated quest tracker with tracked quest: {}", quest.title);
            return;
        }
    }

    // No tracked quest - show placeholder
    commands.entity(scroll_entity).with_children(|parent| {
        parent.spawn((
            QuestTitleText,
            Text::new("No tracked quests"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(earth::SOIL_BROWN),
        ));
    });
}

// ============================================================================
// QUEST CONTENT SPAWNING
// ============================================================================

fn spawn_quest_content(parent: &mut ChildBuilder, quest: &Quest) {
    // Quest title with type indicator
    let quest_type_symbol = match quest.quest_type {
        bevy_shaman_story::systems::quest_system::QuestType::Main => "⭐",
        bevy_shaman_story::systems::quest_system::QuestType::Side => "◆",
        bevy_shaman_story::systems::quest_system::QuestType::Tutorial => "📖",
        bevy_shaman_story::systems::quest_system::QuestType::Bounty => "⚔️",
        _ => "◆",
    };

    parent.spawn((
        QuestTitleText,
        Text::new(format!("{} {}", quest_type_symbol, quest.title)),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(wood::EBONY),
    ));

    // Quest giver
    parent.spawn((
        Text::new(format!("From: {}", quest.quest_giver)),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(metal::BRONZE_PATINA),
    ));

    // Spacer
    parent.spawn(Node {
        height: Val::Px(SPACING_SMALL),
        ..default()
    });

    // Current objective (highlighted)
    if let Some(current_obj) = quest.get_current_objective() {
        parent
            .spawn((
                Node {
                    padding: UiRect::all(Val::Px(SPACING_SMALL)),
                    border: UiRect::all(Val::Px(BORDER_THIN)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.95, 0.92, 0.85, 0.5)), // Slight highlight
                BorderColor(metal::GOLD),
            ))
            .with_children(|obj_parent| {
                // Objective description
                obj_parent.spawn((
                    QuestObjectiveText { objective_index: 0 },
                    Text::new(format!("• {}", current_obj.description)),
                    TextFont {
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(wood::EBONY),
                ));

                // Progress indicator
                obj_parent.spawn((
                    QuestProgressText,
                    Text::new(current_obj.progress_text()),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(dye::INDIGO),
                ));
            });
    }

    // Additional objectives (faded)
    for (i, objective) in quest.objectives.iter().enumerate().skip(1) {
        if !objective.is_complete() {
            parent.spawn((
                QuestObjectiveText { objective_index: i },
                Text::new(format!("  ◦ {}", objective.description)),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgba(0.3, 0.3, 0.3, 0.6)), // Faded
            ));
        }
    }

    // Spacer
    parent.spawn(Node {
        height: Val::Px(SPACING_SMALL),
        ..default()
    });

    // Location hint (if available)
    if let Some(hint) = &quest.location_hint {
        parent.spawn((
            Text::new(format!("📍 {}", hint)),
            TextFont {
                font_size: 11.0,
                ..default()
            },
            TextColor(dye::RED_OCHRE),
        ));
    }

    // Rewards preview
    let rewards_text = quest.rewards.rewards_text();
    if rewards_text != "No rewards" {
        parent.spawn(Node {
            height: Val::Px(SPACING_SMALL),
            ..default()
        });

        parent.spawn((
            QuestRewardsText,
            Text::new(format!("Rewards: {}", rewards_text)),
            TextFont {
                font_size: 11.0,
                ..default()
            },
            TextColor(metal::GOLD),
        ));
    }
}

// ============================================================================
// QUEST LOG FULL DISPLAY
// ============================================================================

#[derive(Resource, Default)]
pub struct QuestLogUIState {
    pub visible: bool,
}

#[derive(Component)]
pub struct QuestLogRoot;

#[derive(Component)]
pub struct QuestEntryButton {
    pub quest_id: String,
}

/// Display full quest log when player presses 'Q' or opens menu
pub fn display_quest_log_full(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<QuestLogUIState>,
    quest_log: Res<QuestLog>,
    quest_registry: Res<QuestRegistry>,
    root_query: Query<Entity, With<QuestLogRoot>>,
) {
    // Toggle with 'Q' key
    if keyboard.just_pressed(KeyCode::KeyQ) {
        ui_state.visible = !ui_state.visible;
    }

    // Hide if not visible
    if !ui_state.visible {
        for entity in root_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    // Only spawn once
    if !root_query.is_empty() {
        return;
    }

    // Spawn quest log UI
    commands
        .spawn((
            QuestLogRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)), // Dim background
            GlobalZIndex(2000),
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    width: Val::Px(800.0),
                    height: Val::Px(600.0),
                    padding: UiRect::all(Val::Px(SPACING_LARGE)),
                    border: UiRect::all(Val::Px(BORDER_CARVED)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(SPACING_MEDIUM),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.89, 0.82, 0.69)), // Papyrus
                BorderColor(metal::BRONZE),
            ))
            .with_children(|panel| {
                // Title
                panel.spawn((
                    Text::new("QUEST LOG"),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(metal::GOLD),
                ));

                // Quest list
                panel
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(SPACING_SMALL),
                        flex_grow: 1.0,
                        overflow: Overflow::scroll_y(),
                        ..default()
                    })
                    .with_children(|list_parent| {
                        // Show all quests (active, completed, failed)
                        for quest_id in &quest_log.active_quests {
                            if let Some(quest) = quest_registry.get(quest_id) {
                                spawn_quest_entry(list_parent, quest);
                            }
                        }
                        for quest_id in &quest_log.completed_quests {
                            if let Some(quest) = quest_registry.get(quest_id) {
                                spawn_quest_entry(list_parent, quest);
                            }
                        }
                        for quest_id in &quest_log.failed_quests {
                            if let Some(quest) = quest_registry.get(quest_id) {
                                spawn_quest_entry(list_parent, quest);
                            }
                        }
                    });

                // Close instruction
                panel.spawn((
                    Text::new("Press Q to close"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(earth::SOIL_BROWN),
                ));
            });
        });
}

fn spawn_quest_entry(parent: &mut ChildBuilder, quest: &Quest) {
    let (bg_color, border_color) = match quest.status {
        QuestStatus::Active => (Color::srgb(0.95, 0.93, 0.88), metal::GOLD),
        QuestStatus::Completed => (Color::srgb(0.88, 0.92, 0.88), dye::INDIGO),
        QuestStatus::Failed => (Color::srgb(0.92, 0.88, 0.88), dye::RED_OCHRE),
        _ => (Color::srgb(0.9, 0.9, 0.9), wood::CARVED_LIGHT),
    };

    parent
        .spawn((
            QuestEntryButton {
                quest_id: quest.id.clone(),
            },
            Button,
            Node {
                padding: UiRect::all(Val::Px(SPACING_MEDIUM)),
                border: UiRect::all(Val::Px(BORDER_MEDIUM)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(SPACING_SMALL),
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor(border_color),
        ))
        .with_children(|entry| {
            // Title with status
            entry.spawn((
                Text::new(format!("{} [{}]", quest.title, format_quest_status(&quest.status))),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(wood::EBONY),
            ));

            // Description
            entry.spawn((
                Text::new(&quest.description),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(earth::SOIL_BROWN),
            ));

            // Progress
            if quest.status == QuestStatus::Active {
                entry.spawn((
                    Text::new(quest.progress_summary()),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(dye::INDIGO),
                ));
            }
        });
}

fn format_quest_status(status: &QuestStatus) -> &str {
    match status {
        QuestStatus::NotStarted => "Available",
        QuestStatus::Active => "Active",
        QuestStatus::Completed => "Complete",
        QuestStatus::Failed => "Failed",
    }
}
