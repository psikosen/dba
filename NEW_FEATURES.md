# New Features - Video Intro, Menus, Shop, and Foraging Systems

This document describes the major features added to the Bevy Shaman game.

## 1. Video Intro Loading Screen (Boot State)

**Location**: `crates/bevy_shaman_ui/src/lib.rs` (systems::loading_screen module)

### Features:
- Displays on game startup (GameState::Boot)
- Shows a video intro image (place your video at `assets/intro_video.png`)
- Gritty dark aesthetic with "SHAMAN" title overlay
- 5-second timer before transitioning to main menu
- Automatic cleanup and state transition

### Usage:
1. Place your intro video/image at `assets/intro_video.png`
2. Game automatically shows this on startup
3. After 5 seconds, transitions to main menu

## 2. Main Menu System

**Location**: `crates/bevy_shaman_ui/src/lib.rs` (systems::main_menu module)

### Features:
- Three main options:
  - **New Game**: Starts a fresh game (transitions to Playing state)
  - **Load Game**: Loads saved game (currently transitions to Playing state)
  - **Settings**: Settings menu (placeholder for future implementation)
- Dark, gritty aesthetic matching the game theme
- Automatic button interaction handling

### Usage:
- Menu appears after loading screen
- Click buttons to navigate
- New Game and Load Game both start the game

## 3. Shop/Store System

**Location**: `crates/bevy_shaman_shop/`

### Features:
- **20 item maximum** inventory (configurable)
- **Currency system** (Gold)
- **Buy/Sell transactions** via events
- **Pre-stocked items**:
  - Seeds: Moon Petal, Star Root, Life Leaf (10-20 gold)
  - Tools: Shovel (100 gold) - required for digging!
  - Items: Remedy (50 gold)
  - Spirit Orbs: Small (25 gold), Medium (60 gold)
  - Crafting Materials: Wood (5 gold), Stone (8 gold), Iron Ore (30 gold)

### Shop UI:
**Hotkey**: Press `S` to toggle shop UI

The shop displays:
- Current gold balance
- All available items with prices and stock
- Item names and quantities
- Scrollable list for many items

### Usage:
1. Press `S` to open shop
2. View items and prices
3. Use PurchaseEvent to buy items (events system)
4. Use SellEvent to sell items at 50% price

## 4. Seed Foraging System

**Location**: `crates/bevy_shaman_world/src/systems/interactions.rs`

### Features:
- **Hotkey**: Press `F` to forage nearby spots
- **Auto-spawn**: Forageable spots spawn every 30 seconds
- **Max spots**: 10 concurrent forageable locations
- **Random seeds**: 1-3 seeds per forage
- **Plant types**:
  - Moon Petal (calming)
  - Star Root (calming)
  - Eternal Bark (healing)
  - Crystal Moss (spirit)
  - Aether Grass (spirit)

### How It Works:
1. Forageable spots spawn randomly in the world
2. Walk adjacent to a spot (1 tile away)
3. Press `F` to forage
4. Receive 1-3 random seeds
5. Spot becomes depleted
6. New spots spawn periodically

## 5. Dig System for Tunnels

**Location**: `crates/bevy_shaman_world/src/systems/interactions.rs`

### Features:
- **Hotkey**: Press `G` to dig
- **Requirement**: Must have a Shovel (buy from shop!)
- **Auto-spawn**: Dig spots spawn every 20 seconds in dungeons
- **Max spots**: 15 concurrent dig locations
- **Random loot** with weighted chances:
  - **30%**: Spirit Orbs (Small/Medium/Large)
  - **30%**: Crafting Materials (Wood, Stone, Iron Ore) - 1-5 units
  - **20%**: Healing Items (Herbs, Remedies)
  - **20%**: Seeds (Moon Petal, Star Root, Crystal Moss) - 1-3 seeds

### How It Works:
1. Buy a Shovel from the shop (100 gold)
2. Find dig spots in dungeons (auto-spawn)
3. Stand on a dig spot
4. Press `G` to dig
5. Receive random loot
6. Spot becomes depleted

## 6. Plant Monster Seed Drops

**Location**: `crates/bevy_shaman_items/src/resources.rs`

### Features:
- New loot table: `"plant_monster"`
- Drops various seeds when defeated
- **Seed drops** with weights:
  - Moon Petal Seed (30%) - 1-3 seeds
  - Shadow Root Seed (25%) - 1-2 seeds
  - Crystal Moss Seed (20%) - 1-2 seeds
  - Void Flower Seed (15%) - 1 seed
  - Eternal Bark Seed (10%) - 1 seed

### Usage:
- Defeat plant-type monsters
- Automatically drop seeds based on loot table
- Use seeds for planting or crafting

## Controls Summary

| Key | Action |
|-----|--------|
| `S` | Toggle Shop UI |
| `F` | Forage for seeds (when near forageable spot) |
| `G` | Dig for items (requires Shovel, when on dig spot) |
| `B` | Toggle Bestiary (existing feature) |

## Integration Notes

### State Flow:
1. **Boot** (Loading screen with video) → 5 seconds
2. **MainMenu** (New Game, Load Game, Settings)
3. **Playing** (Game with shop, foraging, digging)

### Systems Architecture:
- **Shop Plugin**: Separate crate for store management
- **World Interactions**: Foraging and digging integrated into world systems
- **UI Systems**: All UI elements use state-based activation
- **Event-Driven**: Purchase/sell events for shop transactions

### Future Enhancements:
- Add video animation support (currently static image)
- Implement settings menu functionality
- Add interactive shop transactions with click-to-buy
- Expand loot tables with more items
- Add visual indicators for forageable spots and dig sites
- Implement farming system for planted seeds

## Technical Details

### New Crates:
- `bevy_shaman_shop`: Complete shop system with currency and inventory

### Modified Crates:
- `bevy_shaman_ui`: Added loading screen, main menu, and shop UI
- `bevy_shaman_world`: Added foraging and digging interaction systems
- `bevy_shaman_items`: Added plant monster loot table

### Resources Added:
- `Currency`: Tracks player gold
- `ShopInventory`: Manages shop items, prices, and stock
- `VideoIntroTimer`: Controls loading screen duration
- `ShopVisible`: Toggles shop UI

### Components Added:
- `ForageableSpot`: Marks locations with seeds
- `DigSpot`: Marks diggable locations
- Shop UI components (ShopUI, ShopItemButton, etc.)
- Menu UI components (MainMenuUI, buttons, etc.)

## Testing

All features compile successfully. To test:
1. Run `cargo build`
2. Run `cargo run`
3. Watch intro screen (5 seconds)
4. Navigate main menu
5. In game, press `S` for shop
6. Press `F` near forageable spots
7. Buy shovel, press `G` to dig

Enjoy the new features!
