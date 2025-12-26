#[cfg(test)]
mod audio_tests {
    use super::super::resources::*;

    // ============================================================================
    // BEAT CLOCK TESTS
    // ============================================================================

    #[test]
    fn test_beat_clock_default() {
        let clock = BeatClock::default();
        assert_eq!(clock.bpm, 120.0);
        assert_eq!(clock.beat_duration, 0.5);
        assert_eq!(clock.current_beat, 0);
        assert_eq!(clock.time_in_beat, 0.0);
        assert!(!clock.is_playing);
    }

    #[test]
    fn test_beat_clock_set_bpm() {
        let mut clock = BeatClock::default();
        clock.set_bpm(60.0);
        assert_eq!(clock.bpm, 60.0);
        assert_eq!(clock.beat_duration, 1.0);
    }

    #[test]
    fn test_beat_clock_set_bpm_fast() {
        let mut clock = BeatClock::default();
        clock.set_bpm(180.0);
        assert_eq!(clock.bpm, 180.0);
        assert!((clock.beat_duration - 0.333333).abs() < 0.001);
    }

    #[test]
    fn test_beat_clock_tick_when_not_playing() {
        let mut clock = BeatClock::default();
        clock.tick(0.1);
        assert_eq!(clock.time_in_beat, 0.0);
        assert_eq!(clock.current_beat, 0);
    }

    #[test]
    fn test_beat_clock_tick_advances_time() {
        let mut clock = BeatClock::default();
        clock.is_playing = true;
        clock.tick(0.1);
        assert_eq!(clock.time_in_beat, 0.1);
        assert_eq!(clock.current_beat, 0);
    }

    #[test]
    fn test_beat_clock_tick_advances_beat() {
        let mut clock = BeatClock::default();
        clock.is_playing = true;
        clock.set_bpm(120.0); // 0.5 seconds per beat
        clock.tick(0.5);
        assert_eq!(clock.current_beat, 1);
        assert_eq!(clock.time_in_beat, 0.0);
    }

    #[test]
    fn test_beat_clock_tick_multiple_beats() {
        let mut clock = BeatClock::default();
        clock.is_playing = true;
        clock.set_bpm(120.0); // 0.5 seconds per beat
        clock.tick(1.5);
        assert_eq!(clock.current_beat, 3);
        assert_eq!(clock.time_in_beat, 0.0);
    }

    #[test]
    fn test_beat_clock_tick_partial_beat() {
        let mut clock = BeatClock::default();
        clock.is_playing = true;
        clock.set_bpm(120.0); // 0.5 seconds per beat
        clock.tick(0.7);
        assert_eq!(clock.current_beat, 1);
        assert!((clock.time_in_beat - 0.2).abs() < 0.001);
    }

    // ============================================================================
    // TIMING QUALITY EVALUATION TESTS
    // ============================================================================

    #[test]
    fn test_evaluate_timing_perfect_at_start() {
        let mut clock = BeatClock::default();
        clock.is_playing = true;
        clock.time_in_beat = 0.0;
        assert_eq!(clock.evaluate_timing(), TimingQuality::Perfect);
    }

    #[test]
    fn test_evaluate_timing_perfect_near_start() {
        let mut clock = BeatClock::default();
        clock.is_playing = true;
        clock.set_bpm(120.0); // beat_duration = 0.5
        clock.time_in_beat = 0.02; // Within 0.05 of beat
        assert_eq!(clock.evaluate_timing(), TimingQuality::Perfect);
    }

    #[test]
    fn test_evaluate_timing_perfect_near_end() {
        let mut clock = BeatClock::default();
        clock.is_playing = true;
        clock.set_bpm(120.0); // beat_duration = 0.5
        clock.time_in_beat = 0.48; // Within 0.05 of next beat
        assert_eq!(clock.evaluate_timing(), TimingQuality::Perfect);
    }

    #[test]
    fn test_evaluate_timing_great() {
        let mut clock = BeatClock::default();
        clock.is_playing = true;
        clock.set_bpm(120.0); // beat_duration = 0.5
        clock.time_in_beat = 0.06; // Between 0.05 and 0.15
        assert_eq!(clock.evaluate_timing(), TimingQuality::Great);
    }

    #[test]
    fn test_evaluate_timing_good() {
        let mut clock = BeatClock::default();
        clock.is_playing = true;
        clock.set_bpm(120.0); // beat_duration = 0.5
        clock.time_in_beat = 0.1; // distance = 0.2 (20% of beat, between 0.15 and 0.3)
        assert_eq!(clock.evaluate_timing(), TimingQuality::Good);
    }

    #[test]
    fn test_evaluate_timing_miss() {
        let mut clock = BeatClock::default();
        clock.is_playing = true;
        clock.set_bpm(120.0); // beat_duration = 0.5
        clock.time_in_beat = 0.25; // > 0.3 from beat
        assert_eq!(clock.evaluate_timing(), TimingQuality::Miss);
    }

    // ============================================================================
    // TIMING QUALITY MODIFIER TESTS
    // ============================================================================

    #[test]
    fn test_timing_quality_damage_multipliers() {
        assert_eq!(TimingQuality::Perfect.damage_multiplier(), 1.5);
        assert_eq!(TimingQuality::Great.damage_multiplier(), 1.2);
        assert_eq!(TimingQuality::Good.damage_multiplier(), 1.0);
        assert_eq!(TimingQuality::Miss.damage_multiplier(), 0.5);
    }

    #[test]
    fn test_timing_quality_control_modifiers() {
        assert_eq!(TimingQuality::Perfect.control_modifier(), 1.3);
        assert_eq!(TimingQuality::Great.control_modifier(), 1.1);
        assert_eq!(TimingQuality::Good.control_modifier(), 1.0);
        assert_eq!(TimingQuality::Miss.control_modifier(), 0.7);
    }

    // ============================================================================
    // SONG DATABASE TESTS
    // ============================================================================

    #[test]
    fn test_song_db_default() {
        let db = SongDB::default();
        assert_eq!(db.songs.len(), 0);
    }

    #[test]
    fn test_song_db_register() {
        let mut db = SongDB::default();
        let song = Song {
            id: "test".to_string(),
            display_name: "Test Song".to_string(),
            bpm: 100.0,
            style: MusicStyle::Calm,
            unlock_level: 1,
            spirit_cost: 10.0,
            stamina_cost: 5.0,
        };
        db.register(song);
        assert_eq!(db.songs.len(), 1);
    }

    #[test]
    fn test_song_db_get() {
        let mut db = SongDB::default();
        let song = Song {
            id: "test".to_string(),
            display_name: "Test Song".to_string(),
            bpm: 100.0,
            style: MusicStyle::Calm,
            unlock_level: 1,
            spirit_cost: 10.0,
            stamina_cost: 5.0,
        };
        db.register(song);

        let retrieved = db.get("test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().display_name, "Test Song");
    }

    #[test]
    fn test_song_db_get_nonexistent() {
        let db = SongDB::default();
        assert!(db.get("nonexistent").is_none());
    }

    #[test]
    fn test_song_db_populate_defaults() {
        let mut db = SongDB::default();
        db.populate_defaults();

        assert!(db.get("dawn_hymn").is_some());
        assert!(db.get("war_chant").is_some());
        assert!(db.get("purification_rite").is_some());

        let dawn = db.get("dawn_hymn").unwrap();
        assert_eq!(dawn.bpm, 90.0);
        assert_eq!(dawn.style, MusicStyle::Calm);
    }

    // ============================================================================
    // ACTIVE SONG TESTS
    // ============================================================================

    #[test]
    fn test_active_song_default() {
        let active = ActiveSong::default();
        assert_eq!(active.current_song_id, None);
        assert_eq!(active.combo, 0);
    }

    #[test]
    fn test_active_song_start() {
        let mut active = ActiveSong::default();
        active.start_song("test".to_string());
        assert_eq!(active.current_song_id, Some("test".to_string()));
        assert_eq!(active.combo, 0);
    }

    #[test]
    fn test_active_song_stop() {
        let mut active = ActiveSong::default();
        active.start_song("test".to_string());
        active.increment_combo();
        active.stop_song();
        assert_eq!(active.current_song_id, None);
        assert_eq!(active.combo, 0);
    }

    #[test]
    fn test_active_song_increment_combo() {
        let mut active = ActiveSong::default();
        active.start_song("test".to_string());
        active.increment_combo();
        active.increment_combo();
        assert_eq!(active.combo, 2);
    }

    #[test]
    fn test_active_song_break_combo() {
        let mut active = ActiveSong::default();
        active.start_song("test".to_string());
        active.increment_combo();
        active.increment_combo();
        active.break_combo();
        assert_eq!(active.combo, 0);
    }

    // ============================================================================
    // MUSIC STYLE TESTS
    // ============================================================================

    #[test]
    fn test_music_styles_exist() {
        let styles = vec![
            MusicStyle::Calm,
            MusicStyle::Aggressive,
            MusicStyle::Purifying,
            MusicStyle::Harmonizing,
        ];

        // Just ensure they all compile and are different
        for style in styles {
            let song = Song {
                id: "test".to_string(),
                display_name: "Test".to_string(),
                bpm: 120.0,
                style,
                unlock_level: 1,
                spirit_cost: 10.0,
                stamina_cost: 5.0,
            };
            assert_eq!(song.bpm, 120.0);
        }
    }
}
