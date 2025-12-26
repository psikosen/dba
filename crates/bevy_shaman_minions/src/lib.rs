use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct MinionsPlugin;

impl Plugin for MinionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<systems::formation::FormationConfig>()
            .add_systems(Update, (
                systems::taming::process_taming_attempts,
                systems::formation::update_minion_formation,
                systems::commands::process_minion_commands,
            ).run_if(in_state(GameState::Playing)))
            .add_event::<systems::events::MinionTamed>()
            .add_event::<systems::events::MinionCommandIssued>();
    }
}

pub mod components {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct MinionFormation {
        pub leader: Entity,
        pub position_index: usize,
    }

    #[derive(Component)]
    pub struct MinionCommand {
        pub command_type: CommandType,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum CommandType {
        Follow,
        Attack(Entity),
        Stay,
    }
}

pub mod systems {
    pub mod events {
        use bevy::prelude::*;

        #[derive(Event)]
        pub struct MinionTamed {
            pub entity: Entity,
        }

        #[derive(Event)]
        pub struct MinionCommandIssued {
            pub minion: Entity,
            pub command: super::super::components::CommandType,
        }
    }

    pub mod taming {
        use bevy::prelude::*;
        use bevy_shaman_core::components::Player;
        use bevy_shaman_monsters::components::{MonsterState, Tamed};

        pub fn process_taming_attempts(
            mut commands: Commands,
            keyboard: Res<ButtonInput<KeyCode>>,
            player: Query<Entity, With<Player>>,
            monsters: Query<(Entity, &MonsterState), Without<Tamed>>,
        ) {
            if !keyboard.just_pressed(KeyCode::KeyT) {
                return;
            }

            // Simplified: tame nearest monster if controllable
            for (entity, state) in monsters.iter() {
                if state.is_controllable() {
                    commands.entity(entity).insert(Tamed {
                        tamed_at: 0.0,
                    });
                    info!("Monster tamed!");
                    break;
                }
            }
        }
    }

    pub mod formation {
        use bevy::prelude::*;
        use bevy_shaman_core::components::{GridPosition, MovementQueue, MovementCommand, Player};
        use bevy_shaman_monsters::components::Tamed;
        use crate::components::MinionFormation;

        #[derive(Resource)]
        pub struct FormationConfig {
            pub pattern: FormationPattern,
            pub spacing: i32,
        }

        impl Default for FormationConfig {
            fn default() -> Self {
                Self {
                    pattern: FormationPattern::VShape,
                    spacing: 2,
                }
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum FormationPattern {
            VShape,
            Circle,
            Line,
            Box,
        }

        pub fn update_minion_formation(
            mut commands: Commands,
            player: Query<(Entity, &GridPosition), With<Player>>,
            mut minions: Query<
                (Entity, &mut GridPosition, &mut MovementQueue, Option<&MinionFormation>),
                With<Tamed>
            >,
            config: Res<FormationConfig>,
        ) {
            let Ok((player_entity, player_pos)) = player.get_single() else {
                return;
            };

            let minion_count = minions.iter().count();
            if minion_count == 0 {
                return;
            }

            // Assign formation positions to minions
            for (i, (minion_entity, mut minion_pos, mut movement_queue, formation)) in minions.iter_mut().enumerate() {
                // Add formation component if missing
                if formation.is_none() {
                    commands.entity(minion_entity).insert(MinionFormation {
                        leader: player_entity,
                        position_index: i,
                    });
                }

                // Calculate target position based on formation pattern
                let target_offset = match config.pattern {
                    FormationPattern::VShape => {
                        let side = if i % 2 == 0 { 1 } else { -1 };
                        let row = (i / 2) as i32;
                        IVec2::new(side * (row + 1) * config.spacing, -(row + 1) * config.spacing)
                    }
                    FormationPattern::Circle => {
                        let angle = (i as f32 / minion_count as f32) * std::f32::consts::TAU;
                        let radius = 3.0;
                        IVec2::new(
                            (angle.cos() * radius) as i32,
                            (angle.sin() * radius) as i32,
                        )
                    }
                    FormationPattern::Line => {
                        IVec2::new(0, -(i as i32 + 1) * config.spacing)
                    }
                    FormationPattern::Box => {
                        let side = (minion_count as f32).sqrt().ceil() as i32;
                        let x = (i as i32 % side) - side / 2;
                        let y = -((i as i32 / side) + 1);
                        IVec2::new(x * config.spacing, y * config.spacing)
                    }
                };

                let target_pos = GridPosition::new(
                    player_pos.x + target_offset.x,
                    player_pos.y + target_offset.y,
                );

                // Move toward target position if not already there
                if minion_pos.x != target_pos.x || minion_pos.y != target_pos.y {
                    let dx = (target_pos.x - minion_pos.x).signum();
                    let dy = (target_pos.y - minion_pos.y).signum();
                    movement_queue.commands.push(MovementCommand::Move(IVec2::new(dx, dy)));
                }
            }
        }
    }

    pub mod commands {
        use bevy::prelude::*;
        use bevy_shaman_core::components::{GridPosition, MovementQueue, MovementCommand, Player};
        use bevy_shaman_monsters::components::{Tamed, AiState};
        use crate::components::{MinionCommand, CommandType};
        use super::events::MinionCommandIssued;

        pub fn process_minion_commands(
            mut commands: Commands,
            keyboard: Res<ButtonInput<KeyCode>>,
            player: Query<&GridPosition, With<Player>>,
            mut minions: Query<
                (Entity, &GridPosition, &mut MovementQueue, &mut AiState, Option<&MinionCommand>),
                With<Tamed>
            >,
            mut command_events: EventWriter<MinionCommandIssued>,
        ) {
            // Issue attack command with '1' key
            if keyboard.just_pressed(KeyCode::Digit1) {
                for (minion_entity, _, _, mut ai_state, _) in minions.iter_mut() {
                    *ai_state = AiState::Aggressive;
                    commands.entity(minion_entity).insert(MinionCommand {
                        command_type: CommandType::Attack(Entity::PLACEHOLDER),
                    });
                    command_events.send(MinionCommandIssued {
                        minion: minion_entity,
                        command: CommandType::Attack(Entity::PLACEHOLDER),
                    });
                    info!("Minion commanded to attack");
                }
            }

            // Issue defend/stay command with '2' key
            if keyboard.just_pressed(KeyCode::Digit2) {
                for (minion_entity, _, _, mut ai_state, _) in minions.iter_mut() {
                    *ai_state = AiState::Idle;
                    commands.entity(minion_entity).insert(MinionCommand {
                        command_type: CommandType::Stay,
                    });
                    command_events.send(MinionCommandIssued {
                        minion: minion_entity,
                        command: CommandType::Stay,
                    });
                    info!("Minion commanded to stay/defend");
                }
            }

            // Issue follow command with '3' key
            if keyboard.just_pressed(KeyCode::Digit3) {
                for (minion_entity, _, _, mut ai_state, _) in minions.iter_mut() {
                    *ai_state = AiState::Patrol;
                    commands.entity(minion_entity).insert(MinionCommand {
                        command_type: CommandType::Follow,
                    });
                    command_events.send(MinionCommandIssued {
                        minion: minion_entity,
                        command: CommandType::Follow,
                    });
                    info!("Minion commanded to follow");
                }
            }

            // Execute commands for minions
            for (minion_entity, minion_pos, mut movement_queue, mut ai_state, command_opt) in minions.iter_mut() {
                if let Some(command) = command_opt {
                    match command.command_type {
                        CommandType::Follow => {
                            // Formation system handles following
                            *ai_state = AiState::Patrol;
                        }
                        CommandType::Attack(target) => {
                            *ai_state = AiState::Aggressive;
                            // Attack logic would be handled by combat system
                        }
                        CommandType::Stay => {
                            *ai_state = AiState::Idle;
                            movement_queue.commands.clear();
                        }
                    }
                }
            }
        }
    }
}
