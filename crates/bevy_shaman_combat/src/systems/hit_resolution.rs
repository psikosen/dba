use crate::components::Attack;
use crate::systems::events::HitLanded;
use bevy::prelude::*;
use bevy_shaman_core::components::Health;

pub fn resolve_hits(
    mut commands: Commands,
    attacks: Query<(Entity, &Attack)>,
    mut targets: Query<&mut Health>,
    mut hit_events: EventWriter<HitLanded>,
    mut death_events: EventWriter<bevy_shaman_core::events::EntityDied>,
) {
    for (attack_entity, attack) in attacks.iter() {
        if let Ok(mut health) = targets.get_mut(attack.target) {
            health.damage(attack.damage);

            hit_events.send(HitLanded {
                attacker: attack.attacker,
                target: attack.target,
                damage: attack.damage,
            });

            // Check if target died
            if health.is_dead() {
                death_events.send(bevy_shaman_core::events::EntityDied {
                    entity: attack.target,
                    was_player: false, // Will be updated by death handler
                });
            }

            commands.entity(attack_entity).despawn();
        }
    }
}
