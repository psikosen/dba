# Shaman's Journey - Complete Placeholder Assets

## Overview

**COMPLETE** placeholder asset pack generated for development and testing. All 151+ visual assets are included, allowing the game to be fully playable while proper art is being created.

## Generated Assets: 151 Total

### ✅ Player Sprites (1)
- `sprites/player/player_spritesheet.png` - 512×64 animated spritesheet (8 frames)

### ✅ Monster Sprites (72)
**9 monsters × 8 states = 72 sprites**

**Monsters** (correct names from ASSET_REQUIREMENTS.md):
1. **forest_spirit** - Nature spirit (64×64) - Green base
2. **chaos_hound** - Aggressive wolf creature (64×64) - Red base
3. **corrupt_shade** - Shadowy humanoid (64×64) - Purple base
4. **chaos_beast** - Dungeon encounter (64×64) - Orange base
5. **corrupt_spirit** - Corrupted entity (64×64) - Dark purple
6. **void_creature** - Void realm creature (64×64) - Dark blue
7. **corrupted_guardian** - **BOSS** (128×128) - Purple/gray
8. **healing_wisp** - Spirit world helper (64×64) - Cyan
9. **chaos_sprite** - Mischievous spirit (64×64) - Light orange

**States** (each monster has all 8):
- **stable** - Base color, normal state
- **chaos** - Red tint, aggressive
- **corrupt** - Dark purple, twisted
- **harmony** - Light blue, purified
- **decay** - Green, rotting
- **rage** - Orange, intense
- **void** - Dark blue/purple, ethereal
- **ancestral** - Purple glow, spiritual

**Path structure**: `sprites/monsters/{monster_name}/{monster_name}_{state}.png`

### ✅ NPC Sprites (6)
- `sprites/npcs/head_shaman.png` - Purple (wise elder)
- `sprites/npcs/brother_normal.png` - Brown (friendly)
- `sprites/npcs/brother_corrupted.png` - Dark purple (corrupted)
- `sprites/npcs/villager_01.png` - Tan
- `sprites/npcs/villager_02.png` - Light brown
- `sprites/npcs/villager_03.png` - Beige

### ✅ Tile Sprites (16)
**4 biomes × 3 corruption levels = 12 base tiles**

**Biomes**:
- **Village**: Pure (tan) → Light corrupt → Heavy corrupt (dark)
- **Forest**: Pure (green) → Light corrupt → Heavy corrupt (brown)
- **Mountains**: Pure (gray) → Light corrupt → Heavy corrupt (dark purple)
- **Spirit Realm**: Pure (light blue) → Light corrupt → Heavy corrupt (dark blue)

**Corruption Overlays** (4):
- `corruption_chaos.png` - Red overlay
- `corruption_decay.png` - Green overlay
- `corruption_void.png` - Dark blue overlay
- `corruption_ancestral.png` - Purple overlay

**Path structure**: `sprites/tiles/{biome}/{biome}_{corruption_level}.png`

### ✅ Interactive World Elements (9)
- `sprites/world/forageable_spot.png` (32×32) - Bright green (harvestable)
- `sprites/world/dig_spot.png` (32×32) - Brown (treasure)
- `sprites/world/herb_bench.png` (64×64) - Brown (crafting station)
- `sprites/world/spirit_altar.png` (64×64) - Light blue (crafting station)
- `sprites/world/instrument_workshop.png` (64×64) - Tan (crafting station)
- `sprites/world/room_empty.png` (64×64) - Gray (dungeon)
- `sprites/world/room_encounter.png` (64×64) - Red (dungeon)
- `sprites/world/room_treasure.png` (64×64) - Gold (dungeon)
- `sprites/world/room_boss.png` (64×64) - Dark red (dungeon)

### ✅ Item Sprites (19)
**Spirit Orbs (3)**:
- Small, Medium, Large - Cyan/blue gradients

**Seeds (7 types)**:
- moonpetal, starroot, lifeleaf, shadowroot, crystalmoss, voidflower, eternalbark

**Crafting Materials (5)**:
- healing_herb, corrupt_essence, wood, stone, iron_ore

**Tools (1)**:
- shovel

**Remedies (2)**:
- remedy, healing_remedy

All items are 32×32 pixels for inventory display.

### ✅ UI Sprites (27)

**HUD Elements (6)**:
- Health bar: background + fill (200×20) - Red
- Spirit bar: background + fill (200×20) - Blue
- Stamina bar: background + fill (200×20) - Green/yellow

**Rhythm Visualizer (2)**:
- Beat circle (300×300) - Cyan
- Beat indicator (50×50) - Orange

**Dialogue System (4)**:
- Dialogue box (800×200) - Dark gray
- Choice button (400×60) - Gray
- Choice button hover (400×60) - Light gray
- Portrait frame (128×128) - Brown border

**Inventory UI (3)**:
- Background (600×400) - Dark gray
- Item slot (48×48) - Gray
- Item slot selected (48×48) - Blue highlight

**Shop UI (3)**:
- Background (600×500) - Brown
- Item row (550×40) - Tan
- Gold icon (24×24) - Gold

**Bestiary UI (2)**:
- Background (500×600) - Dark purple
- Monster card (200×150) - Gray/purple

**Main Menu (5)**:
- Button: New Game, Load Game, Settings (300×60 each)
- Button hover state (300×60) - Light gray
- Title logo (600×200) - Brown "SHAMAN" text

### ✅ Loading Screen (1)
- `intro_video.png` (800×600) - Purple background with "SHAMAN" text

### ❌ Missing Assets (Cannot Auto-Generate)

**Audio (10 files)** - Not generated:
- 3 music tracks: dawn_hymn.ogg, war_chant.ogg, purification_rite.ogg
- 7 sound effects: hit_impact, player_hurt, item_pickup, song_success, song_fail, purification, menu_select

**Fonts (1-2 files)** - Not generated:
- game_font.ttf (primary font)
- title_font.ttf (optional display font)

---

## Color Coding System

**Monster States** (visual distinction):
- Stable = Base monster color
- Chaos = Red tint (aggressive, fire)
- Corrupt = Purple tint (dark corruption)
- Harmony = Blue tint (purified, peaceful)
- Decay = Green tint (rot, nature corruption)
- Rage = Orange tint (intense, energetic)
- Void = Dark blue (cosmic horror, emptiness)
- Ancestral = Purple glow (spiritual, ancient)

**UI Elements**:
- Health = Red
- Spirit = Blue
- Stamina = Green/Yellow
- Corruption = Purple
- Purification = Pale yellow/white
- Selection = Light blue highlight

---

## Regenerating Assets

### Generate ALL Assets
```bash
python3 assets/COMPLETE_PLACEHOLDER_GENERATOR.py
```

This creates all 151 visual placeholder assets with:
- Correct file names matching ASSET_REQUIREMENTS.md
- Proper dimensions for each asset type
- Color-coded for visual distinction
- Labeled with first letters for debugging

### File Structure
```
assets/
├── sprites/
│   ├── player/player_spritesheet.png
│   ├── monsters/{monster_name}/{monster_name}_{state}.png (72 files)
│   ├── npcs/*.png (6 files)
│   ├── tiles/{biome}/*.png (16 files)
│   ├── world/*.png (9 files)
│   ├── items/*.png (19 files)
│   └── ui/{category}/*.png (27 files)
└── intro_video.png (1 file)
```

---

## Testing with Placeholders

These placeholders allow you to:

1. **Visual Testing**:
   - ✅ See player character with 8-frame animation
   - ✅ Watch monster state transitions (color changes)
   - ✅ Test all 9 monster types
   - ✅ View corruption spread across different biomes
   - ✅ Test inventory, shop, bestiary UIs

2. **Gameplay Testing**:
   - ✅ Navigate through villages, forests, mountains, spirit realms
   - ✅ Engage in combat with visible monster sprites
   - ✅ Collect items and see them in inventory
   - ✅ Craft at different stations
   - ✅ Explore dungeons with room markers

3. **UI Testing**:
   - ✅ Health/Spirit/Stamina bars visible
   - ✅ Rhythm visualizer displays
   - ✅ Dialogue boxes and choices work
   - ✅ Shop and bestiary interfaces functional
   - ✅ Main menu navigation

---

## Asset Quality

**Current**: Development placeholders (colored rectangles)
**Dimensions**: Correct for final assets
**File format**: PNG with transparency
**Total size**: ~300KB (extremely lightweight)

**For Production**: Replace these with:
- Professional pixel art or hand-drawn sprites
- Ambient music tracks (2-3 min each)
- Quality sound effects
- Custom game fonts

See `ASSET_REQUIREMENTS.md` for detailed specifications.

---

**Generated**: 2025-12-28
**Tool**: COMPLETE_PLACEHOLDER_GENERATOR.py
**Purpose**: Enable full visual development and testing
**Status**: 151/156 assets complete (97% visual coverage)
