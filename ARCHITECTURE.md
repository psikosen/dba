# ECS Architecture Deep Dive

## Data Flow Diagram

```
Player Input
    ↓
Rhythm Evaluation System
    ↓
RhythmInputEvaluated Event
    ↓
Combat Damage System → Attack Component
    ↓
Hit Resolution System → Modifies Health
    ↓
Monster State Transition → StateChanged Event
    ↓
Sprite Swap System → Updates Visuals
```

## Component Composition Patterns

### Entity Archetypes

**Player Entity**:
```rust
commands.spawn((
    Player,
    GridPosition::new(0, 0),
    Spirit::default(),
    Stamina::default(),
    Health::new(100.0),
    Inventory::new(20),
    CameraTarget,
    BlocksMovement,
));
```

**Monster Entity**:
```rust
commands.spawn((
    MonsterId("forest_spirit".into()),
    GridPosition::new(5, 5),
    Health::new(50.0),
    MonsterState::default(),
    MonsterStats::default(),
    MusicAffinityProfile::default(),
    AiBehavior::default(),
    AiState::default(),
    BlocksMovement,
));
```

**World Tile Entity**:
```rust
commands.spawn((
    WorldTile {
        biome: BiomeType::Forest,
        walkable: true,
    },
    GridPosition::new(x, y),
    TileCorruption::default(),
    SpriteBundle { ... },
));
```

## System Execution Order

### Update Schedule

```rust
.add_systems(Update, (
    // Input & Clock
    update_beat_clock,
    evaluate_rhythm_inputs,

    // State Updates
    update_monster_state_meters,
    evaluate_state_transitions,

    // Actions
    apply_music_influence,
    propagate_corruption_influence,
    process_movement_commands,

    // Resolution
    resolve_hits,
    apply_status_effects,

    // Visuals
    swap_sprites_on_state_change,
    update_tile_visuals,
    update_sprite_animations,

    // AI & Camera
    process_monster_ai,
    follow_player,
))
```

## Query Performance Patterns

### Optimal Queries

```rust
// GOOD: Specific filters
Query<(&MonsterState, &Transform), (With<CorruptionInfluence>, Without<Tamed>)>

// GOOD: Minimal component access
Query<&GridPosition, With<BlocksMovement>>

// AVOID: Broad queries in hot loops
Query<&mut Transform>  // Touches ALL entities with Transform
```

### Change Detection

```rust
// Only process tiles that changed
Query<(&TileCorruption, &mut Sprite), Changed<TileCorruption>>

// Only update moved entities
Query<&GridPosition, Changed<GridPosition>>
```

## Memory Layout Analysis

### Component Sizes

```rust
GridPosition:          8 bytes  // i32 + i32
Health:               8 bytes  // f32 + f32
MonsterState:        20 bytes  // enum(4) + f32×4
Spirit:              12 bytes  // f32×3
Stamina:             12 bytes  // f32×3
MusicAffinityProfile: 16 bytes  // f32×4
```

### Archetype Fragmentation

**Bad**: Runtime component addition
```rust
// Creates new archetype every time
commands.entity(e).insert(TempMarker);
commands.entity(e).remove::<TempMarker>();
```

**Good**: Use state enums
```rust
// Same archetype, just data change
monster.state = StateType::Chaos;
```

## Event Propagation Patterns

### Cascading Events

```rust
// State change triggers sprite swap
MonsterStateChanged → swap_sprites_on_state_change()

// Boss defeat triggers multiple effects
BossDefeated → unlock_purification_range()
            → wake_npcs()
            → unlock_songs()
            → expand_spirit_world_access()
```

### Event Ordering

Events are processed in the **next frame**:
```
Frame N:   emit MonsterStateChanged
Frame N+1: read MonsterStateChanged in sprite_swap_system
```

## Parallelism Opportunities

### Parallel System Sets

```rust
// These can run simultaneously (disjoint queries)
(
    update_monster_ai,      // Queries monsters
    process_player_input,   // Queries player
    update_tile_visuals,    // Queries tiles
)
```

### Sequential Dependencies

```rust
// Must run in order (shared mutable access)
.chain()  // Forces sequential execution
```

## Resource Contention

### Shared Resources

```rust
// Both need BeatClock, but only one writes
Res<BeatClock>   // Read-only, parallel safe
ResMut<BeatClock>  // Mutable, blocks parallel execution
```

### Solution: Split Resources

```rust
// Instead of one large resource
struct GameState { /* many fields */ }

// Use multiple focused resources
BeatClock
ActiveSong
PurificationAbility
InstrumentChoice
```

## Anti-Patterns to Avoid

### ❌ Manager Structs

```rust
// BAD: OOP-style manager
struct MonsterManager {
    monsters: Vec<Monster>,
}

impl MonsterManager {
    fn update_all(&mut self) { ... }
}
```

**Fix**: Use ECS queries
```rust
fn update_monsters(mut monsters: Query<&mut MonsterState>) { ... }
```

### ❌ Component Methods with Side Effects

```rust
// BAD: Component modifies itself
impl MonsterState {
    fn apply_music(&mut self, music: MusicStyle) { ... }
}
```

**Fix**: System owns logic
```rust
fn apply_music_influence(
    music: Res<ActiveSong>,
    mut monsters: Query<&mut MonsterState>,
) { ... }
```

### ❌ Global Mutable State

```rust
// BAD: Static mut
static mut MONSTER_COUNT: u32 = 0;
```

**Fix**: Use Resources
```rust
#[derive(Resource)]
struct MonsterCount(u32);
```

## Testing Patterns

### Unit Test Components

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monster_state_evaluation() {
        let mut state = MonsterState::default();
        state.corruption_meter = 0.8;
        assert_eq!(state.evaluate_state(), StateType::Corrupt);
    }
}
```

### Integration Test Systems

```rust
#[test]
fn test_rhythm_damage() {
    let mut app = App::new();
    app.add_systems(Update, apply_rhythm_based_damage);

    // Spawn test entities
    let monster = app.world.spawn(MonsterStats::default()).id();

    // Emit event
    app.world.send_event(RhythmInputEvaluated {
        quality: TimingQuality::Perfect,
        combo: 5,
    });

    // Run one frame
    app.update();

    // Assert results
    // ...
}
```

## Profiling Hotspots

### Expected Bottlenecks

1. **Monster AI** (pathfinding, spatial queries)
2. **Corruption propagation** (tile-to-tile iteration)
3. **Sprite swapping** (texture handle lookups)
4. **Collision detection** (grid occupancy checks)

### Optimization Strategies

**Spatial Hashing**:
```rust
// Instead of O(N²) distance checks
HashMap<(i32, i32), Vec<Entity>>  // Grid cell → entities
```

**Time Slicing**:
```rust
// Only run expensive systems every N frames
if frame_count % 10 == 0 {
    propagate_corruption();
}
```

**SIMD Candidates**:
- Meter updates (parallel f32 math)
- State evaluations (parallel comparisons)
- Tile color blending

## Extending the Architecture

### Adding a New Plugin

1. Create `crates/bevy_shaman_X/`
2. Add to workspace `Cargo.toml`
3. Define components/resources
4. Implement systems
5. Register plugin in `main.rs`

### Adding a New Monster State

1. Add variant to `StateType` enum
2. Add sprite mapping in `MonsterSpriteDB`
3. Update `evaluate_state()` logic
4. Define state-specific behavior in AI system

### Adding a New Music Style

1. Add variant to `MusicStyle` enum
2. Create song entries in `SongDB`
3. Define influence rules in `apply_music_influence()`
4. Update UI to display new style

## Debugging Tools

### Gizmos for Visualization

```rust
fn debug_corruption(
    mut gizmos: Gizmos,
    tiles: Query<(&GridPosition, &TileCorruption)>,
) {
    for (pos, corruption) in tiles.iter() {
        if corruption.is_corrupt() {
            gizmos.circle_2d(
                Vec2::new(pos.x as f32 * 32.0, pos.y as f32 * 32.0),
                corruption.level * 10.0,
                corruption.corruption_color(),
            );
        }
    }
}
```

### Console Commands

```rust
// Example dev console integration
if input.just_pressed(KeyCode::F1) {
    commands.spawn(MonsterBundle::new("chaos_hound"));
}
```

## Deployment Considerations

### Save File Format

Use JSON for human-readable saves:
```json
{
  "player": {
    "position": {"x": 5, "y": 10},
    "spirit": {"current": 80.0, "max": 100.0},
    "instrument_choice": "Kora"
  },
  "world": {
    "boss_flags": ["forest_guardian", "chaos_titan"],
    "brother_progress": 2
  }
}
```

### Asset Loading

Bevy's asset system handles async loading:
```rust
let sprite_handle: Handle<Image> = asset_server.load("sprites/monster_chaos.png");
```

Use `AssetServer` + `Assets<T>` for runtime asset management.
