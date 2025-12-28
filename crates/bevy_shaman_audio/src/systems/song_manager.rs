use crate::resources::{ActiveSong, BeatClock, SongDB};
use bevy::prelude::*;

/// Manages active song state and beat clock synchronization
pub fn manage_active_song(
    active_song: Res<ActiveSong>,
    song_db: Res<SongDB>,
    mut clock: ResMut<BeatClock>,
) {
    if let Some(song_id) = &active_song.current_song_id {
        if let Some(song) = song_db.get(song_id) {
            // Synchronize BPM with active song
            if (clock.bpm - song.bpm).abs() > 0.1 {
                clock.set_bpm(song.bpm);
            }

            if !clock.is_playing {
                clock.is_playing = true;
            }
        }
    } else {
        clock.is_playing = false;
    }
}
