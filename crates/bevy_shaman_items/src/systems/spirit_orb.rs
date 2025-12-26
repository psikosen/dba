use bevy::prelude::*;
use bevy_shaman_core::components::{Spirit, Stamina};
use crate::components::{Inventory, ItemType};
use crate::systems::events::SpiritOrbConsumed;

const AUTO_USE_THRESHOLD_SPIRIT: f32 = 0.3;  // 30% of max
const AUTO_USE_THRESHOLD_STAMINA: f32 = 0.3;

/// Auto-consumes Spirit Orbs when resources are low
pub fn consume_spirit_orbs(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<(Entity, &mut Spirit, &mut Stamina, &mut Inventory)>,
    mut consume_events: EventWriter<SpiritOrbConsumed>,
) {
    for (entity, mut spirit, mut stamina, mut inventory) in players.iter_mut() {
        let should_auto_consume = spirit.current / spirit.max < AUTO_USE_THRESHOLD_SPIRIT
            || stamina.current / stamina.max < AUTO_USE_THRESHOLD_STAMINA;

        let manual_consume = keyboard.just_pressed(KeyCode::KeyE);

        if !should_auto_consume && !manual_consume {
            continue;
        }

        // Try to consume smallest orb first
        let orb_sizes = [
            ("spirit_orb_small", crate::components::SpiritOrbSize::Small),
            ("spirit_orb_medium", crate::components::SpiritOrbSize::Medium),
            ("spirit_orb_large", crate::components::SpiritOrbSize::Large),
        ];

        for (orb_id, orb_size) in orb_sizes {
            if inventory.count_item(orb_id) > 0 {
                if inventory.remove_item(orb_id, 1) {
                    let spirit_restored = orb_size.spirit_restore();
                    let stamina_restored = orb_size.stamina_restore();

                    spirit.heal(spirit_restored);
                    stamina.current = (stamina.current + stamina_restored).min(stamina.max);

                    consume_events.send(SpiritOrbConsumed {
                        entity,
                        spirit_restored,
                        stamina_restored,
                    });

                    break; // Only consume one orb per tick
                }
            }
        }
    }
}
