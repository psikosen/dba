# Dialogue System Guide

This guide explains how to use the CrossCode-inspired dialogue system with character portraits.

## Overview

The dialogue system provides:
- **CrossCode-style UI** with floating character portraits
- **Emotional expressions** based on NPC state
- **Semi-transparent dialogue boxes** with sci-fi aesthetic
- **Cinematic presentation** with bottom gradient
- **Bark text** for quick reactions

## Architecture

### Components

**NpcName** (`bevy_shaman_story::components::NpcName`)
```rust
#[derive(Component, Clone)]
pub struct NpcName {
    pub name: String,
    pub current_emotion: PortraitEmotion,
}
```

**NpcDialogue** (`bevy_shaman_story::components::NpcDialogue`)
```rust
#[derive(Component)]
pub struct NpcDialogue {
    pub full_dialogue: String,
    pub partial_dialogue: Option<String>,
    pub sick_dialogue: String,
}
```

**NpcSicknessState** (`bevy_shaman_story::components::NpcSicknessState`)
```rust
pub enum NpcSicknessState {
    AsleepSick,  // Shows sick_dialogue
    Waking,      // Shows partial_dialogue
    Awake,       // Shows full_dialogue
}
```

### Resources

**PortraitDB** - Manages character portraits
```rust
pub struct PortraitDB {
    pub portraits: HashMap<(String, PortraitEmotion), Handle<Image>>,
}
```

**AfricanNamesDB** - Database of character names with cultural information
```rust
pub struct AfricanNamesDB {
    pub names: HashMap<String, (CharacterType, String, String)>,
}
```

**DialogueUIState** - Tracks active dialogue
```rust
pub struct DialogueUIState {
    pub active: bool,
    pub npc_entity: Option<Entity>,
    pub npc_name: String,
    pub dialogue_text: String,
    pub npc_portrait_side: PortraitSide,
}
```

### Events

**DialogueRequested** (`bevy_shaman_core::events::DialogueRequested`)
```rust
pub struct DialogueRequested {
    pub npc_entity: Entity,
    pub player_entity: Entity,
}
```

Triggered when player presses 'E' near an NPC (within 1.5 tiles).

## Creating NPCs with Dialogue

### Basic NPC Setup

```rust
fn spawn_npc(mut commands: Commands) {
    commands.spawn((
        // Core components
        GridPosition { x: 10, y: 10 },

        // NPC identity
        NpcName {
            name: "Chike".to_string(),
            current_emotion: PortraitEmotion::Neutral,
        },

        // Dialogue content
        NpcDialogue {
            full_dialogue: "Welcome, traveler! The village has been plagued by darkness.".to_string(),
            partial_dialogue: Some("The... darkness... fading...".to_string()),
            sick_dialogue: "... ... ...".to_string(),
        },

        // Initial state
        NpcSicknessState::AsleepSick,
    ));
}
```

### Using African Names

The system includes 30+ traditional African names. Access them via `AfricanNamesDB`:

```rust
fn setup_npc_with_cultural_name(
    mut commands: Commands,
    names_db: Res<AfricanNamesDB>,
) {
    // Get a name with cultural info
    if let Some((char_type, origin, meaning)) = names_db.names.get("Chike") {
        info!("Spawning {} - {} ({})", "Chike", origin, meaning);
    }

    commands.spawn((
        NpcName {
            name: "Chike".to_string(), // "Power of God" (Igbo, Nigeria)
            current_emotion: PortraitEmotion::Neutral,
        },
        // ... other components
    ));
}
```

Available names include:
- **Bosses**: Anansi, Mami Wata, Jengu, Oya, Eshu, Shango, etc.
- **Elders**: Chike, Nala, Tendaji
- **Merchants**: Kamari, Zola, Thabo, Ife, Sefu, Azizi
- **Villagers**: 20+ characters from various African cultures

See `crates/bevy_shaman_story/src/resources.rs` for the complete list.

## Portrait System

### Portrait Structure

Portraits are stored in:
```
assets/portraits/
├── characters/    # Main characters (Kwame, brothers)
├── bosses/        # Boss characters
└── npcs/          # Village NPCs
```

### Naming Convention

Format: `{character_name}_{emotion}.png`

Examples:
- `chike_neutral.png`
- `chike_happy.png`
- `anansi_threatening.png`
- `kofi_sick.png`

### Loading Portraits

```rust
fn load_portraits(
    asset_server: Res<AssetServer>,
    mut portrait_db: ResMut<PortraitDB>,
) {
    // Load a specific portrait
    let handle = asset_server.load("portraits/npcs/chike_neutral.png");
    portrait_db.add_portrait(
        "Chike".to_string(),
        PortraitEmotion::Neutral,
        handle
    );

    // Load multiple emotions for one character
    for emotion in [
        PortraitEmotion::Neutral,
        PortraitEmotion::Happy,
        PortraitEmotion::Worried,
    ] {
        let path = format!(
            "portraits/npcs/chike_{}.png",
            emotion_to_string(emotion)
        );
        let handle = asset_server.load(path);
        portrait_db.add_portrait("Chike".to_string(), emotion, handle);
    }
}

fn emotion_to_string(emotion: PortraitEmotion) -> &'static str {
    match emotion {
        PortraitEmotion::Neutral => "neutral",
        PortraitEmotion::Happy => "happy",
        PortraitEmotion::Sad => "sad",
        PortraitEmotion::Angry => "angry",
        PortraitEmotion::Surprised => "surprised",
        PortraitEmotion::Worried => "worried",
        PortraitEmotion::Sick => "sick",
        PortraitEmotion::Waking => "waking",
        // ... etc
    }
}
```

### Changing NPC Emotions

```rust
fn update_npc_emotion(
    mut npc_query: Query<&mut NpcName>,
    npc_entity: Entity,
) {
    if let Ok(mut npc_name) = npc_query.get_mut(npc_entity) {
        npc_name.current_emotion = PortraitEmotion::Happy;
    }
}
```

The dialogue UI will automatically use the current emotion when displaying the portrait.

## Triggering Dialogue

### Automatic (Player Interaction)

The system automatically triggers dialogue when:
1. Player is within 1.5 tiles of NPC
2. Player presses 'E' key
3. `DialogueRequested` event is emitted

### Manual (Cutscenes)

```rust
fn trigger_cutscene_dialogue(
    mut dialogue_events: EventWriter<DialogueRequested>,
    npc_query: Query<Entity, With<NpcName>>,
    player_query: Query<Entity, With<Player>>,
) {
    let player = player_query.single();

    // Find specific NPC
    for npc_entity in npc_query.iter() {
        // Trigger dialogue
        dialogue_events.send(DialogueRequested {
            npc_entity,
            player_entity: player,
        });
        break;
    }
}
```

## Portrait Specifications

### Image Requirements

- **Format**: PNG with alpha transparency
- **Resolution**: 512x512 pixels (high-res for dialogue busts)
- **Style**: Pixel art, waist-up cutout
- **Background**: Fully transparent
- **Art Style**: Consistent across all characters

### Recommended Emotions

**For NPCs:**
- neutral, happy, sad, worried, sick, waking

**For Bosses:**
- neutral, angry, laughing, threatening, defeated, enraged

**For Main Characters:**
- neutral, happy, sad, angry, surprised, determined

## UI Controls

- **Close Dialogue**: Press `ESC`
- **Interact with NPC**: Press `E` when near NPC

## Visual Style

The dialogue UI follows CrossCode's design:

1. **Floating Portrait Busts** - Large 512x512 portraits anchored to bottom corners
2. **Semi-transparent Dialogue Box** - Dark grey (85% opacity) with sci-fi borders
3. **Cinematic Gradient** - Bottom 30% of screen darkened for readability
4. **Speaker Handle** - Light blue bracket connecting dialogue to portrait
5. **Character Name** - Displayed above dialogue text
6. **Continue Prompt** - "Press ESC to close" at bottom

## Bark Text (Reactions)

For quick reactions without full dialogue:

```rust
use bevy_shaman_ui::systems::dialogue_ui::{spawn_bark_text, PortraitSide};

fn show_npc_reaction(mut commands: Commands) {
    spawn_bark_text(
        commands,
        "Chike",
        "[nods]",
        PortraitSide::Left,
    );
}
```

Bark text:
- Displays for 2 seconds
- Small black box near portrait
- Used for non-verbal reactions

## Advanced: State-Based Dialogue

NPCs can have different dialogue based on their sickness state:

```rust
NpcDialogue {
    // Shown when NpcSicknessState::Awake
    full_dialogue: "The corruption has lifted! Thank you, shaman!".to_string(),

    // Shown when NpcSicknessState::Waking
    partial_dialogue: Some("I... I can feel... the light...".to_string()),

    // Shown when NpcSicknessState::AsleepSick
    sick_dialogue: "... ... ...".to_string(),
}
```

The system automatically selects the appropriate dialogue based on `NpcSicknessState`.

## Example: Complete NPC Setup

```rust
fn spawn_elder_chike(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut portrait_db: ResMut<PortraitDB>,
) {
    // Load portraits
    for emotion in [PortraitEmotion::Neutral, PortraitEmotion::Worried] {
        let path = format!("portraits/npcs/chike_{:?}.png", emotion).to_lowercase();
        let handle = asset_server.load(path);
        portrait_db.add_portrait("Chike".to_string(), emotion, handle);
    }

    // Spawn NPC
    commands.spawn((
        GridPosition { x: 15, y: 20 },

        NpcName {
            name: "Chike".to_string(),
            current_emotion: PortraitEmotion::Worried,
        },

        NpcDialogue {
            full_dialogue: "The darkness came suddenly, young shaman. Our people fell ill one by one. We need your help to restore balance.".to_string(),
            partial_dialogue: Some("The spirits... they whisper of hope...".to_string()),
            sick_dialogue: "... ... ...".to_string(),
        },

        NpcSicknessState::Awake,
    ));
}
```

## Troubleshooting

### Portrait not showing
- Check file path: `assets/portraits/{folder}/{name}_{emotion}.png`
- Verify image is 512x512 PNG with transparency
- Ensure portrait was loaded into `PortraitDB`
- Check console for asset loading errors

### Dialogue not triggering
- Verify NPC has `NpcDialogue` component
- Check player distance (must be ≤ 1.5 tiles)
- Ensure NPC has `GridPosition` component
- Look for `DialogueRequested` event emission

### Wrong dialogue showing
- Check `NpcSicknessState` matches expected state
- Verify `partial_dialogue` is set (or it defaults to `full_dialogue`)
- Review dialogue text in `NpcDialogue` component

## Performance Notes

- Portraits are loaded once and reused
- UI spawns/despawns on dialogue open/close
- Minimap updates based on player movement
- No performance impact when dialogue closed

## Future Enhancements

Potential additions:
- Typewriter text effect
- Multiple choice dialogue options
- Dialogue history/log
- Voice acting support
- Animated portrait expressions
- Portrait side switching based on position
