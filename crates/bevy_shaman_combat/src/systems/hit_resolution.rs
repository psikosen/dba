use bevy::prelude::*;
use bevy_shaman_core::components::Health;
use crate::components::Attack;
use crate::systems::events::HitLanded;

pub fn resolve_hits(
    mut commands: Commands,
    attacks: Query<(Entity, &Attack)>,
    mut targets: Query<&mut Health>,
    mut hit_events: EventWriter<HitLanded>,
) {
    for (attack_entity, attack) in attacks.iter() {
        if let Ok(mut health) = targets.get_mut(attack.target) {
            health.damage(attack.damage);

            hit_events.send(HitLanded {
                attacker: Entity::PLACEHOLDER,
                target: attack.target,
                damage: attack.damage,
            });

            commands.entity(attack_entity).despawn();
        }
    }
}
