use crate::components::{NpcName, NpcSicknessState};
use crate::resources::BrotherCleansingProgress;
use crate::systems::events::NpcWokenUp;
use bevy::prelude::*;
use bevy_shaman_world::resources::BossUnlockFlags;

/// Updates NPC waking state based on boss defeats and brother cleansing
pub fn update_npc_waking_state(
    boss_flags: Res<BossUnlockFlags>,
    brother_progress: Res<BrotherCleansingProgress>,
    mut npcs: Query<(Entity, &mut NpcSicknessState, &NpcName)>,
    mut wake_events: EventWriter<NpcWokenUp>,
) {
    let bosses_defeated = boss_flags.bosses_defeated_count();
    let brother_fights_completed = brother_progress.fights_completed;

    for (entity, mut state, npc_name) in npcs.iter_mut() {
        let old_state = *state;

        // Wake NPCs based on progression milestones
        if bosses_defeated >= 2 && brother_fights_completed >= 1 {
            *state = NpcSicknessState::Waking;
        }

        if bosses_defeated >= 5 && brother_fights_completed >= 3 {
            *state = NpcSicknessState::Awake;
        }

        // Emit event if state changed
        if old_state != *state && *state == NpcSicknessState::Awake {
            wake_events.send(NpcWokenUp {
                npc_entity: entity,
                npc_name: npc_name.name.clone(),
            });
        }
    }
}
