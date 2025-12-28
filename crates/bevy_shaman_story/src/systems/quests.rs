use crate::components::PlayerBrother;
use crate::resources::BrotherCleansingProgress;
use crate::systems::events::BrotherFightCompleted;
use bevy::prelude::*;

/// Updates brother cleansing progress after fights
pub fn update_brother_cleansing_progress(
    mut progress: ResMut<BrotherCleansingProgress>,
    mut brother: Query<&mut PlayerBrother>,
    mut fight_events: EventReader<BrotherFightCompleted>,
) {
    for _event in fight_events.read() {
        progress.complete_fight();

        if let Ok(mut brother) = brother.get_single_mut() {
            brother.fights_remaining = brother.fights_remaining.saturating_sub(1);
            brother.soul_corruption = progress.corruption_percentage();

            info!(
                "Brother cleansing progress: {}/{}",
                progress.fights_completed, progress.total_fights
            );
        }
    }
}
