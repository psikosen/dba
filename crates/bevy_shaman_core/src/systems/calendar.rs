use crate::resources::GameCalendar;
use bevy::prelude::*;

/// Update the in-game calendar based on real-time passage
pub fn update_calendar(mut calendar: ResMut<GameCalendar>, time: Res<Time>) {
    calendar.update(time.delta_secs());
}
