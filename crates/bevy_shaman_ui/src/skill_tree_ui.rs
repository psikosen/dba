use bevy::prelude::*;
use bevy_shaman_combat::systems::skill_tree::{SkillTree, SkillDatabase, SkillPath, SkillId};
use crate::ancestral_theme::*;

/// Resource tracking skill tree UI state
#[derive(Resource, Default)]
pub struct SkillTreeUIState {
    pub visible: bool,
    pub selected_path: Option<SkillPath>,
}

/// Marker component for skill tree UI root
#[derive(Component)]
pub struct SkillTreeUIRoot;

/// Marker component for skill node buttons
#[derive(Component)]
pub struct SkillNodeButton {
    pub skill_id: SkillId,
}

/// System to display skill tree interface
pub fn display_skill_tree_ui(
    mut commands: Commands,
    ui_state: Res<SkillTreeUIState>,
    skill_tree_query: Query<&SkillTree>,
    skill_db: Res<SkillDatabase>,
    existing_ui: Query<Entity, With<SkillTreeUIRoot>>,
) {
    // Clean up existing UI
    for entity in existing_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if !ui_state.visible {
        return;
    }

    let Ok(skill_tree) = skill_tree_query.get_single() else { return };

    // Spawn UI panel - carved wooden panel with intricate bronze frame
    commands
        .spawn((
            SkillTreeUIRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(10.0),
                top: Val::Percent(10.0),
                width: Val::Percent(80.0),
                height: Val::Percent(80.0),
                padding: UiRect::all(Val::Px(SPACING_LARGE)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(wood::EBONY),
            BorderColor(metal::GOLD),
        ))
        .with_children(|parent| {
            // Title - ancestral wisdom
            parent.spawn((
                Text::new("Path of the Ancestors"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(metal::GOLD_SHINE),
                Node {
                    margin: UiRect::bottom(Val::Px(SPACING_MEDIUM)),
                    justify_self: JustifySelf::Center,
                    ..default()
                },
            ));

            // Skill points display
            parent.spawn((
                Text::new(format!("Skill Points: {}", skill_tree.skill_points)),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(dye::TURMERIC),
                Node {
                    margin: UiRect::bottom(Val::Px(SPACING_LARGE)),
                    justify_self: JustifySelf::Center,
                    ..default()
                },
            ));

            // Path tabs - horizontal container
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        margin: UiRect::bottom(Val::Px(SPACING_MEDIUM)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    spawn_path_tab(parent, SkillPath::Ngoma, "Ngoma\n(Rhythm)", metal::BRONZE);
                    spawn_path_tab(parent, SkillPath::Ubuntu, "Ubuntu\n(Community)", dye::FOREST_GREEN);
                    spawn_path_tab(parent, SkillPath::Ashe, "Ashe\n(Power)", dye::RED_OCHRE);
                    spawn_path_tab(parent, SkillPath::Ubiqa, "Ubiqa\n(Nature)", earth::TERRACOTTA);
                    spawn_path_tab(parent, SkillPath::Tempo, "Tempo\n(Speed)", dye::INDIGO);
                });

            // Skills display area
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(SPACING_MEDIUM)),
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(wood::MAHOGANY),
                ))
                .with_children(|parent| {
                    // Display skills based on selected path
                    if let Some(path) = ui_state.selected_path {
                        display_skills_for_path(parent, path, skill_tree, &skill_db);
                    } else {
                        // Show overview of all paths
                        parent.spawn((
                            Text::new("Select a path to view skills"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(bone::AGED_BONE),
                            Node {
                                align_self: AlignSelf::Center,
                                margin: UiRect::top(Val::Px(SPACING_XLARGE)),
                                ..default()
                            },
                        ));

                        // Show stats for each path
                        for path in [SkillPath::Ngoma, SkillPath::Ubuntu, SkillPath::Ashe,
                                    SkillPath::Ubiqa, SkillPath::Tempo] {
                            let count = skill_tree.skills_in_path(path);
                            let path_name = path_display_name(path);
                            let path_color = path_color(path);

                            parent.spawn((
                                Text::new(format!("{}: {} skills unlocked", path_name, count)),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(path_color),
                                Node {
                                    margin: UiRect::all(Val::Px(SPACING_SMALL)),
                                    ..default()
                                },
                            ));
                        }
                    }
                });

            // Instructions
            parent.spawn((
                Text::new("Press 'K' to close | Click skills to unlock"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(bone::BONE_SHADOW),
                Node {
                    margin: UiRect::top(Val::Px(SPACING_MEDIUM)),
                    justify_self: JustifySelf::Center,
                    ..default()
                },
            ));
        });
}

fn spawn_path_tab(parent: &mut ChildBuilder, path: SkillPath, label: &str, color: Color) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(140.0),
                height: Val::Px(60.0),
                padding: UiRect::all(Val::Px(SPACING_SMALL)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(color),
            BorderColor(metal::BRONZE),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(bone::IVORY),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
            ));
        });
}

fn display_skills_for_path(
    parent: &mut ChildBuilder,
    path: SkillPath,
    skill_tree: &SkillTree,
    skill_db: &SkillDatabase,
) {
    // Get all skills for this path
    let skills: Vec<_> = skill_db.skills.values()
        .filter(|s| s.path == path)
        .collect();

    // Sort by tier
    let mut sorted_skills = skills.clone();
    sorted_skills.sort_by_key(|s| s.tier);

    // Display skills by tier
    for skill in sorted_skills {
        let is_unlocked = skill_tree.has_skill(skill.id);
        let can_unlock = skill_tree.skill_points >= skill.cost
            && skill.prerequisites.iter().all(|p| skill_tree.has_skill(*p));

        let bg_color = if is_unlocked {
            metal::GOLD
        } else if can_unlock {
            dye::FOREST_GREEN
        } else {
            earth::CHARCOAL
        };

        parent
            .spawn((
                SkillNodeButton { skill_id: skill.id },
                Button,
                Node {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(SPACING_MEDIUM)),
                    margin: UiRect::bottom(Val::Px(SPACING_SMALL)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(bg_color),
                BorderColor(metal::BRONZE),
            ))
            .with_children(|parent| {
                // Skill name and tier
                parent.spawn((
                    Text::new(format!("Tier {} - {}", skill.tier, skill.name)),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(if is_unlocked { wood::EBONY } else { bone::IVORY }),
                ));

                // Description
                parent.spawn((
                    Text::new(skill.description),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(if is_unlocked { wood::MAHOGANY } else { bone::AGED_BONE }),
                ));

                // Cost
                parent.spawn((
                    Text::new(format!("Cost: {} points", skill.cost)),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(dye::TURMERIC),
                ));

                // Prerequisites
                if !skill.prerequisites.is_empty() {
                    parent.spawn((
                        Text::new(format!("Requires: {:?}", skill.prerequisites)),
                        TextFont {
                            font_size: 10.0,
                            ..default()
                        },
                        TextColor(bone::BONE_SHADOW),
                    ));
                }

                // Status
                if is_unlocked {
                    parent.spawn((
                        Text::new("✓ UNLOCKED"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(dye::FOREST_GREEN),
                    ));
                } else if !can_unlock {
                    parent.spawn((
                        Text::new("✗ LOCKED"),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        TextColor(dye::RED_OCHRE),
                    ));
                }
            });
    }
}

fn path_display_name(path: SkillPath) -> &'static str {
    match path {
        SkillPath::Ngoma => "Ngoma (Rhythm)",
        SkillPath::Ubuntu => "Ubuntu (Community)",
        SkillPath::Ashe => "Ashe (Power)",
        SkillPath::Ubiqa => "Ubiqa (Nature)",
        SkillPath::Tempo => "Tempo (Speed)",
    }
}

fn path_color(path: SkillPath) -> Color {
    match path {
        SkillPath::Ngoma => metal::BRONZE,
        SkillPath::Ubuntu => dye::FOREST_GREEN,
        SkillPath::Ashe => dye::RED_OCHRE,
        SkillPath::Ubiqa => earth::TERRACOTTA,
        SkillPath::Tempo => dye::INDIGO,
    }
}

/// System to toggle skill tree UI
pub fn toggle_skill_tree_ui(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<SkillTreeUIState>,
) {
    if keyboard.just_pressed(KeyCode::KeyK) {
        ui_state.visible = !ui_state.visible;
    }
}

/// System to handle skill unlock clicks
pub fn handle_skill_unlock_clicks(
    mut skill_tree_query: Query<&mut SkillTree>,
    skill_db: Res<SkillDatabase>,
    button_query: Query<(&Interaction, &SkillNodeButton), Changed<Interaction>>,
) {
    for (interaction, button) in button_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut skill_tree) = skill_tree_query.get_single_mut() {
                match skill_tree.try_unlock(button.skill_id, &skill_db) {
                    Ok(_) => {
                        info!("Unlocked skill: {:?}", button.skill_id);
                    }
                    Err(err) => {
                        warn!("Failed to unlock skill: {}", err);
                    }
                }
            }
        }
    }
}
