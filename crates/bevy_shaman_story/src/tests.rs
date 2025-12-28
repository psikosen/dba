#[cfg(test)]
mod story_tests {
    use super::super::components::*;
    use super::super::resources::*;

    // ============================================================================
    // COMPONENT TESTS - NPC Sickness
    // ============================================================================

    #[test]
    fn test_npc_sickness_state_default() {
        let state = NpcSicknessState::default();
        assert_eq!(state, NpcSicknessState::AsleepSick);
    }

    #[test]
    fn test_npc_sickness_state_variants() {
        let asleep = NpcSicknessState::AsleepSick;
        let waking = NpcSicknessState::Waking;
        let awake = NpcSicknessState::Awake;

        assert_ne!(asleep, waking);
        assert_ne!(waking, awake);
        assert_ne!(asleep, awake);
    }

    #[test]
    fn test_npc_dialogue_default() {
        let dialogue = NpcDialogue::default();
        assert_eq!(dialogue.full_dialogue, "");
        assert!(dialogue.partial_dialogue.is_none());
        assert_eq!(dialogue.sick_dialogue, "... ... ...");
    }

    #[test]
    fn test_npc_dialogue_get_sick() {
        let dialogue = NpcDialogue {
            full_dialogue: "Hello traveler!".to_string(),
            partial_dialogue: Some("H...hello...".to_string()),
            sick_dialogue: "...".to_string(),
        };

        assert_eq!(dialogue.get_dialogue(NpcSicknessState::AsleepSick), "...");
    }

    #[test]
    fn test_npc_dialogue_get_waking() {
        let dialogue = NpcDialogue {
            full_dialogue: "Hello traveler!".to_string(),
            partial_dialogue: Some("H...hello...".to_string()),
            sick_dialogue: "...".to_string(),
        };

        assert_eq!(
            dialogue.get_dialogue(NpcSicknessState::Waking),
            "H...hello..."
        );
    }

    #[test]
    fn test_npc_dialogue_get_awake() {
        let dialogue = NpcDialogue {
            full_dialogue: "Hello traveler!".to_string(),
            partial_dialogue: Some("H...hello...".to_string()),
            sick_dialogue: "...".to_string(),
        };

        assert_eq!(
            dialogue.get_dialogue(NpcSicknessState::Awake),
            "Hello traveler!"
        );
    }

    #[test]
    fn test_npc_dialogue_waking_fallback() {
        let dialogue = NpcDialogue {
            full_dialogue: "Hello!".to_string(),
            partial_dialogue: None,
            sick_dialogue: "...".to_string(),
        };

        // Should fall back to full dialogue when partial is None
        assert_eq!(dialogue.get_dialogue(NpcSicknessState::Waking), "Hello!");
    }

    #[test]
    fn test_npc_name_default() {
        let name = NpcName::default();
        assert_eq!(name.name, "Villager");
        assert_eq!(name.current_emotion, PortraitEmotion::Neutral);
    }

    #[test]
    fn test_player_brother_default() {
        let brother = PlayerBrother::default();
        assert_eq!(brother.soul_corruption, 1.0);
        assert_eq!(brother.fights_remaining, 4);
    }

    #[test]
    fn test_quest_creation() {
        let quest = Quest {
            quest_id: "tutorial_quest".to_string(),
            description: "Learn the basics".to_string(),
            completed: false,
        };

        assert_eq!(quest.quest_id, "tutorial_quest");
        assert!(!quest.completed);
    }

    // ============================================================================
    // RESOURCE TESTS - Instrument Choice
    // ============================================================================

    #[test]
    fn test_instrument_choice_default() {
        let choice = InstrumentChoice::default();
        assert_eq!(choice, InstrumentChoice::NotChosen);
    }

    #[test]
    fn test_instrument_choice_kora_modifiers() {
        let kora = InstrumentChoice::Kora;

        assert_eq!(kora.stability_modifier(), 1.3);
        assert_eq!(kora.damage_modifier(), 0.9);
        assert_eq!(kora.stamina_cost_modifier(), 0.8);
        assert_eq!(kora.obedience_modifier(), 1.2);
    }

    #[test]
    fn test_instrument_choice_ngoni_modifiers() {
        let ngoni = InstrumentChoice::Ngoni;

        assert_eq!(ngoni.stability_modifier(), 0.9);
        assert_eq!(ngoni.damage_modifier(), 1.3);
        assert_eq!(ngoni.stamina_cost_modifier(), 1.2);
        assert_eq!(ngoni.obedience_modifier(), 0.8);
    }

    #[test]
    fn test_instrument_choice_not_chosen_modifiers() {
        let not_chosen = InstrumentChoice::NotChosen;

        assert_eq!(not_chosen.stability_modifier(), 1.0);
        assert_eq!(not_chosen.damage_modifier(), 1.0);
        assert_eq!(not_chosen.stamina_cost_modifier(), 1.0);
        assert_eq!(not_chosen.obedience_modifier(), 1.0);
    }

    #[test]
    fn test_instrument_choice_kora_vs_ngoni_damage() {
        let kora = InstrumentChoice::Kora;
        let ngoni = InstrumentChoice::Ngoni;

        // Ngoni should have higher damage
        assert!(ngoni.damage_modifier() > kora.damage_modifier());
    }

    #[test]
    fn test_instrument_choice_kora_vs_ngoni_stability() {
        let kora = InstrumentChoice::Kora;
        let ngoni = InstrumentChoice::Ngoni;

        // Kora should have higher stability
        assert!(kora.stability_modifier() > ngoni.stability_modifier());
    }

    // ============================================================================
    // RESOURCE TESTS - Brother Cleansing Progress
    // ============================================================================

    #[test]
    fn test_brother_cleansing_new() {
        let progress = BrotherCleansingProgress::new();
        assert_eq!(progress.fights_completed, 0);
        assert_eq!(progress.total_fights, 4);
    }

    #[test]
    fn test_brother_cleansing_default() {
        let progress = BrotherCleansingProgress::default();
        assert_eq!(progress.fights_completed, 0);
        // Default uses derive which zeros all fields
        // Use new() to get proper initialization
    }

    #[test]
    fn test_brother_cleansing_complete_fight() {
        let mut progress = BrotherCleansingProgress::new();

        progress.complete_fight();
        assert_eq!(progress.fights_completed, 1);

        progress.complete_fight();
        assert_eq!(progress.fights_completed, 2);
    }

    #[test]
    fn test_brother_cleansing_is_fully_cleansed() {
        let mut progress = BrotherCleansingProgress::new();
        assert!(!progress.is_fully_cleansed());

        progress.fights_completed = 4;
        assert!(progress.is_fully_cleansed());
    }

    #[test]
    fn test_brother_cleansing_max_fights_cap() {
        let mut progress = BrotherCleansingProgress::new();

        // Complete all fights plus extras
        for _ in 0..10 {
            progress.complete_fight();
        }

        // Should cap at total_fights
        assert_eq!(progress.fights_completed, 4);
    }

    #[test]
    fn test_brother_cleansing_corruption_percentage() {
        let mut progress = BrotherCleansingProgress::new();

        assert_eq!(progress.corruption_percentage(), 1.0); // 100% corrupt

        progress.complete_fight();
        assert_eq!(progress.corruption_percentage(), 0.75); // 75% corrupt

        progress.complete_fight();
        assert_eq!(progress.corruption_percentage(), 0.5); // 50% corrupt

        progress.complete_fight();
        assert_eq!(progress.corruption_percentage(), 0.25); // 25% corrupt

        progress.complete_fight();
        assert_eq!(progress.corruption_percentage(), 0.0); // 0% corrupt (cleansed)
    }

    // ============================================================================
    // RESOURCE TESTS - African Names Database
    // ============================================================================

    #[test]
    fn test_african_names_db_default() {
        let db = AfricanNamesDB::default();
        assert!(!db.names.is_empty());
    }

    #[test]
    fn test_african_names_db_main_character() {
        let db = AfricanNamesDB::default();

        let kwame = db.names.get("Kwame");
        assert!(kwame.is_some());

        let (char_type, origin, meaning) = kwame.unwrap();
        assert_eq!(*char_type, CharacterType::MainCharacter);
        assert_eq!(origin, "Akan, Ghana");
        assert_eq!(meaning, "Born on Saturday");
    }

    #[test]
    fn test_african_names_db_brothers() {
        let db = AfricanNamesDB::default();

        let brother_names = vec!["Kofi", "Kwesi", "Kwadwo", "Yaw"];

        for name in brother_names {
            let entry = db.names.get(name);
            assert!(entry.is_some(), "Brother name {} should exist", name);

            let (char_type, _, _) = entry.unwrap();
            assert_eq!(*char_type, CharacterType::Brother);
        }
    }

    #[test]
    fn test_african_names_db_bosses() {
        let db = AfricanNamesDB::default();

        let boss_names = vec!["Anansi", "Mami Wata", "Oya", "Shango"];

        for name in boss_names {
            let entry = db.names.get(name);
            assert!(entry.is_some(), "Boss name {} should exist", name);

            let (char_type, _, _) = entry.unwrap();
            assert_eq!(*char_type, CharacterType::Boss);
        }
    }

    #[test]
    fn test_african_names_db_character_types() {
        let db = AfricanNamesDB::default();

        let mut type_counts = std::collections::HashMap::new();

        for (_name, (char_type, _origin, _meaning)) in &db.names {
            *type_counts.entry(*char_type).or_insert(0) += 1;
        }

        // Verify we have multiple character types
        assert!(type_counts.len() >= 8);

        // Verify we have at least one of each major type
        assert!(type_counts.contains_key(&CharacterType::MainCharacter));
        assert!(type_counts.contains_key(&CharacterType::Brother));
        assert!(type_counts.contains_key(&CharacterType::Boss));
        assert!(type_counts.contains_key(&CharacterType::Merchant));
    }

    // ============================================================================
    // RESOURCE TESTS - Portrait Database
    // ============================================================================

    #[test]
    fn test_portrait_db_default() {
        let db = PortraitDB::default();
        assert_eq!(db.portraits.len(), 0);
    }

    #[test]
    fn test_portrait_db_get_none() {
        let db = PortraitDB::default();
        let portrait = db.get_portrait("Kwame", PortraitEmotion::Happy);
        assert!(portrait.is_none());
    }

    #[test]
    fn test_portrait_emotion_variants() {
        let emotions = vec![
            PortraitEmotion::Neutral,
            PortraitEmotion::Happy,
            PortraitEmotion::Sad,
            PortraitEmotion::Angry,
            PortraitEmotion::Surprised,
            PortraitEmotion::Determined,
            PortraitEmotion::Sick,
            PortraitEmotion::Waking,
            PortraitEmotion::Laughing,
            PortraitEmotion::Threatening,
            PortraitEmotion::Defeated,
            PortraitEmotion::Enraged,
            PortraitEmotion::Worried,
        ];

        assert_eq!(emotions.len(), 13);
    }

    // ============================================================================
    // QA TESTS - Story System Validation
    // ============================================================================

    #[test]
    fn test_qa_brother_cleansing_progression() {
        // Test the 4-fight brother cleansing arc
        let mut progress = BrotherCleansingProgress::new();

        let expected_corruptions = vec![1.0, 0.75, 0.5, 0.25, 0.0];

        for (i, expected) in expected_corruptions.iter().enumerate() {
            assert_eq!(progress.corruption_percentage(), *expected);

            if i < 4 {
                progress.complete_fight();
            }
        }

        assert!(progress.is_fully_cleansed());
    }

    #[test]
    fn test_qa_instrument_choice_tradeoffs() {
        // Verify instrument choices have meaningful tradeoffs

        let kora = InstrumentChoice::Kora;
        let ngoni = InstrumentChoice::Ngoni;

        // Kora is better for stability/control
        assert!(kora.stability_modifier() > 1.0);
        assert!(kora.obedience_modifier() > 1.0);
        assert!(kora.stamina_cost_modifier() < 1.0);

        // But worse for damage
        assert!(kora.damage_modifier() < 1.0);

        // Ngoni is better for damage
        assert!(ngoni.damage_modifier() > 1.0);

        // But worse for stability/control and stamina
        assert!(ngoni.stability_modifier() < 1.0);
        assert!(ngoni.obedience_modifier() < 1.0);
        assert!(ngoni.stamina_cost_modifier() > 1.0);
    }

    #[test]
    fn test_qa_npc_dialogue_progression() {
        // Test that NPCs have proper dialogue progression through sickness states

        let dialogue = NpcDialogue {
            full_dialogue: "Welcome, young shaman! The village needs your help!".to_string(),
            partial_dialogue: Some("W...welcome... h...help...".to_string()),
            sick_dialogue: "... ... ...".to_string(),
        };

        // Sick NPCs should only say dots
        let sick_text = dialogue.get_dialogue(NpcSicknessState::AsleepSick);
        assert!(sick_text.contains("..."));
        assert!(!sick_text.contains("Welcome"));

        // Waking NPCs should have partial dialogue
        let waking_text = dialogue.get_dialogue(NpcSicknessState::Waking);
        assert!(waking_text.contains("welcome"));
        assert!(waking_text.contains("help"));

        // Awake NPCs should have full dialogue
        let awake_text = dialogue.get_dialogue(NpcSicknessState::Awake);
        assert_eq!(
            awake_text,
            "Welcome, young shaman! The village needs your help!"
        );
    }

    #[test]
    fn test_qa_african_names_cultural_diversity() {
        // Verify the name database includes diverse African cultures

        let db = AfricanNamesDB::default();

        let cultures = vec!["Akan", "Igbo", "Swahili", "Yoruba", "Zulu", "Kikuyu"];

        for culture in cultures {
            let has_culture = db
                .names
                .values()
                .any(|(_, origin, _)| origin.contains(culture));
            assert!(has_culture, "Database should include {} names", culture);
        }
    }

    #[test]
    fn test_qa_boss_names_are_meaningful() {
        // Verify boss names have meaningful origins and are mythologically significant

        let db = AfricanNamesDB::default();

        let bosses = vec!["Anansi", "Mami Wata", "Oya", "Shango"];

        for boss in bosses {
            if let Some((char_type, origin, meaning)) = db.names.get(boss) {
                assert_eq!(*char_type, CharacterType::Boss);
                assert!(!origin.is_empty());
                assert!(!meaning.is_empty());

                // Boss meanings should reference their mythological significance
                assert!(
                    meaning.to_lowercase().contains("spirit")
                        || meaning.to_lowercase().contains("god")
                        || meaning.to_lowercase().contains("trickster")
                        || meaning.to_lowercase().contains("water")
                        || meaning.to_lowercase().contains("thunder")
                        || meaning.to_lowercase().contains("storm"),
                    "Boss {} should have mythological significance in meaning: {}",
                    boss,
                    meaning
                );
            }
        }
    }

    #[test]
    fn test_qa_character_type_distribution() {
        // Verify we have a good distribution of character types

        let db = AfricanNamesDB::default();

        let mut type_counts = std::collections::HashMap::new();
        for (_name, (char_type, _, _)) in &db.names {
            *type_counts.entry(*char_type).or_insert(0) += 1;
        }

        // Should have at least 9 bosses (major encounters)
        assert!(*type_counts.get(&CharacterType::Boss).unwrap_or(&0) >= 9);

        // Should have merchants
        assert!(*type_counts.get(&CharacterType::Merchant).unwrap_or(&0) >= 4);

        // Should have villagers
        assert!(*type_counts.get(&CharacterType::Villager).unwrap_or(&0) >= 5);
    }
}
