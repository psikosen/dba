use bevy::prelude::*;
use crate::{TutorialProgress, TutorialMissionRegistry, UiHighlightZone};

/// Tutorial overlay system
/// Provides visual hints and highlights without text dumps
/// Mission-based approach: show by doing, not by telling

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_tutorial_overlay,
                animate_highlight_pulse,
            ));
    }
}

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct TutorialOverlayRoot;

#[derive(Component)]
pub struct TutorialHintText {
    pub duration: f32,
    pub elapsed: f32,
}

#[derive(Component)]
pub struct TutorialHighlight {
    pub zone: UiHighlightZone,
    pub pulse_timer: f32,
}

#[derive(Component)]
pub struct TutorialArrow {
    pub target_position: Vec2,
    pub bounce_timer: f32,
}

// ============================================================================
// SYSTEMS
// ============================================================================

fn update_tutorial_overlay(
    mut commands: Commands,
    progress: Res<TutorialProgress>,
    missions: Res<TutorialMissionRegistry>,
    existing_overlays: Query<Entity, With<TutorialOverlayRoot>>,
    asset_server: Res<AssetServer>,
) {
    if progress.tutorial_completed {
        // Clean up any existing overlays
        for entity in existing_overlays.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }

    let Some(mission_id) = &progress.current_mission else {
        return;
    };

    let Some(mission) = missions.get(mission_id) else {
        return;
    };

    let step_idx = progress.current_step as usize;
    let Some(step) = mission.steps.get(step_idx) else {
        return;
    };

    // Clean up old overlay
    for entity in existing_overlays.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Create new overlay if there's a hint
    if let Some(hint) = &step.hint {
        spawn_tutorial_hint(&mut commands, &asset_server, hint, step.ui_highlight);
    }

    // Create highlight if specified
    if let Some(highlight_zone) = step.ui_highlight {
        spawn_tutorial_highlight(&mut commands, highlight_zone);
    }
}

fn animate_highlight_pulse(
    time: Res<Time>,
    mut highlights: Query<(&mut TutorialHighlight, &mut BackgroundColor)>,
) {
    for (mut highlight, mut bg_color) in highlights.iter_mut() {
        highlight.pulse_timer += time.delta_secs() * 2.0;

        // Pulse between 0.3 and 0.7 alpha
        let alpha = 0.5 + (highlight.pulse_timer.sin() * 0.2);
        bg_color.0.set_alpha(alpha);
    }
}

// ============================================================================
// SPAWN FUNCTIONS
// ============================================================================

fn spawn_tutorial_hint(
    commands: &mut Commands,
    #[allow(unused_variables)]
    asset_server: &AssetServer,
    hint_text: &str,
    highlight_zone: Option<UiHighlightZone>,
) {
    // Position hint based on highlighted zone
    let (top, left) = match highlight_zone {
        Some(UiHighlightZone::HealthBar) => (Val::Px(80.0), Val::Px(20.0)),
        Some(UiHighlightZone::SpiritBar) => (Val::Px(110.0), Val::Px(20.0)),
        Some(UiHighlightZone::StaminaBar) => (Val::Px(140.0), Val::Px(20.0)),
        Some(UiHighlightZone::RhythmIndicator) => (Val::Px(50.0), Val::Percent(50.0)),
        Some(UiHighlightZone::ComboDisplay) => (Val::Px(200.0), Val::Px(20.0)),
        Some(UiHighlightZone::Minimap) => (Val::Px(20.0), Val::Percent(80.0)),
        Some(UiHighlightZone::Inventory) => (Val::Percent(50.0), Val::Percent(50.0)),
        _ => (Val::Px(50.0), Val::Percent(50.0)), // Default center-ish
    };

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top,
            left,
            padding: UiRect::all(Val::Px(12.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        TutorialOverlayRoot,
    )).with_children(|parent| {
        parent.spawn((
            Text::new(hint_text),
            TextFont {
                font_size: 18.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.6)), // Slight yellow tint
        ));
    });
}

fn spawn_tutorial_highlight(
    commands: &mut Commands,
    zone: UiHighlightZone,
) {
    let (width, height, top, left) = match zone {
        UiHighlightZone::HealthBar => (Val::Px(200.0), Val::Px(30.0), Val::Px(50.0), Val::Px(20.0)),
        UiHighlightZone::SpiritBar => (Val::Px(200.0), Val::Px(30.0), Val::Px(80.0), Val::Px(20.0)),
        UiHighlightZone::StaminaBar => (Val::Px(200.0), Val::Px(30.0), Val::Px(110.0), Val::Px(20.0)),
        UiHighlightZone::RhythmIndicator => (Val::Px(400.0), Val::Px(100.0), Val::Px(30.0), Val::Percent(45.0)),
        UiHighlightZone::ComboDisplay => (Val::Px(150.0), Val::Px(80.0), Val::Px(180.0), Val::Px(20.0)),
        UiHighlightZone::Minimap => (Val::Px(250.0), Val::Px(250.0), Val::Px(20.0), Val::Percent(75.0)),
        UiHighlightZone::Inventory => (Val::Px(600.0), Val::Px(400.0), Val::Percent(30.0), Val::Percent(25.0)),
        UiHighlightZone::Custom(x, y, w, h) => (Val::Px(w), Val::Px(h), Val::Px(y), Val::Px(x)),
    };

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width,
            height,
            top,
            left,
            border: UiRect::all(Val::Px(3.0)),
            ..default()
        },
        BorderColor(Color::srgb(1.0, 0.8, 0.0)), // Gold border
        BackgroundColor(Color::srgba(1.0, 1.0, 0.0, 0.3)), // Transparent yellow
        TutorialHighlight {
            zone,
            pulse_timer: 0.0,
        },
        TutorialOverlayRoot,
    ));
}

/// Spawn an animated arrow pointing to a specific screen position
pub fn spawn_tutorial_arrow(
    commands: &mut Commands,
    #[allow(unused_variables)]
    asset_server: &AssetServer,
    target_position: Vec2,
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(target_position.x - 20.0),
            top: Val::Px(target_position.y - 40.0),
            width: Val::Px(40.0),
            height: Val::Px(40.0),
            ..default()
        },
        BackgroundColor(Color::srgb(1.0, 1.0, 0.0)),
        // TODO: Use actual arrow image when asset system is ready
        TutorialArrow {
            target_position,
            bounce_timer: 0.0,
        },
        TutorialOverlayRoot,
    ));
}

/// Animate tutorial arrows with a bouncing motion
pub fn animate_tutorial_arrows(
    time: Res<Time>,
    mut arrows: Query<(&mut TutorialArrow, &mut Node)>,
) {
    for (mut arrow, mut style) in arrows.iter_mut() {
        arrow.bounce_timer += time.delta_secs() * 3.0;

        // Bounce up and down
        let offset = arrow.bounce_timer.sin() * 10.0;
        style.top = Val::Px(arrow.target_position.y - 40.0 + offset);
    }
}
