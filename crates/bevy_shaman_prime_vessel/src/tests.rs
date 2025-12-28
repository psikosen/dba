#[cfg(test)]
mod prime_vessel_tests {
    use super::super::components::*;
    use super::super::resources::*;

    // ============================================================================
    // PRIME VESSEL COMPONENT TESTS
    // ============================================================================

    #[test]
    fn test_prime_vessel_default() {
        let vessel = PrimeVessel::default();
        assert_eq!(vessel.power_level, 100.0);
        assert_eq!(vessel.max_power_reached, 100.0);
        assert_eq!(vessel.evolution_tier, 0);
        assert_eq!(vessel.lesser_selves_shed, 0);
        assert!(vessel.is_active);
        assert!(!vessel.is_defeated);
        assert!(vessel.current_target.is_none());
        assert_eq!(vessel.roaming_state, VesselRoamingState::Wandering);
    }

    #[test]
    fn test_next_evolution_threshold() {
        let mut vessel = PrimeVessel::default();

        // Tier 0: 500 * 2.5^0 = 500
        assert_eq!(vessel.next_evolution_threshold(), 500.0);

        // Tier 1: 500 * 2.5^1 = 1250
        vessel.evolution_tier = 1;
        assert_eq!(vessel.next_evolution_threshold(), 1250.0);

        // Tier 2: 500 * 2.5^2 = 3125
        vessel.evolution_tier = 2;
        assert_eq!(vessel.next_evolution_threshold(), 3125.0);

        // Tier 3: 500 * 2.5^3 = 7812.5
        vessel.evolution_tier = 3;
        assert_eq!(vessel.next_evolution_threshold(), 7812.5);
    }

    #[test]
    fn test_should_evolve() {
        let mut vessel = PrimeVessel::default();
        vessel.power_level = 499.0;
        assert!(!vessel.should_evolve());

        vessel.power_level = 500.0;
        assert!(vessel.should_evolve());

        vessel.power_level = 1000.0;
        assert!(vessel.should_evolve());

        // Should not evolve at max tier (10)
        vessel.evolution_tier = 10;
        vessel.power_level = 1000000.0;
        assert!(!vessel.should_evolve());
    }

    #[test]
    fn test_vessel_titles() {
        let mut vessel = PrimeVessel::default();

        vessel.evolution_tier = 0;
        assert_eq!(vessel.title(), "The Nascent Vessel");

        vessel.evolution_tier = 1;
        assert_eq!(vessel.title(), "The Awakened Vessel");

        vessel.evolution_tier = 5;
        assert_eq!(vessel.title(), "The Dominating Vessel");

        vessel.evolution_tier = 10;
        assert_eq!(vessel.title(), "The Prime Chaos");

        vessel.evolution_tier = 99;
        assert_eq!(vessel.title(), "The Vessel");
    }

    // ============================================================================
    // METABOLIC HISTORY TESTS
    // ============================================================================

    #[test]
    fn test_metabolic_history_absorb() {
        let mut history = MetabolicHistory::default();
        assert_eq!(history.spirit_count(), 0);

        history.absorb(10.0, 0);
        assert_eq!(history.spirit_count(), 1);
        assert_eq!(history.total_power(), 10.0);

        history.absorb(20.0, 1);
        assert_eq!(history.spirit_count(), 2);
        assert_eq!(history.total_power(), 30.0);
    }

    #[test]
    fn test_metabolic_history_decay() {
        let mut history = MetabolicHistory::default();

        // Add spirits at different days
        history.absorb(10.0, 0);
        history.absorb(20.0, 100);
        history.absorb(30.0, 500);

        assert_eq!(history.spirit_count(), 3);
        assert_eq!(history.total_power(), 60.0);

        // Process decay at day 799 - nothing should decay yet
        let power_lost = history.process_decay(799);
        assert_eq!(power_lost, 0.0);
        assert_eq!(history.spirit_count(), 3);

        // Process decay at day 800 - first spirit should decay (absorbed day 0)
        let power_lost = history.process_decay(800);
        assert_eq!(power_lost, 10.0);
        assert_eq!(history.spirit_count(), 2);
        assert_eq!(history.total_power(), 50.0);

        // Process decay at day 900 - second spirit should decay (absorbed day 100)
        let power_lost = history.process_decay(900);
        assert_eq!(power_lost, 20.0);
        assert_eq!(history.spirit_count(), 1);
        assert_eq!(history.total_power(), 30.0);

        // Process decay at day 1300 - third spirit should decay (absorbed day 500)
        let power_lost = history.process_decay(1300);
        assert_eq!(power_lost, 30.0);
        assert_eq!(history.spirit_count(), 0);
        assert_eq!(history.total_power(), 0.0);
    }

    #[test]
    fn test_metabolic_history_multiple_decay_at_once() {
        let mut history = MetabolicHistory::default();

        // Add multiple spirits that will all decay at same time
        history.absorb(10.0, 0);
        history.absorb(20.0, 1);
        history.absorb(30.0, 2);

        // All three should decay when we hit day 802 (0+800, 1+800, 2+800)
        let power_lost = history.process_decay(802);
        assert_eq!(power_lost, 60.0);
        assert_eq!(history.spirit_count(), 0);
    }

    #[test]
    fn test_metabolic_history_total_power() {
        let mut history = MetabolicHistory::default();
        assert_eq!(history.total_power(), 0.0);

        history.absorb(15.5, 0);
        history.absorb(25.3, 10);
        history.absorb(33.7, 20);

        let expected = 15.5 + 25.3 + 33.7;
        assert!((history.total_power() - expected).abs() < 0.01);
    }

    // ============================================================================
    // LESSER SELF TESTS
    // ============================================================================

    #[test]
    fn test_lesser_self_from_prime_vessel() {
        let mutations = vec![
            VesselMutation::VenomousStrike,
            VesselMutation::ChitinousArmor,
        ];
        let lesser = LesserSelf::from_prime_vessel(3, 250.0, 1, 100, mutations.clone(), (10, 20));

        assert_eq!(lesser.origin_tier, 3);
        assert_eq!(lesser.power_level, 250.0);
        assert_eq!(lesser.generation, 1);
        assert_eq!(lesser.creation_day, 100);
        assert_eq!(lesser.mutations, mutations);
        assert_eq!(lesser.patrol_route.len(), 4);
        assert_eq!(lesser.patrol_index, 0);
    }

    #[test]
    fn test_lesser_self_power_cap() {
        let mutations = vec![];

        // Power above cap should be clamped
        let lesser = LesserSelf::from_prime_vessel(5, 10000.0, 1, 100, mutations.clone(), (0, 0));

        assert_eq!(lesser.power_level, LesserSelf::MAX_POWER);
        assert_eq!(lesser.power_level, 500.0);

        // Power below cap should remain unchanged
        let lesser2 = LesserSelf::from_prime_vessel(2, 300.0, 2, 200, mutations, (0, 0));

        assert_eq!(lesser2.power_level, 300.0);
    }

    #[test]
    fn test_lesser_self_title() {
        let lesser = LesserSelf::from_prime_vessel(3, 250.0, 5, 100, vec![], (0, 0));

        assert_eq!(lesser.title(), "Lesser Self (Gen 5)");
    }

    #[test]
    fn test_lesser_self_patrol_route() {
        let lesser = LesserSelf::from_prime_vessel(1, 100.0, 1, 0, vec![], (10, 20));

        // Should generate 4-point patrol route
        assert_eq!(lesser.patrol_route.len(), 4);
        assert_eq!(lesser.patrol_route[0], (10, 20));
        assert_eq!(lesser.patrol_route[1], (15, 20));
        assert_eq!(lesser.patrol_route[2], (15, 25));
        assert_eq!(lesser.patrol_route[3], (10, 25));
    }

    // ============================================================================
    // WORLD SPIRIT TESTS
    // ============================================================================

    #[test]
    fn test_world_spirit_default() {
        let spirit = WorldSpirit::default();
        assert_eq!(spirit.power, 10.0);
        assert_eq!(spirit.spirit_type, WorldSpiritType::Neutral);
        assert!(!spirit.is_purified);
        assert!(!spirit.being_absorbed);
        assert!(spirit.absorber.is_none());
    }

    #[test]
    fn test_world_spirit_new() {
        let spirit = WorldSpirit::new(25.0, WorldSpiritType::Ancestral);
        assert_eq!(spirit.power, 25.0);
        assert_eq!(spirit.spirit_type, WorldSpiritType::Ancestral);
        assert!(!spirit.is_purified);
        assert!(!spirit.being_absorbed);
    }

    #[test]
    fn test_world_spirit_type_multipliers() {
        assert_eq!(WorldSpiritType::Neutral.power_multiplier(), 1.0);
        assert_eq!(WorldSpiritType::Ancestral.power_multiplier(), 2.0);
        assert_eq!(WorldSpiritType::Chaos.power_multiplier(), 1.5);
        assert_eq!(WorldSpiritType::Harmony.power_multiplier(), 1.5);
        assert_eq!(WorldSpiritType::Void.power_multiplier(), 5.0);
        assert_eq!(WorldSpiritType::Trapped.power_multiplier(), 0.5);
    }

    // ============================================================================
    // SOUL ALIGNMENT TESTS
    // ============================================================================

    #[test]
    fn test_soul_alignment_default() {
        let alignment = SoulAlignment::default();
        assert_eq!(alignment.alignment, 0.0);
        assert_eq!(alignment.aggression_modifier, 1.0);
        assert!(!alignment.chaos_locked);
    }

    #[test]
    fn test_soul_alignment_update_from_corruption() {
        let mut alignment = SoulAlignment::default();

        // Low corruption
        alignment.update_from_corruption(0.2);
        assert_eq!(alignment.alignment, 0.2);
        assert_eq!(alignment.aggression_modifier, 1.0 + (0.2 * 0.2 * 2.0));

        // Medium corruption
        alignment.update_from_corruption(0.5);
        assert_eq!(alignment.alignment, 0.5);
        assert_eq!(alignment.aggression_modifier, 1.0 + (0.5 * 0.5 * 2.0));

        // High corruption
        alignment.update_from_corruption(0.9);
        assert_eq!(alignment.alignment, 0.9);
        assert_eq!(alignment.aggression_modifier, 1.0 + (0.9 * 0.9 * 2.0));
    }

    #[test]
    fn test_soul_alignment_chaos_lock() {
        let mut alignment = SoulAlignment::default();
        alignment.update_from_corruption(0.5);

        alignment.lock_to_chaos();
        assert_eq!(alignment.alignment, 1.0);
        assert_eq!(alignment.aggression_modifier, 3.0);
        assert!(alignment.chaos_locked);

        // Should not update after being locked
        alignment.update_from_corruption(0.0);
        assert_eq!(alignment.alignment, 1.0);
        assert_eq!(alignment.aggression_modifier, 3.0);
    }

    // ============================================================================
    // RESURRECTION RITUAL TESTS
    // ============================================================================

    #[test]
    fn test_resurrection_ritual_default() {
        let ritual = ResurrectionRitual::default();
        assert_eq!(ritual.spirits_required, 1000);
        assert_eq!(ritual.spirits_offered, 0);
        assert!(!ritual.is_complete);
        assert!(ritual.started_day.is_none());
    }

    #[test]
    fn test_resurrection_ritual_offer_spirit() {
        let mut ritual = ResurrectionRitual::default();

        ritual.offer_spirit(100.0);
        assert_eq!(ritual.spirits_offered, 100);
        assert!(!ritual.is_complete);

        ritual.offer_spirit(400.0);
        assert_eq!(ritual.spirits_offered, 500);
        assert!(!ritual.is_complete);

        ritual.offer_spirit(500.0);
        assert_eq!(ritual.spirits_offered, 1000);
        assert!(ritual.is_complete);

        // Should still work after completion
        ritual.offer_spirit(100.0);
        assert_eq!(ritual.spirits_offered, 1100);
    }

    #[test]
    fn test_resurrection_ritual_progress_percentage() {
        let mut ritual = ResurrectionRitual::default();

        assert_eq!(ritual.progress_percentage(), 0.0);

        ritual.offer_spirit(250.0);
        assert_eq!(ritual.progress_percentage(), 0.25);

        ritual.offer_spirit(250.0);
        assert_eq!(ritual.progress_percentage(), 0.5);

        ritual.offer_spirit(500.0);
        assert_eq!(ritual.progress_percentage(), 1.0);

        // Should cap at 1.0
        ritual.offer_spirit(500.0);
        assert_eq!(ritual.progress_percentage(), 1.0);
    }

    // ============================================================================
    // GLOBAL CORRUPTION INDEX TESTS
    // ============================================================================

    #[test]
    fn test_global_corruption_index_default() {
        let index = GlobalCorruptionIndex::default();
        assert_eq!(index.initial_spirit_count, 15_000);
        assert_eq!(index.free_spirit_count, 15_000);
        assert_eq!(index.vessel_consumed, 0);
        assert_eq!(index.player_consumed, 0);
        assert_eq!(index.spirits_purified, 0);
        assert_eq!(index.spirits_freed, 0);
        assert!(!index.total_collapse_triggered);
        assert!(!index.ui_revealed);
        assert_eq!(index.corruption_percentage, 0.0);
    }

    #[test]
    fn test_corruption_consume_spirit_by_vessel() {
        let mut index = GlobalCorruptionIndex::default();

        index.consume_spirit(true);
        assert_eq!(index.free_spirit_count, 14_999);
        assert_eq!(index.vessel_consumed, 1);
        assert_eq!(index.player_consumed, 0);
        assert!(index.corruption_percentage > 0.0);
    }

    #[test]
    fn test_corruption_consume_spirit_by_player() {
        let mut index = GlobalCorruptionIndex::default();

        index.consume_spirit(false);
        assert_eq!(index.free_spirit_count, 14_999);
        assert_eq!(index.vessel_consumed, 0);
        assert_eq!(index.player_consumed, 1);
        assert!(index.corruption_percentage > 0.0);
    }

    #[test]
    fn test_corruption_consume_multiple_spirits() {
        let mut index = GlobalCorruptionIndex::default();

        index.consume_spirits(100, true);
        assert_eq!(index.free_spirit_count, 14_900);
        assert_eq!(index.vessel_consumed, 100);

        index.consume_spirits(50, false);
        assert_eq!(index.free_spirit_count, 14_850);
        assert_eq!(index.player_consumed, 50);
    }

    #[test]
    fn test_corruption_consume_more_than_available() {
        let mut index = GlobalCorruptionIndex::default();
        index.free_spirit_count = 10;

        // Try to consume more than available
        index.consume_spirits(20, true);

        // Should only consume what's available
        assert_eq!(index.free_spirit_count, 0);
        assert_eq!(index.vessel_consumed, 10);
    }

    #[test]
    fn test_corruption_purify_spirit() {
        let mut index = GlobalCorruptionIndex::default();

        // Consume some spirits first
        index.consume_spirits(1000, true);
        let corruption_before = index.corruption_percentage;

        // Purify spirits (should reduce corruption)
        index.purify_spirit();
        assert_eq!(index.spirits_purified, 1);

        // Corruption should be lower
        assert!(index.corruption_percentage < corruption_before);
    }

    #[test]
    fn test_corruption_free_spirit() {
        let mut index = GlobalCorruptionIndex::default();

        // Consume spirits
        index.consume_spirits(100, true);
        assert_eq!(index.free_spirit_count, 14_900);

        // Free a spirit
        index.free_spirit();
        assert_eq!(index.free_spirit_count, 14_901);
        assert_eq!(index.spirits_freed, 1);
    }

    #[test]
    fn test_corruption_free_multiple_spirits() {
        let mut index = GlobalCorruptionIndex::default();

        index.consume_spirits(200, false);
        index.free_spirits(50);

        assert_eq!(index.free_spirit_count, 14_850);
        assert_eq!(index.spirits_freed, 50);
    }

    #[test]
    fn test_corruption_danger_levels() {
        let mut index = GlobalCorruptionIndex::default();

        // 0% corruption = level 0
        assert_eq!(index.danger_level(), 0);

        // 25% corruption = level 1
        index.corruption_percentage = 0.25;
        assert_eq!(index.danger_level(), 1);

        // 50% corruption = level 2
        index.corruption_percentage = 0.5;
        assert_eq!(index.danger_level(), 2);

        // 70% corruption = level 3
        index.corruption_percentage = 0.7;
        assert_eq!(index.danger_level(), 3);

        // 90% corruption = level 4
        index.corruption_percentage = 0.9;
        assert_eq!(index.danger_level(), 4);

        // 100% corruption = level 5
        index.corruption_percentage = 1.0;
        assert_eq!(index.danger_level(), 5);
    }

    #[test]
    fn test_corruption_world_state_descriptions() {
        let mut index = GlobalCorruptionIndex::default();

        index.corruption_percentage = 0.0;
        assert_eq!(
            index.world_state_description(),
            "The spirits rest peacefully. Order prevails."
        );

        index.corruption_percentage = 0.3;
        assert_eq!(
            index.world_state_description(),
            "An unease stirs in the spirit realm. Something hungers."
        );

        index.corruption_percentage = 0.5;
        assert_eq!(
            index.world_state_description(),
            "The veil thins. Creatures grow restless and unpredictable."
        );

        index.corruption_percentage = 0.7;
        assert_eq!(
            index.world_state_description(),
            "Chaos seeps into the world. Violence begets violence."
        );

        index.corruption_percentage = 0.9;
        assert_eq!(
            index.world_state_description(),
            "The order crumbles. Only the strong survive."
        );

        index.corruption_percentage = 1.0;
        assert_eq!(
            index.world_state_description(),
            "TOTAL COLLAPSE: All souls have shifted to Chaos. There is no peace."
        );
    }

    #[test]
    fn test_corruption_total_collapse_trigger() {
        let mut index = GlobalCorruptionIndex::default();
        assert!(!index.total_collapse_triggered);

        // Consume all spirits
        index.consume_spirits(15_000, true);
        assert_eq!(index.free_spirit_count, 0);
        assert!(index.total_collapse_triggered);
        assert_eq!(index.corruption_percentage, 1.0);
    }

    #[test]
    fn test_corruption_ui_reveal() {
        let mut index = GlobalCorruptionIndex::default();
        assert!(!index.is_visible());

        index.reveal_ui();
        assert!(index.ui_revealed);
        assert!(index.is_visible());
    }

    #[test]
    fn test_corruption_player_contribution() {
        let mut index = GlobalCorruptionIndex::default();

        // No consumption = 0%
        assert_eq!(index.player_corruption_contribution(), 0.0);

        // Vessel consumes 100, player consumes 0 = 0%
        index.consume_spirits(100, true);
        assert_eq!(index.player_corruption_contribution(), 0.0);

        // Player consumes 50, total is now 150 = 33.3%
        index.consume_spirits(50, false);
        assert!((index.player_corruption_contribution() - 0.3333).abs() < 0.01);

        // Player consumes 50 more, total is now 200 = 50%
        index.consume_spirits(50, false);
        assert_eq!(index.player_corruption_contribution(), 0.5);
    }

    #[test]
    fn test_corruption_percentage_calculation() {
        let mut index = GlobalCorruptionIndex::default();

        // Consume 50% of spirits
        index.consume_spirits(7500, true);
        assert!((index.corruption_percentage - 0.5).abs() < 0.01);

        // Consume 25% more (75% total)
        index.consume_spirits(3750, false);
        assert!((index.corruption_percentage - 0.75).abs() < 0.01);

        // Purify some spirits (should reduce corruption)
        // Purified spirits count as 2x toward Order
        for _ in 0..1000 {
            index.purify_spirit();
        }
        assert!(index.corruption_percentage < 0.75);
    }

    // ============================================================================
    // PRIME VESSEL STATE TESTS
    // ============================================================================

    #[test]
    fn test_prime_vessel_state_default() {
        let state = PrimeVesselState::default();
        assert!(!state.has_spawned);
        assert!(state.vessel_entity.is_none());
        assert_eq!(state.lesser_selves.len(), 0);
        assert_eq!(state.total_lesser_selves, 0);
        assert!(!state.vessel_defeated);
        assert!(!state.resurrection_available);
        assert!(state.defeat_day.is_none());
    }

    #[test]
    fn test_prime_vessel_state_spawn_vessel() {
        use bevy::prelude::Entity;

        let mut state = PrimeVesselState::default();
        let entity = Entity::from_raw(1);

        state.spawn_vessel(entity);
        assert!(state.has_spawned);
        assert_eq!(state.vessel_entity, Some(entity));
        assert!(!state.vessel_defeated);
    }

    #[test]
    fn test_prime_vessel_state_add_lesser_self() {
        use bevy::prelude::Entity;

        let mut state = PrimeVesselState::default();

        let lesser1 = Entity::from_raw(10);
        let lesser2 = Entity::from_raw(20);

        state.add_lesser_self(lesser1);
        assert_eq!(state.lesser_selves.len(), 1);
        assert_eq!(state.total_lesser_selves, 1);

        state.add_lesser_self(lesser2);
        assert_eq!(state.lesser_selves.len(), 2);
        assert_eq!(state.total_lesser_selves, 2);
    }

    #[test]
    fn test_prime_vessel_state_defeat_vessel() {
        use bevy::prelude::Entity;

        let mut state = PrimeVesselState::default();
        let entity = Entity::from_raw(1);

        state.spawn_vessel(entity);
        state.defeat_vessel(100);

        assert!(state.vessel_defeated);
        assert!(state.vessel_entity.is_none());
        assert_eq!(state.defeat_day, Some(100));
        assert!(!state.resurrection_available);
    }

    #[test]
    fn test_prime_vessel_state_enable_resurrection() {
        let mut state = PrimeVesselState::default();

        // Can't enable if not defeated
        state.enable_resurrection();
        assert!(!state.resurrection_available);

        // Defeat first
        state.defeat_vessel(100);
        state.enable_resurrection();
        assert!(state.resurrection_available);
    }

    #[test]
    fn test_prime_vessel_state_resurrect_vessel() {
        use bevy::prelude::Entity;

        let mut state = PrimeVesselState::default();
        let entity1 = Entity::from_raw(1);
        let entity2 = Entity::from_raw(2);

        state.spawn_vessel(entity1);
        state.defeat_vessel(100);
        state.enable_resurrection();

        state.resurrect_vessel(entity2);

        assert_eq!(state.vessel_entity, Some(entity2));
        assert!(!state.vessel_defeated);
        assert!(!state.resurrection_available);
        assert!(state.defeat_day.is_none());
    }

    // ============================================================================
    // SPIRIT SPAWN CONFIG TESTS
    // ============================================================================

    #[test]
    fn test_spirit_spawn_weights_total() {
        let weights = SpiritSpawnWeights::default();
        assert_eq!(weights.total(), 60 + 15 + 10 + 10 + 5);
        assert_eq!(weights.total(), 100);
    }

    #[test]
    fn test_spirit_spawn_weights_custom() {
        let weights = SpiritSpawnWeights {
            neutral: 50,
            ancestral: 20,
            chaos: 15,
            harmony: 10,
            void: 5,
        };
        assert_eq!(weights.total(), 100);
    }

    #[test]
    fn test_spirit_spawn_config_default() {
        let config = SpiritSpawnConfig::default();
        assert_eq!(config.min_spawn_distance, 10.0);
        assert_eq!(config.max_per_biome, 3000);
        assert_eq!(config.neutral_power_range, (5.0, 25.0));
        assert!(!config.initial_population_spawned);
        assert_eq!(config.spawn_weights.total(), 100);
    }

    // ============================================================================
    // VESSEL ROAMING STATE TESTS
    // ============================================================================

    #[test]
    fn test_vessel_roaming_state_equality() {
        assert_eq!(VesselRoamingState::Wandering, VesselRoamingState::Wandering);
        assert_ne!(VesselRoamingState::Wandering, VesselRoamingState::Hunting);
        assert_ne!(VesselRoamingState::Combat, VesselRoamingState::Dormant);
    }

    // ============================================================================
    // VESSEL MUTATION TESTS
    // ============================================================================

    #[test]
    fn test_vessel_mutation_equality() {
        assert_eq!(
            VesselMutation::VenomousStrike,
            VesselMutation::VenomousStrike
        );
        assert_ne!(
            VesselMutation::VenomousStrike,
            VesselMutation::ChitinousArmor
        );
    }

    #[test]
    fn test_vessel_mutation_in_vec() {
        let mutations = vec![
            VesselMutation::VenomousStrike,
            VesselMutation::ChitinousArmor,
            VesselMutation::SpiritSense,
        ];

        assert!(mutations.contains(&VesselMutation::VenomousStrike));
        assert!(mutations.contains(&VesselMutation::ChitinousArmor));
        assert!(!mutations.contains(&VesselMutation::FrenzyAura));
    }
}
