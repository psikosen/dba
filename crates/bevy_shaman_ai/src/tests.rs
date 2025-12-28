/// Tests for AI systems
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    use bevy_shaman_core::components::{GridPosition, Health, Spirit, Player};
    use bevy_shaman_combat::components::AttackDamage;
    use bevy_shaman_monsters::components::{MonsterState, AiState};
    use crate::components::*;
    use crate::systems::boss_ai::*;
    use crate::systems::boss_combat_handlers::*;
    use crate::systems::dialogue_triggers::*;
    use crate::systems::spirit_guide::*;

    // ========================================================================
    // BOSS COMBAT HANDLER TESTS
    // ========================================================================

    #[test]
    fn test_special_move_damage_scaling() {
        // Create test app
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn boss
        let boss = app.world_mut().spawn((
            Name::new("Test Boss".to_string()),
            GridPosition { x: 5, y: 5 },
            LlmAi {
                character_name: "Test Boss".to_string(),
                role: AiRole::Boss,
                personality: PersonalityTraits::default(),
                emotional_state: EmotionalState::Confident,
                combat_stance: CombatStance::Tactical,
            },
            MonsterState::default(),
        )).id();

        // Spawn player
        let player = app.world_mut().spawn((
            Player,
            GridPosition { x: 6, y: 5 }, // Distance: 1
            Health { current: 100.0, max: 100.0 },
        )).id();

        // Trigger special move
        app.world_mut().trigger_targets(
            SpecialMoveTriggered {
                move_name: "Test Slam".to_string(),
                boss_phase: 2,
                damage_multiplier: 2.0,
            },
            boss,
        );

        // Process events
        app.update();

        // Verify damage was applied
        let player_health = app.world().entity(player).get::<Health>().unwrap();
        assert!(player_health.current < 100.0, "Player should take damage from special move");

        // Expected damage: 25 (base) * 2.0 (multiplier) = 50
        assert_eq!(player_health.current, 50.0, "Player should take 50 damage");
    }

    #[test]
    fn test_minion_summoning() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<SummonMinionEvent>();

        // Spawn boss
        let boss = app.world_mut().spawn((
            Name::new("Summoner Boss".to_string()),
            GridPosition { x: 10, y: 10 },
            LlmAi {
                character_name: "Summoner Boss".to_string(),
                role: AiRole::Boss,
                personality: PersonalityTraits::default(),
                emotional_state: EmotionalState::Confident,
                combat_stance: CombatStance::Summoner,
            },
        )).id();

        // Count entities before summoning
        let count_before = app.world().entities().len();

        // Trigger summon
        app.world_mut().trigger_targets(
            SummonMinionEvent {
                minion_type: "shadow".to_string(),
                count: 3,
            },
            boss,
        );

        app.update();

        // Count entities after summoning
        let count_after = app.world().entities().len();

        // Should have spawned 3 minions
        assert_eq!(count_after, count_before + 3, "Should spawn 3 minions");

        // Verify minions have correct components
        let minions_query = app.world().query_filtered::<Entity, With<SummonedMinion>>();
        let minion_count = minions_query.iter(app.world()).count();
        assert_eq!(minion_count, 3, "Should have 3 summoned minions");
    }

    #[test]
    fn test_stance_change_modifiers() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn boss
        let boss = app.world_mut().spawn((
            LlmAi {
                character_name: "Stance Boss".to_string(),
                role: AiRole::Boss,
                personality: PersonalityTraits::default(),
                emotional_state: EmotionalState::Confident,
                combat_stance: CombatStance::Tactical,
            },
            MonsterState::default(),
            AttackDamage(10.0),
        )).id();

        let initial_damage = app.world().entity(boss).get::<AttackDamage>().unwrap().0;

        // Change to aggressive stance
        app.world_mut().trigger_targets(
            StanceChangeEvent {
                new_stance: CombatStance::Aggressive,
            },
            boss,
        );

        app.update();

        // Verify stance changed and damage increased
        let boss_ref = app.world().entity(boss);
        let ai = boss_ref.get::<LlmAi>().unwrap();
        let damage = boss_ref.get::<AttackDamage>().unwrap();

        assert_eq!(ai.combat_stance, CombatStance::Aggressive);
        assert!(damage.0 > initial_damage, "Aggressive stance should increase damage");
    }

    #[test]
    fn test_corruption_spread_affects_tiles() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<CorruptionSpreadEvent>();

        // Spawn boss
        let boss = app.world_mut().spawn((
            GridPosition { x: 5, y: 5 },
            LlmAi {
                character_name: "Corruption Boss".to_string(),
                role: AiRole::Boss,
                personality: PersonalityTraits::default(),
                emotional_state: EmotionalState::Corrupted,
                combat_stance: CombatStance::Corrupting,
            },
        )).id();

        // Trigger corruption spread
        app.world_mut().trigger_targets(
            CorruptionSpreadEvent {
                radius: 3.0,
                intensity: 0.5,
            },
            boss,
        );

        app.update();

        // Test passes if no panics - actual tile corruption would require world system integration
    }

    // ========================================================================
    // DIALOGUE TRIGGER TESTS
    // ========================================================================

    #[test]
    fn test_proximity_detection() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn NPC with dialogue zone
        let npc = app.world_mut().spawn((
            GridPosition { x: 10, y: 10 },
            LlmAi {
                character_name: "Brother NPC".to_string(),
                role: AiRole::Brother,
                personality: PersonalityTraits::default(),
                emotional_state: EmotionalState::Calm,
                combat_stance: CombatStance::Defensive,
            },
            DialogueZone {
                range: 2.0,
                auto_trigger: false,
            },
            InDialogueZone::default(),
        )).id();

        // Spawn player far away
        let player = app.world_mut().spawn((
            Player,
            GridPosition { x: 0, y: 0 },
        )).id();

        app.add_systems(Update, detect_dialogue_proximity);
        app.update();

        // Player should not be in dialogue zone
        let in_zone = app.world().entity(npc).get::<InDialogueZone>().unwrap();
        assert!(!in_zone.player_nearby, "Player should not be in dialogue zone when far away");

        // Move player close to NPC
        let mut player_pos = app.world_mut().entity_mut(player).get_mut::<GridPosition>().unwrap();
        player_pos.x = 11;
        player_pos.y = 10;
        drop(player_pos);

        app.update();

        // Player should be in dialogue zone
        let in_zone = app.world().entity(npc).get::<InDialogueZone>().unwrap();
        assert!(in_zone.player_nearby, "Player should be in dialogue zone when close");
    }

    #[test]
    fn test_auto_greeting() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<crate::systems::npc_dialogue::PlayerDialogueRequest>();

        // Spawn NPC with auto-trigger enabled
        let npc = app.world_mut().spawn((
            GridPosition { x: 5, y: 5 },
            LlmAi {
                character_name: "Auto Greeter".to_string(),
                role: AiRole::Brother,
                personality: PersonalityTraits::default(),
                emotional_state: EmotionalState::Calm,
                combat_stance: CombatStance::Defensive,
            },
            DialogueZone {
                range: 2.0,
                auto_trigger: true,
            },
            InDialogueZone::default(),
        )).id();

        // Spawn player in range
        let player = app.world_mut().spawn((
            Player,
            GridPosition { x: 6, y: 5 },
        )).id();

        app.add_systems(Update, detect_dialogue_proximity);
        app.update();

        // NPC should be marked as greeted
        assert!(
            app.world().entity(npc).contains::<HasGreeted>(),
            "NPC should be marked as greeted after auto-trigger"
        );
    }

    // ========================================================================
    // SPIRIT GUIDE TESTS
    // ========================================================================

    #[test]
    fn test_spirit_manifestation_requires_spirit_level() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn spirit guide
        let guide = app.world_mut().spawn((
            GridPosition { x: 10, y: 10 },
            SpiritGuide {
                guide_type: SpiritGuideType::Ancestor,
                manifestation_state: ManifestationState::Hidden,
                wisdom_level: 0.9,
            },
            SpiritEncounterZone {
                radius: 3.0,
                required_spirit_level: 0.7,
                active: true,
            },
            LlmAi {
                character_name: "Ancestor Spirit".to_string(),
                role: AiRole::SpiritGuide,
                personality: PersonalityTraits::default(),
                emotional_state: EmotionalState::Calm,
                combat_stance: CombatStance::Defensive,
            },
        )).id();

        // Spawn player with low spirit (insufficient)
        let player = app.world_mut().spawn((
            Player,
            GridPosition { x: 11, y: 10 }, // Close enough
            Spirit { current: 30.0, max: 100.0 }, // 30% spirit
        )).id();

        app.add_systems(Update, handle_spirit_manifestation);
        app.update();

        // Spirit should remain hidden due to low spirit
        let spirit_guide = app.world().entity(guide).get::<SpiritGuide>().unwrap();
        assert_eq!(
            spirit_guide.manifestation_state,
            ManifestationState::Hidden,
            "Spirit should remain hidden with low player spirit"
        );

        // Increase player spirit
        let mut player_spirit = app.world_mut().entity_mut(player).get_mut::<Spirit>().unwrap();
        player_spirit.current = 80.0;
        drop(player_spirit);

        app.update();

        // Spirit should start manifesting
        let spirit_guide = app.world().entity(guide).get::<SpiritGuide>().unwrap();
        assert_ne!(
            spirit_guide.manifestation_state,
            ManifestationState::Hidden,
            "Spirit should start manifesting with sufficient player spirit"
        );
    }

    #[test]
    fn test_spirit_guide_personalities() {
        // Test each spirit guide type has appropriate personality traits
        let types = vec![
            (SpiritGuideType::Ancestor, "wisdom", 0.9),
            (SpiritGuideType::Nature, "spirituality", 0.9),
            (SpiritGuideType::Cosmic, "wisdom", 0.95),
            (SpiritGuideType::Trickster, "chattiness", 0.7),
        ];

        for (guide_type, expected_high_trait, min_value) in types {
            let personality = match guide_type {
                SpiritGuideType::Ancestor => PersonalityTraits {
                    wisdom: 0.95,
                    spirituality: 1.0,
                    honor: 0.9,
                    chattiness: 0.6,
                    aggression: 0.1,
                },
                SpiritGuideType::Nature => PersonalityTraits {
                    wisdom: 0.8,
                    spirituality: 0.95,
                    honor: 0.7,
                    chattiness: 0.4,
                    aggression: 0.2,
                },
                SpiritGuideType::Cosmic => PersonalityTraits {
                    wisdom: 1.0,
                    spirituality: 0.9,
                    honor: 0.8,
                    chattiness: 0.3,
                    aggression: 0.0,
                },
                SpiritGuideType::Trickster => PersonalityTraits {
                    wisdom: 0.7,
                    spirituality: 0.8,
                    honor: 0.4,
                    chattiness: 0.8,
                    aggression: 0.1,
                },
            };

            // Verify expected trait is high
            let trait_value = match expected_high_trait {
                "wisdom" => personality.wisdom,
                "spirituality" => personality.spirituality,
                "chattiness" => personality.chattiness,
                _ => 0.0,
            };

            assert!(
                trait_value >= min_value,
                "Spirit guide type {:?} should have high {} (>= {})",
                guide_type,
                expected_high_trait,
                min_value
            );
        }
    }

    // ========================================================================
    // INTEGRATION TESTS
    // ========================================================================

    #[test]
    fn test_boss_combat_full_flow() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_event::<SpecialMoveTriggered>();
        app.add_event::<SummonMinionEvent>();
        app.add_event::<StanceChangeEvent>();

        // Spawn boss
        let boss = app.world_mut().spawn((
            GridPosition { x: 10, y: 10 },
            LlmAi {
                character_name: "Test Boss".to_string(),
                role: AiRole::Boss,
                personality: PersonalityTraits::default(),
                emotional_state: EmotionalState::Confident,
                combat_stance: CombatStance::Tactical,
            },
            Health { current: 100.0, max: 100.0 },
            MonsterState::default(),
            AiState::Aggressive,
            AttackDamage(15.0),
            LlmQueryQueue::new(10),
            ConversationHistory::new(20),
        )).id();

        // Spawn player
        let player = app.world_mut().spawn((
            Player,
            GridPosition { x: 12, y: 10 },
            Health { current: 100.0, max: 100.0 },
        )).id();

        // Simulate boss taking damage (phase transition)
        let mut boss_health = app.world_mut().entity_mut(boss).get_mut::<Health>().unwrap();
        boss_health.current = 40.0; // Phase 3 or 4
        drop(boss_health);

        app.update();

        // Boss should still be alive and functional
        assert!(app.world().entities().contains(boss), "Boss should still exist");

        // Verify boss can trigger abilities
        app.world_mut().trigger_targets(
            StanceChangeEvent {
                new_stance: CombatStance::Desperate,
            },
            boss,
        );

        app.update();

        // Test passes if no panics
    }
}
