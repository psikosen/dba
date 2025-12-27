/// Ancestral Inventory System
/// Full-screen inventory with:
/// - Navigation tabs as physical objects (pouch, mask, scroll, drum, weights)
/// - Paperdoll character model with brass gear slots
/// - Trading board item grid (Mancala-style compartments)
/// - Tactile item icons (gourds, woven cloth, hammered metal)
/// - Bark cloth/papyrus background with geometric Benin bronze patterns

use bevy::prelude::*;
use bevy_shaman_core::components::Player;
use bevy_shaman_items::components::{Inventory, ItemStack};
use crate::ancestral_theme::*;

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct AncestralInventoryRoot;

#[derive(Component)]
pub struct InventoryBackground;

#[derive(Component)]
pub struct GeometricBorder;

#[derive(Component)]
pub struct NavigationTabs;

#[derive(Component)]
pub struct TabButton {
    pub tab_type: InventoryTab,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum InventoryTab {
    Inventory,  // Leather pouch
    Character,  // Ceremonial mask
    Map,        // Bamboo scroll case
    Quests,     // Talking drum
    Settings,   // Bronze trading weights
}

#[derive(Component)]
pub struct TabIcon;

#[derive(Component)]
pub struct PaperdollPanel;

#[derive(Component)]
pub struct CharacterModel;

#[derive(Component)]
pub struct GearSlot {
    pub slot_type: GearSlotType,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GearSlotType {
    Head,
    Neck,
    Chest,
    Hands,
    MainHand,
    OffHand,
    Legs,
    Feet,
    Ring1,
    Ring2,
}

#[derive(Component)]
pub struct TradingBoardPanel;

#[derive(Component)]
pub struct ItemCompartment {
    pub index: usize,
}

#[derive(Component)]
pub struct ItemIcon {
    pub item_id: String,
}

#[derive(Component)]
pub struct ItemQuantityText;

#[derive(Component)]
pub struct ItemTooltipUI;

#[derive(Component)]
pub struct TooltipTitle;

#[derive(Component)]
pub struct TooltipDescription;

#[derive(Component)]
pub struct ContextMenuUI;

#[derive(Component)]
pub struct ContextButton {
    pub action: ItemAction,
    pub item_index: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ItemAction {
    Use,
    Equip,
    Drop,
    DropStack,
    Examine,
}

// ============================================================================
// RESOURCES
// ============================================================================

#[derive(Resource)]
pub struct AncestralInventoryState {
    pub visible: bool,
    pub current_tab: InventoryTab,
    pub selected_compartment: Option<usize>,
    pub hovered_compartment: Option<usize>,
    pub dragging_item: Option<usize>,
    pub show_tooltip: bool,
    pub tooltip_item: Option<String>,
    pub show_context_menu: bool,
    pub context_menu_position: Vec2,
    pub context_menu_item: Option<usize>,
}

impl Default for AncestralInventoryState {
    fn default() -> Self {
        Self {
            visible: false,
            current_tab: InventoryTab::Inventory,
            selected_compartment: None,
            hovered_compartment: None,
            dragging_item: None,
            show_tooltip: false,
            tooltip_item: None,
            show_context_menu: false,
            context_menu_position: Vec2::ZERO,
            context_menu_item: None,
        }
    }
}

// ============================================================================
// CONSTANTS
// ============================================================================

const GRID_COLS: usize = 8;
const GRID_ROWS: usize = 5;
const TOTAL_COMPARTMENTS: usize = GRID_COLS * GRID_ROWS;

const COMPARTMENT_SIZE: f32 = 72.0;
const COMPARTMENT_GAP: f32 = 8.0;

const PAPERDOLL_WIDTH: f32 = 320.0;
const TRADING_BOARD_WIDTH: f32 = (COMPARTMENT_SIZE + COMPARTMENT_GAP) * GRID_COLS as f32;

const TAB_WIDTH: f32 = 80.0;
const TAB_HEIGHT: f32 = 60.0;

const GEAR_SLOT_SIZE: f32 = 64.0;

// ============================================================================
// MAIN DISPLAY SYSTEM
// ============================================================================

pub fn display_ancestral_inventory(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inventory_state: ResMut<AncestralInventoryState>,
    root_query: Query<Entity, With<AncestralInventoryRoot>>,
    player_inventory: Query<&Inventory, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    // Toggle with 'I' key
    if keyboard.just_pressed(KeyCode::KeyI) {
        inventory_state.visible = !inventory_state.visible;
    }

    // Hide if not visible
    if !inventory_state.visible {
        for entity in root_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    // Only spawn once
    if !root_query.is_empty() {
        return;
    }

    // Get player inventory
    let inventory = if let Ok(inv) = player_inventory.get_single() {
        inv
    } else {
        return;
    };

    // Spawn complete inventory UI
    spawn_inventory_ui(&mut commands, inventory, &inventory_state, &asset_server);
}

fn spawn_inventory_ui(
    commands: &mut Commands,
    inventory: &Inventory,
    state: &AncestralInventoryState,
    _asset_server: &Res<AssetServer>,
) {
    commands
        .spawn((
            AncestralInventoryRoot,
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
            // Main inventory panel (royal tent interior aesthetic)
            root.spawn((
                InventoryBackground,
                Node {
                    width: Val::Px(1200.0),
                    height: Val::Px(700.0),
                    border: UiRect::all(Val::Px(BORDER_CARVED)),
                    padding: UiRect::all(Val::Px(SPACING_LARGE)),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(SPACING_MEDIUM),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.89, 0.82, 0.69)), // Papyrus background
                BorderColor(metal::BRONZE),
            ))
            .with_children(|panel| {
                // Geometric border pattern (Benin bronze inspired)
                spawn_geometric_border(panel);

                // Navigation tabs at top
                spawn_navigation_tabs(panel, state.current_tab);

                // Content area based on active tab
                match state.current_tab {
                    InventoryTab::Inventory => {
                        spawn_inventory_content(panel, inventory);
                    }
                    InventoryTab::Character => {
                        spawn_character_content(panel);
                    }
                    _ => {
                        // Other tabs (Map, Quests, Settings) - placeholder
                        spawn_placeholder_content(panel, state.current_tab);
                    }
                }
            });
        });
}

// ============================================================================
// GEOMETRIC BORDER
// ============================================================================

fn spawn_geometric_border(parent: &mut ChildBuilder) {
    // Top border with triangular pattern
    for i in 0..40 {
        let x = i as f32 * 30.0;
        parent.spawn((
            GeometricBorder,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(x),
                top: Val::Px(0.0),
                width: Val::Px(20.0),
                height: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(if i % 2 == 0 {
                metal::BRONZE
            } else {
                metal::COPPER
            }),
        ));
    }
}

// ============================================================================
// NAVIGATION TABS
// ============================================================================

fn spawn_navigation_tabs(parent: &mut ChildBuilder, active_tab: InventoryTab) {
    parent
        .spawn((
            NavigationTabs,
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(SPACING_MEDIUM),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|tabs_parent| {
            // Inventory - Leather Pouch
            spawn_tab(tabs_parent, InventoryTab::Inventory, "🎒", active_tab);

            // Character - Ceremonial Mask
            spawn_tab(tabs_parent, InventoryTab::Character, "🎭", active_tab);

            // Map - Bamboo Scroll
            spawn_tab(tabs_parent, InventoryTab::Map, "🗺️", active_tab);

            // Quests - Talking Drum
            spawn_tab(tabs_parent, InventoryTab::Quests, "🪘", active_tab);

            // Settings - Bronze Weights
            spawn_tab(tabs_parent, InventoryTab::Settings, "⚖️", active_tab);
        });
}

fn spawn_tab(
    parent: &mut ChildBuilder,
    tab_type: InventoryTab,
    icon: &str,
    active_tab: InventoryTab,
) {
    let is_active = tab_type == active_tab;
    let (bg_color, border_color) = if is_active {
        (wood::MAHOGANY, metal::GOLD)
    } else {
        (wood::CARVED_LIGHT, metal::BRONZE_PATINA)
    };

    parent
        .spawn((
            TabButton { tab_type },
            Button,
            Node {
                width: Val::Px(TAB_WIDTH),
                height: Val::Px(TAB_HEIGHT),
                border: UiRect::all(Val::Px(BORDER_MEDIUM)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(bg_color),
            BorderColor(border_color),
        ))
        .with_children(|tab| {
            // Tab icon
            tab.spawn((
                TabIcon,
                Text::new(icon),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ));
        });
}

// ============================================================================
// INVENTORY CONTENT (Paperdoll + Trading Board)
// ============================================================================

fn spawn_inventory_content(parent: &mut ChildBuilder, inventory: &Inventory) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(SPACING_LARGE),
            flex_grow: 1.0,
            ..default()
        })
        .with_children(|content| {
            // Left: Paperdoll with gear slots
            spawn_paperdoll_panel(content);

            // Right: Trading board item grid
            spawn_trading_board(content, inventory);
        });
}

// ============================================================================
// PAPERDOLL PANEL
// ============================================================================

fn spawn_paperdoll_panel(parent: &mut ChildBuilder) {
    parent
        .spawn((
            PaperdollPanel,
            Node {
                width: Val::Px(PAPERDOLL_WIDTH),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(SPACING_MEDIUM),
                padding: UiRect::all(Val::Px(SPACING_MEDIUM)),
                border: UiRect::all(Val::Px(BORDER_MEDIUM)),
                ..default()
            },
            BackgroundColor(earth::TERRACOTTA),
            BorderColor(metal::BRONZE),
        ))
        .with_children(|paperdoll_parent| {
            // Title
            paperdoll_parent.spawn((
                Text::new("CHARACTER"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(metal::GOLD),
            ));

            // Character model on raffia mat
            paperdoll_parent
                .spawn((
                    CharacterModel,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(300.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.82, 0.71, 0.55)), // Raffia mat color
                ))
                .with_children(|model_parent| {
                    // Placeholder for 3D character model
                    model_parent.spawn((
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(240.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.4, 0.3, 0.25)),
                    ));
                });

            // Gear slots arranged around character
            spawn_gear_slots(paperdoll_parent);
        });
}

fn spawn_gear_slots(parent: &mut ChildBuilder) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(SPACING_SMALL),
            ..default()
        })
        .with_children(|slots_parent| {
            // Top row: Head, Neck
            spawn_gear_slot_row(
                slots_parent,
                vec![GearSlotType::Head, GearSlotType::Neck],
            );

            // Middle row: Chest
            spawn_gear_slot_row(
                slots_parent,
                vec![GearSlotType::Chest],
            );

            // Weapons row: MainHand, OffHand
            spawn_gear_slot_row(
                slots_parent,
                vec![GearSlotType::MainHand, GearSlotType::OffHand],
            );

            // Bottom row: Legs, Feet
            spawn_gear_slot_row(
                slots_parent,
                vec![GearSlotType::Legs, GearSlotType::Feet],
            );

            // Accessories: Rings
            spawn_gear_slot_row(
                slots_parent,
                vec![GearSlotType::Ring1, GearSlotType::Ring2],
            );
        });
}

fn spawn_gear_slot_row(parent: &mut ChildBuilder, slots: Vec<GearSlotType>) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(SPACING_SMALL),
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|row_parent| {
            for slot_type in slots {
                spawn_single_gear_slot(row_parent, slot_type);
            }
        });
}

fn spawn_single_gear_slot(parent: &mut ChildBuilder, slot_type: GearSlotType) {
    parent
        .spawn((
            GearSlot { slot_type },
            Button,
            Node {
                width: Val::Px(GEAR_SLOT_SIZE),
                height: Val::Px(GEAR_SLOT_SIZE),
                border: UiRect::all(Val::Px(BORDER_MEDIUM)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(wood::EBONY),
            BorderColor(metal::BRONZE), // Brass rings
            BorderRadius::all(Val::Px(4.0)),
        ))
        .with_children(|slot_parent| {
            // Slot label
            let label = match slot_type {
                GearSlotType::Head => "HEAD",
                GearSlotType::Neck => "NECK",
                GearSlotType::Chest => "BODY",
                GearSlotType::Hands => "HANDS",
                GearSlotType::MainHand => "WEAPON",
                GearSlotType::OffHand => "SHIELD",
                GearSlotType::Legs => "LEGS",
                GearSlotType::Feet => "FEET",
                GearSlotType::Ring1 | GearSlotType::Ring2 => "RING",
            };

            slot_parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 10.0,
                    ..default()
                },
                TextColor(metal::GOLD),
            ));
        });
}

// ============================================================================
// TRADING BOARD (Item Grid)
// ============================================================================

fn spawn_trading_board(parent: &mut ChildBuilder, inventory: &Inventory) {
    parent
        .spawn((
            TradingBoardPanel,
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(SPACING_MEDIUM),
                padding: UiRect::all(Val::Px(SPACING_MEDIUM)),
                border: UiRect::all(Val::Px(BORDER_THICK)),
                ..default()
            },
            BackgroundColor(wood::MAHOGANY),
            BorderColor(metal::BRONZE),
        ))
        .with_children(|board_parent| {
            // Title
            board_parent.spawn((
                Text::new("INVENTORY"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(metal::GOLD),
            ));

            // Grid of compartments (Mancala-style)
            board_parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(COMPARTMENT_GAP),
                    ..default()
                })
                .with_children(|grid_parent| {
                    for row in 0..GRID_ROWS {
                        spawn_compartment_row(grid_parent, row, inventory);
                    }
                });
        });
}

fn spawn_compartment_row(
    parent: &mut ChildBuilder,
    row: usize,
    inventory: &Inventory,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(COMPARTMENT_GAP),
            ..default()
        })
        .with_children(|row_parent| {
            for col in 0..GRID_COLS {
                let index = row * GRID_COLS + col;
                let item_stack = inventory.items.get(index);
                spawn_item_compartment(row_parent, index, item_stack);
            }
        });
}

fn spawn_item_compartment(
    parent: &mut ChildBuilder,
    index: usize,
    item_stack: Option<&ItemStack>,
) {
    parent
        .spawn((
            ItemCompartment { index },
            Button,
            Node {
                width: Val::Px(COMPARTMENT_SIZE),
                height: Val::Px(COMPARTMENT_SIZE),
                border: UiRect::all(Val::Px(BORDER_MEDIUM)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(wood::CARVED_LIGHT),
            BorderColor(wood::EBONY),
            BorderRadius::all(Val::Px(COMPARTMENT_SIZE / 2.0)), // Circular pits
        ))
        .with_children(|compartment| {
            if let Some(stack) = item_stack {
                // Item icon (gourd, cloth, metal texture based on item)
                compartment.spawn((
                    ItemIcon {
                        item_id: stack.item.id.clone(),
                    },
                    Text::new(get_tactile_icon(&stack.item.id)),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                ));

                // Quantity badge
                if stack.quantity > 1 {
                    compartment
                        .spawn((
                            Node {
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(4.0),
                                right: Val::Px(4.0),
                                width: Val::Px(20.0),
                                height: Val::Px(20.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BackgroundColor(metal::BRONZE),
                            BorderColor(metal::GOLD),
                            BorderRadius::all(Val::Px(10.0)),
                        ))
                        .with_children(|badge| {
                            badge.spawn((
                                ItemQuantityText,
                                Text::new(stack.quantity.to_string()),
                                TextFont {
                                    font_size: 12.0,
                                    ..default()
                                },
                                TextColor(wood::EBONY),
                            ));
                        });
                }
            }
        });
}

/// Get tactile icon representation based on item type
fn get_tactile_icon(item_id: &str) -> &'static str {
    match item_id {
        // Potions - Small gourds with stoppers
        id if id.contains("health") || id.contains("potion") => "🧪",
        id if id.contains("spirit") || id.contains("mana") => "🔮",

        // Food - Natural items
        id if id.contains("bread") => "🍞",
        id if id.contains("meat") => "🍖",

        // Weapons - Hammered/carved
        id if id.contains("sword") => "⚔️",
        id if id.contains("spear") => "🗡️",
        id if id.contains("axe") => "🪓",
        id if id.contains("staff") => "🪄",

        // Armor - Woven/tooled
        id if id.contains("helmet") => "⛑️",
        id if id.contains("armor") || id.contains("chest") => "🛡️",
        id if id.contains("boots") => "🥾",

        // Resources
        id if id.contains("wood") => "🪵",
        id if id.contains("stone") => "🪨",
        id if id.contains("herb") => "🌿",

        // Quest items
        id if id.contains("drum") => "🪘",
        id if id.contains("mask") => "🎭",
        id if id.contains("scroll") => "📜",

        _ => "📦",
    }
}

// ============================================================================
// CHARACTER CONTENT (Alternative view)
// ============================================================================

fn spawn_character_content(parent: &mut ChildBuilder) {
    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|content| {
            content.spawn((
                Text::new("CHARACTER STATS\n(Coming Soon)"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(metal::GOLD),
            ));
        });
}

// ============================================================================
// PLACEHOLDER CONTENT
// ============================================================================

fn spawn_placeholder_content(parent: &mut ChildBuilder, tab: InventoryTab) {
    let text = match tab {
        InventoryTab::Map => "MAP\n(Coming Soon)",
        InventoryTab::Quests => "QUEST LOG\n(Coming Soon)",
        InventoryTab::Settings => "SETTINGS\n(Coming Soon)",
        _ => "CONTENT\n(Coming Soon)",
    };

    parent
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|content| {
            content.spawn((
                Text::new(text),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(metal::GOLD),
            ));
        });
}

// ============================================================================
// INTERACTION SYSTEMS
// ============================================================================

pub fn handle_tab_clicks(
    mut inventory_state: ResMut<AncestralInventoryState>,
    tab_query: Query<(&Interaction, &TabButton), Changed<Interaction>>,
) {
    for (interaction, tab) in tab_query.iter() {
        if *interaction == Interaction::Pressed {
            inventory_state.current_tab = tab.tab_type;
        }
    }
}

pub fn handle_compartment_hover(
    mut inventory_state: ResMut<AncestralInventoryState>,
    compartment_query: Query<(&Interaction, &ItemCompartment), Changed<Interaction>>,
) {
    for (interaction, compartment) in compartment_query.iter() {
        match *interaction {
            Interaction::Hovered => {
                inventory_state.hovered_compartment = Some(compartment.index);
            }
            Interaction::None => {
                if inventory_state.hovered_compartment == Some(compartment.index) {
                    inventory_state.hovered_compartment = None;
                }
            }
            _ => {}
        }
    }
}

pub fn handle_compartment_clicks(
    mut inventory_state: ResMut<AncestralInventoryState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    compartment_query: Query<(&Interaction, &ItemCompartment)>,
) {
    for (interaction, compartment) in compartment_query.iter() {
        if *interaction == Interaction::Hovered {
            // Left click - select/use
            if mouse_button.just_pressed(MouseButton::Left) {
                inventory_state.selected_compartment = Some(compartment.index);
            }

            // Right click - context menu
            if mouse_button.just_pressed(MouseButton::Right) {
                inventory_state.show_context_menu = true;
                inventory_state.context_menu_item = Some(compartment.index);
            }
        }
    }
}
