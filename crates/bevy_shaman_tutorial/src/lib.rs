use bevy::prelude::*;
use bevy_shaman_core::states::GameState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod cutscene;
pub mod missions;
pub mod overlay;

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<TutorialProgress>()
            .init_resource::<TutorialSettings>()
            .init_resource::<TutorialMissionRegistry>()

            // Plugins
            .add_plugins((
                cutscene::CutscenePlugin,
                overlay::OverlayPlugin,
            ))

            // Systems
            .add_systems(OnEnter(GameState::Playing), setup_tutorial)
            .add_systems(
                Update,
                (
                    check_tutorial_conditions,
                    update_tutorial_ui,
                    handle_skip_tutorial,
                    track_combat_events,
                    track_purification_events,
                ).run_if(in_state(GameState::Playing))
            )

            // Events
            .add_event::<TutorialEvent>()
            .add_event::<TutorialStepCompleted>()
            .add_event::<TutorialMissionCompleted>();
    }
}

// ============================================================================
// CORE COMPONENTS
// ============================================================================

#[derive(Component, Debug, Clone)]
pub struct TutorialEntity;

#[derive(Component, Debug, Clone)]
pub struct TutorialMarker {
    pub mission_id: String,
    pub step_id: u8,
}

// ============================================================================
// RESOURCES
// ============================================================================

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct TutorialProgress {
    pub tutorial_started: bool,
    pub tutorial_completed: bool,
    pub current_mission: Option<String>,
    pub current_step: u8,
    pub completed_missions: Vec<String>,
    pub mission_flags: HashMap<String, bool>,
    pub cutscene_viewed: HashMap<String, bool>,
}

impl Default for TutorialProgress {
    fn default() -> Self {
        Self {
            tutorial_started: false,
            tutorial_completed: false,
            current_mission: None,
            current_step: 0,
            completed_missions: Vec::new(),
            mission_flags: HashMap::new(),
            cutscene_viewed: HashMap::new(),
        }
    }
}

impl TutorialProgress {
    pub fn start_mission(&mut self, mission_id: &str) {
        self.tutorial_started = true;
        self.current_mission = Some(mission_id.to_string());
        self.current_step = 0;
    }

    pub fn complete_step(&mut self) {
        self.current_step += 1;
    }

    pub fn complete_mission(&mut self, mission_id: &str) {
        self.completed_missions.push(mission_id.to_string());
        self.current_mission = None;
        self.current_step = 0;
    }

    pub fn set_flag(&mut self, flag: &str, value: bool) {
        self.mission_flags.insert(flag.to_string(), value);
    }

    pub fn get_flag(&self, flag: &str) -> bool {
        self.mission_flags.get(flag).copied().unwrap_or(false)
    }

    pub fn mark_cutscene_viewed(&mut self, cutscene_id: &str) {
        self.cutscene_viewed.insert(cutscene_id.to_string(), true);
    }

    pub fn has_viewed_cutscene(&self, cutscene_id: &str) -> bool {
        self.cutscene_viewed.get(cutscene_id).copied().unwrap_or(false)
    }
}

#[derive(Resource, Debug, Clone)]
pub struct TutorialSettings {
    pub skip_tutorial: bool,
    pub show_hints: bool,
    pub hint_duration: f32,
}

impl Default for TutorialSettings {
    fn default() -> Self {
        Self {
            skip_tutorial: false,
            show_hints: true,
            hint_duration: 5.0,
        }
    }
}

// ============================================================================
// MISSION DEFINITIONS
// ============================================================================

#[derive(Debug, Clone)]
pub struct TutorialMission {
    pub id: String,
    pub title: String,
    pub description: String,
    pub steps: Vec<TutorialStep>,
    pub required_flags: Vec<String>,
    pub reward_xp: u32,
}

#[derive(Debug, Clone)]
pub struct TutorialStep {
    pub step_id: u8,
    pub objective: String,
    pub hint: Option<String>,
    pub condition: TutorialCondition,
    pub dialogue: Option<String>,
    pub ui_highlight: Option<UiHighlightZone>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TutorialCondition {
    DefeatMonster(usize),           // Number of monsters to defeat
    LandCombo(usize),               // Combo length to achieve
    ReachPosition(i32, i32),        // Grid position to reach
    TriggerRhythmAttack(usize),     // Number of rhythm attacks
    PurifyTiles(usize),             // Number of corrupted tiles to purify
    InteractWithNpc(String),        // NPC name/id
    CollectItems(String, u32),      // Item name and count
    WaitForDialogue,                // Wait for dialogue to complete
    WaitForCutscene(String),        // Wait for cutscene to finish
    Custom(String),                 // Custom flag-based condition
}

#[derive(Debug, Clone, Copy)]
pub enum UiHighlightZone {
    HealthBar,
    SpiritBar,
    StaminaBar,
    RhythmIndicator,
    ComboDisplay,
    Minimap,
    Inventory,
    Custom(f32, f32, f32, f32), // x, y, width, height
}

// ============================================================================
// EVENTS
// ============================================================================

#[derive(Event, Debug, Clone)]
pub enum TutorialEvent {
    MonsterDefeated,
    ComboLanded(usize),
    RhythmAttackTriggered,
    TilePurified,
    DialogueCompleted(String),
    CutsceneCompleted(String),
    FlagSet(String, bool),
}

#[derive(Event, Debug, Clone)]
pub struct TutorialStepCompleted {
    pub mission_id: String,
    pub step_id: u8,
}

#[derive(Event, Debug, Clone)]
pub struct TutorialMissionCompleted {
    pub mission_id: String,
}

// ============================================================================
// SYSTEMS
// ============================================================================

fn setup_tutorial(
    mut progress: ResMut<TutorialProgress>,
    settings: Res<TutorialSettings>,
) {
    if settings.skip_tutorial {
        progress.tutorial_completed = true;
        info!("Tutorial skipped by user settings");
        return;
    }

    if !progress.tutorial_started {
        info!("Starting tutorial sequence");
        // The dream cutscene will be triggered separately
        progress.set_flag("ready_for_dream_cutscene", true);
    }
}

fn check_tutorial_conditions(
    mut progress: ResMut<TutorialProgress>,
    mut tutorial_events: EventReader<TutorialEvent>,
    mut step_completed: EventWriter<TutorialStepCompleted>,
    mut mission_completed: EventWriter<TutorialMissionCompleted>,
    missions: Res<TutorialMissionRegistry>,
) {
    if progress.tutorial_completed {
        return;
    }

    let Some(current_mission_id) = &progress.current_mission.clone() else {
        return;
    };

    let Some(mission) = missions.get(current_mission_id) else {
        warn!("Tutorial mission not found: {}", current_mission_id);
        return;
    };

    let current_step_idx = progress.current_step as usize;
    if current_step_idx >= mission.steps.len() {
        // Mission complete
        progress.complete_mission(current_mission_id);
        mission_completed.send(TutorialMissionCompleted {
            mission_id: current_mission_id.clone(),
        });
        info!("Tutorial mission completed: {}", current_mission_id);
        return;
    }

    let step = &mission.steps[current_step_idx];

    // Check if step condition is met
    for event in tutorial_events.read() {
        if is_condition_met(&step.condition, event, &progress) {
            info!("Tutorial step completed: {}.{}", current_mission_id, step.step_id);
            step_completed.send(TutorialStepCompleted {
                mission_id: current_mission_id.clone(),
                step_id: step.step_id,
            });
            progress.complete_step();
            break;
        }
    }
}

fn is_condition_met(
    condition: &TutorialCondition,
    event: &TutorialEvent,
    progress: &TutorialProgress,
) -> bool {
    match (condition, event) {
        (TutorialCondition::DefeatMonster(_), TutorialEvent::MonsterDefeated) => true,
        (TutorialCondition::LandCombo(required), TutorialEvent::ComboLanded(actual)) => {
            actual >= required
        }
        (TutorialCondition::TriggerRhythmAttack(_), TutorialEvent::RhythmAttackTriggered) => true,
        (TutorialCondition::PurifyTiles(_), TutorialEvent::TilePurified) => true,
        (TutorialCondition::InteractWithNpc(npc), TutorialEvent::DialogueCompleted(completed_npc)) => {
            npc == completed_npc
        }
        (TutorialCondition::WaitForCutscene(cutscene_id), TutorialEvent::CutsceneCompleted(completed_id)) => {
            cutscene_id == completed_id
        }
        (TutorialCondition::Custom(flag), TutorialEvent::FlagSet(set_flag, value)) => {
            flag == set_flag && *value
        }
        _ => false,
    }
}

fn update_tutorial_ui(
    progress: Res<TutorialProgress>,
    settings: Res<TutorialSettings>,
    missions: Res<TutorialMissionRegistry>,
    commands: Commands,
) {
    if !settings.show_hints || progress.tutorial_completed {
        return;
    }

    // This will be expanded with actual UI rendering
    // For now, just log the current objective
    if let Some(mission_id) = &progress.current_mission {
        if let Some(mission) = missions.get(mission_id) {
            let step_idx = progress.current_step as usize;
            if let Some(step) = mission.steps.get(step_idx) {
                // UI overlay will be rendered here in overlay.rs
                // For now, just track that we need to display it
            }
        }
    }
}

fn handle_skip_tutorial(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut progress: ResMut<TutorialProgress>,
    mut settings: ResMut<TutorialSettings>,
) {
    // Press ESC + T to skip tutorial
    if keyboard.pressed(KeyCode::Escape) && keyboard.just_pressed(KeyCode::KeyT) {
        info!("Skipping tutorial");
        settings.skip_tutorial = true;
        progress.tutorial_completed = true;
    }
}

// ============================================================================
// EVENT TRACKING SYSTEMS
// ============================================================================

/// Track combat events and convert them to tutorial events
fn track_combat_events(
    mut tutorial_events: EventWriter<TutorialEvent>,
    mut hit_events: EventReader<bevy_shaman_combat::systems::events::HitLanded>,
    mut combo_query: Query<&bevy_shaman_combat::components::RhythmCombo>,
) {
    // Track monster defeats
    for event in hit_events.read() {
        // Check if target was defeated (health <= 0)
        // This is a simplified check - in reality you'd query the target's health
        if event.damage > 0.0 {
            tutorial_events.send(TutorialEvent::MonsterDefeated);
        }
    }

    // Track combos
    if let Ok(combo) = combo_query.get_single_mut() {
        if combo.current_combo.len() >= 3 {
            tutorial_events.send(TutorialEvent::ComboLanded(combo.current_combo.len()));
        }
    }

    // Track rhythm attacks (when player attacks on beat)
    for event in hit_events.read() {
        tutorial_events.send(TutorialEvent::RhythmAttackTriggered);
    }
}

/// Track purification events
fn track_purification_events(
    mut tutorial_events: EventWriter<TutorialEvent>,
    corruption_query: Query<&bevy_shaman_world::components::TileCorruption, Changed<bevy_shaman_world::components::TileCorruption>>,
) {
    // Track when tiles are purified (corruption reduced)
    for corruption in corruption_query.iter() {
        if corruption.level == 0.0 {
            tutorial_events.send(TutorialEvent::TilePurified);
        }
    }
}

// ============================================================================
// MISSION REGISTRY
// ============================================================================

#[derive(Resource)]
pub struct TutorialMissionRegistry {
    missions: HashMap<String, TutorialMission>,
}

impl Default for TutorialMissionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl TutorialMissionRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            missions: HashMap::new(),
        };

        // Register all tutorial missions
        registry.register(missions::create_dream_cutscene_mission());
        registry.register(missions::create_exam_mission());
        registry.register(missions::create_purification_mission());
        registry.register(missions::create_spirit_choice_mission());
        registry.register(missions::create_basic_combat_mission());
        registry.register(missions::create_village_corruption_mission());
        registry.register(missions::create_land_grant_mission());

        registry
    }

    pub fn register(&mut self, mission: TutorialMission) {
        self.missions.insert(mission.id.clone(), mission);
    }

    pub fn get(&self, id: &str) -> Option<&TutorialMission> {
        self.missions.get(id)
    }
}
