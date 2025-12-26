use bevy::prelude::*;
use crate::resources::{BossUnlockFlags, PurificationAbility, PurificationShape};

/// Updates purification range and shapes based on boss progression
pub fn update_purification_range(
    boss_flags: Res<BossUnlockFlags>,
    mut ability: ResMut<PurificationAbility>,
) {
    let bosses_defeated = boss_flags.bosses_defeated_count();

    // Expand purification ability as bosses are defeated (levels 1-8)
    match bosses_defeated {
        0 => {
            ability.range = 1;
            ability.shape = PurificationShape::Single;
        }
        1..=2 => {
            ability.range = 2;
            ability.shape = PurificationShape::Cross;
        }
        3..=4 => {
            ability.range = 2;
            ability.shape = PurificationShape::Cluster;
        }
        5..=6 => {
            ability.range = 3;
            ability.shape = PurificationShape::Line;
        }
        7.. => {
            ability.range = 4;
            ability.shape = PurificationShape::Ring;
        }
    }
}
