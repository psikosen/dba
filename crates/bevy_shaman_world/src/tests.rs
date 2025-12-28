#[cfg(test)]
mod world_tests {
    use super::super::components::*;
    use super::super::resources::*;
    use super::super::systems::generation::*;

    // ============================================================================
    // COMPONENT TESTS
    // ============================================================================

    #[test]
    fn test_tile_corruption_default() {
        let corruption = TileCorruption::default();
        assert_eq!(corruption.level, 0.0);
        assert_eq!(corruption.corruption_type, CorruptionType::None);
        assert!(!corruption.purified);
        assert!(corruption.purified_timestamp.is_none());
    }

    #[test]
    fn test_tile_corruption_is_corrupt() {
        let mut corruption = TileCorruption::default();
        assert!(!corruption.is_corrupt());

        corruption.level = 0.4;
        assert!(corruption.is_corrupt());
    }

    #[test]
    fn test_biome_can_have_dungeons() {
        assert!(BiomeType::Jungle.can_have_dungeons());
        assert!(BiomeType::Desert.can_have_dungeons());
        assert!(!BiomeType::Village.can_have_dungeons());
    }

    // ============================================================================
    // RESOURCE TESTS
    // ============================================================================

    #[test]
    fn test_purification_ability_default() {
        let ability = PurificationAbility::default();
        assert_eq!(ability.range, 1);
        assert_eq!(ability.spirit_cost, 20.0);
    }

    #[test]
    fn test_boss_unlock_flags_default() {
        let flags = BossUnlockFlags::default();
        assert_eq!(flags.bosses_defeated_count(), 0);
    }

    // ============================================================================
    // WORLD GENERATION CONFIG TESTS
    // ============================================================================

    #[test]
    fn test_world_gen_config_default() {
        let config = WorldGenConfig::default();
        assert_eq!(config.world_radius, 150);
        assert_eq!(config.village_radius, 20);
    }

    #[test]
    fn test_world_generated_default() {
        let generated = WorldGenerated::default();
        assert!(!generated.0);
    }

    // ============================================================================
    // ADDITIONAL CORRUPTION TESTS
    // ============================================================================

    #[test]
    fn test_corruption_type_variants() {
        let none = CorruptionType::None;
        let chaos = CorruptionType::Chaos;
        let decay = CorruptionType::Decay;
        let void = CorruptionType::Void;
        let ancestral = CorruptionType::Ancestral;

        assert!(matches!(none, CorruptionType::None));
        assert!(matches!(chaos, CorruptionType::Chaos));
        assert!(matches!(decay, CorruptionType::Decay));
        assert!(matches!(void, CorruptionType::Void));
        assert!(matches!(ancestral, CorruptionType::Ancestral));
    }

    #[test]
    fn test_tile_corruption_levels() {
        let mut corruption = TileCorruption::default();

        // Test boundary conditions
        corruption.level = 0.0;
        assert!(!corruption.is_corrupt());

        corruption.level = 0.29;
        assert!(!corruption.is_corrupt());

        corruption.level = 0.3;
        assert!(corruption.is_corrupt());

        corruption.level = 1.0;
        assert!(corruption.is_corrupt());
    }

    #[test]
    fn test_tile_corruption_purification() {
        let mut corruption = TileCorruption {
            level: 0.8,
            corruption_type: CorruptionType::Shadow,
            purified: false,
            purified_timestamp: None,
        };

        assert!(corruption.is_corrupt());
        assert!(!corruption.purified);

        corruption.purified = true;
        corruption.level = 0.0;
        assert!(corruption.purified);
        assert!(!corruption.is_corrupt());
    }

    #[test]
    fn test_biome_type_all_variants() {
        // Test that all biome types exist and can be constructed
        let biomes = vec![
            BiomeType::Grassland,
            BiomeType::Forest,
            BiomeType::Desert,
            BiomeType::Mountain,
            BiomeType::Swamp,
            BiomeType::Jungle,
            BiomeType::Village,
        ];

        assert_eq!(biomes.len(), 7);
    }

    #[test]
    fn test_biome_dungeon_permissions() {
        // Test which biomes can have dungeons
        assert!(BiomeType::Forest.can_have_dungeons());
        assert!(BiomeType::Mountain.can_have_dungeons());
        assert!(BiomeType::Swamp.can_have_dungeons());
        assert!(!BiomeType::Grassland.can_have_dungeons());
    }

    #[test]
    fn test_purification_ability_upgrade() {
        let mut ability = PurificationAbility::default();
        assert_eq!(ability.range, 1);
        assert_eq!(ability.spirit_cost, 20.0);

        // Simulate upgrade
        ability.range = 3;
        ability.spirit_cost = 30.0;

        assert_eq!(ability.range, 3);
        assert_eq!(ability.spirit_cost, 30.0);
    }

    #[test]
    fn test_boss_unlock_flags_progression() {
        let mut flags = BossUnlockFlags::default();
        assert_eq!(flags.bosses_defeated_count(), 0);

        // Simulate defeating bosses
        flags.jungle_guardian = true;
        assert_eq!(flags.bosses_defeated_count(), 1);

        flags.desert_tyrant = true;
        assert_eq!(flags.bosses_defeated_count(), 2);

        flags.mountain_elder = true;
        flags.swamp_horror = true;
        assert_eq!(flags.bosses_defeated_count(), 4);
    }

    #[test]
    fn test_world_seed_is_numeric() {
        let seed = WorldSeed::default();
        assert!(seed.0 > 0);
    }

    #[test]
    fn test_world_gen_config_custom_values() {
        let mut config = WorldGenConfig::default();

        config.world_radius = 200;
        config.village_radius = 30;
        config.ecosystem_count = 7;
        config.dungeons_per_ecosystem = 5;

        assert_eq!(config.world_radius, 200);
        assert_eq!(config.village_radius, 30);
        assert_eq!(config.ecosystem_count, 7);
        assert_eq!(config.dungeons_per_ecosystem, 5);
    }

    #[test]
    fn test_world_generated_flag() {
        let mut generated = WorldGenerated::default();
        assert!(!generated.0);

        generated.0 = true;
        assert!(generated.0);
    }

    // ============================================================================
    // ERROR TESTS
    // ============================================================================

    #[test]
    fn test_world_generation_error_display() {
        use super::super::WorldGenerationError;

        let err = WorldGenerationError::InvalidConfiguration("test error".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Invalid world generation configuration"));
        assert!(display.contains("test error"));
    }

    #[test]
    fn test_world_generation_error_variants() {
        use super::super::WorldGenerationError;

        let errors = vec![
            WorldGenerationError::InvalidConfiguration("config".to_string()),
            WorldGenerationError::GenerationFailed("failed".to_string()),
            WorldGenerationError::BiomePlacementFailed("biome".to_string()),
            WorldGenerationError::DungeonSpawnFailed("dungeon".to_string()),
            WorldGenerationError::TimeError("time".to_string()),
        ];

        assert_eq!(errors.len(), 5);
    }

    #[test]
    fn test_world_generation_error_is_error_trait() {
        use super::super::WorldGenerationError;
        use std::error::Error;

        let err = WorldGenerationError::GenerationFailed("test".to_string());
        let _: &dyn Error = &err; // Should compile if Error trait is implemented
    }
}
