use bevy::prelude::*;
use bevy_shaman_core::components::{Health, GridPosition, Spirit, Player};
use rand::Rng;

use crate::components::focus_abilities::*;
use crate::components::StatusEffects;

// ============================================================================
// FOCUS REGENERATION
// ============================================================================

/// Regenerate shaman focus over time
pub fn regenerate_shaman_focus(
    mut focus_query: Query<&mut ShamanFocus>,
    time: Res<Time>,
) {
    for mut focus in focus_query.iter_mut() {
        if focus.current < focus.max {
            focus.current = (focus.current + focus.regen_rate * time.delta_secs()).min(focus.max);
        }
    }
}

// ============================================================================
// FOCUS ABILITY CASTING
// ============================================================================

/// Handle focus ability casting input and initiation
pub fn handle_focus_ability_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<(Entity, &ShamanFocus, &FocusAbilityCooldowns), With<Player>>,
    mut cast_events: EventWriter<FocusAbilityCast>,
) {
    let Ok((player_entity, focus, cooldowns)) = player_query.get_single() else {
        return;
    };

    // Map keyboard inputs to abilities
    let ability = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(FocusAbilityType::Heal)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(FocusAbilityType::Stun)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(FocusAbilityType::PushBack)
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        Some(FocusAbilityType::PullIn)
    } else if keyboard.just_pressed(KeyCode::Digit5) {
        Some(FocusAbilityType::Lift)
    } else if keyboard.just_pressed(KeyCode::Digit6) {
        Some(FocusAbilityType::Pacify)
    } else if keyboard.just_pressed(KeyCode::Digit7) {
        Some(FocusAbilityType::Purify)
    } else if keyboard.just_pressed(KeyCode::Digit8) {
        Some(FocusAbilityType::SpiritInfusion)
    } else {
        None
    };

    if let Some(ability_type) = ability {
        // Check cooldown
        if cooldowns.is_on_cooldown(ability_type) {
            info!("Ability {:?} on cooldown: {:.1}s", ability_type, cooldowns.get_remaining(ability_type));
            return;
        }

        // Check focus cost
        if focus.current < ability_type.focus_cost() {
            info!("Not enough focus for {:?}", ability_type);
            return;
        }

        // Start casting
        cast_events.send(FocusAbilityCast {
            caster: player_entity,
            ability: ability_type,
            target: None,  // Target selection would be handled separately
        });
    }
}

/// Process focus ability casting
pub fn process_focus_ability_casting(
    mut commands: Commands,
    mut cast_events: EventReader<FocusAbilityCast>,
    mut player_query: Query<(Entity, &mut ShamanFocus, &mut FocusAbilityCooldowns), With<Player>>,
) {
    for event in cast_events.read() {
        if let Ok((entity, mut focus, mut cooldowns)) = player_query.get_mut(event.caster) {
            // Deduct focus cost
            focus.current -= event.ability.focus_cost();

            // Add active ability component
            commands.entity(entity).insert(ActiveFocusAbility {
                ability_type: event.ability,
                target: event.target,
                cast_timer: Timer::from_seconds(event.ability.cast_time(), TimerMode::Once),
                duration_timer: event.ability.duration().map(|d| Timer::from_seconds(d, TimerMode::Once)),
            });

            info!("Casting {:?}...", event.ability);
        }
    }
}

/// Update active ability casting timers
pub fn update_focus_ability_casting(
    mut commands: Commands,
    mut casting_query: Query<(Entity, &mut ActiveFocusAbility)>,
    mut completed_events: EventWriter<FocusAbilityCompleted>,
    time: Res<Time>,
) {
    for (entity, mut active_ability) in casting_query.iter_mut() {
        active_ability.cast_timer.tick(time.delta());

        if active_ability.cast_timer.just_finished() {
            // Cast complete!
            completed_events.send(FocusAbilityCompleted {
                caster: entity,
                ability: active_ability.ability_type,
                target: active_ability.target,
                success: true,
            });

            // Remove casting component
            commands.entity(entity).remove::<ActiveFocusAbility>();
        }
    }
}

/// Apply focus ability effects when casting completes
pub fn apply_focus_ability_effects(
    mut commands: Commands,
    mut completed_events: EventReader<FocusAbilityCompleted>,
    mut player_query: Query<(&mut FocusAbilityCooldowns, &GridPosition), With<Player>>,
    mut health_query: Query<&mut Health>,
    mut status_effects_query: Query<&mut StatusEffects>,
    position_query: Query<&GridPosition>,
) {
    for event in completed_events.read() {
        if !event.success {
            continue;
        }

        let Ok((mut cooldowns, caster_pos)) = player_query.get_mut(event.caster) else {
            continue;
        };

        match event.ability {
            FocusAbilityType::Heal => {
                apply_heal_effect(&mut health_query, event.target, 50.0);
            }

            FocusAbilityType::Stun => {
                apply_stun_effect(&mut status_effects_query, event.target, 3.0);
            }

            FocusAbilityType::Disable => {
                apply_disable_effect(&mut status_effects_query, event.target, 5.0);
            }

            FocusAbilityType::Pacify => {
                apply_pacify_effect(event.target);
            }

            FocusAbilityType::PushBack => {
                apply_push_effect(caster_pos, event.target, &position_query, 5.0, &mut commands);
            }

            FocusAbilityType::PullIn => {
                apply_pull_effect(caster_pos, event.target, &position_query, 3.0, &mut commands);
            }

            FocusAbilityType::Lift => {
                apply_lift_effect(event.target, &mut commands);
            }

            FocusAbilityType::Purify => {
                apply_purify_effect(&mut status_effects_query, event.target);
            }

            FocusAbilityType::SpiritInfusion => {
                apply_spirit_infusion(&mut commands, event.caster, &mut cooldowns);
            }
        }

        // Add cooldown
        let cooldown_duration = if event.ability == FocusAbilityType::SpiritInfusion {
            // Spirit infusion uses random cooldown from the infusion component
            0.0  // Will be set by infusion system
        } else {
            event.ability.cooldown()
        };

        if cooldown_duration > 0.0 {
            cooldowns.add_cooldown(event.ability, cooldown_duration);
        }

        info!("Applied {:?} effect", event.ability);
    }
}

// ============================================================================
// ABILITY EFFECT IMPLEMENTATIONS
// ============================================================================

fn apply_heal_effect(
    health_query: &mut Query<&mut Health>,
    target: Option<Entity>,
    heal_amount: f32,
) {
    if let Some(target_entity) = target {
        if let Ok(mut health) = health_query.get_mut(target_entity) {
            health.current = (health.current + heal_amount).min(health.max);
            info!("Healed {} for {}", target_entity, heal_amount);
        }
    }
}

fn apply_stun_effect(
    status_effects_query: &mut Query<&mut StatusEffects>,
    target: Option<Entity>,
    duration: f32,
) {
    if let Some(target_entity) = target {
        if let Ok(mut effects) = status_effects_query.get_mut(target_entity) {
            effects.effects.push(crate::components::StatusEffect {
                effect_type: crate::components::StatusEffectType::Stun,
                duration,
                strength: 1.0,
            });
        }
    }
}

fn apply_disable_effect(
    status_effects_query: &mut Query<&mut StatusEffects>,
    target: Option<Entity>,
    duration: f32,
) {
    if let Some(target_entity) = target {
        if let Ok(mut effects) = status_effects_query.get_mut(target_entity) {
            effects.effects.push(crate::components::StatusEffect {
                effect_type: crate::components::StatusEffectType::Slow,
                duration,
                strength: 0.7,  // 70% speed reduction
            });
        }
    }
}

fn apply_pacify_effect(target: Option<Entity>) {
    // Pacify would set aggression to 0 - requires monster AI system integration
    if let Some(_target_entity) = target {
        // TODO: Integrate with monster AI to set calm state
    }
}

fn apply_push_effect(
    caster_pos: &GridPosition,
    target: Option<Entity>,
    position_query: &Query<&GridPosition>,
    force: f32,
    commands: &mut Commands,
) {
    if let Some(target_entity) = target {
        if let Ok(target_pos) = position_query.get(target_entity) {
            // Calculate push direction
            let dx = target_pos.x - caster_pos.x;
            let dy = target_pos.y - caster_pos.y;
            let direction = Vec2::new(dx as f32, dy as f32).normalize_or_zero();

            commands.entity(target_entity).insert(BeingManipulated {
                force_direction: direction,
                force_strength: force,
                manipulator: target_entity,
            });
        }
    }
}

fn apply_pull_effect(
    caster_pos: &GridPosition,
    target: Option<Entity>,
    position_query: &Query<&GridPosition>,
    force: f32,
    commands: &mut Commands,
) {
    if let Some(target_entity) = target {
        if let Ok(target_pos) = position_query.get(target_entity) {
            // Calculate pull direction (opposite of push)
            let dx = caster_pos.x - target_pos.x;
            let dy = caster_pos.y - target_pos.y;
            let direction = Vec2::new(dx as f32, dy as f32).normalize_or_zero();

            commands.entity(target_entity).insert(BeingManipulated {
                force_direction: direction,
                force_strength: force,
                manipulator: target_entity,
            });
        }
    }
}

fn apply_lift_effect(target: Option<Entity>, commands: &mut Commands) {
    if let Some(target_entity) = target {
        // Lift disables movement - would need integration with movement system
        commands.entity(target_entity).insert(BeingManipulated {
            force_direction: Vec2::ZERO,
            force_strength: 0.0,
            manipulator: target_entity,
        });
    }
}

fn apply_purify_effect(
    status_effects_query: &mut Query<&mut StatusEffects>,
    target: Option<Entity>,
) {
    if let Some(target_entity) = target {
        if let Ok(mut effects) = status_effects_query.get_mut(target_entity) {
            // Clear all negative status effects
            effects.effects.clear();
        }
    }
}

fn apply_spirit_infusion(
    commands: &mut Commands,
    caster: Entity,
    cooldowns: &mut FocusAbilityCooldowns,
) {
    let infusion = SpiritInfused::new_random();

    // Add cooldown for next spirit infusion use
    cooldowns.add_cooldown(FocusAbilityType::SpiritInfusion, infusion.cooldown_override);

    // Apply infusion to caster
    commands.entity(caster).insert(infusion);

    info!("Spirit infusion active! Duration: {:.1}s, Next cooldown: {:.1}s",
        commands.entity(caster).get::<SpiritInfused>().map(|si| si.duration_remaining).unwrap_or(0.0),
        cooldowns.get_remaining(FocusAbilityType::SpiritInfusion)
    );
}

// ============================================================================
// COOLDOWN MANAGEMENT
// ============================================================================

/// Update ability cooldowns
pub fn update_ability_cooldowns(
    mut cooldown_query: Query<&mut FocusAbilityCooldowns>,
    time: Res<Time>,
) {
    for mut cooldowns in cooldown_query.iter_mut() {
        for cooldown in &mut cooldowns.cooldowns {
            cooldown.remaining = (cooldown.remaining - time.delta_secs()).max(0.0);
        }

        // Remove expired cooldowns
        cooldowns.cooldowns.retain(|cd| cd.remaining > 0.0);
    }
}

// ============================================================================
// SPIRIT INFUSION DURATION
// ============================================================================

/// Update spirit infusion duration and remove when expired
pub fn update_spirit_infusion(
    mut commands: Commands,
    mut infused_query: Query<(Entity, &mut SpiritInfused)>,
    time: Res<Time>,
) {
    for (entity, mut infusion) in infused_query.iter_mut() {
        infusion.duration_remaining -= time.delta_secs();

        if infusion.duration_remaining <= 0.0 {
            commands.entity(entity).remove::<SpiritInfused>();
            info!("Spirit infusion expired");
        }
    }
}

// ============================================================================
// OBJECT MANIPULATION
// ============================================================================

/// Apply force to manipulated objects
pub fn apply_object_manipulation(
    mut commands: Commands,
    mut manipulated_query: Query<(Entity, &BeingManipulated, &mut GridPosition)>,
    time: Res<Time>,
) {
    for (entity, manipulation, mut position) in manipulated_query.iter_mut() {
        if manipulation.force_strength > 0.0 {
            // Apply force as grid movement
            let movement = manipulation.force_direction * manipulation.force_strength * time.delta_secs();
            position.x += movement.x as i32;
            position.y += movement.y as i32;
        }

        // Remove manipulation after one frame (instantaneous effect)
        commands.entity(entity).remove::<BeingManipulated>();
    }
}

/// Handle object breaking from manipulation
pub fn check_object_breaking(
    mut commands: Commands,
    manipulated_query: Query<(Entity, &BeingManipulated, &ManipulableObject)>,
    mut broken_events: EventWriter<ObjectManipulated>,
) {
    for (entity, manipulation, object) in manipulated_query.iter() {
        if object.can_be_broken(manipulation.force_strength) {
            broken_events.send(ObjectManipulated {
                object: entity,
                manipulator: manipulation.manipulator,
                manipulation_type: ManipulationType::Broken,
            });

            // Despawn broken object
            commands.entity(entity).despawn_recursive();
            info!("Object {:?} broken!", object.object_type);
        }
    }
}
