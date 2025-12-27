use bevy::prelude::*;
use bevy_shaman_core::components::Health;
use crate::components::{StatusEffectType, StatusEffects};

pub fn apply_status_effects(
    time: Res<Time>,
    mut entities: Query<(&mut StatusEffects, &mut Health)>,
) {
    for (mut status_effects, mut health) in entities.iter_mut() {
        status_effects.effects.retain_mut(|effect| {
            effect.duration -= time.delta_secs();

            match effect.effect_type {
                StatusEffectType::Burn | StatusEffectType::Poison => {
                    health.damage(effect.strength * time.delta_secs());
                }
                _ => {}
            }

            effect.duration > 0.0
        });
    }
}
