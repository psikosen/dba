# Minimap System Guide

This guide explains how to use the minimap with fog of war for dungeons and overworld exploration.

## Overview

The minimap provides:
- **Real-time exploration tracking** with fog of war
- **Color-coded biomes** for easy navigation
- **Player position marker** (yellow)
- **Circular view radius** around player
- **Persistent exploration** - areas stay revealed
- **Works in all game modes** - overworld, dungeons, spirit realms

## Architecture

### Resource

**MinimapState** (`bevy_shaman_ui::systems::minimap::MinimapState`)
```rust
pub struct MinimapState {
    pub visible: bool,
    pub explored_tiles: HashSet<(i32, i32)>,
    pub view_radius: i32,
}
```

### Components

**MinimapRoot** - Root UI container
**MinimapTile** - Individual tile in minimap grid

```rust
pub struct MinimapTile {
    pub grid_x: i32,
    pub grid_y: i32,
}
```

## Features

### Fog of War

The minimap uses a **fog of war** system:
- Unexplored areas appear dark grey with transparency
- Areas within player's view radius (10 tiles) are revealed
- **Revealed areas remain visible** even after leaving
- Circular exploration pattern (not square)

### View Radius

Default view radius: **10 tiles**

This creates a circular area around the player where tiles are automatically revealed.

Calculation:
```rust
let dx = tile_x - player_x;
let dy = tile_y - player_y;
if dx * dx + dy * dy <= view_radius * view_radius {
    // Tile is within view radius
}
```

### Biome Colors

The minimap uses color coding for different biome types:

| Biome | Color | RGB |
|-------|-------|-----|
| Forest | Dark Green | `(0.1, 0.4, 0.1)` |
| Mountains | Grey | `(0.5, 0.5, 0.5)` |
| Village | Tan/Brown | `(0.8, 0.6, 0.4)` |
| Spirit Realm | Purple | `(0.4, 0.2, 0.8)` |
| **Player** | **Yellow** | **`(1.0, 1.0, 0.0)`** |
| Unexplored | Dark Grey (50% alpha) | `(0.1, 0.1, 0.1, 0.5)` |

## Configuration

### Changing View Radius

```rust
fn adjust_minimap_radius(
    mut minimap_state: ResMut<MinimapState>,
) {
    // Increase exploration radius
    minimap_state.view_radius = 15;

    // Decrease for harder exploration
    minimap_state.view_radius = 5;
}
```

### Toggling Visibility

```rust
fn toggle_minimap(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut minimap_state: ResMut<MinimapState>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        minimap_state.visible = !minimap_state.visible;
    }
}
```

### Clearing Fog of War

```rust
fn reset_exploration(
    mut minimap_state: ResMut<MinimapState>,
) {
    // Clear all explored tiles
    minimap_state.explored_tiles.clear();
}
```

### Revealing Entire Map

```rust
fn reveal_all_tiles(
    mut minimap_state: ResMut<MinimapState>,
    tile_query: Query<&GridPosition, With<WorldTile>>,
) {
    for tile_pos in tile_query.iter() {
        minimap_state.explored_tiles.insert((tile_pos.x, tile_pos.y));
    }
}
```

## UI Layout

The minimap appears in the **top-right corner** of the screen:

- **Position**: 10px from top, 10px from right
- **Size**: 250x250 pixels
- **Background**: Dark blue-grey with 90% opacity
- **Border**: Medium grey
- **Grid**: 20x20 tiles (±10 from player position)
- **Tile Size**: 10x10 pixels each

## Integration with World

The minimap automatically reads world tiles from the `WorldTile` component:

```rust
#[derive(Component)]
pub struct WorldTile {
    pub biome: BiomeType,
    pub walkable: bool,
}

pub enum BiomeType {
    Village,
    Forest,
    Mountains,
    SpiritRealm,
}
```

## Usage Scenarios

### Dungeon Exploration

Perfect for procedurally generated dungeons:
1. Player enters dungeon
2. Minimap shows only nearby tiles
3. As player explores, fog of war clears
4. Previously visited areas remain visible
5. Player can navigate back using minimap

### Overworld Navigation

Helps with large overworld areas:
1. Shows surrounding biomes
2. Reveals villages, forests, mountains
3. Yellow marker shows current position
4. Easy to see where you've been

### Spirit Realm

Useful in spirit world areas:
1. Purple color distinguishes spirit realms
2. Track exploration progress
3. Find exit portals
4. Avoid getting lost

## Performance Optimization

The minimap system is optimized for performance:

### Efficient Updates
- Only updates when player moves
- Spawns UI once, then maintains it
- Uses HashSet for O(1) exploration lookups
- Circular radius calculation prevents unnecessary checks

### Respawn Prevention
```rust
// Only spawn minimap once
if !minimap_root_query.is_empty() {
    return;
}
```

### Conditional Rendering
- Only renders tiles within ±10 range of player
- Doesn't render entire world map
- 20x20 grid = 400 tiles maximum

## Advanced Customization

### Custom Biome Colors

Add custom colors for modded biomes:

```rust
// In minimap.rs update_minimap function
found_color = match world_tile.biome {
    BiomeType::Forest => Color::srgb(0.1, 0.4, 0.1),
    BiomeType::Mountains => Color::srgb(0.5, 0.5, 0.5),
    BiomeType::Village => Color::srgb(0.8, 0.6, 0.4),
    BiomeType::SpiritRealm => Color::srgb(0.4, 0.2, 0.8),
    BiomeType::Custom => Color::srgb(1.0, 0.5, 0.0), // Add custom
};
```

### Larger Minimap

```rust
// In minimap.rs, adjust MinimapRoot spawn
Node {
    position_type: PositionType::Absolute,
    top: Val::Px(10.0),
    right: Val::Px(10.0),
    width: Val::Px(400.0),  // Increased from 250
    height: Val::Px(400.0), // Increased from 250
    // ...
}
```

### Different Grid Sizes

```rust
// Change map_range in update_minimap
let map_range = 15; // Shows 30x30 grid instead of 20x20
```

Remember to also update grid template:
```rust
grid_template_columns: vec![GridTrack::auto(); 30],
grid_template_rows: vec![GridTrack::auto(); 30],
```

### Icons for Special Locations

Add markers for important locations:

```rust
// After spawning regular tiles, add special markers
if tile_has_boss {
    grid_parent.spawn((
        Node {
            width: Val::Px(10.0),
            height: Val::Px(10.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(1.0, 0.0, 0.0)), // Red for boss
        BorderColor(Color::WHITE),
    ));
}
```

## Fog of War Mechanics

### How It Works

1. **Exploration Tracking**
   - Each tile coordinate `(x, y)` is stored in `explored_tiles` HashSet
   - Once added, tile remains explored forever (until manually cleared)

2. **Visibility Calculation**
   ```rust
   // Check if tile has been explored
   if minimap_state.explored_tiles.contains(&(world_x, world_y)) {
       // Show actual biome color
   } else {
       // Show fog of war (dark semi-transparent)
   }
   ```

3. **Exploration Updates**
   - Every frame, tiles within view radius are added to explored set
   - Uses circular radius calculation for realistic vision
   - No ray casting - all tiles in radius are visible

### Strategic Gameplay

The fog of war adds strategic value:
- **Exploration incentive** - Players want to fill in the map
- **Navigation challenge** - Must remember or return to unexplored areas
- **Discovery feeling** - Seeing new areas revealed is satisfying
- **Dungeon mastery** - Full exploration shows completion

## Troubleshooting

### Minimap not showing
- Check `MinimapState.visible` is `true` (default)
- Verify player has `GridPosition` component
- Ensure UI plugin is loaded

### Colors wrong
- Verify tiles have `WorldTile` component
- Check `BiomeType` enum values
- Review color definitions in `update_minimap`

### Fog not clearing
- Check player `GridPosition` is updating
- Verify `view_radius` is > 0 (default: 10)
- Ensure tiles are within circular radius

### Performance issues
- Reduce `view_radius` if needed
- Check tile count in view range
- Consider increasing update interval

### Tiles not appearing
- Verify tiles have both `GridPosition` and `WorldTile`
- Check coordinates are within ±10 of player
- Review tile query in `update_minimap`

## Example: Dungeon Entry

```rust
fn enter_dungeon(
    mut minimap_state: ResMut<MinimapState>,
) {
    // Clear fog when entering new dungeon
    minimap_state.explored_tiles.clear();

    // Optional: Reduce view radius for harder dungeons
    minimap_state.view_radius = 7;
}
```

## Example: Map Reveal Power-Up

```rust
fn use_map_reveal_item(
    mut minimap_state: ResMut<MinimapState>,
    player_query: Query<&GridPosition, With<Player>>,
) {
    let player_pos = player_query.single();

    // Reveal large area around player
    let reveal_radius = 25;
    for x in (player_pos.x - reveal_radius)..=(player_pos.x + reveal_radius) {
        for y in (player_pos.y - reveal_radius)..=(player_pos.y + reveal_radius) {
            let dx = x - player_pos.x;
            let dy = y - player_pos.y;
            if dx * dx + dy * dy <= reveal_radius * reveal_radius {
                minimap_state.explored_tiles.insert((x, y));
            }
        }
    }
}
```

## Future Enhancements

Potential additions:
- Quest markers on minimap
- NPC indicators
- Danger zone highlighting
- Multiple zoom levels
- Waypoint system
- Minimap rotation based on player facing
- Different icons for different POIs
- Minimap panning/dragging
- Screenshot/export map feature
