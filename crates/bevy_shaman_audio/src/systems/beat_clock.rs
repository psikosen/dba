use bevy::prelude::*;
use crate::resources::BeatClock;
use crate::systems::events::BeatHit;

/// Updates the global beat clock
pub fn update_beat_clock(
    time: Res<Time>,
    mut clock: ResMut<BeatClock>,
    mut beat_events: EventWriter<BeatHit>,
) {
    let previous_beat = clock.current_beat;
    clock.tick(time.delta_secs());

    // Emit event when beat changes
    if clock.current_beat != previous_beat {
        beat_events.send(BeatHit {
            beat_number: clock.current_beat,
        });
    }
}
