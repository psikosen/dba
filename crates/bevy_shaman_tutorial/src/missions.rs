use crate::{TutorialCondition, TutorialMission, TutorialStep, UiHighlightZone};

/// Dream Cutscene Mission - Opening sequence
/// Shows grotesque ball with teeth, claws trying to pull MC into forest
pub fn create_dream_cutscene_mission() -> TutorialMission {
    TutorialMission {
        id: "dream_cutscene".to_string(),
        title: "The Nightmare".to_string(),
        description: "A strange dream haunts you...".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Experience the nightmare".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForCutscene("dream_grotesque_ball".to_string()),
                dialogue: Some("You see a grotesque ball with teeth, gnawing at you, chomping pieces off...".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Witness the claws".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForCutscene("dream_claws".to_string()),
                dialogue: Some("Irregularly long hands with sharp claws reach from the darkness, pulling you toward the forest...".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Awaken".to_string(),
                hint: Some("Press any key to wake up".to_string()),
                condition: TutorialCondition::Custom("dream_awakened".to_string()),
                dialogue: Some("You jolt awake, heart racing. It felt so real...".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec![],
        reward_xp: 0,
    }
}

/// Exam Mission - Defeat lesser demon with spirit control
/// Teaches spirit control mechanic and mental resistance
pub fn create_exam_mission() -> TutorialMission {
    TutorialMission {
        id: "shaman_exam".to_string(),
        title: "The Shaman Exam".to_string(),
        description: "Prove you can resist mental corruption while fighting a lesser demon.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Face the Lesser Demon".to_string(),
                hint: Some("The demon will try to invade your mind. Stay focused.".to_string()),
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("The shaman places a spirit gem against your skin. 'Face the demon within. Control your spirit, or go mad.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Land 3 attacks on the Lesser Demon".to_string(),
                hint: Some("Use rhythm attacks - time your button presses with the beat indicator.".to_string()),
                condition: TutorialCondition::TriggerRhythmAttack(3),
                dialogue: None,
                ui_highlight: Some(UiHighlightZone::RhythmIndicator),
            },
            TutorialStep {
                step_id: 2,
                objective: "Achieve a 3-hit combo".to_string(),
                hint: Some("Chain attacks together! Light → Light → Heavy for a finishing move.".to_string()),
                condition: TutorialCondition::LandCombo(3),
                dialogue: None,
                ui_highlight: Some(UiHighlightZone::ComboDisplay),
            },
            TutorialStep {
                step_id: 3,
                objective: "Defeat the Lesser Demon".to_string(),
                hint: Some("Keep attacking with rhythm! Don't let the mental visions distract you.".to_string()),
                condition: TutorialCondition::DefeatMonster(1),
                dialogue: None,
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Maintain Spirit Control".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("The shaman pulls the gem away from your skin. The demon's influence fades instantly. 'You resisted... but not perfectly. You need more training.'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["dream_awakened".to_string()],
        reward_xp: 100,
    }
}

/// Purification Mission - Cleanse spoiled ground
/// Teaches purification mechanic and introduces horrifying spirit creatures
pub fn create_purification_mission() -> TutorialMission {
    TutorialMission {
        id: "purification_training".to_string(),
        title: "Purification Training".to_string(),
        description: "Learn to cleanse corrupted lands of horrifying spirits.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Approach the spoiled ground".to_string(),
                hint: Some("Corrupted tiles glow with a sickly aura.".to_string()),
                condition: TutorialCondition::ReachPosition(15, 15),
                dialogue: Some("The shaman leads you to a patch of spoiled ground. 'Do you see them? The eyes, the mouths, the writhing fingers? Not everyone can.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Learn about Aura".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'Aura protects humanity from corruption. Your aura is strong - you can see the horrors clearly. Use that sight to purify them.'".to_string()),
                ui_highlight: Some(UiHighlightZone::SpiritBar),
            },
            TutorialStep {
                step_id: 2,
                objective: "Purify 5 corrupted tiles".to_string(),
                hint: Some("Stand on corrupted tiles and channel your spirit energy to cleanse them.".to_string()),
                condition: TutorialCondition::PurifyTiles(5),
                dialogue: None,
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Defeat corrupted spirit creatures".to_string(),
                hint: Some("Some spirits manifest physically - only eyes, worms with eyes, fingers with holes...".to_string()),
                condition: TutorialCondition::DefeatMonster(3),
                dialogue: None,
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Complete purification".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("purification_complete".to_string()),
                dialogue: Some("The ground returns to normal. The horrifying creatures fade. 'Excellent work. You have talent for purification - from demons to aura.'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["exam_failed_but_trained".to_string()],
        reward_xp: 150,
    }
}

/// Spirit Choice Mission - Choose your starting spirit alignment
/// This determines the final boss and story variations
pub fn create_spirit_choice_mission() -> TutorialMission {
    TutorialMission {
        id: "spirit_choice".to_string(),
        title: "The Four Spirits".to_string(),
        description: "Four powerful spirits battle. Choose which one to aid.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Witness the spirit battle".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForCutscene("four_spirits_battle".to_string()),
                dialogue: Some("Four spirits clash in a spectacular display of power. Each radiates a different energy...".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Meet the Angelic Spirit".to_string(),
                hint: Some("The Angelic Spirit wishes to rebuild the world by cleansing the original.".to_string()),
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'Aid me, child. Together we shall purify this world and rebuild it in perfect harmony.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Meet the Neutral Spirit".to_string(),
                hint: Some("The Neutral Spirit can see the future and wants control to prevent suffering.".to_string()),
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'I see all futures. Humanity brings suffering. Let me control all, remove free will, and prevent the darkness I foresee.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Meet the Chaotic Spirit".to_string(),
                hint: Some("The Chaotic Spirit wants to empty the world of all life.".to_string()),
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'All life brings pain. Let it all rot away. Emptiness is peace.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Meet the Dark Spirit".to_string(),
                hint: Some("The Dark Spirit wants to maintain the world but gain power from evil deeds.".to_string()),
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'The world is fine as it is. Let me feed on humanity's darkness and grow strong.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 5,
                objective: "Make your choice".to_string(),
                hint: Some("This choice will determine your starting spirit and the final boss.".to_string()),
                condition: TutorialCondition::Custom("spirit_chosen".to_string()),
                dialogue: Some("Each spirit looks at you expectantly. Who will you aid?".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec![],
        reward_xp: 50,
    }
}

/// Basic Combat Mission - Learn combat fundamentals
/// Teaches movement, attacking, and spirit companion system
pub fn create_basic_combat_mission() -> TutorialMission {
    TutorialMission {
        id: "basic_combat".to_string(),
        title: "Combat Basics".to_string(),
        description: "Learn the fundamentals of combat with your spirit-imbued spear.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Move around the training area".to_string(),
                hint: Some("Use WASD or Arrow Keys to move.".to_string()),
                condition: TutorialCondition::ReachPosition(12, 12),
                dialogue: Some("'First, learn to move. Your feet carry you through this world.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Your auto-spirit attacks for you".to_string(),
                hint: Some("Notice your spirit companion automatically attacks nearby enemies.".to_string()),
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'Your spirit companion will fight alongside you. Watch as it strikes.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Use your spirit-imbued spear".to_string(),
                hint: Some("Press Space to attack with your spear.".to_string()),
                condition: TutorialCondition::TriggerRhythmAttack(1),
                dialogue: Some("'The spear is imbued with spiritual energy. Strike!'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Perform an AOE spin attack".to_string(),
                hint: Some("Press E to unleash a spinning attack that hits all nearby enemies.".to_string()),
                condition: TutorialCondition::Custom("aoe_spin_used".to_string()),
                dialogue: None,
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Defeat 3 training minions".to_string(),
                hint: Some("Combine your attacks to defeat the minions.".to_string()),
                condition: TutorialCondition::DefeatMonster(3),
                dialogue: None,
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 5,
                objective: "Training complete".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'Well done. You've mastered the basics. Now, let's see how you handle real combat.'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["spirit_chosen".to_string()],
        reward_xp: 100,
    }
}

/// Story Integration Mission - Village Corruption Discovery
/// This triggers after the spirit choice and leads into Act 1
pub fn create_village_corruption_mission() -> TutorialMission {
    TutorialMission {
        id: "village_corruption".to_string(),
        title: "The Affliction".to_string(),
        description: "Discover the curse afflicting your village.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Wake up in your bed".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("awakened_in_village".to_string()),
                dialogue: Some("You awaken in your bed. Something feels wrong...".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Check on your parents".to_string(),
                hint: Some("Your parents are covered in a black aura...".to_string()),
                condition: TutorialCondition::ReachPosition(8, 10),
                dialogue: Some("Your mother and father lie in their bed, covered in a sickly black aura. They don't respond to your voice.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Run to the shaman doctor".to_string(),
                hint: Some("The shaman might know what's happening.".to_string()),
                condition: TutorialCondition::InteractWithNpc("shaman_doctor".to_string()),
                dialogue: None,
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Learn about the village affliction".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("The shaman looks grave. 'It's not just your parents. 70% of the village is afflicted. My vessel is too weak to purify them all. We need a new shaman...'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Decide to become a shaman".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("decided_to_become_shaman".to_string()),
                dialogue: Some("'I'll do it. I'll become a shaman and save everyone. It's my fault for helping that spirit win...'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["basic_combat_complete".to_string()],
        reward_xp: 200,
    }
}

/// Land Grant Mission - Receive your shaman land
/// Teaches building and farming mechanics
pub fn create_land_grant_mission() -> TutorialMission {
    TutorialMission {
        id: "land_grant".to_string(),
        title: "Your Shaman Land".to_string(),
        description: "Receive land to grow herbs and build spiritual structures.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Meet the shaman priest".to_string(),
                hint: None,
                condition: TutorialCondition::InteractWithNpc("shaman_priest".to_string()),
                dialogue: Some("'Welcome, apprentice. We've granted you land to cultivate herbs and build spiritual vessels.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Visit your land".to_string(),
                hint: Some("Your land is located to the west of the village.".to_string()),
                condition: TutorialCondition::ReachPosition(5, 20),
                dialogue: Some("This is your plot. Here you can grow herbs, store spirits, and mix potions.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Plant your first herb seeds".to_string(),
                hint: Some("Use the Build menu to plant seeds.".to_string()),
                condition: TutorialCondition::Custom("planted_seeds".to_string()),
                dialogue: None,
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Build a spirit vessel storage".to_string(),
                hint: Some("Spirit vessels store the spirits you catch for later use.".to_string()),
                condition: TutorialCondition::Custom("built_spirit_storage".to_string()),
                dialogue: None,
                ui_highlight: None,
            },
        ],
        required_flags: vec!["decided_to_become_shaman".to_string()],
        reward_xp: 150,
    }
}
