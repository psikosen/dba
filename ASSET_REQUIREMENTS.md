# Asset Requirements for Shaman Game

This document specifies all assets (sprites, audio, fonts, etc.) needed for the game to function. All assets should be placed in the `assets/` directory with the structure outlined below.

## Directory Structure

```
assets/
├── sprites/
│   ├── player/
│   ├── monsters/
│   ├── npcs/
│   ├── tiles/
│   ├── items/
│   └── ui/
├── audio/
│   ├── music/
│   └── sfx/
├── fonts/
└── intro_video.png
```

---

## 1. PLAYER SPRITES

### Player Character Spritesheet
- **File**: `assets/sprites/player/player_spritesheet.png`
- **Dimensions**: 512x64 pixels (8 frames @ 64x64 each)
- **Format**: Horizontal spritesheet
- **Animation Frames**:
  - Frames 1-2: Standing/idle
  - Frames 3-5: Walking cycle
  - Frames 6-8: Playing instrument/attacking
- **Notes**: Character should represent a West African shaman with traditional attire

---

## 2. MONSTER SPRITES

Each monster requires 8 state variations. All sprites should be 64x64 pixels.

### State Types (8 per monster)
1. **Stable** - Normal, balanced appearance
2. **Chaos** - Red-tinted, aggressive pose
3. **Corrupt** - Dark, twisted appearance
4. **Harmony** - Purified, glowing, peaceful
5. **Decay** - Green-tinted, rotting appearance
6. **Rage** - Intense, energetic pose
7. **Void** - Dark blue/purple, ethereal
8. **Ancestral** - Purple-tinted, spiritual glow

### Monster List

#### Forest Spirit (Tameable)
- **Base File**: `assets/sprites/monsters/forest_spirit/`
- Files needed:
  - `forest_spirit_stable.png` (64x64) - Gentle nature spirit
  - `forest_spirit_chaos.png` (64x64) - Aggressive, red-tinted
  - `forest_spirit_corrupt.png` (64x64) - Darkened, twisted
  - `forest_spirit_harmony.png` (64x64) - Glowing green/white
  - `forest_spirit_decay.png` (64x64) - Withered, moss-covered
  - `forest_spirit_rage.png` (64x64) - Intense, flaming
  - `forest_spirit_void.png` (64x64) - Shadowy, ethereal
  - `forest_spirit_ancestral.png` (64x64) - Purple glow, wise appearance

#### Chaos Hound (Aggressive)
- **Base File**: `assets/sprites/monsters/chaos_hound/`
- Files needed:
  - `chaos_hound_stable.png` (64x64) - Wolf-like creature, balanced
  - `chaos_hound_chaos.png` (64x64) - Fiery red, snarling (DEFAULT)
  - `chaos_hound_corrupt.png` (64x64) - Dark corrupted hound
  - `chaos_hound_harmony.png` (64x64) - Calmed, white-blue glow
  - `chaos_hound_decay.png` (64x64) - Decaying flesh, green
  - `chaos_hound_rage.png` (64x64) - Berserk, flames
  - `chaos_hound_void.png` (64x64) - Shadow hound
  - `chaos_hound_ancestral.png` (64x64) - Spirit hound, purple

#### Corrupt Shade (Enemy)
- **Base File**: `assets/sprites/monsters/corrupt_shade/`
- Files needed:
  - `corrupt_shade_stable.png` (64x64) - Shadowy humanoid
  - `corrupt_shade_chaos.png` (64x64) - Red-tinged shadow
  - `corrupt_shade_corrupt.png` (64x64) - Dark, oozing corruption (DEFAULT)
  - `corrupt_shade_harmony.png` (64x64) - Translucent, peaceful
  - `corrupt_shade_decay.png` (64x64) - Rotting shadow
  - `corrupt_shade_rage.png` (64x64) - Violent, swirling
  - `corrupt_shade_void.png` (64x64) - Pure void essence
  - `corrupt_shade_ancestral.png` (64x64) - Ancient spirit form

#### Chaos Beast (Dungeon Encounter)
- **Base File**: `assets/sprites/monsters/chaos_beast/`
- Files needed: (Same 8 states as above)
- **Description**: Large, aggressive creature found in dungeons

#### Corrupt Spirit (Dungeon Encounter)
- **Base File**: `assets/sprites/monsters/corrupt_spirit/`
- Files needed: (Same 8 states as above)
- **Description**: Corrupted spiritual entity in dungeons

#### Void Creature (Dungeon Encounter)
- **Base File**: `assets/sprites/monsters/void_creature/`
- Files needed: (Same 8 states as above)
- **Description**: Creature from the void realm

#### Corrupted Guardian (Boss)
- **Base File**: `assets/sprites/monsters/corrupted_guardian/`
- Files needed: (Same 8 states as above)
- **Dimensions**: 128x128 pixels (larger boss sprite)
- **Description**: Massive corrupted boss enemy

#### Healing Wisp (Spirit World)
- **Base File**: `assets/sprites/monsters/healing_wisp/`
- Files needed: (Same 8 states as above)
- **Description**: Small, glowing healing spirit

#### Chaos Sprite (Spirit World)
- **Base File**: `assets/sprites/monsters/chaos_sprite/`
- Files needed: (Same 8 states as above)
- **Description**: Mischievous chaos spirit

**Total Monster Sprites**: 9 monsters × 8 states = **72 sprite files**

---

## 3. NPC SPRITES

All NPC sprites should be 64x64 pixels.

### Required NPCs

#### Head Shaman
- **File**: `assets/sprites/npcs/head_shaman.png` (64x64)
- **Description**: Wise elder, traditional shaman attire with ceremonial items

#### Player's Brother
- **File**: `assets/sprites/npcs/brother_normal.png` (64x64)
- **File**: `assets/sprites/npcs/brother_corrupted.png` (64x64)
- **Description**:
  - Normal: Young villager, friendly appearance
  - Corrupted: Same character with dark corruption overlay

#### Villagers (Generic)
- **File**: `assets/sprites/npcs/villager_01.png` (64x64)
- **File**: `assets/sprites/npcs/villager_02.png` (64x64)
- **File**: `assets/sprites/npcs/villager_03.png` (64x64)
- **Description**: Various villagers for background NPCs

**Total NPC Sprites**: **6 files**

---

## 4. WORLD TILE SPRITES

All tiles should be 32x32 pixels for proper grid alignment.

### Biome Types

#### Village Tiles
- **Base**: `assets/sprites/tiles/village/`
- Files needed:
  - `village_pure.png` (32x32) - Clean village tile
  - `village_corrupt_light.png` (32x32) - Slightly corrupted
  - `village_corrupt_heavy.png` (32x32) - Heavily corrupted

#### Forest Tiles
- **Base**: `assets/sprites/tiles/forest/`
- Files needed:
  - `forest_pure.png` (32x32) - Healthy forest
  - `forest_corrupt_light.png` (32x32) - Slightly withered
  - `forest_corrupt_heavy.png` (32x32) - Dead, corrupted trees

#### Mountain Tiles
- **Base**: `assets/sprites/tiles/mountains/`
- Files needed:
  - `mountains_pure.png` (32x32) - Clean mountain terrain
  - `mountains_corrupt_light.png` (32x32) - Cracked, dark stone
  - `mountains_corrupt_heavy.png` (32x32) - Void-corrupted mountains

#### Spirit Realm Tiles
- **Base**: `assets/sprites/tiles/spirit_realm/`
- Files needed:
  - `spirit_realm_pure.png` (32x32) - Ethereal, glowing
  - `spirit_realm_corrupt_light.png` (32x32) - Dimmed spiritual energy
  - `spirit_realm_corrupt_heavy.png` (32x32) - Darkened spirit realm

### Corruption Overlays (Optional Enhancement)
If using overlay system instead of separate tiles:
- `corruption_chaos.png` (32x32, transparent) - Red overlay
- `corruption_decay.png` (32x32, transparent) - Green overlay
- `corruption_void.png` (32x32, transparent) - Dark blue overlay
- `corruption_ancestral.png` (32x32, transparent) - Purple overlay

**Total Tile Sprites**: **12 base tiles** + 4 optional overlays

---

## 5. INTERACTIVE WORLD ELEMENTS

### Forageable Spots
- **File**: `assets/sprites/world/forageable_spot.png` (32x32)
- **Description**: Glowing plant/seed icon indicating harvestable location

### Dig Spots
- **File**: `assets/sprites/world/dig_spot.png` (32x32)
- **Description**: Ground marker showing diggable treasure location

### Crafting Stations

#### Herb Bench
- **File**: `assets/sprites/world/herb_bench.png` (64x64)
- **Description**: Wooden table with mortar and pestle, herbs

#### Spirit Altar
- **File**: `assets/sprites/world/spirit_altar.png` (64x64)
- **Description**: Glowing altar with spiritual symbols

#### Instrument Workshop
- **File**: `assets/sprites/world/instrument_workshop.png` (64x64)
- **Description**: Crafting station with musical instruments

### Dungeon Room Markers (Optional)
- `room_empty.png` (64x64) - Empty room
- `room_encounter.png` (64x64) - Monster encounter room
- `room_treasure.png` (64x64) - Treasure room
- `room_boss.png` (64x64) - Boss arena

**Total Interactive Sprites**: **9-10 files**

---

## 6. ITEM SPRITES

All item sprites should be 32x32 pixels for inventory display.

### Spirit Orbs
- `assets/sprites/items/spirit_orb_small.png` (32x32)
- `assets/sprites/items/spirit_orb_medium.png` (32x32)
- `assets/sprites/items/spirit_orb_large.png` (32x32)

### Seeds
- `assets/sprites/items/moonpetal_seed.png` (32x32)
- `assets/sprites/items/starroot_seed.png` (32x32)
- `assets/sprites/items/lifeleaf_seed.png` (32x32)
- `assets/sprites/items/shadowroot_seed.png` (32x32)
- `assets/sprites/items/crystalmoss_seed.png` (32x32)
- `assets/sprites/items/voidflower_seed.png` (32x32)
- `assets/sprites/items/eternalbark_seed.png` (32x32)

### Crafting Materials
- `assets/sprites/items/healing_herb.png` (32x32)
- `assets/sprites/items/corrupt_essence.png` (32x32)
- `assets/sprites/items/wood.png` (32x32)
- `assets/sprites/items/stone.png` (32x32)
- `assets/sprites/items/iron_ore.png` (32x32)

### Tools
- `assets/sprites/items/shovel.png` (32x32)

### Remedies
- `assets/sprites/items/remedy.png` (32x32)
- `assets/sprites/items/healing_remedy.png` (32x32)

**Total Item Sprites**: **19 files**

---

## 7. UI GRAPHICS

### HUD Elements

#### Health Bar Components
- `assets/sprites/ui/hud/health_bar_background.png` (200x20)
- `assets/sprites/ui/hud/health_bar_fill.png` (200x20)
- **Color**: Red (0.8, 0.2, 0.2) or custom gradient

#### Spirit Bar Components
- `assets/sprites/ui/hud/spirit_bar_background.png` (200x20)
- `assets/sprites/ui/hud/spirit_bar_fill.png` (200x20)
- **Color**: Blue (0.2, 0.5, 0.9) or custom gradient

#### Stamina Bar Components
- `assets/sprites/ui/hud/stamina_bar_background.png` (200x20)
- `assets/sprites/ui/hud/stamina_bar_fill.png` (200x20)
- **Color**: Green (0.3, 0.7, 0.3) or custom gradient

### Rhythm Visualizer
- `assets/sprites/ui/rhythm/beat_circle.png` (300x300)
- `assets/sprites/ui/rhythm/beat_indicator.png` (50x50)
- **Notes**: Circle pulses with beat timing

### Dialogue System

#### Dialogue Box
- `assets/sprites/ui/dialogue/dialogue_box.png` (800x200)
- **Description**: Traditional-style text box with borders
- **Position**: Bottom of screen

#### Choice Buttons
- `assets/sprites/ui/dialogue/choice_button.png` (400x60)
- `assets/sprites/ui/dialogue/choice_button_hover.png` (400x60)

#### NPC Portrait Frame
- `assets/sprites/ui/dialogue/portrait_frame.png` (128x128)
- **Description**: Decorative frame for NPC portraits

### Inventory UI
- `assets/sprites/ui/inventory/inventory_background.png` (600x400)
- `assets/sprites/ui/inventory/item_slot.png` (48x48)
- `assets/sprites/ui/inventory/item_slot_selected.png` (48x48)

### Shop UI
- `assets/sprites/ui/shop/shop_background.png` (600x500)
- `assets/sprites/ui/shop/item_row.png` (550x40)
- `assets/sprites/ui/shop/gold_icon.png` (24x24)

### Bestiary UI
- `assets/sprites/ui/bestiary/bestiary_background.png` (500x600)
- `assets/sprites/ui/bestiary/monster_card.png` (200x150)

### Main Menu
- `assets/sprites/ui/menu/button_new_game.png` (300x60)
- `assets/sprites/ui/menu/button_load_game.png` (300x60)
- `assets/sprites/ui/menu/button_settings.png` (300x60)
- `assets/sprites/ui/menu/button_hover.png` (300x60) - Highlight state
- `assets/sprites/ui/menu/title_logo.png` (600x200) - "SHAMAN" title graphic

### Loading Screen
- `assets/intro_video.png` (800x600) - **ALREADY REFERENCED IN CODE**
- **Description**: Main loading screen image/animation frame

**Total UI Sprites**: **27 files**

---

## 8. AUDIO ASSETS

### Music Tracks

#### Dawn Hymn
- **File**: `assets/audio/music/dawn_hymn.ogg`
- **Style**: Calm, peaceful
- **BPM**: 90
- **Duration**: 2-3 minutes (loopable)
- **Instruments**: Kora, light percussion

#### War Chant
- **File**: `assets/audio/music/war_chant.ogg`
- **Style**: Aggressive, energetic
- **BPM**: 140
- **Duration**: 2-3 minutes (loopable)
- **Instruments**: Heavy drums, chanting

#### Purification Rite
- **File**: `assets/audio/music/purification_rite.ogg`
- **Style**: Purifying, meditative
- **BPM**: 80
- **Duration**: 2-3 minutes (loopable)
- **Instruments**: Ngoni, soft vocals, bells

### Sound Effects (Future)
- `assets/audio/sfx/hit_impact.ogg` - Monster hit sound
- `assets/audio/sfx/player_hurt.ogg` - Player damage
- `assets/audio/sfx/item_pickup.ogg` - Item collection
- `assets/audio/sfx/song_success.ogg` - Rhythm success
- `assets/audio/sfx/song_fail.ogg` - Rhythm failure
- `assets/audio/sfx/purification.ogg` - Corruption cleansing
- `assets/audio/sfx/menu_select.ogg` - UI navigation

**Total Audio Files**: **3 music tracks** + 7 SFX (10 total)

---

## 9. FONTS

### Primary Game Font
- **File**: `assets/fonts/game_font.ttf`
- **Usage**:
  - Titles: 80pt
  - Headers: 24pt
  - Body text: 14-20pt
  - HUD labels: 16pt
- **Style**: Clean, readable, suitable for fantasy RPG
- **Recommendation**: African-inspired or traditional fantasy font

### Optional: Display Font
- **File**: `assets/fonts/title_font.ttf`
- **Usage**: Main menu "SHAMAN" title, large headers
- **Style**: Decorative, stylized

**Total Fonts**: **1-2 files**

---

## 10. PARTICLE EFFECTS (Optional)

### Corruption Particles
- `corruption_particle.png` (16x16) - Dark wisps for corruption visual

### Healing Particles
- `healing_particle.png` (16x16) - Green sparkles for healing

### Spirit Particles
- `spirit_particle.png` (16x16) - Blue glow for spirit effects

### Hit Effect
- `hit_particle.png` (32x32) - Impact flash for attacks

**Total Particle Sprites**: **4 files** (optional)

---

## SUMMARY

### Critical Assets (Required for Gameplay)
| Category | Count | Priority |
|----------|-------|----------|
| Player Sprites | 1 | ⚠️ CRITICAL |
| Monster Sprites | 72 | ⚠️ CRITICAL |
| NPC Sprites | 6 | 🔴 HIGH |
| Tile Sprites | 12-16 | 🔴 HIGH |
| Item Sprites | 19 | 🔴 HIGH |
| UI Sprites | 27 | 🔴 HIGH |
| Audio Tracks | 3 | 🟡 MEDIUM |
| Fonts | 1-2 | 🟢 LOW |
| Interactive Elements | 9 | 🟢 LOW |
| Particles | 4 | 🟢 LOW |

### Total Asset Count
- **Sprites**: ~146 image files
- **Audio**: ~10 audio files
- **Fonts**: 1-2 font files
- **Grand Total**: ~156-158 files

---

## IMPLEMENTATION NOTES

### Asset Loading System
The codebase already has infrastructure for:
- **MonsterSpriteDB**: Maps (monster_id, state) → Handle<Image>
- **SongDB**: Registry of available songs
- **AssetServer**: Bevy's asset loading system

### Next Steps After Assets Are Created
1. Implement asset loading in `bevy_shaman_core/src/systems/asset_loader.rs`
2. Populate MonsterSpriteDB with loaded sprites
3. Connect UI rendering to sprite assets instead of colored rectangles
4. Implement audio playback for music tracks
5. Add font loading for custom typography

### Naming Conventions
- **Monster sprites**: `{monster_id}_{state}.png`
- **Items**: `{item_id}.png`
- **Tiles**: `{biome}_{corruption_level}.png`
- **UI**: Descriptive names in category folders
- **Audio**: `{song_id}.ogg` or `{effect_name}.ogg`

### File Format Recommendations
- **Sprites**: PNG (supports transparency)
- **Audio**: OGG Vorbis (smaller than MP3, royalty-free)
- **Fonts**: TTF (TrueType Font)
- **Animations**: Horizontal spritesheets (single PNG with multiple frames)

---

## ASSET CREATION GUIDELINES

### Art Style Recommendations
- **Theme**: West African shamanism with fantasy elements
- **Palette**: Earth tones (browns, greens) with corruption colors (red, purple, dark blue)
- **Resolution**: Pixel art or low-poly style for performance
- **Consistency**: All sprites should match in style and scale

### Corruption Visual Design
Each corruption type should have a distinct color scheme:
- **Chaos**: Red (#CC3333) - Fire, aggression
- **Decay**: Green (#669933) - Rot, nature corruption
- **Void**: Dark Blue (#1A1A4D) - Empty, cosmic horror
- **Ancestral**: Purple (#9966CC) - Spiritual, ancient

### Animation Guidelines
- **Frame Rate**: 8-12 FPS for sprites
- **Frame Count**: 2-8 frames per animation
- **Format**: Horizontal spritesheets with equal-width frames
- **Looping**: All animations should loop seamlessly

### Audio Guidelines
- **Format**: OGG Vorbis, 44.1kHz, stereo
- **Normalization**: -6dB peak to avoid clipping
- **Looping**: Music tracks should have seamless loop points
- **Length**: 2-3 minutes minimum for music

---

## PLACEHOLDER STRATEGY

Until final assets are created, use:
1. **Colored rectangles** (current approach) for UI elements
2. **Simple geometric shapes** for sprites (circles, squares with colors)
3. **Bevy default font** for text
4. **No audio** (systems work without playback)

This allows development to continue while assets are being created.