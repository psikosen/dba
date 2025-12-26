use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, Player};
use crate::components::{Inventory, Pickupable};
use crate::systems::events::ItemPickedUp;

const PICKUP_RADIUS: f32 = 1.5;

/// Processes item pickups when player is near
pub fn process_item_pickups(
    mut commands: Commands,
    player: Query<(&GridPosition, &mut Inventory), With<Player>>,
    pickupables: Query<(Entity, &GridPosition, &Pickupable)>,
    mut pickup_events: EventWriter<ItemPickedUp>,
) {
    let Ok((player_pos, mut inventory)) = player.get_single() else {
        return;
    };

    for (entity, item_pos, pickupable) in pickupables.iter() {
        let distance = player_pos.distance(item_pos);

        if distance as f32 <= PICKUP_RADIUS {
            if inventory.add_item(pickupable.item.clone(), pickupable.quantity) {
                pickup_events.send(ItemPickedUp {
                    entity,
                    item_id: pickupable.item.id.clone(),
                    quantity: pickupable.quantity,
                });

                commands.entity(entity).despawn();
            }
        }
    }
}
