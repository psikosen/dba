/// Ancestral HUD System
/// Implements the "Ancestral Legacy" HUD design with:
/// - Bronze-framed character portrait
/// - Carved ebony wood health/spirit/stamina bars with glossy ochre/indigo filling
/// - Sun compass mini-map
/// - Fetish belt combat abilities
/// - Griot's scroll quest tracker

use bevy::prelude::*;
use bevy_shaman_core::components::{Health, Spirit, Stamina, Player};
use crate::ancestral_theme::*;

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct AncestralHudRoot;

#[derive(Component)]
pub struct CharacterPortrait;

#[derive(Component)]
pub struct PortraitBronzeFrame;

#[derive(Component)]
pub struct CowrieShellDecoration;

#[derive(Component)]
pub struct HealthBarTrough;

#[derive(Component)]
pub struct HealthBarFill;

#[derive(Component)]
pub struct SpiritBarTrough;

#[derive(Component)]
pub struct SpiritBarFill;

#[derive(Component)]
pub struct StaminaBarTrough;

#[derive(Component)]
pub struct StaminaBarFill;

#[derive(Component)]
pub struct BarLabel {
    pub text: String,
}

#[derive(Component)]
pub struct SunCompassMinimap;

#[derive(Component)]
pub struct CompassNeedle;

#[derive(Component)]
pub struct FetishBelt;

#[derive(Component)]
pub struct AbilitySlot {
    pub slot_index: usize,
}

#[derive(Component)]
pub struct GriotScroll;

#[derive(Component)]
pub struct ScrollPin;

#[derive(Component)]
pub struct DevModeToggleButton;

#[derive(Component)]
pub struct DevModePanel;

#[derive(Component)]
pub struct SpawnBrotherButton;

#[derive(Component)]
pub struct SpawnBossButton;

// ============================================================================
// RESOURCES
// ============================================================================

#[derive(Resource)]
pub struct AncestralHudState {
    pub show_minimap: bool,
    pub show_abilities: bool,
    pub show_quest_tracker: bool,
}

impl Default for AncestralHudState {
    fn default() -> Self {
        Self {
            show_minimap: true,
            show_abilities: true,
            show_quest_tracker: true,
        }
    }
}

// ============================================================================
// CONSTANTS
// ============================================================================

const HEALTH_BAR_WIDTH: f32 = 280.0;
const HEALTH_BAR_HEIGHT: f32 = 28.0;
const SPIRIT_BAR_WIDTH: f32 = 280.0;
const SPIRIT_BAR_HEIGHT: f32 = 22.0;
const STAMINA_BAR_WIDTH: f32 = 280.0;
const STAMINA_BAR_HEIGHT: f32 = 18.0;

const PORTRAIT_OUTER_SIZE: f32 = PORTRAIT_SIZE + 24.0; // Bronze frame adds 24px
const COWRIE_SIZE: f32 = 12.0;

const ABILITY_SLOT_COUNT: usize = 6;
const ABILITY_SLOT_SIZE: f32 = SLOT_MEDIUM;

// ============================================================================
// SETUP SYSTEM
// ============================================================================

pub fn setup_ancestral_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Root HUD container
    commands
        .spawn((
            AncestralHudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            GlobalZIndex(1000),
        ))
        .with_children(|parent| {
            // ================================================================
            // TOP LEFT: CHARACTER PORTRAIT + VITALITY BARS
            // ================================================================
            parent
                .spawn(Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(SPACING_LARGE),
                    top: Val::Px(SPACING_LARGE),
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(SPACING_MEDIUM),
                    ..default()
                })
                .with_children(|vitality_parent| {
                    // Portrait with bronze frame
                    spawn_character_portrait(vitality_parent, &asset_server);

                    // Health, Spirit, Stamina bars
                    spawn_vitality_bars(vitality_parent);
                });

            // ================================================================
            // TOP RIGHT: SUN COMPASS MINIMAP
            // ================================================================
            spawn_sun_compass_minimap(parent);

            // ================================================================
            // BOTTOM LEFT: FETISH BELT (Combat Abilities)
            // ================================================================
            spawn_fetish_belt(parent);

            // ================================================================
            // MID RIGHT: GRIOT'S SCROLL (Quest Tracker)
            // ================================================================
            spawn_griot_scroll(parent);

            // ================================================================
            // TOP RIGHT (BELOW MINIMAP): DEV MODE TOGGLE
            // ================================================================
            spawn_dev_mode_toggle(parent);
        });
}

// ============================================================================
// PORTRAIT SYSTEM
// ============================================================================

fn spawn_character_portrait(parent: &mut ChildBuilder, _asset_server: &Res<AssetServer>) {
    parent
        .spawn((
            Node {
                width: Val::Px(PORTRAIT_OUTER_SIZE),
                height: Val::Px(PORTRAIT_OUTER_SIZE),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .with_children(|frame_parent| {
            // Bronze frame outer ring with glossy effect
            frame_parent
                .spawn((
                    PortraitBronzeFrame,
                    Node {
                        width: Val::Px(PORTRAIT_OUTER_SIZE),
                        height: Val::Px(PORTRAIT_OUTER_SIZE),
                        border: UiRect::all(Val::Px(BORDER_THICK)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(metal::BRONZE),
                    BorderColor(metal::BRONZE_PATINA),
                ))
                .with_children(|portrait_parent| {
                    // Inner portrait area
                    portrait_parent.spawn((
                        CharacterPortrait,
                        Node {
                            width: Val::Px(PORTRAIT_SIZE),
                            height: Val::Px(PORTRAIT_SIZE),
                            ..default()
                        },
                        BackgroundColor(wood::EBONY), // Placeholder - will show 3D portrait
                    ));
                });

            // Cowrie shell decorations around frame (4 corners)
            for i in 0..4 {
                let (x, y) = match i {
                    0 => (0.0, 0.0),                                          // Top-left
                    1 => (PORTRAIT_OUTER_SIZE - COWRIE_SIZE, 0.0),            // Top-right
                    2 => (0.0, PORTRAIT_OUTER_SIZE - COWRIE_SIZE),            // Bottom-left
                    3 => (PORTRAIT_OUTER_SIZE - COWRIE_SIZE,
                          PORTRAIT_OUTER_SIZE - COWRIE_SIZE),                 // Bottom-right
                    _ => (0.0, 0.0),
                };

                frame_parent.spawn((
                    CowrieShellDecoration,
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(x),
                        top: Val::Px(y),
                        width: Val::Px(COWRIE_SIZE),
                        height: Val::Px(COWRIE_SIZE),
                        ..default()
                    },
                    BackgroundColor(bone::IVORY),
                    BorderRadius::all(Val::Px(COWRIE_SIZE / 2.0)), // Circular
                ));
            }
        });
}

// ============================================================================
// VITALITY BARS SYSTEM
// ============================================================================

fn spawn_vitality_bars(parent: &mut ChildBuilder) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(SPACING_SMALL),
            ..default()
        })
        .with_children(|bars_parent| {
            // Health Bar (The Lifeblood) - Thick carved ebony with ochre filling
            spawn_resource_bar(
                bars_parent,
                "HEALTH",
                HEALTH_BAR_WIDTH,
                HEALTH_BAR_HEIGHT,
                wood::EBONY,
                dye::RED_OCHRE,
                HealthBarTrough,
                HealthBarFill,
            );

            // Spirit Bar - Polished ivory with indigo filling
            spawn_resource_bar(
                bars_parent,
                "SPIRIT",
                SPIRIT_BAR_WIDTH,
                SPIRIT_BAR_HEIGHT,
                bone::IVORY,
                dye::INDIGO,
                SpiritBarTrough,
                SpiritBarFill,
            );

            // Stamina Bar - Lighter wood with turmeric filling
            spawn_resource_bar(
                bars_parent,
                "STAMINA",
                STAMINA_BAR_WIDTH,
                STAMINA_BAR_HEIGHT,
                wood::CARVED_LIGHT,
                dye::TURMERIC,
                StaminaBarTrough,
                StaminaBarFill,
            );
        });
}

fn spawn_resource_bar<T: Component, F: Component>(
    parent: &mut ChildBuilder,
    label: &str,
    width: f32,
    height: f32,
    trough_color: Color,
    fill_color: Color,
    trough_component: T,
    fill_component: F,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(3.0),
            ..default()
        })
        .with_children(|bar_parent| {
            // Label (carved glyph style)
            bar_parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(metal::GOLD),
                BarLabel {
                    text: label.to_string(),
                },
            ));

            // Trough (carved wooden container)
            bar_parent
                .spawn((
                    trough_component,
                    Node {
                        width: Val::Px(width),
                        height: Val::Px(height),
                        border: UiRect::all(Val::Px(BORDER_MEDIUM)),
                        padding: UiRect::all(Val::Px(3.0)),
                        ..default()
                    },
                    BackgroundColor(trough_color),
                    BorderColor(wood::EBONY),
                ))
                .with_children(|trough_parent| {
                    // Fill (colored paste/liquid)
                    trough_parent.spawn((
                        fill_component,
                        Node {
                            width: Val::Percent(100.0), // Will be updated dynamically
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(fill_color),
                    ));
                });
        });
}

// ============================================================================
// MINIMAP SYSTEM
// ============================================================================

fn spawn_sun_compass_minimap(parent: &mut ChildBuilder) {
    parent
        .spawn((
            SunCompassMinimap,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(SPACING_LARGE),
                top: Val::Px(SPACING_LARGE),
                width: Val::Px(160.0),
                height: Val::Px(160.0),
                border: UiRect::all(Val::Px(BORDER_THICK)),
                ..default()
            },
            BackgroundColor(earth::SOIL_BROWN), // Aged hide background
            BorderColor(fabric::LEATHER_TOOLED),
            BorderRadius::all(Val::Px(80.0)), // Circular
        ))
        .with_children(|compass_parent| {
            // Sun emblem at top (North indicator)
            compass_parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(64.0),
                    top: Val::Px(4.0),
                    width: Val::Px(32.0),
                    height: Val::Px(32.0),
                    ..default()
                },
                BackgroundColor(metal::GOLD),
                BorderRadius::all(Val::Px(16.0)),
            ));

            // Compass needle (player direction indicator)
            compass_parent.spawn((
                CompassNeedle,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(76.0),
                    top: Val::Px(76.0),
                    width: Val::Px(8.0),
                    height: Val::Px(8.0),
                    ..default()
                },
                BackgroundColor(wood::EBONY),
            ));

            // Map area (will render actual minimap here)
            compass_parent.spawn((
                Node {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.5)),
            ));
        });
}

// ============================================================================
// FETISH BELT SYSTEM
// ============================================================================

fn spawn_fetish_belt(parent: &mut ChildBuilder) {
    parent
        .spawn((
            FetishBelt,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(SPACING_LARGE),
                bottom: Val::Px(SPACING_LARGE),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(SPACING_SMALL),
                ..default()
            },
        ))
        .with_children(|belt_parent| {
            // Main weapon slot (larger, framed by lion teeth)
            spawn_ability_slot(belt_parent, 0, SLOT_LARGE, true);

            // Additional ability slots (Mancala pits style)
            for i in 1..ABILITY_SLOT_COUNT {
                spawn_ability_slot(belt_parent, i, ABILITY_SLOT_SIZE, false);
            }
        });
}

fn spawn_ability_slot(parent: &mut ChildBuilder, index: usize, size: f32, is_main_weapon: bool) {
    let border_width = if is_main_weapon { BORDER_THICK } else { BORDER_MEDIUM };
    let border_color = if is_main_weapon { metal::BRONZE } else { wood::CARVED_LIGHT };

    parent
        .spawn((
            AbilitySlot { slot_index: index },
            Node {
                width: Val::Px(size),
                height: Val::Px(size),
                border: UiRect::all(Val::Px(border_width)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(wood::MAHOGANY),
            BorderColor(border_color),
            BorderRadius::all(Val::Px(size / 2.0)), // Circular pits
        ))
        .with_children(|slot_parent| {
            // Placeholder for ability icon
            slot_parent.spawn((
                Node {
                    width: Val::Px(size * 0.7),
                    height: Val::Px(size * 0.7),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
            ));
        });
}

// ============================================================================
// GRIOT'S SCROLL SYSTEM
// ============================================================================

fn spawn_griot_scroll(parent: &mut ChildBuilder) {
    parent
        .spawn((
            GriotScroll,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(SPACING_LARGE),
                top: Val::Px(200.0), // Below minimap
                width: Val::Px(300.0),
                max_height: Val::Px(400.0),
                padding: UiRect::all(Val::Px(SPACING_MEDIUM)),
                border: UiRect::all(Val::Px(BORDER_THIN)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(SPACING_SMALL),
                ..default()
            },
            BackgroundColor(Color::srgba(0.89, 0.85, 0.71, 0.95)), // Papyrus/bark cloth
            BorderColor(wood::CARVED_LIGHT),
        ))
        .with_children(|scroll_parent| {
            // Decorative pin/dagger at top
            scroll_parent.spawn((
                ScrollPin,
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(130.0),
                    top: Val::Px(-10.0),
                    width: Val::Px(40.0),
                    height: Val::Px(20.0),
                    ..default()
                },
                BackgroundColor(metal::BRONZE),
            ));

            // Title
            scroll_parent.spawn((
                Text::new("CURRENT QUEST"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(wood::EBONY),
            ));

            // Quest content (placeholder)
            scroll_parent.spawn((
                Text::new("No active quests"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(earth::SOIL_BROWN),
            ));
        });
}

// ============================================================================
// UPDATE SYSTEMS
// ============================================================================

pub fn update_vitality_bars(
    player_query: Query<(&Health, &Spirit, &Stamina), With<Player>>,
    mut health_fill_query: Query<&mut Node, (With<HealthBarFill>, Without<SpiritBarFill>, Without<StaminaBarFill>)>,
    mut spirit_fill_query: Query<&mut Node, (With<SpiritBarFill>, Without<HealthBarFill>, Without<StaminaBarFill>)>,
    mut stamina_fill_query: Query<&mut Node, (With<StaminaBarFill>, Without<HealthBarFill>, Without<SpiritBarFill>)>,
) {
    let Ok((health, spirit, stamina)) = player_query.get_single() else {
        return;
    };

    // Update health bar fill
    if let Ok(mut node) = health_fill_query.get_single_mut() {
        let percentage = (health.current / health.max) * 100.0;
        node.width = Val::Percent(percentage);
    }

    // Update spirit bar fill
    if let Ok(mut node) = spirit_fill_query.get_single_mut() {
        let percentage = (spirit.current / spirit.max) * 100.0;
        node.width = Val::Percent(percentage);
    }

    // Update stamina bar fill
    if let Ok(mut node) = stamina_fill_query.get_single_mut() {
        let percentage = (stamina.current / stamina.max) * 100.0;
        node.width = Val::Percent(percentage);
    }
}

/// Animate the health bar with subtle breathing effect
pub fn animate_health_bar(
    time: Res<Time>,
    mut health_fill_query: Query<&mut BackgroundColor, With<HealthBarFill>>,
) {
    let breath_scale = health_breath(time.elapsed_secs());

    if let Ok(mut bg_color) = health_fill_query.get_single_mut() {
        let base_color = dye::RED_OCHRE;
        bg_color.0 = Color::srgb(
            base_color.to_srgba().red * breath_scale,
            base_color.to_srgba().green * breath_scale,
            base_color.to_srgba().blue * breath_scale,
        );
    }
}

/// Animate spirit bar with pulsing glow
pub fn animate_spirit_bar(
    time: Res<Time>,
    mut spirit_fill_query: Query<&mut BackgroundColor, With<SpiritBarFill>>,
) {
    let pulse = spirit_pulse(time.elapsed_secs(), 2.0);

    if let Ok(mut bg_color) = spirit_fill_query.get_single_mut() {
        let base_color = dye::INDIGO;
        let intensity = 0.8 + (pulse * 0.4); // Pulse between 0.8 and 1.2
        bg_color.0 = Color::srgb(
            (base_color.to_srgba().red * intensity).min(1.0),
            (base_color.to_srgba().green * intensity).min(1.0),
            (base_color.to_srgba().blue * intensity).min(1.0),
        );
    }
}

// ============================================================================
// DEV MODE TOGGLE & SPAWN PANEL
// ============================================================================

fn spawn_dev_mode_toggle(parent: &mut ChildBuilder) {
    parent
        .spawn((
            DevModeToggleButton,
            Button,
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(SPACING_LARGE),
                top: Val::Px(SPACING_LARGE + 180.0), // Below minimap
                width: Val::Px(120.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(BORDER_MEDIUM)),
                ..default()
            },
            BackgroundColor(wood::EBONY),
            BorderColor(metal::GOLD),
        ))
        .with_children(|button_parent| {
            button_parent.spawn((
                Text::new("DEV MODE"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(metal::GOLD),
            ));
        });
}

/// System to spawn the dev mode panel when dev mode is enabled
pub fn update_dev_mode_panel(
    mut commands: Commands,
    dev_mode: Res<bevy_shaman_core::resources::DevMode>,
    panel_query: Query<Entity, With<DevModePanel>>,
) {
    if dev_mode.is_enabled() {
        // Spawn panel if it doesn't exist
        if panel_query.is_empty() {
            commands
                .spawn((
                    DevModePanel,
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(SPACING_LARGE),
                        top: Val::Px(SPACING_LARGE + 230.0), // Below dev mode toggle
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(SPACING_SMALL),
                        padding: UiRect::all(Val::Px(SPACING_MEDIUM)),
                        border: UiRect::all(Val::Px(BORDER_MEDIUM)),
                        ..default()
                    },
                    BackgroundColor(wood::EBONY),
                    BorderColor(metal::GOLD),
                    GlobalZIndex(1001),
                ))
                .with_children(|panel_parent| {
                    // Title
                    panel_parent.spawn((
                        Text::new("DEV SPAWNS"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(metal::GOLD),
                    ));

                    // Spawn Brother Button
                    panel_parent
                        .spawn((
                            SpawnBrotherButton,
                            Button,
                            Node {
                                width: Val::Px(160.0),
                                height: Val::Px(35.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(BORDER_THIN)),
                                ..default()
                            },
                            BackgroundColor(dye::RED_OCHRE),
                            BorderColor(metal::BRONZE),
                        ))
                        .with_children(|btn_parent| {
                            btn_parent.spawn((
                                Text::new("Spawn Brother"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(bone::IVORY),
                            ));
                        });

                    // Spawn Boss Button
                    panel_parent
                        .spawn((
                            SpawnBossButton,
                            Button,
                            Node {
                                width: Val::Px(160.0),
                                height: Val::Px(35.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(BORDER_THIN)),
                                ..default()
                            },
                            BackgroundColor(metal::GOLD),
                            BorderColor(metal::BRONZE),
                        ))
                        .with_children(|btn_parent| {
                            btn_parent.spawn((
                                Text::new("Spawn Boss"),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(wood::EBONY),
                            ));
                        });
                });
        }
    } else {
        // Remove panel if it exists
        for entity in panel_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Handle dev mode toggle button clicks
pub fn handle_dev_mode_toggle(
    mut dev_mode: ResMut<bevy_shaman_core::resources::DevMode>,
    mut toggle_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<DevModeToggleButton>)>,
) {
    for (interaction, mut bg_color) in toggle_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                dev_mode.toggle();
                // Update button color based on state
                *bg_color = if dev_mode.is_enabled() {
                    BackgroundColor(dye::RED_OCHRE)
                } else {
                    BackgroundColor(wood::EBONY)
                };
            }
            Interaction::Hovered => {
                if !dev_mode.is_enabled() {
                    *bg_color = BackgroundColor(Color::srgb(0.15, 0.1, 0.08));
                }
            }
            Interaction::None => {
                if !dev_mode.is_enabled() {
                    *bg_color = BackgroundColor(wood::EBONY);
                }
            }
        }
    }
}

/// Handle spawn brother button clicks
pub fn handle_spawn_brother_button(
    commands: Commands,
    button_query: Query<&Interaction, (Changed<Interaction>, With<SpawnBrotherButton>)>,
    names_db: Res<bevy_shaman_story::resources::AfricanNamesDB>,
    player_query: Query<&bevy_shaman_core::components::GridPosition, With<bevy_shaman_core::components::Player>>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            // Call the spawn function as a nested system call
            bevy_shaman_story::systems::npc_spawning::spawn_test_brother(
                commands,
                names_db,
                player_query,
            );
            // Only process the first click
            break;
        }
    }
}

/// Handle spawn boss button clicks
pub fn handle_spawn_boss_button(
    commands: Commands,
    button_query: Query<&Interaction, (Changed<Interaction>, With<SpawnBossButton>)>,
    template_db: Res<bevy_shaman_monsters::resources::MonsterTemplateDB>,
    player_query: Query<&bevy_shaman_core::components::GridPosition, With<bevy_shaman_core::components::Player>>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            // Call the spawn function as a nested system call
            bevy_shaman_monsters::systems::spawning::spawn_test_boss(
                commands,
                template_db,
                player_query,
            );
            // Only process the first click
            break;
        }
    }
}
