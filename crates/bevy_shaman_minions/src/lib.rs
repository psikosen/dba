use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

#[cfg(test)]
mod tests;

pub struct MinionsPlugin;

impl Plugin for MinionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<systems::formation::FormationConfig>()
            .add_systems(Update, (
                systems::taming::process_taming_attempts,
                systems::taming::update_taming_progress,
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

    #[derive(Component)]
    pub struct TamingProgress {
        pub progress: f32, // 0.0 to 1.0
        pub timer: bevy::time::Timer,
    }

    impl TamingProgress {
        pub fn new(duration_secs: f32) -> Self {
            Self {
                progress: 0.0,
                timer: bevy::time::Timer::from_seconds(duration_secs, bevy::time::TimerMode::Once),
            }
        }

        pub fn is_complete(&self) -> bool {
            self.progress >= 1.0
        }
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
            monsters: Query<(Entity, &MonsterState), (Without<Tamed>, Without<crate::components::TamingProgress>)>,
        ) {
            if !keyboard.just_pressed(KeyCode::KeyT) {
                return;
            }

            // Start taming the nearest controllable monster
            for (entity, state) in monsters.iter() {
                if state.is_controllable() {
                    commands.entity(entity).insert(crate::components::TamingProgress::new(3.0)); // 3 seconds to tame
                    info!("Started taming monster...");
                    break;
                }
            }
        }

        pub fn update_taming_progress(
            mut commands: Commands,
            time: Res<Time>,
            mut taming_query: Query<(Entity, &mut crate::components::TamingProgress), Without<Tamed>>,
        ) {
            for (entity, mut taming) in taming_query.iter_mut() {
                taming.timer.tick(time.delta());
                taming.progress = taming.timer.fraction();

                if taming.is_complete() {
                    commands.entity(entity).remove::<crate::components::TamingProgress>();
                    commands.entity(entity).insert(Tamed {
                        tamed_at: time.elapsed_secs_f64(),
                    });
                    info!("Monster tamed!");
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
            keyboard: Res<ButtonInput<KeyCode>>,
            player: Query<(Entity, &GridPosition), With<Player>>,
            mut minions: Query<
                (Entity, &mut GridPosition, &mut MovementQueue, Option<&MinionFormation>),
                With<Tamed>
            >,
            mut config: ResMut<FormationConfig>,
        ) {
            // Switch formation patterns with F1-F4 keys
            if keyboard.just_pressed(KeyCode::F1) {
                config.pattern = FormationPattern::VShape;
                info!("Formation: V-Shape");
            } else if keyboard.just_pressed(KeyCode::F2) {
                config.pattern = FormationPattern::Circle;
                info!("Formation: Circle");
            } else if keyboard.just_pressed(KeyCode::F3) {
                config.pattern = FormationPattern::Line;
                info!("Formation: Line");
            } else if keyboard.just_pressed(KeyCode::F4) {
                config.pattern = FormationPattern::Box;
                info!("Formation: Box");
            }
            let Ok((player_entity, player_pos)) = player.get_single() else {
                return;
            };

            let minion_count = minions.iter().count();
            if minion_count == 0 {
                return;
            }

            // Assign formation positions to minions
            for (i, (minion_entity, minion_pos, mut movement_queue, formation)) in minions.iter_mut().enumerate() {
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
            _player: Query<&GridPosition, With<Player>>,
            mut minions: Query<
                (Entity, &GridPosition, &mut MovementQueue, &mut AiState, Option<&MinionCommand>),
                With<Tamed>
            >,
            enemies: Query<(Entity, &GridPosition), (Without<Tamed>, Without<Player>)>,
            mut command_events: EventWriter<MinionCommandIssued>,
        ) {
            // Issue attack command with '1' key - find nearest enemy
            if keyboard.just_pressed(KeyCode::Digit1) {
                for (minion_entity, minion_pos, _, mut ai_state, _) in minions.iter_mut() {
                    // Find nearest enemy
                    let nearest_enemy = enemies
                        .iter()
                        .min_by_key(|(_, enemy_pos)| minion_pos.distance(enemy_pos));

                    if let Some((enemy_entity, _)) = nearest_enemy {
                        *ai_state = AiState::Aggressive;
                        commands.entity(minion_entity).insert(MinionCommand {
                            command_type: CommandType::Attack(enemy_entity),
                        });
                        command_events.send(MinionCommandIssued {
                            minion: minion_entity,
                            command: CommandType::Attack(enemy_entity),
                        });
                        info!("Minion commanded to attack nearest enemy");
                    } else {
                        info!("No enemies found to attack");
                    }
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
            for (_minion_entity, minion_pos, mut movement_queue, mut ai_state, command_opt) in minions.iter_mut() {
                if let Some(command) = command_opt {
                    match command.command_type {
                        CommandType::Follow => {
                            // Formation system handles following
                            *ai_state = AiState::Patrol;
                        }
                        CommandType::Attack(target) => {
                            *ai_state = AiState::Aggressive;
                            // Move toward target if it exists
                            if let Ok((_, target_pos)) = enemies.get(target) {
                                let dx = (target_pos.x - minion_pos.x).signum();
                                let dy = (target_pos.y - minion_pos.y).signum();
                                if dx != 0 || dy != 0 {
                                    movement_queue.commands.push(MovementCommand::Move(IVec2::new(dx, dy)));
                                }
                            }
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
