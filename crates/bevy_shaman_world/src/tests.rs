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
}
