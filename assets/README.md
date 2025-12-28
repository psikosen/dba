# Shaman's Journey - Placeholder Assets

## Overview

These are **temporary placeholder assets** generated for development and testing. They consist of simple colored rectangles that allow the game to be visually playable while proper art is being created.

## Generated Assets (107 total)

### Player Sprites (1)
- `sprites/player/player_idle.png` - Green rectangle (32×32)

### Monster Sprites (72)
9 monsters × 8 states each:

**Monsters**: Lion, Hyena, Serpent, Vulture, Crocodile, Leopard, Elephant, Monkey, Rhino

**States**:
- **Stable** - Base color for each monster
- **Chaos** - Red (corrupted/aggressive)
- **Corrupt** - Purple (heavily corrupted)
- **Harmony** - Light blue (balanced/peaceful)
- **Spirit** - Pale blue (spiritual form)
- **Tamed** - Light green (controlled by player)
- **Enraged** - Orange (very aggressive)
- **Purified** - Pale yellow (cleansed)

### NPC Sprites (6)
- `sprites/npcs/elder.png` - Purple
- `sprites/npcs/brother.png` - Red
- `sprites/npcs/merchant.png` - Yellow
- `sprites/npcs/healer.png` - Teal
- `sprites/npcs/child.png` - Peach
- `sprites/npcs/shaman.png` - Blue

### Item Sprites (9)
- Spirit Orbs: Small, Medium, Large (light blue, 16×16)
- `health_herb.png` - Green (16×16)
- `stamina_root.png` - Orange (16×16)
- `kora.png` - Brown (24×24)
- `ngoni.png` - Dark brown (24×24)
- `corruption_shard.png` - Purple (16×16)
- `purification_crystal.png` - Pale yellow (16×16)

### UI Elements (10)
- Health bar: fill (red) and background (100×10)
- Spirit bar: fill (blue) and background (100×10)
- Stamina bar: fill (orange) and background (100×10)
- Buttons: normal, hover, pressed (64×32)
- Panel: background (200×200)

### Tile Sprites (9)
All 32×32 pixels:
- `grass.png` - Green
- `dirt.png` - Brown
- `water.png` - Blue
- `corruption_light.png` - Light purple
- `corruption_medium.png` - Medium purple
- `corruption_heavy.png` - Dark purple
- `corruption_extreme.png` - Very dark purple
- `purified.png` - Pale yellow/white
- `spirit_world.png` - Pale blue

## Color Coding

The placeholder colors are chosen to be **visually distinct** for testing:

- **Green** - Player, health, nature
- **Red** - Chaos, danger, combat
- **Purple** - Corruption, dark magic
- **Blue** - Spirit, harmony, water
- **Yellow/Orange** - Stamina, energy, purification
- **Brown** - NPCs, earth, items

## Regenerating Placeholders

If you need to regenerate or modify the placeholders:

```bash
python3 assets/PLACEHOLDER_GENERATOR.py
```

The generator script creates:
- Colored rectangles with borders
- Optional labels (first letter of filename)
- Proper dimensions for each asset type

## Next Steps

These placeholders allow you to:
- ✅ Test all game systems visually
- ✅ Verify sprite loading and rendering
- ✅ Debug state transitions (watch color changes)
- ✅ Develop UI layout and positioning

**For production**, replace these with proper pixel art or commissioned assets.

## Asset Requirements

See `ASSET_REQUIREMENTS.md` in the project root for detailed specifications for production assets.

---

**Generated**: 2025-12-28
**Tool**: PLACEHOLDER_GENERATOR.py
**Purpose**: Development and testing only
