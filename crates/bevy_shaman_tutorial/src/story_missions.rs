use crate::{TutorialCondition, TutorialMission, TutorialStep, UiHighlightZone};

// =============================================================================
// ACT 1: THE PRELIMINARY SHAMAN
// Conduit Level: 1 → 3
// =============================================================================

/// Act 1 - Meet Rival Shamans
pub fn create_meet_rivals_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_meet_rivals".to_string(),
        title: "The Other Apprentices".to_string(),
        description: "Meet your fellow shaman apprentices. You'll compete, train, and grow together.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Gather at the training grounds".to_string(),
                hint: Some("The head shaman has called all apprentices together.".to_string()),
                condition: TutorialCondition::ReachPosition(25, 25),
                dialogue: Some("Four other apprentices stand before you. The head shaman gestures to each in turn...".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Meet Amara".to_string(),
                hint: Some("Amara specializes in spirit seeds and cultivation.".to_string()),
                condition: TutorialCondition::InteractWithNpc("amara".to_string()),
                dialogue: Some("'I'm Amara. I grow the strongest spirit herbs. Don't think you'll beat me in the exams.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Meet Zuri".to_string(),
                hint: Some("Zuri is a weapons specialist.".to_string()),
                condition: TutorialCondition::InteractWithNpc("zuri".to_string()),
                dialogue: Some("'Zuri. I forge spirit-imbued weapons. You'll need my blades if you want to survive.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Meet Tau".to_string(),
                hint: Some("Tau is skilled at catching spirits.".to_string()),
                condition: TutorialCondition::InteractWithNpc("tau".to_string()),
                dialogue: Some("'Name's Tau. I catch more spirits than anyone. Watch and learn.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Meet Jabari".to_string(),
                hint: Some("Jabari is a prodigy in purification techniques.".to_string()),
                condition: TutorialCondition::InteractWithNpc("jabari".to_string()),
                dialogue: Some("'Jabari. My purification speed is unmatched. Try to keep up.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 5,
                objective: "Listen to the head shaman".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'You five will compete in three trials. Only those who pass will become full shamans. Begin training!'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["decided_to_become_shaman".to_string()],
        reward_xp: 250,
    }
}

/// Act 1 - Purification Race
pub fn create_purification_race_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_purification_race".to_string(),
        title: "The First Trial: Purification Race".to_string(),
        description: "Race your rivals to purify corrupted land. First to finish wins!".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Enter the corrupted dungeon".to_string(),
                hint: Some("A time limit will begin once you enter.".to_string()),
                condition: TutorialCondition::Custom("entered_dungeon_race".to_string()),
                dialogue: Some("'On my mark... Go! First to purify all corruption wins!'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Purify corrupted tiles faster than your rivals".to_string(),
                hint: Some("If they beat you, they'll steal one of your spirits or all your gold!".to_string()),
                condition: TutorialCondition::PurifyTiles(15),
                dialogue: None,
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Defeat any corrupted creatures blocking your path".to_string(),
                hint: Some("Don't let them slow you down!".to_string()),
                condition: TutorialCondition::DefeatMonster(5),
                dialogue: None,
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Complete the purification before time runs out".to_string(),
                hint: Some("You can see your rivals' progress on the UI.".to_string()),
                condition: TutorialCondition::Custom("purification_race_complete".to_string()),
                dialogue: Some("You finish just as the others complete their sections. Jabari finished first, but you came in second!".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["met_all_rivals".to_string()],
        reward_xp: 300,
    }
}

/// Act 1 - Team Battle Trial
pub fn create_team_battle_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_team_battle".to_string(),
        title: "The Second Trial: 2v2 Battle".to_string(),
        description: "You're randomly paired with a rival for a 2v2 battle. Win to pass the exam!".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Draw your partner".to_string(),
                hint: Some("The pairings are random.".to_string()),
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("The head shaman draws lots. 'Your partner is... Zuri!'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Strategize with Zuri".to_string(),
                hint: Some("Talk to her before the battle begins.".to_string()),
                condition: TutorialCondition::InteractWithNpc("zuri".to_string()),
                dialogue: Some("'Alright, here's the plan: You draw their attention, I'll hit hard from behind.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Defeat the opposing team".to_string(),
                hint: Some("Your opponents are Tau and Amara.".to_string()),
                condition: TutorialCondition::Custom("team_battle_won".to_string()),
                dialogue: None,
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Celebration".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'Victory! We make a good team!' Zuri grins and slaps your back. You've passed the second trial!".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["purification_race_complete".to_string()],
        reward_xp: 350,
    }
}

/// Act 1 - Shaman Ceremony
pub fn create_shaman_ceremony_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_shaman_ceremony".to_string(),
        title: "The Shaman Marks Ceremony".to_string(),
        description: "Receive your shaman marks in a sacred ceremony.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Attend the ceremony at nightfall".to_string(),
                hint: Some("The full moon rises over the sacred grove.".to_string()),
                condition: TutorialCondition::WaitForCutscene("shaman_ceremony".to_string()),
                dialogue: Some("The elders surround you, chanting in ancient tongues. The air thrums with spiritual energy.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Receive the shaman marks".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("The head shaman presses glowing ink to your forehead. Searing pain, then... power. You are now a Preliminary Shaman.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Learn about your temporary promotion".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'This is a temporary rank. Most shamans were afflicted by the curse. You must prove yourself worthy of a permanent title.'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["team_battle_won".to_string()],
        reward_xp: 400,
    }
}

/// Act 1 - Spirit Hunt (No Spirit Help)
pub fn create_spirit_hunt_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_spirit_hunt".to_string(),
        title: "The Spirit Hunt".to_string(),
        description: "Hunt a chaotic spirit without the help of your spirit companion.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Enter the Spirit Realm".to_string(),
                hint: Some("Your conduit bar allows you to perceive spirit realm tiles.".to_string()),
                condition: TutorialCondition::Custom("entered_spirit_realm".to_string()),
                dialogue: Some("The world shifts. Colors bleed and twist. You see tiles that exist between worlds - Spirit Realm Tiles.".to_string()),
                ui_highlight: Some(UiHighlightZone::SpiritBar),
            },
            TutorialStep {
                step_id: 1,
                objective: "Track the chaotic spirit".to_string(),
                hint: Some("Follow the spiritual traces on Spirit Realm Tiles.".to_string()),
                condition: TutorialCondition::ReachPosition(40, 40),
                dialogue: Some("You see footprints made of shadow and fear. The spirit was here recently.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Catch the chaotic spirit without your companion".to_string(),
                hint: Some("You must rely on your own power alone.".to_string()),
                condition: TutorialCondition::Custom("caught_chaotic_spirit".to_string()),
                dialogue: Some("The spirit manifests - a writhing mass of eyes and teeth. You steel yourself and begin the capture ritual alone.".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["shaman_ceremony_complete".to_string()],
        reward_xp: 450,
    }
}

/// Act 1 - Imbument Training
pub fn create_imbument_training_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_imbument_training".to_string(),
        title: "The Art of Imbument".to_string(),
        description: "Learn to imbue weapons and armor with captured spirits.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Meet the head shaman at the forge".to_string(),
                hint: None,
                condition: TutorialCondition::InteractWithNpc("head_shaman".to_string()),
                dialogue: Some("'You caught a chaotic spirit alone. Impressive. Now, let me teach you imbument.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Watch the demonstration".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("The shaman holds a spirit vessel to his staff. The spirit flows into the weapon, which glows with otherworldly power.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Imbue your first weapon".to_string(),
                hint: Some("Select a spirit from your collection and merge it with your spear.".to_string()),
                condition: TutorialCondition::Custom("imbued_weapon".to_string()),
                dialogue: Some("The spirit and weapon become one. You feel its power coursing through the spear.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Fight the head shaman".to_string(),
                hint: Some("He's going easy on you... or is he?".to_string()),
                condition: TutorialCondition::Custom("fought_head_shaman".to_string()),
                dialogue: Some("'Come! Show me the power of imbument!' The shaman moves with blinding speed. You're completely outmatched.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Survive 2 minutes".to_string(),
                hint: Some("You don't need to win, just survive!".to_string()),
                condition: TutorialCondition::Custom("survived_shaman_test".to_string()),
                dialogue: Some("'Enough!' The shaman lowers his weapon. 'You lasted longer than expected. The power of imbument is now yours to wield.'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["caught_chaotic_spirit".to_string()],
        reward_xp: 500,
    }
}

/// Act 1 - Meet the Priest
pub fn create_meet_priest_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_meet_priest".to_string(),
        title: "The Shaman Priest's Teachings".to_string(),
        description: "Learn about herbs, blood rituals, and exorcism from the shaman priest.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Visit the shaman priest".to_string(),
                hint: Some("The priest oversees all healing and purification work.".to_string()),
                condition: TutorialCondition::InteractWithNpc("shaman_priest".to_string()),
                dialogue: Some("'Ah, the new shaman. Come, I'll teach you the sacred arts of healing and exorcism.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Learn about spirit herbs".to_string(),
                hint: Some("Certain herbs can strengthen your conduit or purify tainted souls.".to_string()),
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'These herbs grow in spiritually charged soil. Use them wisely - they can save lives or empower your rituals.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Perform a blood ritual".to_string(),
                hint: Some("Blood rituals can boost your temporary power.".to_string()),
                condition: TutorialCondition::Custom("performed_blood_ritual".to_string()),
                dialogue: Some("You prick your finger and let blood fall onto the ritual circle. Power surges through you, but at a cost.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Exorcise a spirit from a beast".to_string(),
                hint: Some("A corrupted dog has been brought to the temple.".to_string()),
                condition: TutorialCondition::Custom("exorcised_beast".to_string()),
                dialogue: Some("You place your hands on the thrashing creature and pull the malevolent spirit free. The dog collapses, breathing normally again.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Exorcise a spirit from a human".to_string(),
                hint: Some("A villager with a tainted soul needs your help.".to_string()),
                condition: TutorialCondition::Custom("exorcised_human".to_string()),
                dialogue: Some("The villager convulses as you draw out the dark spirit. Their aura clears. They open their eyes and whisper 'Thank you...'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["imbued_weapon".to_string()],
        reward_xp: 550,
    }
}

/// Act 1 - Trading Economy Introduction
pub fn create_trading_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_trading".to_string(),
        title: "The Shaman Economy".to_string(),
        description: "Learn to trade with your rivals and the village shop.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Visit the other shamans' lands".to_string(),
                hint: Some("Each rival specializes in different products.".to_string()),
                condition: TutorialCondition::Custom("visited_rival_lands".to_string()),
                dialogue: Some("You walk through the shaman district. Each apprentice has their own plot, filled with their specialty items.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Trade with Amara for spirit seeds".to_string(),
                hint: Some("Amara has the best seeds, but charges high prices.".to_string()),
                condition: TutorialCondition::Custom("traded_with_amara".to_string()),
                dialogue: Some("'These seeds will grow powerful herbs. That'll be 50 gold.' Amara hands you a pouch.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Sell herbs at the village shop".to_string(),
                hint: Some("The shop pays fair prices for quality herbs.".to_string()),
                condition: TutorialCondition::Custom("sold_to_shop".to_string()),
                dialogue: Some("The shopkeeper examines your herbs. 'Fine quality! Here's your payment.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Offer spiritual healing services".to_string(),
                hint: Some("Villagers with unstable spirits will pay for your help.".to_string()),
                condition: TutorialCondition::Custom("healed_paying_customer".to_string()),
                dialogue: Some("A merchant with a minor curse pays you generously for the healing. 'May the spirits bless you, shaman!'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["exorcised_human".to_string()],
        reward_xp: 300,
    }
}

/// Act 1 - Conduit Sensor System
pub fn create_conduit_sensor_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_conduit_sensor".to_string(),
        title: "The Conduit Sensor Warning".to_string(),
        description: "Learn about the conduit sensor that tracks spirit corruption levels.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Notice the conduit sensor alert".to_string(),
                hint: Some("A red indicator flashes on your UI.".to_string()),
                condition: TutorialCondition::Custom("sensor_alert_triggered".to_string()),
                dialogue: Some("A sharp pain in your temples. Your conduit sensor flares - someone in the village is transforming!".to_string()),
                ui_highlight: Some(UiHighlightZone::SpiritBar),
            },
            TutorialStep {
                step_id: 1,
                objective: "Find the transforming villager".to_string(),
                hint: Some("Follow the sensor readings.".to_string()),
                condition: TutorialCondition::ReachPosition(30, 15),
                dialogue: Some("You find a villager convulsing in the street. Their body is half-spirit, half-human!".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Choose: Catch them as a spirit, or cure them?".to_string(),
                hint: Some("Catching gives you a powerful spirit. Curing gives you better rewards.".to_string()),
                condition: TutorialCondition::Custom("transformation_resolved".to_string()),
                dialogue: Some("The choice is yours. What will you do?".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Learn about escalation".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("The priest warns you: 'Each dungeon you clear releases spirits. 11% more villagers will be at risk. You must balance progress with protection.'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["traded_with_amara".to_string()],
        reward_xp: 400,
    }
}

/// Act 1 - First Dungeon Boss
pub fn create_first_boss_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_first_boss".to_string(),
        title: "The Corrupted Guardian".to_string(),
        description: "Clear your first dungeon and defeat the boss!".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Enter the first corrupted dungeon".to_string(),
                hint: Some("This dungeon contains 20 corrupted tiles and a boss.".to_string()),
                condition: TutorialCondition::Custom("entered_first_dungeon".to_string()),
                dialogue: Some("The dungeon entrance yawns before you like a wound in reality. Corruption seeps from within.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Purify the dungeon".to_string(),
                hint: Some("Clear all corrupted tiles before facing the boss.".to_string()),
                condition: TutorialCondition::PurifyTiles(20),
                dialogue: None,
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Defeat the Corrupted Guardian".to_string(),
                hint: Some("Use imbued weapons and your spirit companion!".to_string()),
                condition: TutorialCondition::Custom("defeated_first_boss".to_string()),
                dialogue: Some("The guardian roars, a massive beast of writhing shadows and bone. This is your greatest challenge yet!".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Victory!".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("The guardian dissolves into mist. The dungeon clears. You've done it!".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["sensor_alert_triggered".to_string()],
        reward_xp: 1000,
    }
}

/// Act 1 - Religious Festival
pub fn create_religious_festival_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_religious_festival".to_string(),
        title: "The Religious Festival Returns".to_string(),
        description: "The first festival since the corruption began. Celebrate and honor the ancestors!".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Attend the opening ceremony".to_string(),
                hint: Some("The entire village gathers at the sacred grove.".to_string()),
                condition: TutorialCondition::WaitForCutscene("religious_festival_opening".to_string()),
                dialogue: Some("Drums echo across the valley. The elders light sacred fires. The festival begins!".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Make offerings to ancestral spirits".to_string(),
                hint: Some("Offerings can grant temporary blessings.".to_string()),
                condition: TutorialCondition::Custom("made_offering".to_string()),
                dialogue: Some("You place herbs and food at the shrine. A warm presence washes over you - the ancestors are pleased.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Participate in ritual dances".to_string(),
                hint: Some("Rhythm minigame!".to_string()),
                condition: TutorialCondition::Custom("completed_ritual_dance".to_string()),
                dialogue: Some("You move with the crowd, bodies swaying in ancient patterns. The spirits dance with you.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Socialize with your rivals".to_string(),
                hint: Some("Talk to Amara, Zuri, Tau, and Jabari.".to_string()),
                condition: TutorialCondition::Custom("socialized_at_festival".to_string()),
                dialogue: Some("For one night, you're not competitors. You're just young people celebrating life together.".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["defeated_first_boss".to_string()],
        reward_xp: 500,
    }
}

/// Act 1 - The King's Summons (Act 1 Finale)
pub fn create_kings_summons_mission() -> TutorialMission {
    TutorialMission {
        id: "act1_kings_summons".to_string(),
        title: "Summons to the Palace".to_string(),
        description: "The king wishes to speak with you about the coming war.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Receive the royal messenger".to_string(),
                hint: None,
                condition: TutorialCondition::InteractWithNpc("royal_messenger".to_string()),
                dialogue: Some("'His Majesty King Nkrumah requests your presence at the palace. You are the first shaman to defeat a dungeon boss.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Journey to the capital palace".to_string(),
                hint: Some("The capital is northeast of your village.".to_string()),
                condition: TutorialCondition::ReachPosition(100, 100),
                dialogue: Some("The palace towers above the city, its golden spires catching the sun. Guards salute as you approach.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Speak with King Nkrumah".to_string(),
                hint: None,
                condition: TutorialCondition::InteractWithNpc("king_nkrumah".to_string()),
                dialogue: Some("The king is younger than you expected, but his eyes hold the weight of kingdoms. 'Thank you for coming, young shaman.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Hear the king's warning".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'Rival villages gather their forces. Foreigners with strange machines approach our borders. War is coming. But there's something else...'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Learn about the mysterious scientist".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'Our scouts report a scientist working with foreign powers. He has... unnatural spirit technology. I fear this is connected to the corruption.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 5,
                objective: "Accept the king's request".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("accepted_kings_request".to_string()),
                dialogue: Some("'I need your strength, shaman. Protect our people. Prepare for war.' You bow and accept the responsibility.".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["socialized_at_festival".to_string()],
        reward_xp: 1500,
    }
}

// =============================================================================
// ACT 2: WAR DRUMS AND THE SPIRIT SCIENTIST
// Conduit Level: 3 → 6
// =============================================================================

/// Act 2 - Border Village Attack
pub fn create_border_attack_mission() -> TutorialMission {
    TutorialMission {
        id: "act2_border_attack".to_string(),
        title: "Attack on the Border".to_string(),
        description: "Two weeks after the king's warning, border villages are under siege.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Respond to the emergency summons".to_string(),
                hint: Some("The head shaman sends you as relief.".to_string()),
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'Three border villages are under attack! You're our fastest shaman. Go, now!'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Travel to the border village".to_string(),
                hint: Some("The journey takes half a day by spirit-enhanced travel.".to_string()),
                condition: TutorialCondition::ReachPosition(150, 50),
                dialogue: Some("Smoke rises in the distance. You hear screams and the clash of steel.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Witness the spirit-machine hybrid weapon".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForCutscene("spirit_cannon".to_string()),
                dialogue: Some("A massive cannon fires corrupted spirit projectiles. It's unholy - machinery merged with spirits!".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Meet Lieutenant Osei".to_string(),
                hint: Some("A warrior fighting the invaders.".to_string()),
                condition: TutorialCondition::InteractWithNpc("osei".to_string()),
                dialogue: Some("'Shaman! Thank the ancestors! We can't get close to that weapon without being corrupted!'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Destroy the spirit cannon".to_string(),
                hint: Some("Purify its core to disable it!".to_string()),
                condition: TutorialCondition::Custom("destroyed_spirit_cannon".to_string()),
                dialogue: Some("You reach the cannon's core and pour purifying energy into it. The machine shrieks and explodes!".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["accepted_kings_request".to_string()],
        reward_xp: 800,
    }
}

/// Act 2 - First Glimpse of Dr. Ekow
pub fn create_ekow_glimpse_mission() -> TutorialMission {
    TutorialMission {
        id: "act2_ekow_glimpse".to_string(),
        title: "The Masked Observer".to_string(),
        description: "A mysterious figure watches from the shadows...".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Notice the figure on the hill".to_string(),
                hint: Some("After destroying the cannon, you sense eyes on you.".to_string()),
                condition: TutorialCondition::Custom("spotted_figure".to_string()),
                dialogue: Some("On a distant hill, a tall figure in a strange mask observes you. Even from here, you feel... wrong.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Chase the figure".to_string(),
                hint: Some("He's getting away!".to_string()),
                condition: TutorialCondition::ReachPosition(160, 45),
                dialogue: Some("You sprint up the hill, but when you arrive, he's gone. Only footprints remain.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Examine the footprints".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("examined_footprints".to_string()),
                dialogue: Some("The footprints glow with four different spirit auras: Chaos, Neutral, Light, and Dark. All at once. Impossible...".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["destroyed_spirit_cannon".to_string()],
        reward_xp: 600,
    }
}

/// Act 2 - Purify Three Villages
pub fn create_purify_villages_mission() -> TutorialMission {
    TutorialMission {
        id: "act2_purify_villages".to_string(),
        title: "Relief Missions".to_string(),
        description: "Purify the three border villages under siege.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Purify the first village".to_string(),
                hint: Some("Clear corruption and heal the wounded.".to_string()),
                condition: TutorialCondition::Custom("purified_village_1".to_string()),
                dialogue: Some("You spend hours cleansing the corruption. The villagers weep with relief.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Purify the second village".to_string(),
                hint: Some("This one is worse - 80% corruption rate.".to_string()),
                condition: TutorialCondition::Custom("purified_village_2".to_string()),
                dialogue: Some("The corruption is thick here. You push yourself to your limits.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Purify the third village".to_string(),
                hint: Some("The final village awaits.".to_string()),
                condition: TutorialCondition::Custom("purified_village_3".to_string()),
                dialogue: Some("Exhausted, you complete the final purification. Three villages saved!".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["examined_footprints".to_string()],
        reward_xp: 1200,
    }
}

/// Act 2 - Captured Soldier Intel
pub fn create_soldier_intel_mission() -> TutorialMission {
    TutorialMission {
        id: "act2_soldier_intel".to_string(),
        title: "The Prisoner's Tale".to_string(),
        description: "Interrogate a captured enemy soldier to learn about the spirit scientist.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Return to camp with Lieutenant Osei".to_string(),
                hint: Some("They've captured an enemy soldier.".to_string()),
                condition: TutorialCondition::ReachPosition(155, 48),
                dialogue: Some("'We caught one trying to flee. He might have information.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Speak with the prisoner".to_string(),
                hint: None,
                condition: TutorialCondition::InteractWithNpc("enemy_soldier".to_string()),
                dialogue: Some("The soldier's eyes are wide with fear. 'Please... I was just following orders...'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Learn about Dr. Ekow".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'The scientist... they call him Dr. Ekow. He was exiled from your kingdom 20 years ago. He promised us spirit weapons in exchange for... test subjects.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Discover his goal".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'He said something about transcending realms... merging with spirits permanently. He's insane, but his weapons work...'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["purified_village_3".to_string()],
        reward_xp: 700,
    }
}

/// Act 2 - Dr. Ekow's Abandoned Laboratory
pub fn create_abandoned_lab_mission() -> TutorialMission {
    TutorialMission {
        id: "act2_abandoned_lab".to_string(),
        title: "The Abandoned Laboratory".to_string(),
        description: "Discover Dr. Ekow's old research facility in the corrupted lands.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Follow intelligence reports to the lab location".to_string(),
                hint: Some("The lab is hidden deep in heavily corrupted territory.".to_string()),
                condition: TutorialCondition::ReachPosition(200, 80),
                dialogue: Some("A crumbling structure emerges from the mist. Strange symbols cover the walls.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Enter the laboratory".to_string(),
                hint: Some("The corruption here is overwhelming.".to_string()),
                condition: TutorialCondition::Custom("entered_lab".to_string()),
                dialogue: Some("Inside, you find broken equipment, shattered spirit vessels, and bloodstains on the floor.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Find Dr. Ekow's research notes".to_string(),
                hint: Some("Search the laboratory for clues.".to_string()),
                condition: TutorialCondition::Custom("found_research_notes".to_string()),
                dialogue: Some("A journal lies open on a desk. The handwriting grows more erratic with each page.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Read the final entry".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'They called me mad. But I will transcend the 5 realms - Physical, Chaos, Neutral, Light, Dark. And then... the Unknown Dimension awaits.'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["learned_about_ekow".to_string()],
        reward_xp: 900,
    }
}

/// Act 2 - Harvest Festival Returns
pub fn create_harvest_festival_mission() -> TutorialMission {
    TutorialMission {
        id: "act2_harvest_festival".to_string(),
        title: "The Harvest Festival".to_string(),
        description: "50% of the village is saved. The Harvest Festival returns for the first time!".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Return home for the festival".to_string(),
                hint: Some("Your village celebrates the harvest.".to_string()),
                condition: TutorialCondition::Custom("arrived_at_festival".to_string()),
                dialogue: Some("Music fills the air. People dance and feast. After so much darkness, this feels like a dream.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Compete in the ceremonial spirit duels".to_string(),
                hint: Some("Friendly competition against your rivals!".to_string()),
                condition: TutorialCondition::Custom("competed_in_duels".to_string()),
                dialogue: Some("You face each rival in turn, testing your skills in non-lethal combat. The crowd cheers!".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Romance subplot decision".to_string(),
                hint: Some("You can choose to pursue one of the rivals romantically, or remain focused on your duties.".to_string()),
                condition: TutorialCondition::Custom("romance_choice_made".to_string()),
                dialogue: Some("Under the stars, you have a chance to get closer to one of your companions...".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Enjoy the celebration".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("For one beautiful night, the war seems far away. You laugh with friends and celebrate life.".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["found_research_notes".to_string()],
        reward_xp: 800,
    }
}

/// Act 2 - Coordinated Attack (Choice Mission)
pub fn create_coordinated_attack_mission() -> TutorialMission {
    TutorialMission {
        id: "act2_coordinated_attack".to_string(),
        title: "Five Villages Under Siege".to_string(),
        description: "Five villages are attacked simultaneously. You can only save three!".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Festival interrupted by emergency signals".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("emergency_triggered".to_string()),
                dialogue: Some("Alarm bells ring out. A runner collapses at your feet: 'Five villages... all at once... attack...'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Make a choice: Which three villages to save?".to_string(),
                hint: Some("Each village has different resources and people. Choose carefully!".to_string()),
                condition: TutorialCondition::Custom("villages_chosen".to_string()),
                dialogue: Some("You can't save everyone. Who will you prioritize?".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Save the first chosen village".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("saved_village_choice_1".to_string()),
                dialogue: Some("You arrive in time. The villagers are saved!".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Save the second chosen village".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("saved_village_choice_2".to_string()),
                dialogue: Some("Racing against time, you purge the corruption. Two down, one to go!".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Save the third chosen village".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("saved_village_choice_3".to_string()),
                dialogue: Some("Exhausted, you complete the final rescue. But... what about the other two villages?".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 5,
                objective: "Face the consequences".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("News arrives: The two unsaved villages were destroyed. Everyone dead or corrupted. You saved three, but lost two. The weight crushes you.".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["romance_choice_made".to_string()],
        reward_xp: 1500,
    }
}

/// Act 2 - War Council
pub fn create_war_council_mission() -> TutorialMission {
    TutorialMission {
        id: "act2_war_council".to_string(),
        title: "The War Council".to_string(),
        description: "The king召mons all shamans, warriors, and elders to plan the war effort.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Attend the war council at the palace".to_string(),
                hint: None,
                condition: TutorialCondition::ReachPosition(100, 100),
                dialogue: Some("The council chamber is packed. Warriors, shamans, nobles - all gathered to decide the kingdom's fate.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Receive promotion to Adept Shaman".to_string(),
                hint: Some("Your heroism has been recognized!".to_string()),
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("King Nkrumah speaks: 'For your bravery in saving three villages, we promote you to Adept Shaman.' The crowd applauds.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Vote on strategy: Defense or Offense?".to_string(),
                hint: Some("Defensive war protects villages. Offensive strikes target enemy camps.".to_string()),
                condition: TutorialCondition::Custom("strategy_voted".to_string()),
                dialogue: Some("The council debates heatedly. What is your vote?".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Volunteer to lead the strike team".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'I'll lead the strike on the spirit-weapon factory,' you declare. The king nods grimly. 'Then go with our blessing.'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["faced_village_consequences".to_string()],
        reward_xp: 1000,
    }
}

/// Act 2 - Destroy Spirit Factory
pub fn create_destroy_factory_mission() -> TutorialMission {
    TutorialMission {
        id: "act2_destroy_factory".to_string(),
        title: "Assault on the Spirit Factory".to_string(),
        description: "Lead a strike team to destroy Dr. Ekow's spirit-weapon manufacturing facility.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Gather your strike team".to_string(),
                hint: Some("Choose 3 companions: Lieutenant Osei and 2 of your rivals.".to_string()),
                condition: TutorialCondition::Custom("team_assembled".to_string()),
                dialogue: Some("Your team assembles at dawn. Each member nods grimly. This is a suicide mission.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Infiltrate the enemy encampment".to_string(),
                hint: Some("Stealth is key. Avoid detection.".to_string()),
                condition: TutorialCondition::Custom("infiltrated_camp".to_string()),
                dialogue: Some("You slip past sentries, your spirit sight guiding you through shadows.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Reach the factory core".to_string(),
                hint: Some("Fight through guards if detected.".to_string()),
                condition: TutorialCondition::ReachPosition(250, 120),
                dialogue: Some("The factory looms above you - a nightmarish fusion of metal and writhing spirits.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Plant purification charges".to_string(),
                hint: Some("Set up 5 charges around the factory.".to_string()),
                condition: TutorialCondition::Custom("charges_planted".to_string()),
                dialogue: Some("The charges are in place. Now you just need to get out before they detonate.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Boss: Defeat the Spirit-Forged Golem".to_string(),
                hint: Some("Dr. Ekow's first 'lifeless spirit' - a spirit with no will!".to_string()),
                condition: TutorialCondition::Custom("defeated_golem_boss".to_string()),
                dialogue: Some("The golem emerges - a towering construct of metal and spirit energy, with empty, soulless eyes. It doesn't rage or roar. It simply... obeys.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 5,
                objective: "Escape the factory before detonation".to_string(),
                hint: Some("You have 60 seconds!".to_string()),
                condition: TutorialCondition::Custom("escaped_factory".to_string()),
                dialogue: Some("You sprint as the charges activate. Behind you, the factory erupts in a pillar of purifying light!".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["strategy_voted".to_string()],
        reward_xp: 2000,
    }
}

/// Act 2 Finale - Spirit Contact
pub fn create_spirit_contact_mission() -> TutorialMission {
    TutorialMission {
        id: "act2_spirit_contact".to_string(),
        title: "The Spirit's Offer".to_string(),
        description: "Your chosen spirit reveals that Dr. Ekow has contacted them...".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Celebrate the victory".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("celebration_started".to_string()),
                dialogue: Some("The village erupts in celebration. The factory is destroyed! The tide is turning!".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Retire to your quarters".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("went_to_quarters".to_string()),
                dialogue: Some("Exhausted, you collapse into bed. But sleep doesn't come...".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Witness the private vision".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForCutscene("spirit_appears".to_string()),
                dialogue: Some("Your chosen spirit materializes before you, more powerful than ever before.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Hear the terrible truth".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'The scientist... Dr. Ekow... has contacted me. He offers power beyond imagining. Power to achieve my goals. I am... considering it.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Realize the true threat".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("You understand now: Dr. Ekow isn't just creating weapons. He's recruiting the spirits themselves. And if your spirit joins him...".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["escaped_factory".to_string()],
        reward_xp: 2500,
    }
}

// =============================================================================
// ACT 3: THE PUPPET MASTER
// Conduit Level: 6 → 9
// =============================================================================

/// Act 3 - The Invasion Begins
pub fn create_invasion_begins_mission() -> TutorialMission {
    TutorialMission {
        id: "act3_invasion_begins".to_string(),
        title: "10,000 Soldiers".to_string(),
        description: "The foreign invasion force arrives with advanced spirit-tech weapons.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Witness the invasion force".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForCutscene("invasion_army".to_string()),
                dialogue: Some("From the palace walls, you see them: 10,000 soldiers, spirit-cannons, and flying machines. The sky darkens.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Take command of junior shamans".to_string(),
                hint: Some("You now have a squad you can assign to heal villages!".to_string()),
                condition: TutorialCondition::Custom("took_command".to_string()),
                dialogue: Some("Six junior shamans salute you. 'We're under your command, Adept Shaman!'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Assign shamans to village defense".to_string(),
                hint: Some("This unlocks the squad management system.".to_string()),
                condition: TutorialCondition::Custom("assigned_squad".to_string()),
                dialogue: Some("You deploy your shamans strategically. They'll handle village healing while you fight on the frontline.".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["spirit_contact_witnessed".to_string()],
        reward_xp: 1500,
    }
}

// Additional Act 3-6 missions would continue here...
// For brevity, I'll create the key missions and boss fights

/// Act 3 - Capital Siege
pub fn create_capital_siege_mission() -> TutorialMission {
    TutorialMission {
        id: "act3_capital_siege".to_string(),
        title: "Siege of the Capital".to_string(),
        description: "Defend the capital in a massive siege battle!".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Man the walls".to_string(),
                hint: Some("Use spirit attacks to hold off the invaders!".to_string()),
                condition: TutorialCondition::Custom("manned_walls".to_string()),
                dialogue: Some("The enemy crashes against the walls like a tidal wave. You channel all your power into defense.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Participate in the Warrior Initiation Ceremony mid-siege".to_string(),
                hint: Some("Young warriors are initiated during the battle!".to_string()),
                condition: TutorialCondition::Custom("warrior_ceremony_complete".to_string()),
                dialogue: Some("Between battles, young warriors receive their marks. They join the fight immediately. Desperate times.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Unlock Spirit Realm Tiles combat".to_string(),
                hint: Some("Shift into the spirit realm briefly during combat!".to_string()),
                condition: TutorialCondition::Custom("learned_realm_shift".to_string()),
                dialogue: Some("In desperation, you phase partially into the spirit realm. Time slows. You see weaknesses in the enemy formations!".to_string()),
                ui_highlight: Some(UiHighlightZone::SpiritBar),
            },
            TutorialStep {
                step_id: 3,
                objective: "Boss: Defeat General Kiros".to_string(),
                hint: Some("The enemy commander wears spirit-enhanced armor!".to_string()),
                condition: TutorialCondition::Custom("defeated_general_kiros".to_string()),
                dialogue: Some("General Kiros strides onto the battlefield, his armor blazing with stolen spirit power. 'Face me, shaman!'".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["assigned_squad".to_string()],
        reward_xp: 3000,
    }
}

/// Act 3 - The Bargain
pub fn create_spirit_bargain_mission() -> TutorialMission {
    TutorialMission {
        id: "act3_spirit_bargain".to_string(),
        title: "The Spirit's Bargain".to_string(),
        description: "Your chosen spirit offers you ultimate power during the siege...".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Spirit appears on the battlefield".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForCutscene("spirit_bargain".to_string()),
                dialogue: Some("Your chosen spirit manifests, massive and terrifying. It has grown even stronger.".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Hear the offer".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("'Join us. Dr. Ekow and I will give you power to save everyone instantly. No more death. No more suffering. Just... obey.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Make your choice: Accept or Refuse?".to_string(),
                hint: Some("This choice affects the difficulty and your reputation!".to_string()),
                condition: TutorialCondition::Custom("bargain_choice_made".to_string()),
                dialogue: Some("The fate of everyone hangs on your decision...".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["defeated_general_kiros".to_string()],
        reward_xp: 2000,
    }
}

// Due to length constraints, I'll create a summary structure for the remaining acts
// and key boss missions. Full implementation would continue this pattern.

/// Act 6 - Final Boss Fight
pub fn create_final_boss_mission() -> TutorialMission {
    TutorialMission {
        id: "act6_final_boss".to_string(),
        title: "Reality Itself".to_string(),
        description: "Dr. Ekow has become one with reality. This is the end.".to_string(),
        steps: vec![
            TutorialStep {
                step_id: 0,
                objective: "Face Dr. Ekow - Form 3: Reality Itself".to_string(),
                hint: Some("He can rewrite the rules of the game mid-fight!".to_string()),
                condition: TutorialCondition::Custom("final_boss_phase_1".to_string()),
                dialogue: Some("Dr. Ekow is everywhere and nowhere. Reality bends to his will. How can you fight a god?".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 1,
                objective: "Survive Phase 1: Realm Shifting".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("final_boss_phase_2".to_string()),
                dialogue: Some("He shifts through all spirit realms, attacking from every dimension at once!".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 2,
                objective: "Survive Phase 2: Existence Erasure".to_string(),
                hint: None,
                condition: TutorialCondition::Custom("final_boss_phase_3".to_string()),
                dialogue: Some("'I can simply... erase you.' The world around you begins to fade...".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 3,
                objective: "Learn the truth: How to seal him".to_string(),
                hint: None,
                condition: TutorialCondition::WaitForDialogue,
                dialogue: Some("Your chosen spirit speaks: 'There's one way. Trap him in the Physical Realm by severing all spirit connections. But someone must anchor the barrier... forever.'".to_string()),
                ui_highlight: None,
            },
            TutorialStep {
                step_id: 4,
                objective: "Make the final choice".to_string(),
                hint: Some("This determines your ending!".to_string()),
                condition: TutorialCondition::Custom("final_choice_made".to_string()),
                dialogue: Some("Who will make the sacrifice? What price are you willing to pay?".to_string()),
                ui_highlight: None,
            },
        ],
        required_flags: vec!["act5_complete".to_string()],
        reward_xp: 10000,
    }
}

// Export all mission creation functions for use in the game
pub fn get_all_story_missions() -> Vec<TutorialMission> {
    vec![
        // Act 1
        create_meet_rivals_mission(),
        create_purification_race_mission(),
        create_team_battle_mission(),
        create_shaman_ceremony_mission(),
        create_spirit_hunt_mission(),
        create_imbument_training_mission(),
        create_meet_priest_mission(),
        create_trading_mission(),
        create_conduit_sensor_mission(),
        create_first_boss_mission(),
        create_religious_festival_mission(),
        create_kings_summons_mission(),
        // Act 2
        create_border_attack_mission(),
        create_ekow_glimpse_mission(),
        create_purify_villages_mission(),
        create_soldier_intel_mission(),
        create_abandoned_lab_mission(),
        create_harvest_festival_mission(),
        create_coordinated_attack_mission(),
        create_war_council_mission(),
        create_destroy_factory_mission(),
        create_spirit_contact_mission(),
        // Act 3
        create_invasion_begins_mission(),
        create_capital_siege_mission(),
        create_spirit_bargain_mission(),
        // Act 6
        create_final_boss_mission(),
        // Additional missions would be added here for Acts 3-6
    ]
}

