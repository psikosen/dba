use bevy::prelude::*;
use crate::components::*;
use bevy_shaman_core::components::Spirit;
use bevy_shaman_monsters::components::{MonsterState, AiState, MonsterStats};

/// Event for attempting to control a monster
#[derive(Event)]
pub struct AttemptMonsterControl {
    pub player: Entity,
    pub target_monster: Entity,
}

/// Event for releasing control of a monster
#[derive(Event)]
pub struct ReleaseMonsterControl {
    pub player: Entity,
}

/// System to handle monster control attempts
pub fn monster_control_attempt(
    mut events: EventReader<AttemptMonsterControl>,
    mut player_query: Query<(&mut MonsterControl, &mut Spirit)>,
    mut monster_query: Query<(&MonsterState, &mut AiState)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for event in events.read() {
        if let Ok((mut control, mut spirit)) = player_query.get_mut(event.player) {
            // Check if already controlling a monster
            if control.controlled_monster.is_some() {
                warn!("Already controlling a monster!");
                continue;
            }

            // Check if target is controllable
            if let Ok((monster_state, mut ai_state)) = monster_query.get_mut(event.target_monster) {
                if !monster_state.is_controllable() {
                    warn!("Monster is too corrupted or unstable to control!");
                    continue;
                }

                // Spirit cost based on monster's obedience
                let spirit_cost = 30.0 * (1.0 - monster_state.obedience_meter);

                if spirit.current < spirit_cost {
                    warn!("Not enough spirit to control this monster!");
                    continue;
                }

                // Consume spirit
                spirit.current -= spirit_cost;

                // Take control
                control.controlled_monster = Some(event.target_monster);
                control.control_duration = 30.0 * control.control_strength;
                control.can_use_abilities = control.control_strength > 0.7;

                // Mark monster as controlled
                commands.entity(event.target_monster).insert(UnderPlayerControl {
                    controller: event.player,
                    started_at: time.elapsed_secs_f64(),
                });

                // Override AI state
                *ai_state = AiState::Stunned;

                info!("Monster control successful! Duration: {:.1}s", control.control_duration);
            }
        }
    }
}

/// System to update monster control duration
pub fn monster_control_update(
    mut player_query: Query<&mut MonsterControl>,
    controlled_monsters: Query<Entity, With<UnderPlayerControl>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for mut control in player_query.iter_mut() {
        if let Some(monster_entity) = control.controlled_monster {
            control.control_duration -= time.delta_secs();

            // Release control when duration expires
            if control.control_duration <= 0.0 {
                if controlled_monsters.get(monster_entity).is_ok() {
                    commands.entity(monster_entity).remove::<UnderPlayerControl>();
                }
                control.controlled_monster = None;
                info!("Monster control expired!");
            }
        }
    }
}

/// System to handle manual release of monster control
pub fn monster_control_release(
    mut events: EventReader<ReleaseMonsterControl>,
    mut player_query: Query<&mut MonsterControl>,
    controlled_monsters: Query<Entity, With<UnderPlayerControl>>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let Ok(mut control) = player_query.get_mut(event.player) {
            if let Some(monster_entity) = control.controlled_monster {
                if controlled_monsters.get(monster_entity).is_ok() {
                    commands.entity(monster_entity).remove::<UnderPlayerControl>();
                }
                control.controlled_monster = None;
                info!("Released monster control.");
            }
        }
    }
}

/// System to allow using controlled monster abilities
pub fn controlled_monster_abilities(
    player_query: Query<&MonsterControl, With<bevy_shaman_core::components::Player>>,
    monster_query: Query<(&MonsterStats, &MonsterState), With<UnderPlayerControl>>,
    // TODO: Add ability system
) {
    if let Ok(control) = player_query.get_single() {
        if !control.can_use_abilities {
            return;
        }

        if let Some(monster_entity) = control.controlled_monster {
            if let Ok((stats, state)) = monster_query.get(monster_entity) {
                // Player can use the monster's abilities
                // This would integrate with an ability system
                // For now, just log that abilities are available
                debug!(
                    "Controlled monster abilities available - Attack: {}, Spirit Affinity: {}",
                    stats.attack, stats.spirit_affinity
                );
            }
        }
    }
}
