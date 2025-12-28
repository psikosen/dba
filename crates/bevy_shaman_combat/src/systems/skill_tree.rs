use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

// ============================================================================
// SKILL TREE COMPONENTS
// ============================================================================

/// Player's skill tree progression
#[derive(Component, Default, Clone, Serialize, Deserialize)]
pub struct SkillTree {
    pub unlocked_skills: HashSet<SkillId>,
    pub skill_points: u32,
    pub total_points_earned: u32,
}

impl SkillTree {
    pub fn new() -> Self {
        Self {
            unlocked_skills: HashSet::new(),
            skill_points: 0,
            total_points_earned: 0,
        }
    }

    /// Award skill points (from leveling up, quests, etc.)
    pub fn award_points(&mut self, points: u32) {
        self.skill_points += points;
        self.total_points_earned += points;
    }

    /// Try to unlock a skill
    pub fn try_unlock(&mut self, skill_id: SkillId, skill_db: &SkillDatabase) -> Result<(), String> {
        // Check if already unlocked
        if self.unlocked_skills.contains(&skill_id) {
            return Err("Skill already unlocked".to_string());
        }

        // Get skill data
        let skill = skill_db.get_skill(skill_id)
            .ok_or_else(|| "Skill not found".to_string())?;

        // Check cost
        if self.skill_points < skill.cost {
            return Err(format!("Not enough skill points (need {}, have {})",
                skill.cost, self.skill_points));
        }

        // Check prerequisites
        for prereq in &skill.prerequisites {
            if !self.unlocked_skills.contains(prereq) {
                return Err(format!("Missing prerequisite: {:?}", prereq));
            }
        }

        // Unlock skill
        self.skill_points -= skill.cost;
        self.unlocked_skills.insert(skill_id);
        Ok(())
    }

    /// Check if a skill is unlocked
    pub fn has_skill(&self, skill_id: SkillId) -> bool {
        self.unlocked_skills.contains(&skill_id)
    }

    /// Get total number of unlocked skills in a path
    pub fn skills_in_path(&self, path: SkillPath) -> usize {
        self.unlocked_skills.iter()
            .filter(|id| id.path() == path)
            .count()
    }
}

// ============================================================================
// SKILL DATABASE
// ============================================================================

/// Database of all available skills
#[derive(Resource, Default, Clone)]
pub struct SkillDatabase {
    pub skills: HashMap<SkillId, Skill>,
}

impl SkillDatabase {
    pub fn new() -> Self {
        let mut db = Self {
            skills: HashMap::new(),
        };
        db.populate();
        db
    }

    pub fn get_skill(&self, id: SkillId) -> Option<&Skill> {
        self.skills.get(&id)
    }

    fn add_skill(&mut self, skill: Skill) {
        self.skills.insert(skill.id, skill);
    }

    /// Populate with all skills
    fn populate(&mut self) {
        // ========================================
        // NGOMA PATH (Rhythm & Combos)
        // ========================================
        self.add_skill(Skill {
            id: SkillId::NgomaRhythmSense,
            name: "Rhythm Sense",
            description: "Feel the pulse of the ancestors. +10% rhythm timing window.",
            path: SkillPath::Ngoma,
            tier: 1,
            cost: 1,
            prerequisites: vec![],
            effect: SkillEffect::RhythmWindowIncrease(10.0),
        });

        self.add_skill(Skill {
            id: SkillId::NgomaComboMaster,
            name: "Combo Master",
            description: "Chain attacks with ancestral grace. +2 max combo length.",
            path: SkillPath::Ngoma,
            tier: 2,
            cost: 2,
            prerequisites: vec![SkillId::NgomaRhythmSense],
            effect: SkillEffect::ComboLengthIncrease(2),
        });

        self.add_skill(Skill {
            id: SkillId::NgomaPerfectHarmony,
            name: "Perfect Harmony",
            description: "Achieve perfect synchronization. Perfect hits grant +20% damage.",
            path: SkillPath::Ngoma,
            tier: 3,
            cost: 3,
            prerequisites: vec![SkillId::NgomaComboMaster],
            effect: SkillEffect::PerfectHitDamageBonus(20.0),
        });

        self.add_skill(Skill {
            id: SkillId::NgomaDrumOfWar,
            name: "Drum of War",
            description: "War drums echo through the spirit realm. Combos reduce cooldowns by 15%.",
            path: SkillPath::Ngoma,
            tier: 4,
            cost: 4,
            prerequisites: vec![SkillId::NgomaPerfectHarmony],
            effect: SkillEffect::ComboCooldownReduction(15.0),
        });

        self.add_skill(Skill {
            id: SkillId::NgomaAncestralRhythm,
            name: "Ancestral Rhythm",
            description: "The ancestors guide your strikes. Unlock special combo finishers.",
            path: SkillPath::Ngoma,
            tier: 5,
            cost: 5,
            prerequisites: vec![SkillId::NgomaDrumOfWar],
            effect: SkillEffect::UnlockComboFinishers,
        });

        // ========================================
        // UBUNTU PATH (Party & Community)
        // ========================================
        self.add_skill(Skill {
            id: SkillId::UbuntuSharedStrength,
            name: "Shared Strength",
            description: "Together we are stronger. +10% damage when near allies.",
            path: SkillPath::Ubuntu,
            tier: 1,
            cost: 1,
            prerequisites: vec![],
            effect: SkillEffect::PartyDamageBonus(10.0),
        });

        self.add_skill(Skill {
            id: SkillId::UbuntuHealingCircle,
            name: "Healing Circle",
            description: "Share life force with your party. HP regen affects all nearby allies.",
            path: SkillPath::Ubuntu,
            tier: 2,
            cost: 2,
            prerequisites: vec![SkillId::UbuntuSharedStrength],
            effect: SkillEffect::PartyHealingSharing(50.0),
        });

        self.add_skill(Skill {
            id: SkillId::UbuntuSpiritLink,
            name: "Spirit Link",
            description: "Link spirits across the party. Share 25% of spirit regeneration.",
            path: SkillPath::Ubuntu,
            tier: 3,
            cost: 3,
            prerequisites: vec![SkillId::UbuntuHealingCircle],
            effect: SkillEffect::PartySpiritSharing(25.0),
        });

        self.add_skill(Skill {
            id: SkillId::UbuntuWarChant,
            name: "War Chant",
            description: "Rally the community. Party members gain +15% attack speed.",
            path: SkillPath::Ubuntu,
            tier: 4,
            cost: 4,
            prerequisites: vec![SkillId::UbuntuSpiritLink],
            effect: SkillEffect::PartyAttackSpeed(15.0),
        });

        self.add_skill(Skill {
            id: SkillId::UbuntuAncestralBond,
            name: "Ancestral Bond",
            description: "The ancestors bless your fellowship. Party buffs last 50% longer.",
            path: SkillPath::Ubuntu,
            tier: 5,
            cost: 5,
            prerequisites: vec![SkillId::UbuntuWarChant],
            effect: SkillEffect::PartyBuffDuration(50.0),
        });

        // ========================================
        // ASHE PATH (Power & Stats)
        // ========================================
        self.add_skill(Skill {
            id: SkillId::AsheInnerPower,
            name: "Inner Power",
            description: "Channel your life force. +15% max health.",
            path: SkillPath::Ashe,
            tier: 1,
            cost: 1,
            prerequisites: vec![],
            effect: SkillEffect::MaxHealthIncrease(15.0),
        });

        self.add_skill(Skill {
            id: SkillId::AsheSpiritReservoir,
            name: "Spirit Reservoir",
            description: "Deepen your spiritual well. +20% max spirit.",
            path: SkillPath::Ashe,
            tier: 2,
            cost: 2,
            prerequisites: vec![SkillId::AsheInnerPower],
            effect: SkillEffect::MaxSpiritIncrease(20.0),
        });

        self.add_skill(Skill {
            id: SkillId::AshePowerStrike,
            name: "Power Strike",
            description: "Strike with the force of the ancestors. +20% base damage.",
            path: SkillPath::Ashe,
            tier: 3,
            cost: 3,
            prerequisites: vec![SkillId::AsheSpiritReservoir],
            effect: SkillEffect::BaseDamageIncrease(20.0),
        });

        self.add_skill(Skill {
            id: SkillId::AsheSpiritFortitude,
            name: "Spirit Fortitude",
            description: "Harden your spirit against harm. +10% damage resistance.",
            path: SkillPath::Ashe,
            tier: 4,
            cost: 4,
            prerequisites: vec![SkillId::AshePowerStrike],
            effect: SkillEffect::DamageResistance(10.0),
        });

        self.add_skill(Skill {
            id: SkillId::AsheAncestralMight,
            name: "Ancestral Might",
            description: "Channel the power of all ancestors. +30% damage, +30% spirit regen.",
            path: SkillPath::Ashe,
            tier: 5,
            cost: 5,
            prerequisites: vec![SkillId::AsheSpiritFortitude],
            effect: SkillEffect::AncestralMight,
        });

        // ========================================
        // UBIQA PATH (Plants & Nature)
        // ========================================
        self.add_skill(Skill {
            id: SkillId::UbiqaGreenThumb,
            name: "Green Thumb",
            description: "Understand the spirits of plants. Plant effects last 20% longer.",
            path: SkillPath::Ubiqa,
            tier: 1,
            cost: 1,
            prerequisites: vec![],
            effect: SkillEffect::PlantEffectDuration(20.0),
        });

        self.add_skill(Skill {
            id: SkillId::UbiqaBloodKnowledge,
            name: "Blood Knowledge",
            description: "Reduce blood plant costs. Blood sacrifice cost reduced by 25%.",
            path: SkillPath::Ubiqa,
            tier: 2,
            cost: 2,
            prerequisites: vec![SkillId::UbiqaGreenThumb],
            effect: SkillEffect::BloodCostReduction(25.0),
        });

        self.add_skill(Skill {
            id: SkillId::UbiqaSpiritGardener,
            name: "Spirit Gardener",
            description: "Master spirit plant cultivation. Spirit plant effects +30% stronger.",
            path: SkillPath::Ubiqa,
            tier: 3,
            cost: 3,
            prerequisites: vec![SkillId::UbiqaBloodKnowledge],
            effect: SkillEffect::SpiritPlantPotency(30.0),
        });

        self.add_skill(Skill {
            id: SkillId::UbiqaHarvestBoon,
            name: "Harvest Boon",
            description: "The earth provides abundantly. 20% chance for double plant harvest.",
            path: SkillPath::Ubiqa,
            tier: 4,
            cost: 4,
            prerequisites: vec![SkillId::UbiqaSpiritGardener],
            effect: SkillEffect::DoubleHarvestChance(20.0),
        });

        self.add_skill(Skill {
            id: SkillId::UbiqaNaturesBoon,
            name: "Nature's Boon",
            description: "Become one with nature. Passive plants regenerate resources 50% faster.",
            path: SkillPath::Ubiqa,
            tier: 5,
            cost: 5,
            prerequisites: vec![SkillId::UbiqaHarvestBoon],
            effect: SkillEffect::PassivePlantBonus(50.0),
        });

        // ========================================
        // TEMPO PATH (Speed & Cooldowns)
        // ========================================
        self.add_skill(Skill {
            id: SkillId::TempoSwiftness,
            name: "Swiftness",
            description: "Move like the wind. +15% movement speed.",
            path: SkillPath::Tempo,
            tier: 1,
            cost: 1,
            prerequisites: vec![],
            effect: SkillEffect::MovementSpeed(15.0),
        });

        self.add_skill(Skill {
            id: SkillId::TempoQuickRecovery,
            name: "Quick Recovery",
            description: "Recover abilities faster. -10% all cooldowns.",
            path: SkillPath::Tempo,
            tier: 2,
            cost: 2,
            prerequisites: vec![SkillId::TempoSwiftness],
            effect: SkillEffect::CooldownReduction(10.0),
        });

        self.add_skill(Skill {
            id: SkillId::TempoHaste,
            name: "Haste",
            description: "Attack with blinding speed. +20% attack speed.",
            path: SkillPath::Tempo,
            tier: 3,
            cost: 3,
            prerequisites: vec![SkillId::TempoQuickRecovery],
            effect: SkillEffect::AttackSpeed(20.0),
        });

        self.add_skill(Skill {
            id: SkillId::TempoRapidFire,
            name: "Rapid Fire",
            description: "Strike multiple times in an instant. -25% ability cooldowns.",
            path: SkillPath::Tempo,
            tier: 4,
            cost: 4,
            prerequisites: vec![SkillId::TempoHaste],
            effect: SkillEffect::CooldownReduction(25.0),
        });

        self.add_skill(Skill {
            id: SkillId::TempoTimeless,
            name: "Timeless",
            description: "Transcend the flow of time. +40% attack speed, -30% cooldowns.",
            path: SkillPath::Tempo,
            tier: 5,
            cost: 5,
            prerequisites: vec![SkillId::TempoRapidFire],
            effect: SkillEffect::Timeless,
        });
    }
}

// ============================================================================
// SKILL DEFINITIONS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillId {
    // Ngoma (Rhythm) Path
    NgomaRhythmSense,
    NgomaComboMaster,
    NgomaPerfectHarmony,
    NgomaDrumOfWar,
    NgomaAncestralRhythm,

    // Ubuntu (Community) Path
    UbuntuSharedStrength,
    UbuntuHealingCircle,
    UbuntuSpiritLink,
    UbuntuWarChant,
    UbuntuAncestralBond,

    // Ashe (Power) Path
    AsheInnerPower,
    AsheSpiritReservoir,
    AshePowerStrike,
    AsheSpiritFortitude,
    AsheAncestralMight,

    // Ubiqa (Nature) Path
    UbiqaGreenThumb,
    UbiqaBloodKnowledge,
    UbiqaSpiritGardener,
    UbiqaHarvestBoon,
    UbiqaNaturesBoon,

    // Tempo (Speed) Path
    TempoSwiftness,
    TempoQuickRecovery,
    TempoHaste,
    TempoRapidFire,
    TempoTimeless,
}

impl SkillId {
    pub fn path(&self) -> SkillPath {
        match self {
            SkillId::NgomaRhythmSense | SkillId::NgomaComboMaster
            | SkillId::NgomaPerfectHarmony | SkillId::NgomaDrumOfWar
            | SkillId::NgomaAncestralRhythm => SkillPath::Ngoma,

            SkillId::UbuntuSharedStrength | SkillId::UbuntuHealingCircle
            | SkillId::UbuntuSpiritLink | SkillId::UbuntuWarChant
            | SkillId::UbuntuAncestralBond => SkillPath::Ubuntu,

            SkillId::AsheInnerPower | SkillId::AsheSpiritReservoir
            | SkillId::AshePowerStrike | SkillId::AsheSpiritFortitude
            | SkillId::AsheAncestralMight => SkillPath::Ashe,

            SkillId::UbiqaGreenThumb | SkillId::UbiqaBloodKnowledge
            | SkillId::UbiqaSpiritGardener | SkillId::UbiqaHarvestBoon
            | SkillId::UbiqaNaturesBoon => SkillPath::Ubiqa,

            SkillId::TempoSwiftness | SkillId::TempoQuickRecovery
            | SkillId::TempoHaste | SkillId::TempoRapidFire
            | SkillId::TempoTimeless => SkillPath::Tempo,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillPath {
    Ngoma,   // Rhythm & Combos (Swahili: drum)
    Ubuntu,  // Community & Party (Nguni Bantu: humanity/community)
    Ashe,    // Power & Stats (Yoruba: power/authority)
    Ubiqa,   // Plants & Nature (Xhosa: to bloom/flourish)
    Tempo,   // Speed & Cooldowns
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: SkillId,
    pub name: &'static str,
    pub description: &'static str,
    pub path: SkillPath,
    pub tier: u32,
    pub cost: u32,
    pub prerequisites: Vec<SkillId>,
    pub effect: SkillEffect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillEffect {
    // Ngoma effects
    RhythmWindowIncrease(f32),
    ComboLengthIncrease(usize),
    PerfectHitDamageBonus(f32),
    ComboCooldownReduction(f32),
    UnlockComboFinishers,

    // Ubuntu effects
    PartyDamageBonus(f32),
    PartyHealingSharing(f32),
    PartySpiritSharing(f32),
    PartyAttackSpeed(f32),
    PartyBuffDuration(f32),

    // Ashe effects
    MaxHealthIncrease(f32),
    MaxSpiritIncrease(f32),
    BaseDamageIncrease(f32),
    DamageResistance(f32),
    AncestralMight,  // Combined bonus

    // Ubiqa effects
    PlantEffectDuration(f32),
    BloodCostReduction(f32),
    SpiritPlantPotency(f32),
    DoubleHarvestChance(f32),
    PassivePlantBonus(f32),

    // Tempo effects
    MovementSpeed(f32),
    CooldownReduction(f32),
    AttackSpeed(f32),
    Timeless,  // Combined bonus
}

// ============================================================================
// SKILL TREE SYSTEMS
// ============================================================================

/// System to handle skill unlocking (input-driven)
pub fn skill_unlock_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut skill_tree_query: Query<&mut SkillTree>,
    skill_db: Res<SkillDatabase>,
) {
    // Press 'K' to open skill tree (will be UI-driven later)
    if !keyboard.just_pressed(KeyCode::KeyK) {
        return;
    }

    // For now, just log available skills
    for skill_tree in skill_tree_query.iter_mut() {
        info!("Skill Points: {}", skill_tree.skill_points);
        info!("Unlocked Skills: {}", skill_tree.unlocked_skills.len());
    }
}

/// System to apply passive skill bonuses
pub fn apply_skill_bonuses(
    skill_tree_query: Query<&SkillTree>,
    skill_db: Res<SkillDatabase>,
    mut spirit_query: Query<&mut bevy_shaman_core::components::Spirit>,
    mut health_query: Query<&mut bevy_shaman_core::components::Health>,
) {
    for skill_tree in skill_tree_query.iter() {
        // Apply health bonuses
        if let Ok(mut health) = health_query.get_single_mut() {
            let mut health_bonus = 0.0;
            if skill_tree.has_skill(SkillId::AsheInnerPower) {
                health_bonus += 15.0;
            }
            if health_bonus > 0.0 {
                let bonus_amount = health.max * (health_bonus / 100.0);
                health.max += bonus_amount;
            }
        }

        // Apply spirit bonuses
        if let Ok(mut spirit) = spirit_query.get_single_mut() {
            let mut spirit_bonus = 0.0;
            if skill_tree.has_skill(SkillId::AsheSpiritReservoir) {
                spirit_bonus += 20.0;
            }
            if spirit_bonus > 0.0 {
                let bonus_amount = spirit.max * (spirit_bonus / 100.0);
                spirit.max += bonus_amount;
            }
        }
    }
}
