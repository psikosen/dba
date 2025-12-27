use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct DungeonsPlugin;

impl Plugin for DungeonsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<systems::generation::DungeonGenerator>()
            .add_systems(Update, (
                systems::generation::trigger_dungeon_entry,
                systems::generation::generate_dungeon_rooms,
                systems::encounters::spawn_encounters,
                systems::bosses::manage_boss_fights,
            ).run_if(in_state(GameState::Playing)))
            .add_event::<systems::events::DungeonEntered>()
            .add_event::<systems::events::BossDefeated>();
    }
}

pub mod components {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Component)]
    pub struct DungeonRoom {
        pub room_type: RoomType,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum RoomType {
        Empty,
        Encounter,
        Treasure,
        Boss,
    }

    #[derive(Component)]
    pub struct BossArena {
        pub boss_id: String,
        pub phase: u8,
    }
}

pub mod systems {
    pub mod events {
        use bevy::prelude::*;

        #[derive(Event)]
        pub struct DungeonEntered {
            pub dungeon_id: String,
        }

        #[derive(Event)]
        pub struct BossDefeated {
            pub boss_id: String,
        }
    }

    pub mod generation {
        use bevy::prelude::*;
        use bevy_shaman_core::components::GridPosition;
        use crate::components::{DungeonRoom, RoomType};
        use rand::Rng;

        #[derive(Resource)]
        pub struct DungeonGenerator {
            pub current_dungeon_id: String,
            pub rooms_generated: usize,
            pub dungeon_count: u32,
        }

        impl Default for DungeonGenerator {
            fn default() -> Self {
                Self {
                    current_dungeon_id: String::new(),
                    rooms_generated: 0,
                    dungeon_count: 0,
                }
            }
        }

        /// Trigger dungeon generation with 'D' key (for testing/manual trigger)
        pub fn trigger_dungeon_entry(
            keyboard: Res<ButtonInput<KeyCode>>,
            mut events: EventWriter<super::events::DungeonEntered>,
            mut dungeon_gen: ResMut<DungeonGenerator>,
        ) {
            if keyboard.just_pressed(KeyCode::KeyD) {
                dungeon_gen.dungeon_count += 1;
                let dungeon_id = format!("dungeon_{}", dungeon_gen.dungeon_count);
                events.send(super::events::DungeonEntered {
                    dungeon_id: dungeon_id.clone(),
                });
                info!("Entering dungeon: {}", dungeon_id);
            }
        }

        pub fn generate_dungeon_rooms(
            mut commands: Commands,
            mut dungeon_gen: ResMut<DungeonGenerator>,
            mut dungeon_entered: EventReader<super::events::DungeonEntered>,
        ) {
            for event in dungeon_entered.read() {
                dungeon_gen.current_dungeon_id = event.dungeon_id.clone();

            let mut rng = rand::thread_rng();
            let room_count = rng.gen_range(5..=12);

            info!("Generating dungeon with {} rooms", room_count);

            // Generate room graph using simple linear progression with branches
            let mut room_positions = Vec::new();
            let mut current_pos = IVec2::ZERO;

            for i in 0..room_count {
                let room_type = match i {
                    0 => RoomType::Empty, // Starting room
                    i if i == room_count - 1 => RoomType::Boss, // Final room
                    _ => {
                        let roll: f32 = rng.gen();
                        if roll < 0.6 {
                            RoomType::Encounter
                        } else if roll < 0.85 {
                            RoomType::Empty
                        } else {
                            RoomType::Treasure
                        }
                    }
                };

                // Spawn room entity
                commands.spawn((
                    DungeonRoom { room_type },
                    GridPosition::new(current_pos.x, current_pos.y),
                    SpatialBundle::default(),
                ));

                room_positions.push(current_pos);

                // Move to next room position (simple corridor system)
                let direction = if rng.gen_bool(0.7) {
                    IVec2::new(3, 0) // Prefer horizontal progression
                } else {
                    IVec2::new(0, 3) // Occasional vertical branch
                };
                current_pos += direction;
            }

                dungeon_gen.rooms_generated = room_count;
                info!("Dungeon '{}' generation complete: {} rooms", event.dungeon_id, room_count);
            }
        }
    }

    pub mod encounters {
        use bevy::prelude::*;
        use bevy_shaman_core::components::{GridPosition, Health};
        use bevy_shaman_monsters::components::{
            MonsterId, MonsterStats, MonsterState, AiBehavior, AiState,
            MusicAffinityProfile,
        };
        use crate::components::{DungeonRoom, RoomType};
        use rand::Rng;

        #[derive(Component)]
        pub struct EncounterSpawned;

        pub fn spawn_encounters(
            mut commands: Commands,
            rooms: Query<(Entity, &DungeonRoom, &GridPosition), Without<EncounterSpawned>>,
        ) {
            let mut rng = rand::thread_rng();

            for (room_entity, room, room_pos) in rooms.iter() {
                // Only spawn in encounter rooms
                if !matches!(room.room_type, RoomType::Encounter) {
                    continue;
                }

                // Mark room as processed
                commands.entity(room_entity).insert(EncounterSpawned);

                // Spawn 1-4 monsters per encounter room
                let monster_count = rng.gen_range(1..=4);

                for i in 0..monster_count {
                    let offset = IVec2::new(i % 2, i / 2);
                    let monster_pos = GridPosition::new(
                        room_pos.x + offset.x,
                        room_pos.y + offset.y,
                    );

                    // Randomize monster type
                    let monster_id = match rng.gen_range(0..3) {
                        0 => "chaos_beast",
                        1 => "corrupt_spirit",
                        _ => "void_creature",
                    };

                    let health_max = rng.gen_range(30.0..60.0);

                    commands.spawn((
                        MonsterId(monster_id.to_string()),
                        monster_pos,
                        Health::new(health_max),
                        MonsterState::default(),
                        MonsterStats {
                            attack: rng.gen_range(8.0..15.0),
                            defense: rng.gen_range(3.0..8.0),
                            speed: rng.gen_range(4.0..7.0),
                            spirit_affinity: rng.gen_range(0.3..0.7),
                        },
                        AiBehavior {
                            behavior_tree_id: "dungeon_aggro".to_string(),
                            aggression: rng.gen_range(0.6..0.9),
                            flee_threshold: rng.gen_range(0.15..0.3),
                        },
                        AiState::Aggressive,
                        MusicAffinityProfile::default(),
                        SpatialBundle::default(),
                    ));
                }

                info!("Spawned {} monsters in encounter room at {:?}", monster_count, room_pos);
            }
        }
    }

    pub mod bosses {
        use bevy::prelude::*;
        use bevy_shaman_core::components::{GridPosition, Health, Player};
        use bevy_shaman_monsters::components::{
            MonsterId, MonsterStats, MonsterState, StateType, AiBehavior, AiState,
            MusicAffinityProfile,
        };
        use crate::components::{BossArena, DungeonRoom, RoomType};
        

        #[derive(Component)]
        pub struct BossSpawned;

        pub fn manage_boss_fights(
            mut commands: Commands,
            mut boss_arenas: Query<(&Health, &mut BossArena)>,
            boss_rooms: Query<(Entity, &DungeonRoom, &GridPosition), Without<BossSpawned>>,
            player: Query<&GridPosition, With<Player>>,
            bosses: Query<(&Health, &BossArena)>,
            mut defeated_events: EventWriter<super::events::BossDefeated>,
        ) {
            let rng = rand::thread_rng();

            // Spawn boss in boss rooms when player enters
            if let Ok(player_pos) = player.get_single() {
                for (room_entity, room, room_pos) in boss_rooms.iter() {
                    if !matches!(room.room_type, RoomType::Boss) {
                        continue;
                    }

                    // Check if player is near boss room (within 5 tiles)
                    if player_pos.distance(room_pos) > 5 {
                        continue;
                    }

                    // Mark room as processed
                    commands.entity(room_entity).insert(BossSpawned);

                    // Spawn boss
                    let boss_id = "corrupted_guardian";
                    let boss_entity = commands.spawn((
                        MonsterId(boss_id.to_string()),
                        GridPosition::new(room_pos.x + 1, room_pos.y + 1),
                        Health::new(200.0),
                        MonsterState {
                            state: StateType::Corrupt,
                            stability_meter: 0.2,
                            corruption_meter: 0.9,
                            obedience_meter: 0.0,
                            chaos_output: 1.5,
                        },
                        MonsterStats {
                            attack: 25.0,
                            defense: 15.0,
                            speed: 6.0,
                            spirit_affinity: 0.1,
                        },
                        AiBehavior {
                            behavior_tree_id: "boss_aggressive".to_string(),
                            aggression: 1.0,
                            flee_threshold: 0.0,
                        },
                        AiState::Aggressive,
                        MusicAffinityProfile::default(),
                        BossArena {
                            boss_id: boss_id.to_string(),
                            phase: 1,
                        },
                        SpatialBundle::default(),
                    )).id();

                    info!("Spawned boss {} in arena", boss_id);
                }
            }

            // Handle boss phase transitions
            for (health, mut arena) in boss_arenas.iter_mut() {
                let health_percent = health.current / health.max;

                // Phase 2: Below 66% health
                if arena.phase == 1 && health_percent < 0.66 {
                    arena.phase = 2;
                    info!("Boss {} entered phase 2!", arena.boss_id);
                }

                // Phase 3: Below 33% health
                if arena.phase == 2 && health_percent < 0.33 {
                    arena.phase = 3;
                    info!("Boss {} entered phase 3 (enraged)!", arena.boss_id);
                }
            }

            // Detect boss defeats
            for (health, arena) in bosses.iter() {
                if health.is_dead() {
                    defeated_events.send(super::events::BossDefeated {
                        boss_id: arena.boss_id.clone(),
                    });
                    info!("Boss {} defeated!", arena.boss_id);
                }
            }
        }
    }
}
