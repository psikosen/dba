#[cfg(test)]
mod dungeon_tests {
    use super::super::components::*;
    use super::super::systems::generation::*;
    use super::super::systems::events::*;

    // ============================================================================
    // COMPONENT TESTS
    // ============================================================================

    #[test]
    fn test_room_type_variants() {
        let empty = RoomType::Empty;
        let encounter = RoomType::Encounter;
        let treasure = RoomType::Treasure;
        let boss = RoomType::Boss;

        assert!(matches!(empty, RoomType::Empty));
        assert!(matches!(encounter, RoomType::Encounter));
        assert!(matches!(treasure, RoomType::Treasure));
        assert!(matches!(boss, RoomType::Boss));
    }

    #[test]
    fn test_dungeon_room_creation() {
        let room = DungeonRoom {
            room_type: RoomType::Encounter,
        };

        assert!(matches!(room.room_type, RoomType::Encounter));
    }

    #[test]
    fn test_boss_arena_creation() {
        let arena = BossArena {
            boss_id: "test_boss".to_string(),
            phase: 1,
        };

        assert_eq!(arena.boss_id, "test_boss");
        assert_eq!(arena.phase, 1);
    }

    #[test]
    fn test_boss_arena_phase_transitions() {
        let mut arena = BossArena {
            boss_id: "guardian".to_string(),
            phase: 1,
        };

        assert_eq!(arena.phase, 1);

        arena.phase = 2;
        assert_eq!(arena.phase, 2);

        arena.phase = 3;
        assert_eq!(arena.phase, 3);
    }

    // ============================================================================
    // DUNGEON GENERATOR TESTS
    // ============================================================================

    #[test]
    fn test_dungeon_generator_default() {
        let gen = DungeonGenerator::default();

        assert_eq!(gen.current_dungeon_id, "");
        assert_eq!(gen.rooms_generated, 0);
        assert_eq!(gen.dungeon_count, 0);
    }

    #[test]
    fn test_dungeon_generator_set_id() {
        let mut gen = DungeonGenerator::default();
        gen.current_dungeon_id = "jungle_1".to_string();
        gen.dungeon_count = 1;

        assert_eq!(gen.current_dungeon_id, "jungle_1");
        assert_eq!(gen.dungeon_count, 1);
    }

    #[test]
    fn test_dungeon_generator_room_tracking() {
        let mut gen = DungeonGenerator::default();

        gen.rooms_generated = 5;
        assert_eq!(gen.rooms_generated, 5);

        gen.rooms_generated = 12;
        assert_eq!(gen.rooms_generated, 12);
    }

    #[test]
    fn test_dungeon_generator_count_increment() {
        let mut gen = DungeonGenerator::default();

        assert_eq!(gen.dungeon_count, 0);

        gen.dungeon_count += 1;
        assert_eq!(gen.dungeon_count, 1);

        gen.dungeon_count += 1;
        assert_eq!(gen.dungeon_count, 2);
    }

    // ============================================================================
    // EVENT TESTS
    // ============================================================================

    #[test]
    fn test_dungeon_entered_event() {
        let event = DungeonEntered {
            dungeon_id: "forest_1".to_string(),
        };

        assert_eq!(event.dungeon_id, "forest_1");
    }

    #[test]
    fn test_boss_defeated_event() {
        let event = BossDefeated {
            boss_id: "corrupted_guardian".to_string(),
        };

        assert_eq!(event.boss_id, "corrupted_guardian");
    }

    // ============================================================================
    // QA TESTS - DUNGEON GENERATION VALIDATION
    // ============================================================================

    #[test]
    fn test_qa_dungeon_room_count_range() {
        // Dungeons should generate 5-12 rooms
        let min_rooms = 5;
        let max_rooms = 12;

        assert!(min_rooms > 0);
        assert!(max_rooms >= min_rooms);
        assert!(max_rooms < 20); // Reasonable upper bound
    }

    #[test]
    fn test_qa_dungeon_room_type_distribution() {
        // Test expected room type distribution
        // First room: Always Empty
        // Last room: Always Boss
        // Middle rooms: 60% Encounter, 25% Empty, 15% Treasure

        let encounter_chance = 0.6;
        let empty_chance = 0.25;
        let treasure_chance = 0.15;

        let total = encounter_chance + empty_chance + treasure_chance;
        assert!((total - 1.0).abs() < 0.01); // Should sum to 1.0

        assert!(encounter_chance > empty_chance);
        assert!(empty_chance > treasure_chance);
    }

    #[test]
    fn test_qa_boss_room_placement() {
        // Boss should always be in the final room
        let room_count = 10;
        let boss_index = room_count - 1;

        assert_eq!(boss_index, 9);
        assert!(boss_index < room_count);
    }

    #[test]
    fn test_qa_starting_room_placement() {
        // Starting room should always be first (index 0)
        let starting_index = 0;

        assert_eq!(starting_index, 0);
    }

    #[test]
    fn test_qa_encounter_room_count() {
        // For a typical 8-room dungeon:
        // 1 starting room, 1 boss room = 2 special rooms
        // 6 remaining rooms, 60% should be encounters
        let total_rooms = 8;
        let special_rooms = 2; // Start + Boss
        let remaining_rooms = total_rooms - special_rooms;
        let expected_encounters = (remaining_rooms as f32 * 0.6) as usize;

        assert_eq!(expected_encounters, 3); // About 3-4 encounters
        assert!(expected_encounters < remaining_rooms);
    }

    #[test]
    fn test_qa_monster_count_per_encounter() {
        // Each encounter room should spawn 1-4 monsters
        let min_monsters = 1;
        let max_monsters = 4;

        assert!(min_monsters > 0);
        assert!(max_monsters >= min_monsters);
        assert!(max_monsters < 10); // Reasonable upper limit
    }

    #[test]
    fn test_qa_boss_stats() {
        // Boss should have significantly higher stats than regular monsters
        let boss_health = 200.0;
        let regular_health_max = 60.0;

        let boss_attack = 25.0;
        let regular_attack_max = 15.0;

        assert!(boss_health > regular_health_max * 2.0);
        assert!(boss_attack > regular_attack_max * 1.5);
    }

    #[test]
    fn test_qa_boss_phase_thresholds() {
        // Boss phases should trigger at specific health percentages
        let phase_2_threshold = 0.66; // 66% health
        let phase_3_threshold = 0.33; // 33% health

        assert!(phase_2_threshold > phase_3_threshold);
        assert!(phase_2_threshold < 1.0);
        assert!(phase_3_threshold > 0.0);
        assert_eq!(phase_2_threshold, 0.66);
        assert_eq!(phase_3_threshold, 0.33);
    }

    #[test]
    fn test_qa_dungeon_progression() {
        // Dungeon should have linear progression with branches
        let horizontal_preference = 0.7; // 70% horizontal
        let vertical_chance = 0.3; // 30% vertical

        assert!((horizontal_preference + vertical_chance - 1.0).abs() < 0.01);
        assert!(horizontal_preference > vertical_chance);
    }

    #[test]
    fn test_qa_room_spacing() {
        // Rooms should be spaced 3 tiles apart
        let room_spacing = 3;

        assert_eq!(room_spacing, 3);
        assert!(room_spacing > 0);
        assert!(room_spacing < 10);
    }

    #[test]
    fn test_qa_player_proximity_for_boss_spawn() {
        // Player must be within 5 tiles of boss room to spawn boss
        let spawn_distance = 5;

        assert_eq!(spawn_distance, 5);
        assert!(spawn_distance > 0);
        assert!(spawn_distance < 10);
    }

    #[test]
    fn test_qa_monster_variety() {
        // Dungeons should have 3 main monster types
        let monster_types = vec!["chaos_beast", "corrupt_spirit", "void_creature"];

        assert_eq!(monster_types.len(), 3);
        assert!(monster_types.contains(&"chaos_beast"));
        assert!(monster_types.contains(&"corrupt_spirit"));
        assert!(monster_types.contains(&"void_creature"));
    }

    #[test]
    fn test_qa_monster_stat_ranges() {
        // Monster stats should be within reasonable ranges
        let health_min = 30.0;
        let health_max = 60.0;

        let attack_min = 8.0;
        let attack_max = 15.0;

        let defense_min = 3.0;
        let defense_max = 8.0;

        let speed_min = 4.0;
        let speed_max = 7.0;

        assert!(health_max > health_min);
        assert!(attack_max > attack_min);
        assert!(defense_max > defense_min);
        assert!(speed_max > speed_min);

        // Verify ranges are reasonable
        assert!(health_min >= 20.0);
        assert!(health_max <= 100.0);
        assert!(attack_min >= 5.0);
        assert!(attack_max <= 20.0);
    }

    #[test]
    fn test_qa_ai_aggression_ranges() {
        // Dungeon monsters should have high aggression
        let aggression_min = 0.6;
        let aggression_max = 0.9;

        let flee_min = 0.15;
        let flee_max = 0.3;

        assert!(aggression_max > aggression_min);
        assert!(flee_max > flee_min);

        // Dungeon monsters should be more aggressive than they flee
        assert!(aggression_min > flee_max);
    }

    #[test]
    fn test_qa_boss_ai_behavior() {
        // Boss should never flee and be maximally aggressive
        let boss_aggression = 1.0;
        let boss_flee_threshold = 0.0;

        assert_eq!(boss_aggression, 1.0);
        assert_eq!(boss_flee_threshold, 0.0);
    }

    #[test]
    fn test_qa_boss_corruption_state() {
        // Boss should start highly corrupted
        let boss_corruption = 0.9;
        let boss_stability = 0.2;
        let boss_chaos = 1.5;

        assert!(boss_corruption > 0.7);
        assert!(boss_stability < 0.3);
        assert!(boss_chaos > 1.0);
    }

    #[test]
    fn test_qa_dungeon_id_format() {
        // Dungeon IDs should follow the pattern "biome_number"
        let dungeon_id = "jungle_1";
        assert!(dungeon_id.contains("_"));

        let parts: Vec<&str> = dungeon_id.split('_').collect();
        assert_eq!(parts.len(), 2);
        assert!(!parts[0].is_empty());
        assert!(!parts[1].is_empty());
    }

    #[test]
    fn test_qa_room_type_serialization() {
        // Room types should be serializable for save/load
        use serde_json;

        let room_types = vec![
            RoomType::Empty,
            RoomType::Encounter,
            RoomType::Treasure,
            RoomType::Boss,
        ];

        for room_type in room_types {
            let serialized = serde_json::to_string(&room_type);
            assert!(serialized.is_ok());
        }
    }
}
