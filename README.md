# Shaman's Journey - Bevy ECS Architecture

[![CI](https://github.com/psikosen/dba/actions/workflows/ci.yml/badge.svg)](https://github.com/psikosen/dba/actions/workflows/ci.yml)
[![Security Audit](https://github.com/psikosen/dba/actions/workflows/security.yml/badge.svg)](https://github.com/psikosen/dba/actions/workflows/security.yml)
[![Docker](https://github.com/psikosen/dba/actions/workflows/docker.yml/badge.svg)](https://github.com/psikosen/dba/actions/workflows/docker.yml)

A rhythm-based shaman healing game built with **Bevy 0.15.4** following strict **data-oriented design** principles.

## High Concept

Play as a young apprentice shaman healing a cursed land through rhythm-based combat. Your music doesn't just attack—it stabilizes, corrupts, and purifies spirits, monsters, and the world itself.

## Architecture Overview

### Design Principles

This codebase follows **data-oriented design** and **composition over inheritance**:

- **Zero OOP**: No classes, no inheritance. Only Components, Systems, and Resources.
- **Pure Composition**: Behavior is defined by Systems acting on Component data.
- **Cache-Friendly Structures**: Components are designed for memory locality.
- **Event-Driven Communication**: Systems communicate via Bevy Events, not direct calls.
- **Stateless Systems**: Logic lives in Systems; state lives in Components/Resources.

### Plugin Architecture

The game is organized into **isolated, composable plugins**:

```
bevy_shaman/               # Main binary
├── bevy_shaman_core/      # Grid, movement, camera, animation, state machines
├── bevy_shaman_combat/    # Hit resolution, rhythm-based damage, status effects
├── bevy_shaman_audio/     # Beat clock, song manager, rhythm evaluation
├── bevy_shaman_monsters/  # State machine, corruption, sprite swapping, AI
├── bevy_shaman_minions/   # Taming, formation, commands
├── bevy_shaman_world/     # Tiles, corruption spread, purification
├── bevy_shaman_dungeons/  # Generation, encounters, bosses
├── bevy_shaman_items/     # Inventory, Spirit Orbs, crafting
├── bevy_shaman_ui/        # HUD, bestiary, rhythm visualizer
├── bevy_shaman_story/     # NPC sickness, dialogue, quests, instrument choice
└── bevy_shaman_save/      # Serialization, autosave, save/load
```

Each plugin is **self-contained** with:
- `components.rs` - Data-only structs
- `resources.rs` - Global singleton data
- `systems/*.rs` - Stateless logic functions
- `lib.rs` - Plugin registration

## Core Systems

### State Machines

**GameState** (top-level flow):
- `Boot` → `MainMenu` → `Playing` → `Paused`/`Dialogue`/`Cutscene`

**WorldState** (location):
- `Overworld` ↔ `SpiritWorld(id)` ↔ `Dungeon` ↔ `BossArena`

**CombatState**:
- `None` → `Encounter` → `Boss`

### Monster State System

Monsters have **dynamic states** that change visuals and behavior:

```rust
pub struct MonsterState {
    pub state: StateType,           // Stable/Chaos/Corrupt/Harmony/etc.
    pub stability_meter: f32,       // 0.0 = chaos, 1.0 = stable
    pub corruption_meter: f32,      // 0.0 = pure, 1.0 = corrupt
    pub obedience_meter: f32,       // 0.0 = wild, 1.0 = tame
    pub chaos_output: f32,          // Damage multiplier
}
```

**State Transitions**:
- Music influences stability/obedience
- Corruption spreads via proximity
- Chaos monsters hit harder but resist control
- Each state has unique sprites (auto-swapped)

### Rhythm Clock System

**Beat-based combat timing**:

```rust
pub struct BeatClock {
    pub bpm: f32,
    pub beat_duration: f32,
    pub current_beat: u32,
    pub time_in_beat: f32,
}

pub enum TimingQuality {
    Perfect,  // 1.5x damage, 1.3x control
    Great,    // 1.2x damage, 1.1x control
    Good,     // 1.0x damage, 1.0x control
    Miss,     // 0.5x damage, 0.7x control
}
```

Input timing determines:
- Damage multiplier
- Control/obedience gains
- Combo maintenance
- State influence strength

### Spirit & Stamina Resources

```rust
pub struct Spirit {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
}

pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
}
```

**Spirit Orbs** restore both resources:
- Small: 20 Spirit + 15 Stamina
- Medium: 50 Spirit + 40 Stamina
- Large: 100 Spirit + 80 Stamina

Auto-consume when resources drop below 30%.

### Corruption & Purification

**Tile Corruption**:
```rust
pub struct TileCorruption {
    pub level: f32,                    // 0.0 = pure, 1.0 = corrupt
    pub corruption_type: CorruptionType,
    pub purified: bool,
}
```

**Purification Progression** (boss-gated):
- Level 1-2: Single tile
- Level 3-4: Cross pattern (+ shape)
- Level 5-6: 3x3 Cluster
- Level 7-8: Line or Ring patterns

### Instrument Choice (Kora vs Ngoni)

Mid-game choice that modifies playstyle:

| Aspect | Kora | Ngoni |
|--------|------|-------|
| Stability | +30% | -10% |
| Damage | -10% | +30% |
| Stamina Cost | -20% | +20% |
| Obedience | +20% | -20% |

This choice affects:
- Monster control difficulty
- Purification effectiveness
- Spirit World accessibility
- Branching quest outcomes

### NPC Sickness System

NPCs progress through sickness states:

```rust
pub enum NpcSicknessState {
    AsleepSick,  // "... ... ..."
    Waking,      // Partial dialogue
    Awake,       // Full dialogue
}
```

Waking triggers:
- Boss defeats (progression milestones)
- Brother cleansing progress (4 fights)

## Key Features

### Music Influence on Monsters

```rust
pub enum MusicStyle {
    Calm,        // +Stability, +Obedience, -Chaos
    Aggressive,  // -Stability, +Damage, +Chaos
    Purifying,   // -Corruption
    Harmonizing, // +Stability in Spirit Worlds
}
```

### Corruption Propagation

Corrupt/Chaos monsters emit **CorruptionInfluence**:
- Nearby monsters accumulate exposure
- Exposure increases corruption_meter
- Corruption spreads tile-to-tile
- Purification blocks spread

### Brother Cleansing Arc

4 escalating fights with your corrupted brother:
- Each victory reduces his soul_corruption by 25%
- Each fight unlocks new NPC dialogues
- Final cleanse is a major story milestone

## Building & Running

### Prerequisites

**Rust Toolchain:**
```bash
cargo --version  # Requires Rust 1.75+ (tested with 1.91.1)
rustc --version
```

**System Dependencies (Linux):**

The game requires several system libraries for graphics, audio, and input handling:

**Debian/Ubuntu:**
```bash
sudo apt-get update
sudo apt-get install -y \
    libasound2-dev \
    libudev-dev \
    pkg-config \
    build-essential \
    libx11-dev \
    libxi-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libxcursor-dev \
    libxinerama-dev \
    libxrandr-dev
```

**Fedora/RHEL:**
```bash
sudo dnf install -y \
    alsa-lib-devel \
    systemd-devel \
    pkgconfig \
    gcc gcc-c++ \
    libX11-devel \
    libXi-devel \
    mesa-libGL-devel \
    mesa-libGLU-devel \
    libXcursor-devel \
    libXinerama-devel \
    libXrandr-devel
```

**Arch Linux:**
```bash
sudo pacman -Syu --noconfirm \
    alsa-lib \
    systemd \
    pkgconf \
    base-devel \
    libx11 libxi mesa \
    libxcursor \
    libxinerama \
    libxrandr
```

**Automated Setup:**
```bash
# Run the setup script (automatically detects your package manager)
bash setup_linux.sh
```

**Docker Alternative:**
If you prefer not to install system dependencies, use Docker:
```bash
docker build -t shaman-journey .
docker run -it shaman-journey
```

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

### Development

```bash
# Run with fast compile times (debug)
cargo run

# Check all crates
cargo check --workspace

# Run tests (requires system dependencies)
cargo test --workspace

# Run tests without audio crate (no ALSA required)
cargo test --workspace --exclude bevy_shaman_audio
```

## Project Structure Details

### Component Design

Components are **data-only**:

```rust
// GOOD: Pure data
#[derive(Component)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

// BAD: Methods that modify state belong in Systems
impl GridPosition {
    pub fn move_to(&mut self, x: i32, y: i32) { /* NO */ }
}
```

### System Design

Systems are **stateless functions**:

```rust
// GOOD: Queries + Resources → Logic
pub fn update_monster_states(
    time: Res<Time>,
    mut monsters: Query<(&mut MonsterState, &MusicAffinityProfile)>,
) {
    // Pure logic acting on data
}

// BAD: Internal state
pub fn bad_system(mut local_counter: Local<u32>) { /* AVOID */ }
```

### Resource Design

Resources are **global singletons**:

```rust
#[derive(Resource)]
pub struct MonsterSpriteDB {
    pub sprites: HashMap<(String, StateType), Handle<Image>>,
}
```

## Memory Layout Considerations

### Cache Efficiency

Components are designed for tight packing:

```rust
// GOOD: 16 bytes, single cache line
#[repr(C)]
pub struct MonsterStats {
    pub attack: f32,    // 4 bytes
    pub defense: f32,   // 4 bytes
    pub speed: f32,     // 4 bytes
    pub affinity: f32,  // 4 bytes
}

// CAREFUL: String allocations break locality
pub struct MonsterTemplate {
    pub id: String,  // Heap allocation, use for lookups only
    pub stats: MonsterStats,  // Inline data
}
```

### Query Filters

Use filters to minimize iteration:

```rust
// GOOD: Only corrupted monsters
monsters.iter().filter(|m| m.corruption_meter > 0.7)

// BETTER: Use Query filters
monsters: Query<&MonsterState, With<CorruptionInfluence>>
```

## Event System

Communication uses Bevy Events:

```rust
#[derive(Event)]
pub struct MonsterStateChanged {
    pub entity: Entity,
    pub old_state: StateType,
    pub new_state: StateType,
}

// Writers emit
fn state_system(mut events: EventWriter<MonsterStateChanged>) {
    events.send(MonsterStateChanged { ... });
}

// Readers consume
fn sprite_system(mut events: EventReader<MonsterStateChanged>) {
    for event in events.read() { ... }
}
```

## Future Enhancements

### Optional "Fire Stack" Integration

For production telemetry and tooling:

**DragonflyDB** (Redis-compatible):
- Dungeon generation caching by seed
- Replay data storage
- Build-time asset caching

**RabbitMQ**:
- Offline balance simulations
- Content validation jobs
- Regression testing pipelines

Setup:
```bash
docker run -d -p 6379:6379 docker.dragonflydb.io/dragonflydb/dragonfly
docker run -d -p 5672:5672 rabbitmq:3-management
```

## Development Guidelines

### Adding a New System

1. Decide which plugin owns it
2. Create system function in `systems/`
3. Register in plugin's `lib.rs`
4. Use Query filters for performance
5. Emit Events for cross-system communication

### Adding a New Component

1. Define struct in `components.rs`
2. Use `#[derive(Component)]`
3. Keep data-only (no methods)
4. Consider memory layout
5. Add to relevant entity bundles

### Adding a New State

1. Define enum in `core/states.rs`
2. Use `#[derive(States)]`
3. Register with `init_state::<T>()`
4. Use `.run_if(in_state(T))` for conditional systems

## Performance Notes

### System Parallelism

Bevy automatically parallelizes systems with **non-overlapping queries**:

```rust
// These run in parallel (different components)
.add_systems(Update, (
    update_monster_states,    // Queries MonsterState
    update_tile_visuals,      // Queries TileCorruption
    process_player_input,     // Queries Player
))
```

### Archetype Optimization

Minimize component additions/removals at runtime:

```rust
// GOOD: State enum (no archetype change)
monster.state = StateType::Chaos;

// AVOID: Runtime component insertion (fragments archetypes)
commands.entity(monster).insert(ChaosMarker);
```

## License

MIT OR Apache-2.0

## Credits

**Architecture**: Data-Oriented Design, Bevy ECS
**Inspiration**: West African shamanic traditions, rhythm games
