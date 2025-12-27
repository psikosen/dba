use bevy::prelude::*;
use crate::TutorialEvent;

/// Cutscene system for tutorial sequences
/// Handles image flashes, transitions, and narrative moments

pub struct CutscenePlugin;

impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_active_cutscenes,
                handle_cutscene_input,
            ))
            .add_event::<CutsceneStartEvent>()
            .add_event::<CutsceneEndEvent>();
    }
}

// ============================================================================
// COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct CutsceneRoot;

#[derive(Component)]
pub struct CutsceneImage {
    pub flash_duration: f32,
    pub elapsed: f32,
}

#[derive(Component)]
pub struct CutsceneText {
    pub display_duration: f32,
    pub elapsed: f32,
}

#[derive(Component)]
pub struct CutsceneFadeOverlay {
    pub fade_in: bool,
    pub duration: f32,
    pub elapsed: f32,
}

// ============================================================================
// RESOURCES
// ============================================================================

#[derive(Resource)]
pub struct ActiveCutscene {
    pub id: String,
    pub current_frame: usize,
    pub frames: Vec<CutsceneFrame>,
    pub auto_advance: bool,
}

#[derive(Clone)]
pub struct CutsceneFrame {
    pub frame_type: CutsceneFrameType,
    pub duration: f32,
    pub text: Option<String>,
}

#[derive(Clone)]
pub enum CutsceneFrameType {
    ImageFlash(String),      // Image path to flash on screen
    BlackScreen,             // Fade to black
    Text(String),            // Display text overlay
    FadeIn,                  // Fade in from black
    FadeOut,                 // Fade out to black
    Wait,                    // Just wait (for player input or time)
}

// ============================================================================
// EVENTS
// ============================================================================

#[derive(Event)]
pub struct CutsceneStartEvent {
    pub cutscene_id: String,
}

#[derive(Event)]
pub struct CutsceneEndEvent {
    pub cutscene_id: String,
}

// ============================================================================
// PREDEFINED CUTSCENES
// ============================================================================

/// Dream Sequence - Grotesque Ball
pub fn create_dream_grotesque_ball_cutscene() -> ActiveCutscene {
    ActiveCutscene {
        id: "dream_grotesque_ball".to_string(),
        current_frame: 0,
        auto_advance: true,
        frames: vec![
            CutsceneFrame {
                frame_type: CutsceneFrameType::FadeIn,
                duration: 1.0,
                text: None,
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::ImageFlash("cutscenes/grotesque_ball.png".to_string()),
                duration: 0.3,
                text: Some("A grotesque ball with teeth...".to_string()),
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::BlackScreen,
                duration: 0.2,
                text: None,
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::ImageFlash("cutscenes/grotesque_ball_chomping.png".to_string()),
                duration: 0.5,
                text: Some("Gnawing at you, chomping pieces off...".to_string()),
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::BlackScreen,
                duration: 0.3,
                text: None,
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::FadeOut,
                duration: 1.0,
                text: Some("The nightmare continues...".to_string()),
            },
        ],
    }
}

/// Dream Sequence - Claws
pub fn create_dream_claws_cutscene() -> ActiveCutscene {
    ActiveCutscene {
        id: "dream_claws".to_string(),
        current_frame: 0,
        auto_advance: true,
        frames: vec![
            CutsceneFrame {
                frame_type: CutsceneFrameType::FadeIn,
                duration: 0.5,
                text: None,
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::ImageFlash("cutscenes/long_clawed_hands.png".to_string()),
                duration: 0.4,
                text: Some("Irregularly long hands with claws...".to_string()),
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::BlackScreen,
                duration: 0.2,
                text: None,
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::ImageFlash("cutscenes/hands_pulling.png".to_string()),
                duration: 0.6,
                text: Some("Pulling you into the forest...".to_string()),
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::BlackScreen,
                duration: 0.3,
                text: None,
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::Text("Press any key to wake up".to_string()),
                duration: 0.0, // Wait for input
                text: Some("...".to_string()),
            },
        ],
    }
}

/// Four Spirits Battle Cutscene
pub fn create_four_spirits_battle_cutscene() -> ActiveCutscene {
    ActiveCutscene {
        id: "four_spirits_battle".to_string(),
        current_frame: 0,
        auto_advance: true,
        frames: vec![
            CutsceneFrame {
                frame_type: CutsceneFrameType::FadeIn,
                duration: 1.0,
                text: Some("Four powerful spirits clash before you...".to_string()),
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::ImageFlash("cutscenes/angelic_spirit.png".to_string()),
                duration: 2.0,
                text: Some("The Angelic Spirit radiates pure light...".to_string()),
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::ImageFlash("cutscenes/neutral_spirit.png".to_string()),
                duration: 2.0,
                text: Some("The Neutral Spirit's eyes see all futures...".to_string()),
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::ImageFlash("cutscenes/chaotic_spirit.png".to_string()),
                duration: 2.0,
                text: Some("The Chaotic Spirit brings decay and rot...".to_string()),
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::ImageFlash("cutscenes/dark_spirit.png".to_string()),
                duration: 2.0,
                text: Some("The Dark Spirit feeds on evil deeds...".to_string()),
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::ImageFlash("cutscenes/four_spirits_combined.png".to_string()),
                duration: 3.0,
                text: Some("They all turn to you. Each wants your aid.".to_string()),
            },
            CutsceneFrame {
                frame_type: CutsceneFrameType::FadeOut,
                duration: 1.0,
                text: None,
            },
        ],
    }
}

// ============================================================================
// SYSTEMS
// ============================================================================

fn update_active_cutscenes(
    time: Res<Time>,
    active_cutscene: Option<ResMut<ActiveCutscene>>,
    mut commands: Commands,
    mut tutorial_events: EventWriter<TutorialEvent>,
    mut end_events: EventWriter<CutsceneEndEvent>,
    asset_server: Res<AssetServer>,
    existing_roots: Query<Entity, With<CutsceneRoot>>,
) {
    let Some(mut cutscene) = active_cutscene else {
        return;
    };

    if cutscene.current_frame >= cutscene.frames.len() {
        // Cutscene complete
        let cutscene_id = cutscene.id.clone();

        // Clean up UI
        for entity in existing_roots.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Send events
        tutorial_events.send(TutorialEvent::CutsceneCompleted(cutscene_id.clone()));
        end_events.send(CutsceneEndEvent { cutscene_id });

        // Remove resource
        commands.remove_resource::<ActiveCutscene>();
        return;
    }

    // Get current frame index before borrowing
    let current_frame_idx = cutscene.current_frame;
    let auto_advance = cutscene.auto_advance;

    if current_frame_idx < cutscene.frames.len() {
        let frame = &mut cutscene.frames[current_frame_idx];

        // Load cutscene assets (images, audio, sound effects) for the current frame
        match &frame.frame_type {
            CutsceneFrameType::ImageFlash(image_path) => {
                // Load cutscene image asset
                let image_handle: Handle<Image> = asset_server.load(image_path.clone());
                info!("Loading cutscene image asset: {}", image_path);

                // Preload associated audio if it exists
                let audio_path = image_path.replace(".png", ".ogg");
                // Audio handle will be loaded on demand by the audio system
                info!("Associated audio will be loaded on demand: {}", audio_path);

                // Trigger image display with the loaded asset
                commands.trigger(CutsceneImageLoadedEvent {
                    image_handle,
                    duration: frame.duration,
                });
            }
            CutsceneFrameType::FadeIn | CutsceneFrameType::FadeOut => {
                // Trigger fade sound effect event
                commands.trigger(CutsceneFadeSoundEvent {
                    fade_type: if matches!(frame.frame_type, CutsceneFrameType::FadeIn) {
                        "fade_in"
                    } else {
                        "fade_out"
                    }.to_string(),
                    volume: 0.3,
                });
            }
            _ => {}
        }

        // Auto-advance after duration
        if auto_advance {
            if frame.duration > 0.0 {
                frame.duration -= time.delta_secs();
                if frame.duration <= 0.0 {
                    cutscene.current_frame += 1;
                }
            }
        }
    }
}

// Cutscene asset loading events
#[derive(Event)]
pub struct CutsceneImageLoadedEvent {
    pub image_handle: Handle<Image>,
    pub duration: f32,
}

#[derive(Event)]
pub struct CutsceneFadeSoundEvent {
    pub fade_type: String,
    pub volume: f32,
}

#[derive(Event)]
pub struct CutsceneImageSoundEvent {
    pub image_path: String,
    pub volume: f32,
}

fn handle_cutscene_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    active_cutscene: Option<ResMut<ActiveCutscene>>,
) {
    let Some(mut cutscene) = active_cutscene else {
        return;
    };

    // Allow player to skip/advance cutscene with Space or Enter
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
        if cutscene.current_frame < cutscene.frames.len() {
            let frame = &cutscene.frames[cutscene.current_frame];

            // If frame is waiting for input (duration 0), advance
            if frame.duration <= 0.0 {
                cutscene.current_frame += 1;
            } else {
                // Skip current frame
                cutscene.current_frame += 1;
            }
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn spawn_cutscene(
    cutscene_id: &str,
    commands: &mut Commands,
) -> Option<ActiveCutscene> {
    // Spawn visual entities for cutscene effects (fade overlays, image flashes, text boxes, UI hints)
    let cutscene = match cutscene_id {
        "dream_grotesque_ball" => {
            // Spawn UI hints for dream sequence
            commands.trigger(SpawnCutsceneUiEvent {
                ui_type: CutsceneUiType::DreamWarning,
                text: "Press SPACE to skip".to_string(),
            });
            Some(create_dream_grotesque_ball_cutscene())
        }
        "dream_claws" => {
            // Spawn UI hints and arrows
            commands.trigger(SpawnCutsceneUiEvent {
                ui_type: CutsceneUiType::InputPrompt,
                text: "Press any key to wake up".to_string(),
            });
            Some(create_dream_claws_cutscene())
        }
        "four_spirits_battle" => {
            // Spawn choice highlights and arrows pointing to spirit options
            commands.trigger(SpawnCutsceneUiEvent {
                ui_type: CutsceneUiType::ChoiceHighlight,
                text: "Choose your spirit ally carefully...".to_string(),
            });

            // Spawn arrow indicators for each spirit option
            for (i, spirit) in ["Angelic", "Neutral", "Chaotic", "Dark"].iter().enumerate() {
                commands.trigger(SpawnSpiritChoiceArrowEvent {
                    spirit_name: spirit.to_string(),
                    position_index: i,
                    highlight_color: match *spirit {
                        "Angelic" => Color::srgb(1.0, 1.0, 0.8),
                        "Neutral" => Color::srgb(0.8, 0.8, 0.8),
                        "Chaotic" => Color::srgb(0.6, 0.8, 0.4),
                        "Dark" => Color::srgb(0.4, 0.2, 0.4),
                        _ => Color::WHITE,
                    },
                });
            }

            Some(create_four_spirits_battle_cutscene())
        }
        _ => None,
    };

    // Spawn fade overlay for all cutscenes
    if cutscene.is_some() {
        commands.trigger(SpawnCutsceneOverlayEvent {
            overlay_type: CutsceneOverlayType::FadeOverlay,
            initial_alpha: 0.0,
            target_alpha: 1.0,
            duration: 1.0,
        });
    }

    cutscene
}

// Cutscene UI events
#[derive(Event)]
pub struct SpawnCutsceneUiEvent {
    pub ui_type: CutsceneUiType,
    pub text: String,
}

#[derive(Debug, Clone)]
pub enum CutsceneUiType {
    DreamWarning,
    InputPrompt,
    ChoiceHighlight,
}

#[derive(Event)]
pub struct SpawnSpiritChoiceArrowEvent {
    pub spirit_name: String,
    pub position_index: usize,
    pub highlight_color: Color,
}

#[derive(Event)]
pub struct SpawnCutsceneOverlayEvent {
    pub overlay_type: CutsceneOverlayType,
    pub initial_alpha: f32,
    pub target_alpha: f32,
    pub duration: f32,
}

#[derive(Debug, Clone)]
pub enum CutsceneOverlayType {
    FadeOverlay,
    VignetteEffect,
    ColorGrade,
}

/// Render cutscene UI (called from update system)
pub fn render_cutscene_frame(
    commands: &mut Commands,
    asset_server: &AssetServer,
    frame: &CutsceneFrame,
    existing_roots: &Query<Entity, With<CutsceneRoot>>,
) {
    // Clean up previous frame
    for entity in existing_roots.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Create root node
    let root = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        CutsceneRoot,
    )).id();

    match &frame.frame_type {
        CutsceneFrameType::ImageFlash(image_path) => {
            // Load frame-specific image asset (dream images, narrative visuals)
            let image_handle: Handle<Image> = asset_server.load(image_path.clone());
            info!("Rendering cutscene frame with image: {}", image_path);

            // Spawn image entity with the loaded asset
            commands.entity(root).with_children(|parent| {
                parent.spawn((
                    ImageNode {
                        image: image_handle,
                        ..default()
                    },
                    Node {
                        width: Val::Percent(80.0),
                        height: Val::Percent(80.0),
                        ..default()
                    },
                    CutsceneImage {
                        flash_duration: frame.duration,
                        elapsed: 0.0,
                    },
                ));
            });

            // Trigger sound effect for this image (loaded by audio system)
            commands.trigger(CutsceneImageSoundEvent {
                image_path: image_path.clone(),
                volume: 0.5,
            });
        }
        CutsceneFrameType::Text(text) => {
            // Spawn text
            commands.entity(root).with_children(|parent| {
                parent.spawn((
                    Text::new(text.clone()),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        }
        CutsceneFrameType::BlackScreen => {
            // Just black overlay (already on root)
        }
        _ => {}
    }

    // Add text overlay if present
    if let Some(text) = &frame.text {
        commands.entity(root).with_children(|parent| {
            parent.spawn((
                Text::new(text.clone()),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(50.0),
                    ..default()
                },
            ));
        });
    }
}
