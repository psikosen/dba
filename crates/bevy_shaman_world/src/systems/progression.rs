use crate::resources::{BossUnlockFlags, PurificationAbility, PurificationShape};
use bevy::prelude::*;

/// Updates purification range and shapes based on boss progression
/// Progression: 1 tile → 3 tiles → 6 tiles → 8 tiles → All screen tiles
pub fn update_purification_range(
    boss_flags: Res<BossUnlockFlags>,
    mut ability: ResMut<PurificationAbility>,
) {
    let bosses_defeated = boss_flags.bosses_defeated_count();

    // Expand purification ability as bosses are defeated
    // 1 → 3 → 6 → 8 → Screen
    match bosses_defeated {
        0 => {
            // Starting: 1 tile at a time
            ability.range = 1;
            ability.shape = PurificationShape::Single;
            ability.spirit_cost = 20.0;
        }
        1..=2 => {
            // First upgrade: 3 tiles
            ability.range = 1;
            ability.shape = PurificationShape::Triangle;
            ability.spirit_cost = 25.0;
        }
        3..=4 => {
            // Second upgrade: 6 tiles
            ability.range = 1;
            ability.shape = PurificationShape::Hexagon;
            ability.spirit_cost = 30.0;
        }
        5..=6 => {
            // Third upgrade: 8 tiles
            ability.range = 1;
            ability.shape = PurificationShape::Octagon;
            ability.spirit_cost = 35.0;
        }
        7.. => {
            // Final upgrade: All tiles on screen
            ability.range = 7; // Screen range
            ability.shape = PurificationShape::Screen;
            ability.spirit_cost = 50.0;
        }
    }
}
