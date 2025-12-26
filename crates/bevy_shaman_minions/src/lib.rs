pub mod components;
pub mod systems;

use bevy::prelude::*;
use bevy_shaman_core::states::GameState;

pub struct MinionsPlugin;

impl Plugin for MinionsPlugin {
    fn build(&self, app: &mut App) {
        app
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
        use crate::components::MinionFormation;

        pub fn update_minion_formation(_formations: Query<&MinionFormation>) {
            // Placeholder: update minion positions based on leader
        }
    }

    pub mod commands {
        use bevy::prelude::*;
        use crate::components::MinionCommand;

        pub fn process_minion_commands(_commands_query: Query<&MinionCommand>) {
            // Placeholder: execute minion commands
        }
    }
}
