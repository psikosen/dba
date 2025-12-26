# Shaman's Journey - Documentation

Welcome to the Shaman's Journey documentation. This guide covers the game's systems and how to use them.

## Game Systems

### UI & Presentation
- **[Dialogue System](DIALOGUE_SYSTEM.md)** - CrossCode-style dialogue with character portraits
- **[Minimap System](MINIMAP_SYSTEM.md)** - Exploration tracking with fog of war

### Core Gameplay
- **Combat System** - Rhythm-based combat with timing windows
- **Monster System** - State-based monster behaviors and taming
- **Dungeon System** - Procedural generation and boss encounters

### Progression
- **Story System** - NPC dialogue, quests, and brother cleansing
- **Inventory System** - Item management and crafting
- **Save System** - Game state persistence

## Quick Start Guides

### Setting Up NPCs with Dialogue

```rust
use bevy_shaman_story::components::{NpcName, NpcDialogue, NpcSicknessState};
use bevy_shaman_story::resources::PortraitEmotion;
use bevy_shaman_core::components::GridPosition;

fn spawn_village_elder(mut commands: Commands) {
    commands.spawn((
        GridPosition { x: 15, y: 20 },
        NpcName {
            name: "Chike".to_string(),
            current_emotion: PortraitEmotion::Neutral,
        },
        NpcDialogue {
            full_dialogue: "Welcome, young shaman.".to_string(),
            partial_dialogue: None,
            sick_dialogue: "... ... ...".to_string(),
        },
        NpcSicknessState::Awake,
    ));
}
```

See **[Dialogue System](DIALOGUE_SYSTEM.md)** for complete guide.

### Adding Character Portraits

1. Create 512x512 PNG with transparency
2. Save to `assets/portraits/{category}/{name}_{emotion}.png`
3. Load in portrait database:

```rust
fn load_portrait(
    asset_server: Res<AssetServer>,
    mut portrait_db: ResMut<PortraitDB>,
) {
    let handle = asset_server.load("portraits/npcs/chike_neutral.png");
    portrait_db.add_portrait("Chike".to_string(), PortraitEmotion::Neutral, handle);
}
```

### Customizing the Minimap

```rust
fn setup_minimap(mut minimap_state: ResMut<MinimapState>) {
    minimap_state.view_radius = 15;  // Larger exploration radius
    minimap_state.visible = true;     // Show by default
}
```

See **[Minimap System](MINIMAP_SYSTEM.md)** for advanced customization.

## Asset Requirements

### Portraits
- **Format**: PNG with transparency
- **Size**: 512x512 pixels
- **Style**: Pixel art, waist-up cutout
- **Location**: `assets/portraits/{characters|bosses|npcs}/`

See `assets/portraits/README.md` for complete specifications.

### Sprites
- **Monsters**: 32x32 per frame
- **Player**: 32x32 per frame
- **Tiles**: 32x32 static

## African Cultural Names

The game features authentic African names with cultural significance:

### Characters
- **Kwame** (Akan, Ghana) - "Born on Saturday" - The Shaman
- **Kofi** (Akan, Ghana) - "Born on Friday" - Brother

### Bosses
- **Anansi** (Akan, Ghana) - Spider Trickster Spirit
- **Mami Wata** (Pan-African) - Mother Water Spirit
- **Shango** (Yoruba, Nigeria) - Thunder God

### NPCs
- **Chike** (Igbo, Nigeria) - "Power of God" - Village Elder
- **Nala** (Swahili) - "Gift" - Wise Woman
- **Kamari** (Swahili) - "Like the moon" - Merchant

See `assets/portraits/{folder}/README.md` for complete lists with origins and meanings.

## Architecture Overview

### Crate Structure

```
bevy_shaman/              # Main executable
├── bevy_shaman_core/     # Movement, camera, grid, events
├── bevy_shaman_combat/   # Rhythm combat, damage, effects
├── bevy_shaman_audio/    # Beat clock, rhythm timing
├── bevy_shaman_monsters/ # Monster AI, states, sprite swapping
├── bevy_shaman_minions/  # Taming, formations, commands
├── bevy_shaman_world/    # Tiles, corruption, purification
├── bevy_shaman_dungeons/ # Procedural generation, bosses
├── bevy_shaman_items/    # Inventory, crafting, pickups
├── bevy_shaman_shop/     # Currency, shops, trading
├── bevy_shaman_ui/       # HUD, menus, dialogue, minimap
├── bevy_shaman_story/    # NPCs, quests, dialogue content
└── bevy_shaman_save/     # Save/load game state
```

### Event Flow

```
Player Input (E key near NPC)
    ↓
DialogueRequested event
    ↓
Dialogue UI spawns
    ↓
Shows: Portrait + Name + Dialogue
    ↓
Press ESC to close
    ↓
Dialogue UI despawns
```

### State Machine

```rust
pub enum GameState {
    Boot,       // Loading screen
    MainMenu,   // Main menu
    Playing,    // Active gameplay
    Paused,     // Paused state
    Dialogue,   // In dialogue (future)
    Cutscene,   // Cutscene playing (future)
}
```

## Development Status

| System | Status | Notes |
|--------|--------|-------|
| Core Gameplay | ✅ 100% | Movement, camera, grid complete |
| Combat System | ✅ 90% | Rhythm combat functional |
| Dialogue UI | ⚠️ 80% | UI complete, needs portrait assets |
| Minimap | ✅ 100% | Fog of war fully functional |
| Monster System | ⚠️ 85% | Sprite swapping needs work |
| Inventory UI | ⚠️ 30% | Backend done, UI missing |
| Save/Load | ⚠️ 30% | Saving works, loading partial |
| LLM Integration | ❌ 0% | gemma3:270m not integrated |

## Contributing

When adding new features:

1. **Follow naming conventions** - Use African names from the database
2. **Document systems** - Add markdown docs to this folder
3. **Add examples** - Include code examples in documentation
4. **Test thoroughly** - Verify compilation and runtime behavior
5. **Update READMEs** - Keep asset requirements current

## Resources

- [Bevy Engine Documentation](https://bevyengine.org/)
- [CrossCode (Inspiration)](http://www.cross-code.com/)
- African Cultural Names - See portrait READMEs

## Getting Help

For questions about:
- **Dialogue System** → See [DIALOGUE_SYSTEM.md](DIALOGUE_SYSTEM.md)
- **Minimap** → See [MINIMAP_SYSTEM.md](MINIMAP_SYSTEM.md)
- **General Issues** → Check GitHub issues
- **Asset Creation** → See `assets/portraits/README.md`

---

**Last Updated**: December 2024
**Game Version**: 0.1.0
**Bevy Version**: 0.15.3
