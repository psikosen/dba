use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export from core to avoid circular dependency
pub use bevy_shaman_core::events::QuestCompleted;

// ============================================================================
// QUEST SYSTEM - Complete Quest Tracking with UI
// ============================================================================

/// A quest with objectives, rewards, and tracking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub objectives: Vec<QuestObjective>,
    pub rewards: QuestRewards,
    pub quest_giver: String,
    pub quest_type: QuestType,
    pub status: QuestStatus,
    pub location_hint: Option<String>,
}

impl Quest {
    pub fn new(id: String, title: String, description: String, quest_giver: String) -> Self {
        Self {
            id,
            title,
            description,
            objectives: Vec::new(),
            rewards: QuestRewards::default(),
            quest_giver,
            quest_type: QuestType::Main,
            status: QuestStatus::NotStarted,
            location_hint: None,
        }
    }

    pub fn with_objective(mut self, objective: QuestObjective) -> Self {
        self.objectives.push(objective);
        self
    }

    pub fn with_reward(mut self, rewards: QuestRewards) -> Self {
        self.rewards = rewards;
        self
    }

    pub fn with_type(mut self, quest_type: QuestType) -> Self {
        self.quest_type = quest_type;
        self
    }

    pub fn with_location_hint(mut self, hint: String) -> Self {
        self.location_hint = Some(hint);
        self
    }

    /// Check if all objectives are complete
    pub fn is_complete(&self) -> bool {
        self.objectives.iter().all(|obj| obj.is_complete())
    }

    /// Get current objective progress as a string
    pub fn progress_summary(&self) -> String {
        let completed = self
            .objectives
            .iter()
            .filter(|obj| obj.is_complete())
            .count();
        format!("{}/{} objectives", completed, self.objectives.len())
    }

    /// Get the next incomplete objective
    pub fn get_current_objective(&self) -> Option<&QuestObjective> {
        self.objectives.iter().find(|obj| !obj.is_complete())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestObjective {
    pub description: String,
    pub objective_type: ObjectiveType,
    pub progress: u32,
    pub required: u32,
    pub completed: bool,
    pub optional: bool,
}

impl QuestObjective {
    pub fn new(description: String, objective_type: ObjectiveType, required: u32) -> Self {
        Self {
            description,
            objective_type,
            progress: 0,
            required,
            completed: false,
            optional: false,
        }
    }

    pub fn as_optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn add_progress(&mut self, amount: u32) {
        self.progress = (self.progress + amount).min(self.required);
        if self.progress >= self.required {
            self.completed = true;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.completed || (self.optional && self.progress >= self.required)
    }

    pub fn progress_text(&self) -> String {
        match &self.objective_type {
            ObjectiveType::Kill(_) => format!("{}/{}", self.progress, self.required),
            ObjectiveType::Collect(_) => format!("{}/{}", self.progress, self.required),
            ObjectiveType::Reach(_) => {
                if self.completed {
                    "Complete".to_string()
                } else {
                    "Incomplete".to_string()
                }
            }
            ObjectiveType::Talk(_) => {
                if self.completed {
                    "Complete".to_string()
                } else {
                    "Talk to NPC".to_string()
                }
            }
            ObjectiveType::Purify => format!("{}/{}", self.progress, self.required),
            ObjectiveType::Custom => format!("{}/{}", self.progress, self.required),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ObjectiveType {
    Kill(String),    // Kill specific monster type
    Collect(String), // Collect specific items
    Reach(String),   // Reach a location
    Talk(String),    // Talk to an NPC
    Purify,          // Purify corrupted tiles
    Custom,          // Custom objective
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct QuestRewards {
    pub gold: u32,
    pub xp: u32,
    pub skill_points: u32,
    pub items: Vec<(String, u32)>,      // (item_id, quantity)
    pub reputation: Vec<(String, i32)>, // (faction, amount)
}

impl QuestRewards {
    pub fn with_gold(mut self, gold: u32) -> Self {
        self.gold = gold;
        self
    }

    pub fn with_xp(mut self, xp: u32) -> Self {
        self.xp = xp;
        self
    }

    pub fn with_skill_points(mut self, skill_points: u32) -> Self {
        self.skill_points = skill_points;
        self
    }

    pub fn with_item(mut self, item_id: String, quantity: u32) -> Self {
        self.items.push((item_id, quantity));
        self
    }

    pub fn with_reputation(mut self, faction: String, amount: i32) -> Self {
        self.reputation.push((faction, amount));
        self
    }

    pub fn rewards_text(&self) -> String {
        let mut rewards = Vec::new();

        if self.gold > 0 {
            rewards.push(format!("{} gold", self.gold));
        }
        if self.xp > 0 {
            rewards.push(format!("{} XP", self.xp));
        }
        if self.skill_points > 0 {
            rewards.push(format!("{} skill points", self.skill_points));
        }
        for (item, qty) in &self.items {
            rewards.push(format!("{} x{}", item, qty));
        }
        for (faction, amt) in &self.reputation {
            rewards.push(format!("{:+} {} reputation", amt, faction));
        }

        if rewards.is_empty() {
            "No rewards".to_string()
        } else {
            rewards.join(", ")
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuestType {
    Main,
    Side,
    Tutorial,
    Bounty,
    Fetch,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuestStatus {
    NotStarted,
    Active,
    Completed,
    Failed,
}

// ============================================================================
// RESOURCES
// ============================================================================

/// Global quest log tracking all quests
#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize)]
pub struct QuestLog {
    pub active_quests: Vec<String>,    // Quest IDs
    pub completed_quests: Vec<String>, // Quest IDs
    pub failed_quests: Vec<String>,    // Quest IDs
    pub tracked_quest: Option<String>, // Currently tracked quest ID
}

impl QuestLog {
    pub fn start_quest(&mut self, quest_id: String) {
        if !self.active_quests.contains(&quest_id) {
            self.active_quests.push(quest_id.clone());
            // Auto-track if no quest is tracked
            if self.tracked_quest.is_none() {
                self.tracked_quest = Some(quest_id);
            }
        }
    }

    pub fn complete_quest(&mut self, quest_id: String) {
        if let Some(pos) = self.active_quests.iter().position(|id| id == &quest_id) {
            self.active_quests.remove(pos);
            self.completed_quests.push(quest_id.clone());

            // Untrack if this was the tracked quest
            if self.tracked_quest.as_ref() == Some(&quest_id) {
                self.tracked_quest = self.active_quests.first().cloned();
            }
        }
    }

    pub fn fail_quest(&mut self, quest_id: String) {
        if let Some(pos) = self.active_quests.iter().position(|id| id == &quest_id) {
            self.active_quests.remove(pos);
            self.failed_quests.push(quest_id.clone());

            // Untrack if this was the tracked quest
            if self.tracked_quest.as_ref() == Some(&quest_id) {
                self.tracked_quest = self.active_quests.first().cloned();
            }
        }
    }

    pub fn track_quest(&mut self, quest_id: String) {
        if self.active_quests.contains(&quest_id) {
            self.tracked_quest = Some(quest_id);
        }
    }

    pub fn is_active(&self, quest_id: &str) -> bool {
        self.active_quests.iter().any(|id| id == quest_id)
    }

    pub fn is_completed(&self, quest_id: &str) -> bool {
        self.completed_quests.iter().any(|id| id == quest_id)
    }
}

/// Registry of all available quests
#[derive(Resource, Default)]
pub struct QuestRegistry {
    pub quests: HashMap<String, Quest>,
}

impl QuestRegistry {
    pub fn register(&mut self, mut quest: Quest) {
        quest.status = QuestStatus::NotStarted;
        self.quests.insert(quest.id.clone(), quest);
    }

    pub fn get(&self, quest_id: &str) -> Option<&Quest> {
        self.quests.get(quest_id)
    }

    pub fn get_mut(&mut self, quest_id: &str) -> Option<&mut Quest> {
        self.quests.get_mut(quest_id)
    }

    pub fn start_quest(&mut self, quest_id: &str) -> bool {
        if let Some(quest) = self.get_mut(quest_id) {
            quest.status = QuestStatus::Active;
            true
        } else {
            false
        }
    }

    pub fn complete_quest(&mut self, quest_id: &str) -> bool {
        if let Some(quest) = self.get_mut(quest_id) {
            quest.status = QuestStatus::Completed;
            true
        } else {
            false
        }
    }

    pub fn update_objective_progress(
        &mut self,
        quest_id: &str,
        objective_index: usize,
        amount: u32,
    ) {
        if let Some(quest) = self.get_mut(quest_id) {
            if let Some(objective) = quest.objectives.get_mut(objective_index) {
                objective.add_progress(amount);
                info!(
                    "Quest '{}' objective {} progress: {}/{}",
                    quest.title, objective_index, objective.progress, objective.required
                );

                // Auto-complete quest if all objectives done
                if quest.is_complete() && quest.status == QuestStatus::Active {
                    quest.status = QuestStatus::Completed;
                    info!("Quest '{}' completed!", quest.title);
                }
            }
        }
    }
}

// ============================================================================
// EVENTS
// ============================================================================

#[derive(Event, Debug, Clone)]
pub struct QuestStarted {
    pub quest_id: String,
}

// QuestCompleted moved to bevy_shaman_core and re-exported above

#[derive(Event, Debug, Clone)]
pub struct QuestFailed {
    pub quest_id: String,
}

#[derive(Event, Debug, Clone)]
pub struct QuestObjectiveUpdated {
    pub quest_id: String,
    pub objective_index: usize,
    pub amount: u32,
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Handle quest start events
pub fn handle_quest_started(
    mut events: EventReader<QuestStarted>,
    mut quest_log: ResMut<QuestLog>,
    mut registry: ResMut<QuestRegistry>,
) {
    for event in events.read() {
        quest_log.start_quest(event.quest_id.clone());
        registry.start_quest(&event.quest_id);
        info!("Quest started: {}", event.quest_id);
    }
}

/// Handle quest completion
pub fn handle_quest_completed(
    mut events: EventReader<QuestCompleted>,
    mut quest_log: ResMut<QuestLog>,
    mut registry: ResMut<QuestRegistry>,
    mut player_level: ResMut<bevy_shaman_core::resources::PlayerLevel>,
    mut currency: ResMut<bevy_shaman_shop::resources::Currency>,
    mut player_inventory: Query<
        &mut bevy_shaman_items::components::Inventory,
        With<bevy_shaman_core::components::Player>,
    >,
    mut skill_tree_query: Query<
        &mut bevy_shaman_combat::systems::skill_tree::SkillTree,
        With<bevy_shaman_core::components::Player>,
    >,
    mut reputation: ResMut<super::dialogue_tree::DialogueReputation>,
) {
    for event in events.read() {
        quest_log.complete_quest(event.quest_id.clone());
        registry.complete_quest(&event.quest_id);

        if let Some(quest) = registry.get(&event.quest_id) {
            info!(
                "Quest completed: {} - Rewards: {}",
                quest.title,
                quest.rewards.rewards_text()
            );

            // Grant XP
            if quest.rewards.xp > 0 {
                let leveled_up = player_level.add_experience(quest.rewards.xp);
                info!("Granted {} XP", quest.rewards.xp);
                if leveled_up {
                    info!("Player leveled up to level {}!", player_level.current);
                }
            }

            // Grant gold
            if quest.rewards.gold > 0 {
                currency.gold += quest.rewards.gold;
                info!("Granted {} gold", quest.rewards.gold);
            }

            // Grant skill points
            if quest.rewards.skill_points > 0 {
                if let Ok(mut skill_tree) = skill_tree_query.get_single_mut() {
                    skill_tree.award_points(quest.rewards.skill_points);
                    info!(
                        "Granted {} skill points (Total: {})",
                        quest.rewards.skill_points, skill_tree.skill_points
                    );
                }
            }

            // Grant items
            if let Ok(mut inventory) = player_inventory.get_single_mut() {
                for (item_id, quantity) in &quest.rewards.items {
                    // Create a basic item (in a real system, you'd fetch from an item database)
                    let item = bevy_shaman_items::components::Item {
                        id: item_id.clone(),
                        display_name: item_id.clone(),
                        item_type: bevy_shaman_items::components::ItemType::KeyItem,
                        max_stack: 99,
                    };

                    if inventory.add_item(item, *quantity) {
                        info!("Granted {} x{}", item_id, quantity);
                    } else {
                        warn!("Failed to grant item: {} (inventory full)", item_id);
                    }
                }
            }

            // Grant reputation
            for (faction, amount) in &quest.rewards.reputation {
                reputation.change(faction, *amount);
                info!("Granted {:+} reputation with {}", amount, faction);
            }
        }
    }
}

/// Handle quest objective updates
pub fn handle_quest_objective_updated(
    mut events: EventReader<QuestObjectiveUpdated>,
    mut registry: ResMut<QuestRegistry>,
    mut quest_completed: EventWriter<QuestCompleted>,
) {
    for event in events.read() {
        registry.update_objective_progress(&event.quest_id, event.objective_index, event.amount);

        // Check if quest is now complete
        if let Some(quest) = registry.get(&event.quest_id) {
            if quest.is_complete() && quest.status == QuestStatus::Active {
                quest_completed.send(QuestCompleted {
                    quest_id: event.quest_id.clone(),
                });
            }
        }
    }
}

// ============================================================================
// EXAMPLE QUESTS
// ============================================================================

/// Create the brother cleansing quest
pub fn create_brother_cleansing_quest() -> Quest {
    Quest::new(
        "brother_cleansing".to_string(),
        "Cleanse Your Brother's Soul".to_string(),
        "Your brother has been corrupted by dark forces. Defeat him in ritual combat 4 times to purify his soul.".to_string(),
        "Head Shaman".to_string(),
    )
    .with_type(QuestType::Main)
    .with_objective(
        QuestObjective::new(
            "Defeat your brother in ritual combat".to_string(),
            ObjectiveType::Kill("Corrupted Brother".to_string()),
            4,
        )
    )
    .with_reward(
        QuestRewards::default()
            .with_xp(500)
            .with_skill_points(2)
            .with_reputation("village".to_string(), 25)
    )
    .with_location_hint("Ritual Arena".to_string())
}

/// Create tutorial quest
pub fn create_tutorial_quest() -> Quest {
    Quest::new(
        "tutorial_basics".to_string(),
        "Learn the Ways of the Shaman".to_string(),
        "Complete your training to become a full shaman.".to_string(),
        "Head Shaman".to_string(),
    )
    .with_type(QuestType::Tutorial)
    .with_objective(QuestObjective::new(
        "Defeat a corrupted monster".to_string(),
        ObjectiveType::Kill("Corrupted Spirit".to_string()),
        1,
    ))
    .with_objective(QuestObjective::new(
        "Purify corrupted tiles".to_string(),
        ObjectiveType::Purify,
        3,
    ))
    .with_objective(QuestObjective::new(
        "Choose your spirit companion".to_string(),
        ObjectiveType::Talk("Head Shaman".to_string()),
        1,
    ))
    .with_reward(
        QuestRewards::default()
            .with_xp(100)
            .with_skill_points(1)
            .with_item("Beginner's Staff".to_string(), 1),
    )
}

/// Create village corruption quest
pub fn create_village_corruption_quest() -> Quest {
    Quest::new(
        "village_corruption".to_string(),
        "Cleanse the Village".to_string(),
        "Dark forces have corrupted the village. Purify the corruption and defeat the demons."
            .to_string(),
        "Village Elder".to_string(),
    )
    .with_type(QuestType::Main)
    .with_objective(QuestObjective::new(
        "Defeat demons in the village".to_string(),
        ObjectiveType::Kill("Demon".to_string()),
        10,
    ))
    .with_objective(QuestObjective::new(
        "Purify corrupted village tiles".to_string(),
        ObjectiveType::Purify,
        20,
    ))
    .with_reward(
        QuestRewards::default()
            .with_xp(300)
            .with_gold(100)
            .with_skill_points(1)
            .with_reputation("village".to_string(), 15),
    )
    .with_location_hint("Village Center".to_string())
}
