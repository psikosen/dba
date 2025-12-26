# Tutorial System

Mission-based tutorial system for Shaman's Journey that teaches gameplay mechanics through story-driven missions rather than text explanations.

## Philosophy: Show, Don't Tell

This tutorial system follows a **mission-based approach** where players learn by doing:
- ✅ Guided missions that require using game mechanics
- ✅ Visual hints and highlights on UI elements
- ✅ Story integration (tutorials are part of the narrative)
- ❌ NO text dumps explaining mechanics
- ❌ NO interrupting popups with instructions

## Architecture

### Core Components

```rust
TutorialProgress      // Resource tracking current mission/step progress
TutorialMission       // Definition of a tutorial mission with steps
TutorialStep          // Individual objective within a mission
TutorialCondition     // Completion criteria (defeat X enemies, reach position, etc.)
```

### Tutorial Flow

```
Dream Cutscene → Spirit Choice → Exam Mission → Purification Training → Village Corruption → ...
```

Each mission advances the story while teaching a specific mechanic.

## Tutorial Missions

### 1. Dream Cutscene (`dream_cutscene`)
- **Teaches**: Nothing (pure story)
- **Shows**: Grotesque ball with teeth, clawed hands
- **Purpose**: Set the tone, introduce MC's connection to spirits

### 2. Spirit Choice (`spirit_choice`)
- **Teaches**: Dialogue choices (when implemented)
- **Determines**: Starting spirit alignment and final boss
- **Choices**: Angelic, Neutral, Chaotic, or Dark spirit

### 3. Shaman Exam (`shaman_exam`)
- **Teaches**:
  - Rhythm combat system
  - Spirit control mechanic
  - Combo system (3-hit chains)
- **Objective**: Defeat lesser demon while resisting mental corruption

### 4. Purification Training (`purification_training`)
- **Teaches**:
  - Purifying corrupted tiles
  - Aura system
  - Fighting spirit creatures
- **Objective**: Cleanse spoiled ground and defeat manifested spirits

### 5. Basic Combat (`basic_combat`)
- **Teaches**:
  - Movement (WASD)
  - Auto-spirit companion
  - Spirit-imbued spear attacks
  - AOE spin attack
- **Objective**: Defeat training minions

### 6. Village Corruption (`village_corruption`)
- **Teaches**: Story progression, NPC interaction
- **Objective**: Discover the curse, decide to become a shaman
- **Triggers**: Act 1 beginning

### 7. Land Grant (`land_grant`)
- **Teaches**: Base building, farming, spirit storage
- **Objective**: Set up your shaman land

## Cutscene System

Located in `cutscene.rs`, handles:
- Image flash sequences
- Fade in/out transitions
- Timed frames with text overlays
- Player-paced advancement (Space/Enter to continue)

### Predefined Cutscenes
- `dream_grotesque_ball` - Opening nightmare (teeth/chomping)
- `dream_claws` - Clawed hands pulling into forest
- `four_spirits_battle` - Spirit choice sequence

### Adding New Cutscenes

```rust
pub fn create_my_cutscene() -> ActiveCutscene {
    ActiveCutscene {
        id: "my_cutscene".to_string(),
        current_frame: 0,
        auto_advance: true,
        frames: vec![
            CutsceneFrame {
                frame_type: CutsceneFrameType::ImageFlash("path/to/image.png".to_string()),
                duration: 2.0,
                text: Some("Overlay text".to_string()),
            },
            // ... more frames
        ],
    }
}
```

## Overlay System

Located in `overlay.rs`, provides visual guidance:

### UI Highlights
- Pulses UI zones with yellow glow
- Zones: HealthBar, SpiritBar, RhythmIndicator, ComboDisplay, etc.
- Non-intrusive (semi-transparent)

### Tutorial Hints
- Small text boxes positioned near relevant UI
- Auto-fade after duration
- Only shown when `TutorialSettings.show_hints` is true

### Tutorial Arrows
- Animated bouncing arrows pointing to targets
- Used for directing attention to specific screen areas

## Integration with Game Systems

### Combat Integration
The tutorial system listens for combat events:
```rust
TutorialEvent::MonsterDefeated
TutorialEvent::ComboLanded(length)
TutorialEvent::RhythmAttackTriggered
```

Track these by sending tutorial events from combat systems when relevant actions occur.

### Purification Integration
```rust
TutorialEvent::TilePurified
```

Send when a corrupted tile is cleansed.

### Dialogue Integration
```rust
TutorialEvent::DialogueCompleted(npc_name)
```

Send when NPC dialogue finishes.

### Save Integration
Tutorial progress is automatically saved as part of `SaveData`:
```rust
pub struct SaveData {
    // ... other fields
    pub tutorial_progress: TutorialProgressData,
}
```

## Skip Tutorial

Players can skip tutorials:
1. **In Settings**: `TutorialSettings.skip_tutorial = true`
2. **Keyboard**: Press `ESC + T` during gameplay

Skipping sets `tutorial_completed = true` and disables all tutorial systems.

## Adding New Tutorial Missions

1. **Define the mission** in `missions.rs`:
```rust
pub fn create_my_mission() -> TutorialMission {
    TutorialMission {
        id: "my_mission".to_string(),
        title: "Mission Title".to_string(),
        description: "Brief description".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Do something".to_string(),
                hint: Some("Helpful hint".to_string()),
                condition: TutorialCondition::DefeatMonster(3),
                dialogue: Some("NPC says this".to_string()),
                ui_highlight: Some(UiHighlightZone::HealthBar),
            },
        ],
        required_flags: vec!["previous_mission_complete".to_string()],
        reward_xp: 100,
    }
}
```

2. **Register it** in `lib.rs`:
```rust
impl TutorialMissionRegistry {
    pub fn new() -> Self {
        // ...
        registry.register(missions::create_my_mission());
        // ...
    }
}
```

3. **Trigger it** from your game logic:
```rust
fn my_system(mut progress: ResMut<TutorialProgress>) {
    if should_start_mission() {
        progress.start_mission("my_mission");
    }
}
```

## Story Integration

The tutorial missions follow this narrative arc:

**Prologue**: Dream → Spirit Choice → Wake in village

**Act 1**: Exam → Purification Training → Village Corruption Discovery → Land Grant → Shaman Training

Each mission advances the story while teaching mechanics. The tutorial is the story.

## Asset Requirements

Place cutscene images in:
```
assets/cutscenes/
├── grotesque_ball.png
├── grotesque_ball_chomping.png
├── long_clawed_hands.png
├── hands_pulling.png
├── angelic_spirit.png
├── neutral_spirit.png
├── chaotic_spirit.png
├── dark_spirit.png
└── four_spirits_combined.png
```

UI assets:
```
assets/ui/
└── tutorial_arrow.png
```

## Debug Commands (TODO)

Planned commands for testing:
```
/tutorial start <mission_id>    - Start specific mission
/tutorial skip                  - Skip to end of current mission
/tutorial complete              - Mark tutorial as complete
/tutorial reset                 - Reset all tutorial progress
```

## Future Enhancements

- [ ] Multiple choice dialogue system integration
- [ ] Quest markers on minimap for tutorial objectives
- [ ] Spirit World visual effects tutorial
- [ ] Advanced combat tutorials (Act 2+)
- [ ] Building/crafting tutorials
- [ ] Taming advanced mechanics
- [ ] Rival shaman competition tutorials
- [ ] Festival event tutorials

## Technical Notes

- Tutorial state is saved/loaded automatically
- Runs only during `GameState::Playing`
- Modular design - easy to disable entire system
- Event-driven - decoupled from game systems
- Mission registry allows runtime mission registration
- Cutscenes are data-driven (easy to add more)
