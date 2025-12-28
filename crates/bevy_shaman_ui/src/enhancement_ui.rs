use bevy::prelude::*;
use bevy_shaman_combat::components::{WeaponEnhancement, EquippedWeapon, EnchantmentType};
use bevy_shaman_core::components::Spirit;
use bevy_shaman_items::components::Inventory;
use crate::ancestral_theme::*;

/// Resource tracking enhancement UI state
#[derive(Resource, Default)]
pub struct EnhancementUIState {
    pub visible: bool,
}

/// Marker component for enhancement UI root
#[derive(Component)]
pub struct EnhancementUIRoot;

/// System to display weapon enhancement interface
pub fn display_enhancement_ui(
    mut commands: Commands,
    ui_state: Res<EnhancementUIState>,
    weapon_query: Query<&EquippedWeapon>,
    enhancement_query: Query<&WeaponEnhancement>,
    spirit_query: Query<&Spirit>,
    inventory_query: Query<&Inventory>,
    existing_ui: Query<Entity, With<EnhancementUIRoot>>,
) {
    // Clean up existing UI
    for entity in existing_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if !ui_state.visible {
        return;
    }

    // Get weapon and enhancement data
    let Ok(weapon) = weapon_query.get_single() else { return };
    let Ok(enhancement) = enhancement_query.get_single() else { return };
    let Ok(spirit) = spirit_query.get_single() else { return };
    let Ok(inventory) = inventory_query.get_single() else { return };

    // Spawn UI panel - carved wooden panel with bronze frame
    commands
        .spawn((
            EnhancementUIRoot,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(100.0),
                top: Val::Px(100.0),
                width: Val::Px(500.0),
                height: Val::Px(600.0),
                padding: UiRect::all(Val::Px(SPACING_LARGE)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(wood::MAHOGANY),
            BorderColor(metal::BRONZE),
        ))
        .with_children(|parent| {
            // Title - carved into wood
            parent.spawn((
                Text::new("Spirit Forge"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(metal::GOLD_SHINE),
                Node {
                    margin: UiRect::bottom(Val::Px(SPACING_MEDIUM)),
                    ..default()
                },
            ));

            // Weapon info panel
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(SPACING_MEDIUM)),
                        margin: UiRect::bottom(Val::Px(SPACING_MEDIUM)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(wood::EBONY),
                    BorderColor(metal::BRONZE_PATINA),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(format!("Weapon: {:?}", weapon.weapon_type)),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(bone::IVORY),
                    ));

                    parent.spawn((
                        Text::new(format!("Enhancement Level: {}/{}",
                            enhancement.level, WeaponEnhancement::MAX_LEVEL)),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(metal::GOLD),
                    ));

                    parent.spawn((
                        Text::new(format!("Damage Bonus: +{:.1}%", enhancement.damage_bonus())),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(dye::RED_OCHRE),
                    ));

                    parent.spawn((
                        Text::new(format!("Spirit Efficiency: -{:.1}%",
                            100.0 - (enhancement.spirit_cost_multiplier() * 100.0))),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(dye::INDIGO),
                    ));
                });

            // Active enchantments panel
            if !enhancement.active_enchantments.is_empty() {
                parent
                    .spawn((
                        Node {
                            width: Val::Percent(100.0),
                            padding: UiRect::all(Val::Px(SPACING_MEDIUM)),
                            margin: UiRect::bottom(Val::Px(SPACING_MEDIUM)),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        BackgroundColor(earth::SOIL_BROWN),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("Active Enchantments"),
                            TextFont {
                                font_size: 18.0,
                                ..default()
                            },
                            TextColor(dye::TURMERIC),
                        ));

                        for enchantment in &enhancement.active_enchantments {
                            let name = match enchantment.enchantment_type {
                                EnchantmentType::BloodFury => "Ropa's Blood Fury",
                                EnchantmentType::SpiritInfusion => "Roho's Spirit Infusion",
                                EnchantmentType::VenomCoating => "Venom Coating",
                                EnchantmentType::AncestralBlessing => "Ancestral Blessing",
                            };

                            parent.spawn((
                                Text::new(format!("{} ({:.0}s)", name, enchantment.duration_remaining)),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(bone::AGED_BONE),
                            ));
                        }
                    });
            }

            // Enhancement cost panel
            let cost = enhancement.next_level_cost();
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(SPACING_MEDIUM)),
                        margin: UiRect::bottom(Val::Px(SPACING_MEDIUM)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(wood::CARVED_LIGHT),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Next Level Cost"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(metal::GOLD),
                    ));

                    parent.spawn((
                        Text::new(format!("Spirit Energy: {:.0}", cost.spirit_orbs)),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(dye::INDIGO),
                    ));

                    parent.spawn((
                        Text::new(format!("Blood Plants: {}", cost.blood_plants)),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(dye::RED_OCHRE),
                    ));

                    parent.spawn((
                        Text::new(format!("Spirit Plants: {}", cost.spirit_plants)),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(dye::FOREST_GREEN),
                    ));
                });

            // Instructions
            parent.spawn((
                Text::new("Press 'M' to merge spirit orbs\nPress 'N' to apply plant enchantments\nPress 'H' to close"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(bone::BONE_SHADOW),
                Node {
                    margin: UiRect::top(Val::Px(SPACING_LARGE)),
                    ..default()
                },
            ));
        });
}

/// System to toggle enhancement UI
pub fn toggle_enhancement_ui(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_state: ResMut<EnhancementUIState>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        ui_state.visible = !ui_state.visible;
    }
}
