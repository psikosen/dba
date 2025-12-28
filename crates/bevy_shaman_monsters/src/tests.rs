#[cfg(test)]
mod monster_tests {
    use super::super::components::*;

    // ============================================================================
    // MONSTER STATE TESTS
    // ============================================================================

    #[test]
    fn test_monster_state_default() {
        let state = MonsterState::default();
        assert_eq!(state.state, StateType::Stable);
        assert_eq!(state.stability_meter, 0.8);
        assert_eq!(state.corruption_meter, 0.0);
        assert_eq!(state.obedience_meter, 0.5);
        assert_eq!(state.chaos_output, 1.0);
    }

    #[test]
    fn test_monster_state_evaluate_stable() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.5,
            corruption_meter: 0.3,
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        assert_eq!(state.evaluate_state(), StateType::Stable);
    }

    #[test]
    fn test_monster_state_evaluate_corrupt() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.5,
            corruption_meter: 0.8, // > 0.7
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        assert_eq!(state.evaluate_state(), StateType::Corrupt);
    }

    #[test]
    fn test_monster_state_evaluate_chaos() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.2, // < 0.3
            corruption_meter: 0.3,
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        assert_eq!(state.evaluate_state(), StateType::Chaos);
    }

    #[test]
    fn test_monster_state_evaluate_harmony() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.9,   // > 0.85
            corruption_meter: 0.05, // < 0.1
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        assert_eq!(state.evaluate_state(), StateType::Harmony);
    }

    #[test]
    fn test_monster_state_corruption_takes_priority() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.1,  // Would be chaos
            corruption_meter: 0.8, // But corruption takes priority
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        assert_eq!(state.evaluate_state(), StateType::Corrupt);
    }

    #[test]
    fn test_monster_state_chaos_takes_priority_over_harmony() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.2,   // < 0.3 = chaos
            corruption_meter: 0.05, // < 0.1 but chaos takes priority
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        assert_eq!(state.evaluate_state(), StateType::Chaos);
    }

    #[test]
    fn test_monster_state_is_controllable_normal() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.5,
            corruption_meter: 0.3,
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        assert!(state.is_controllable());
    }

    #[test]
    fn test_monster_state_not_controllable_low_obedience() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.5,
            corruption_meter: 0.3,
            obedience_meter: 0.2, // < 0.3
            chaos_output: 1.0,
        };
        assert!(!state.is_controllable());
    }

    #[test]
    fn test_monster_state_not_controllable_high_corruption() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.5,
            corruption_meter: 0.9, // > 0.8
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        assert!(!state.is_controllable());
    }

    #[test]
    fn test_monster_state_controllable_boundary_obedience() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.5,
            corruption_meter: 0.3,
            obedience_meter: 0.31, // Just above 0.3
            chaos_output: 1.0,
        };
        assert!(state.is_controllable());
    }

    #[test]
    fn test_monster_state_controllable_boundary_corruption() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.5,
            corruption_meter: 0.79, // Just below 0.8
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        assert!(state.is_controllable());
    }

    // ============================================================================
    // STATE TYPE TESTS
    // ============================================================================

    #[test]
    fn test_all_state_types_exist() {
        let states = vec![
            StateType::Stable,
            StateType::Chaos,
            StateType::Corrupt,
            StateType::Harmony,
            StateType::Decay,
            StateType::Rage,
            StateType::Void,
            StateType::Ancestral,
        ];

        // Ensure all states are unique
        for (i, state1) in states.iter().enumerate() {
            for (j, state2) in states.iter().enumerate() {
                if i == j {
                    assert_eq!(state1, state2);
                } else {
                    assert_ne!(state1, state2);
                }
            }
        }
    }

    // ============================================================================
    // MUSIC AFFINITY PROFILE TESTS
    // ============================================================================

    #[test]
    fn test_music_affinity_default() {
        let profile = MusicAffinityProfile::default();
        assert_eq!(profile.prefers_calm, 0.5);
        assert_eq!(profile.prefers_aggressive, 0.5);
        assert_eq!(profile.corruption_resistance, 0.5);
        assert_eq!(profile.trust_level, 0.0);
    }

    #[test]
    fn test_music_affinity_custom() {
        let profile = MusicAffinityProfile {
            prefers_calm: 0.8,
            prefers_aggressive: 0.2,
            corruption_resistance: 0.9,
            trust_level: 0.5,
        };
        assert_eq!(profile.prefers_calm, 0.8);
        assert_eq!(profile.prefers_aggressive, 0.2);
        assert_eq!(profile.corruption_resistance, 0.9);
        assert_eq!(profile.trust_level, 0.5);
    }

    // ============================================================================
    // MONSTER STATS TESTS
    // ============================================================================

    #[test]
    fn test_monster_stats_default() {
        let stats = MonsterStats::default();
        assert_eq!(stats.attack, 10.0);
        assert_eq!(stats.defense, 5.0);
        assert_eq!(stats.speed, 5.0);
        assert_eq!(stats.spirit_affinity, 0.5);
    }

    #[test]
    fn test_monster_stats_custom() {
        let stats = MonsterStats {
            attack: 20.0,
            defense: 15.0,
            speed: 8.0,
            spirit_affinity: 0.8,
        };
        assert_eq!(stats.attack, 20.0);
        assert_eq!(stats.defense, 15.0);
        assert_eq!(stats.speed, 8.0);
        assert_eq!(stats.spirit_affinity, 0.8);
    }

    // ============================================================================
    // MONSTER ID TESTS
    // ============================================================================

    #[test]
    fn test_monster_id_equality() {
        let id1 = MonsterId("goblin".to_string());
        let id2 = MonsterId("goblin".to_string());
        let id3 = MonsterId("orc".to_string());

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    // ============================================================================
    // CORRUPTION INFLUENCE TESTS
    // ============================================================================

    #[test]
    fn test_corruption_influence() {
        let influence = CorruptionInfluence {
            radius: 5.0,
            strength: 0.5,
        };
        assert_eq!(influence.radius, 5.0);
        assert_eq!(influence.strength, 0.5);
    }

    #[test]
    fn test_corruption_exposure_default() {
        let exposure = CorruptionExposure::default();
        assert_eq!(exposure.accumulated, 0.0);
        assert_eq!(exposure.sources.len(), 0);
    }

    // ============================================================================
    // AI BEHAVIOR TESTS
    // ============================================================================

    #[test]
    fn test_ai_behavior_default() {
        let ai = AiBehavior::default();
        assert_eq!(ai.behavior_tree_id, "default");
        assert_eq!(ai.aggression, 0.5);
        assert_eq!(ai.flee_threshold, 0.2);
    }

    #[test]
    fn test_ai_behavior_custom() {
        let ai = AiBehavior {
            behavior_tree_id: "boss".to_string(),
            aggression: 0.9,
            flee_threshold: 0.1,
        };
        assert_eq!(ai.behavior_tree_id, "boss");
        assert_eq!(ai.aggression, 0.9);
        assert_eq!(ai.flee_threshold, 0.1);
    }

    // ============================================================================
    // AI STATE TESTS
    // ============================================================================

    #[test]
    fn test_ai_state_default() {
        let state = AiState::default();
        assert_eq!(state, AiState::Idle);
    }

    #[test]
    fn test_all_ai_states_exist() {
        let states = vec![
            AiState::Idle,
            AiState::Patrol,
            AiState::Aggressive,
            AiState::Fleeing,
            AiState::Stunned,
        ];

        for (i, state1) in states.iter().enumerate() {
            for (j, state2) in states.iter().enumerate() {
                if i == j {
                    assert_eq!(state1, state2);
                } else {
                    assert_ne!(state1, state2);
                }
            }
        }
    }

    // ============================================================================
    // EDGE CASE TESTS - Finding Bugs!
    // ============================================================================

    #[test]
    fn test_monster_state_boundary_corruption_threshold() {
        // Test exactly at corruption threshold
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.5,
            corruption_meter: 0.7, // Exactly at threshold
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        // Should be Stable because condition is > 0.7, not >= 0.7
        assert_eq!(state.evaluate_state(), StateType::Stable);
    }

    #[test]
    fn test_monster_state_boundary_chaos_threshold() {
        // Test exactly at chaos threshold
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.3, // Exactly at threshold
            corruption_meter: 0.0,
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        // Should be Stable because condition is < 0.3, not <= 0.3
        assert_eq!(state.evaluate_state(), StateType::Stable);
    }

    #[test]
    fn test_monster_state_boundary_harmony_stability() {
        // Test exactly at harmony stability threshold
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.85, // Exactly at threshold
            corruption_meter: 0.0,
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        // Should be Stable because condition is > 0.85, not >= 0.85
        assert_eq!(state.evaluate_state(), StateType::Stable);
    }

    #[test]
    fn test_monster_state_boundary_harmony_corruption() {
        // Test exactly at harmony corruption threshold
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.9,
            corruption_meter: 0.1, // Exactly at threshold
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        // Should be Stable because condition is < 0.1, not <= 0.1
        assert_eq!(state.evaluate_state(), StateType::Stable);
    }

    #[test]
    fn test_controllable_boundary_cases() {
        // Test exactly at obedience boundary
        let state1 = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.5,
            corruption_meter: 0.5,
            obedience_meter: 0.3, // Exactly at threshold
            chaos_output: 1.0,
        };
        assert!(!state1.is_controllable()); // > 0.3, not >= 0.3

        // Test exactly at corruption boundary
        let state2 = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.5,
            corruption_meter: 0.8, // Exactly at threshold
            obedience_meter: 0.5,
            chaos_output: 1.0,
        };
        assert!(!state2.is_controllable()); // < 0.8, not <= 0.8
    }

    #[test]
    fn test_monster_meters_out_of_range() {
        // Test with meters outside normal 0-1 range (shouldn't crash)
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 1.5,   // > 1.0
            corruption_meter: -0.5, // < 0.0
            obedience_meter: 2.0,
            chaos_output: 1.0,
        };

        // Should still evaluate without crashing
        let _ = state.evaluate_state();
        let _ = state.is_controllable();
    }

    #[test]
    fn test_monster_zero_values() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 0.0,
            corruption_meter: 0.0,
            obedience_meter: 0.0,
            chaos_output: 0.0,
        };

        assert_eq!(state.evaluate_state(), StateType::Chaos);
        assert!(!state.is_controllable());
    }

    #[test]
    fn test_monster_max_values() {
        let state = MonsterState {
            state: StateType::Stable,
            stability_meter: 1.0,
            corruption_meter: 1.0, // Max corruption
            obedience_meter: 1.0,
            chaos_output: 1.0,
        };

        assert_eq!(state.evaluate_state(), StateType::Corrupt);
        assert!(!state.is_controllable()); // High corruption blocks control
    }
}
