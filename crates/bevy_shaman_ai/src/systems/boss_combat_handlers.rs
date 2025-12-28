/// Boss Combat Event Handlers
/// Handles the actual execution of boss combat actions triggered by LLM AI decisions
use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, Health};
use bevy_shaman_combat::components::{StatusEffect, StatusEffectType, StatusEffects};
use bevy_shaman_monsters::components::{MonsterState, AiState};
use crate::systems::boss_ai::*;
use crate::components::{CombatStance, LlmAi};

// ============================================================================
// SPECIAL MOVE HANDLER
// ============================================================================

/// Handles boss special move execution with damage and effects
pub fn handle_special_move(
    trigger: Trigger<SpecialMoveTriggered>,
    mut boss_query: Query<(&mut LlmAi, &GridPosition, &MonsterState)>,
    mut player_query: Query<(&mut Health, &GridPosition, Entity), With<bevy_shaman_core::components::Player>>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let boss_entity = trigger.entity();

    if let Ok((mut ai, boss_pos, _monster_state)) = boss_query.get_mut(boss_entity) {
        info!(
            "Executing special move '{}' from boss in phase {}",
            event.move_name, event.boss_phase
        );

        // Find player and apply damage
        if let Ok((mut player_health, player_pos, player_entity)) = player_query.get_single_mut() {
            let distance = calculate_distance(boss_pos, player_pos);

            // Special moves have area of effect
            let move_range = match event.move_name.as_str() {
                name if name.contains("sweep") || name.contains("whirlwind") => 3.0,
                name if name.contains("charge") || name.contains("dash") => 5.0,
                name if name.contains("slam") || name.contains("pound") => 2.0,
                _ => 2.5,
            };

            if distance <= move_range {
                // Calculate damage based on multiplier
                let base_damage = 25.0;
                let final_damage = base_damage * event.damage_multiplier;

                player_health.current = (player_health.current - final_damage).max(0.0);

                info!(
                    "Special move '{}' hits player for {:.1} damage (distance: {:.1})",
                    event.move_name, final_damage, distance
                );

                // Spawn visual effect
                commands.trigger_targets(
                    SpawnSpecialMoveEffect {
                        move_name: event.move_name.clone(),
                        position: *boss_pos,
                        intensity: event.damage_multiplier,
                    },
                    boss_entity,
                );

                // Apply status effects based on move type
                apply_move_status_effects(
                    &event.move_name,
                    event.boss_phase,
                    &mut commands,
                    player_entity,
                );

                // Update boss emotional state after successful hit
                if ai.personality.aggression > 0.7 {
                    ai.emotional_state = crate::components::EmotionalState::Confident;
                }
            } else {
                info!(
                    "Special move '{}' missed - player out of range ({:.1} > {:.1})",
                    event.move_name, distance, move_range
                );
            }
        }
    }
}

fn calculate_distance(pos1: &GridPosition, pos2: &GridPosition) -> f32 {
    let dx = (pos1.x - pos2.x) as f32;
    let dy = (pos1.y - pos2.y) as f32;
    (dx * dx + dy * dy).sqrt()
}

fn apply_move_status_effects(
    move_name: &str,
    phase: u32,
    commands: &mut Commands,
    target: Entity,
) {
    let effect_duration = 3.0 + (phase as f32 * 0.5);

    match move_name {
        name if name.contains("poison") || name.contains("venom") => {
            if let Some(mut entity_commands) = commands.get_entity(target) {
                entity_commands.insert(StatusEffects {
                    effects: vec![StatusEffect {
                        effect_type: StatusEffectType::Poison,
                        duration: effect_duration,
                        strength: 0.5 * phase as f32,
                    }],
                });
            }
        }
        name if name.contains("stun") || name.contains("slam") => {
            if let Some(mut entity_commands) = commands.get_entity(target) {
                entity_commands.insert(StatusEffects {
                    effects: vec![StatusEffect {
                        effect_type: StatusEffectType::Stun,
                        duration: 1.0 + (phase as f32 * 0.3),
                        strength: 1.0,
                    }],
                });
            }
        }
        name if name.contains("burn") || name.contains("fire") => {
            if let Some(mut entity_commands) = commands.get_entity(target) {
                entity_commands.insert(StatusEffects {
                    effects: vec![StatusEffect {
                        effect_type: StatusEffectType::Burn,
                        duration: effect_duration,
                        strength: 0.3 * phase as f32,
                    }],
                });
            }
        }
        name if name.contains("slow") || name.contains("roots") => {
            if let Some(mut entity_commands) = commands.get_entity(target) {
                entity_commands.insert(StatusEffects {
                    effects: vec![StatusEffect {
                        effect_type: StatusEffectType::Slow,
                        duration: effect_duration,
                        strength: 0.6,
                    }],
                });
            }
        }
        _ => {} // No status effect for this move
    }
}

// ============================================================================
// SUMMON MINION HANDLER
// ============================================================================

/// Handles boss summoning minions
pub fn handle_summon_minion(
    trigger: Trigger<SummonMinionEvent>,
    boss_query: Query<(&GridPosition, &LlmAi)>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let boss_entity = trigger.entity();

    if let Ok((boss_pos, ai)) = boss_query.get(boss_entity) {
        info!(
            "Boss {} summoning {} x{}",
            ai.character_name, event.minion_type, event.count
        );

        // Spawn minions around boss
        for i in 0..event.count {
            let angle = (i as f32 / event.count as f32) * std::f32::consts::TAU;
            let offset_x = (angle.cos() * 2.0) as i32;
            let offset_y = (angle.sin() * 2.0) as i32;

            let minion_pos = GridPosition {
                x: boss_pos.x + offset_x,
                y: boss_pos.y + offset_y,
            };

            // Spawn minion
            let minion = commands
                .spawn((
                    Name::new(format!("{} Minion", event.minion_type)),
                    minion_pos,
                    Health { current: 30.0, max: 30.0 },
                    MonsterState::default(),
                    AiState::Aggressive,
                    // Mark as summoned minion
                    SummonedMinion {
                        summoner: boss_entity,
                        minion_type: event.minion_type.clone(),
                    },
                ))
                .id();

            info!("Spawned minion {} at {:?}", minion_type_name(&event.minion_type), minion_pos);

            // Spawn visual effect
            commands.trigger_targets(
                SpawnSummonEffect {
                    position: minion_pos,
                    minion_type: event.minion_type.clone(),
                },
                minion,
            );
        }
    }
}

fn minion_type_name(minion_type: &str) -> &str {
    match minion_type {
        "shadow" => "Shadow Wraith",
        "spirit" => "Corrupted Spirit",
        "beast" => "Summoned Beast",
        "elemental" => "Elemental",
        _ => "Minion",
    }
}

/// Component to mark entities as summoned minions
#[derive(Component)]
pub struct SummonedMinion {
    pub summoner: Entity,
    pub minion_type: String,
}

// ============================================================================
// STANCE CHANGE HANDLER
// ============================================================================

/// Handles boss changing combat stance
pub fn handle_stance_change(
    trigger: Trigger<StanceChangeEvent>,
    mut boss_query: Query<(&mut LlmAi, &mut MonsterState)>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let boss_entity = trigger.entity();

    if let Ok((mut ai, mut monster_state)) = boss_query.get_mut(boss_entity) {
        let old_stance = ai.combat_stance;
        ai.combat_stance = event.new_stance;

        info!(
            "Boss {} changed stance: {:?} -> {:?}",
            ai.character_name, old_stance, event.new_stance
        );

        // Apply stance modifiers
        match event.new_stance {
            CombatStance::Aggressive => {
                // +20% move speed (would be applied via status effects)
            }
            CombatStance::Defensive => {
                // Defense buff would be applied via status effects
            }
            CombatStance::Tactical => {
                // Balanced - no modifiers
            }
            CombatStance::Evasive => {
                // +40% move speed (would be applied via status effects)
            }
            CombatStance::Summoner => {
                // Summoner stance
            }
            CombatStance::Corrupting => {
                // Corruption-focused stance
                monster_state.corruption_meter = (monster_state.corruption_meter + 0.2).min(1.0);
            }
        }

        // Spawn visual effect for stance change
        commands.trigger_targets(
            StanceChangeEffect {
                old_stance,
                new_stance: event.new_stance,
            },
            boss_entity,
        );
    }
}

// ============================================================================
// ABILITY HANDLER
// ============================================================================

/// Handles boss using special abilities
pub fn handle_ability_triggered(
    trigger: Trigger<AbilityTriggered>,
    boss_query: Query<(&LlmAi, &GridPosition)>,
    player_query: Query<(&GridPosition, Entity), With<bevy_shaman_core::components::Player>>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let boss_entity = trigger.entity();

    if let Ok((ai, boss_pos)) = boss_query.get(boss_entity) {
        info!(
            "Boss {} uses ability: {} (distance: {:.1})",
            ai.character_name, event.ability_name, event.distance_to_target
        );

        if let Ok((player_pos, player_entity)) = player_query.get_single() {
            match event.ability_name.as_str() {
                "fear" => apply_fear_ability(&mut commands, player_entity, boss_pos),
                "heal" => apply_heal_ability(&mut commands, boss_entity),
                "teleport" => apply_teleport_ability(&mut commands, boss_entity, player_pos),
                "shield" => apply_shield_ability(&mut commands, boss_entity),
                "rage" => apply_rage_ability(&mut commands, boss_entity),
                _ => {
                    warn!("Unknown ability: {}", event.ability_name);
                }
            }

            // Spawn ability effect
            commands.trigger_targets(
                AbilityEffect {
                    ability_name: event.ability_name.clone(),
                    position: *boss_pos,
                },
                boss_entity,
            );
        }
    }
}

fn apply_fear_ability(commands: &mut Commands, target: Entity, _source_pos: &GridPosition) {
    if let Some(mut entity_commands) = commands.get_entity(target) {
        entity_commands.insert(StatusEffects {
            effects: vec![StatusEffect {
                effect_type: StatusEffectType::Stun, // Use Stun as fear equivalent
                duration: 3.0,
                strength: 1.0,
            }],
        });
    }
    info!("Applied Fear effect (as Stun) to player");
}

fn apply_heal_ability(commands: &mut Commands, boss: Entity) {
    commands.trigger_targets(
        HealEffect { amount: 50.0 },
        boss,
    );
    info!("Boss healed for 50 HP");
}

fn apply_teleport_ability(commands: &mut Commands, boss: Entity, target_pos: &GridPosition) {
    // Teleport near player
    let new_pos = GridPosition {
        x: target_pos.x + 2,
        y: target_pos.y,
    };
    commands.trigger_targets(
        TeleportEffect { new_position: new_pos },
        boss,
    );
    info!("Boss teleported to {:?}", new_pos);
}

fn apply_shield_ability(commands: &mut Commands, boss: Entity) {
    if let Some(mut entity_commands) = commands.get_entity(boss) {
        entity_commands.insert(StatusEffects {
            effects: vec![StatusEffect {
                effect_type: StatusEffectType::Purifying, // Use Purifying as protection
                duration: 5.0,
                strength: 0.5, // 50% damage reduction
            }],
        });
    }
    info!("Boss activated shield");
}

fn apply_rage_ability(commands: &mut Commands, boss: Entity) {
    if let Some(mut entity_commands) = commands.get_entity(boss) {
        entity_commands.insert(StatusEffects {
            effects: vec![StatusEffect {
                effect_type: StatusEffectType::Burn, // Use Burn as rage damage
                duration: 8.0,
                strength: 1.5, // 150% damage increase
            }],
        });
    }
    info!("Boss entered rage mode");
}

// ============================================================================
// CORRUPTION SPREAD HANDLER
// ============================================================================

/// Handles boss spreading corruption
pub fn handle_corruption_spread(
    trigger: Trigger<CorruptionSpreadEvent>,
    boss_query: Query<&GridPosition>,
    player_query: Query<(Entity, &GridPosition), With<bevy_shaman_core::components::Player>>,
    mut commands: Commands,
) {
    let event = trigger.event();
    let boss_entity = trigger.entity();

    if let Ok(boss_pos) = boss_query.get(boss_entity) {
        info!(
            "Boss spreading corruption! Radius: {:.1}, Intensity: {:.1}",
            event.radius, event.intensity
        );

        // Apply corruption damage to player if in range
        if let Ok((player_entity, player_pos)) = player_query.get_single() {
            let distance = calculate_distance(boss_pos, player_pos);
            if distance <= event.radius {
                let corruption_damage = 10.0 * event.intensity;
                commands.trigger_targets(
                    CorruptionDamage {
                        amount: corruption_damage,
                    },
                    player_entity,
                );
                info!("Player takes {:.1} corruption damage", corruption_damage);
            }
        }

        // Spawn corruption visual effect
        commands.trigger_targets(
            CorruptionSpreadEffect {
                position: *boss_pos,
                radius: event.radius,
                intensity: event.intensity,
            },
            boss_entity,
        );
    }
}

// ============================================================================
// VISUAL EFFECT EVENTS (to be handled by rendering systems)
// ============================================================================

#[derive(Event)]
pub struct SpawnSpecialMoveEffect {
    pub move_name: String,
    pub position: GridPosition,
    pub intensity: f32,
}

#[derive(Event)]
pub struct SpawnSummonEffect {
    pub position: GridPosition,
    pub minion_type: String,
}

#[derive(Event)]
pub struct StanceChangeEffect {
    pub old_stance: CombatStance,
    pub new_stance: CombatStance,
}

#[derive(Event)]
pub struct AbilityEffect {
    pub ability_name: String,
    pub position: GridPosition,
}

#[derive(Event)]
pub struct HealEffect {
    pub amount: f32,
}

#[derive(Event)]
pub struct TeleportEffect {
    pub new_position: GridPosition,
}

#[derive(Event)]
pub struct CorruptionDamage {
    pub amount: f32,
}

#[derive(Event)]
pub struct CorruptionSpreadEffect {
    pub position: GridPosition,
    pub radius: f32,
    pub intensity: f32,
}

// ============================================================================
// MINION CLEANUP
// ============================================================================

/// Clean up minions when summoner dies
pub fn cleanup_minions_on_summoner_death(
    mut commands: Commands,
    dead_bosses: Query<Entity, (With<LlmAi>, Without<Health>)>,
    minions: Query<(Entity, &SummonedMinion)>,
) {
    for dead_boss in dead_bosses.iter() {
        // Find and despawn all minions summoned by this boss
        for (minion_entity, summoned) in minions.iter() {
            if summoned.summoner == dead_boss {
                info!("Despawning minion because summoner died");
                commands.entity(minion_entity).despawn_recursive();
            }
        }
    }
}
