use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// DIALOGUE TREE SYSTEM - Multiple Choice Branching Dialogues
// ============================================================================

/// A dialogue tree is a collection of nodes that branch based on player choices
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueTree {
    pub tree_id: String,
    pub nodes: HashMap<String, DialogueNode>,
    pub starting_node_id: String,
}

impl DialogueTree {
    pub fn new(tree_id: String, starting_node_id: String) -> Self {
        Self {
            tree_id,
            nodes: HashMap::new(),
            starting_node_id,
        }
    }

    pub fn add_node(&mut self, node: DialogueNode) {
        self.nodes.insert(node.node_id.clone(), node);
    }

    pub fn get_node(&self, node_id: &str) -> Option<&DialogueNode> {
        self.nodes.get(node_id)
    }

    pub fn get_starting_node(&self) -> Option<&DialogueNode> {
        self.nodes.get(&self.starting_node_id)
    }
}

/// A single node in the dialogue tree
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueNode {
    pub node_id: String,
    pub speaker: String,
    pub text: String,
    pub choices: Vec<DialogueChoice>,
    pub is_end_node: bool,
}

impl DialogueNode {
    pub fn new(node_id: String, speaker: String, text: String) -> Self {
        Self {
            node_id,
            speaker,
            text,
            choices: Vec::new(),
            is_end_node: false,
        }
    }

    pub fn with_choice(mut self, choice: DialogueChoice) -> Self {
        self.choices.push(choice);
        self
    }

    pub fn as_end_node(mut self) -> Self {
        self.is_end_node = true;
        self
    }
}

/// A dialogue choice that the player can select
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DialogueChoice {
    pub text: String,
    pub next_node_id: String,
    pub requirements: Vec<DialogueRequirement>,
    pub consequences: Vec<DialogueConsequence>,
    pub disabled_text: Option<String>, // Text to show if requirements not met
}

impl DialogueChoice {
    pub fn new(text: String, next_node_id: String) -> Self {
        Self {
            text,
            next_node_id,
            requirements: Vec::new(),
            consequences: Vec::new(),
            disabled_text: None,
        }
    }

    pub fn with_requirement(mut self, requirement: DialogueRequirement) -> Self {
        self.requirements.push(requirement);
        self
    }

    pub fn with_consequence(mut self, consequence: DialogueConsequence) -> Self {
        self.consequences.push(consequence);
        self
    }

    pub fn with_disabled_text(mut self, text: String) -> Self {
        self.disabled_text = Some(text);
        self
    }

    /// Check if all requirements are met
    pub fn is_available(&self, flags: &DialogueFlags, reputation: &DialogueReputation) -> bool {
        self.requirements.iter().all(|req| req.is_met(flags, reputation))
    }
}

/// Requirements that must be met for a choice to be available
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DialogueRequirement {
    FlagSet(String),
    FlagNotSet(String),
    ReputationGreaterThan(String, i32),
    ReputationLessThan(String, i32),
    ItemInInventory(String, u32),
    QuestCompleted(String),
}

impl DialogueRequirement {
    pub fn is_met(&self, flags: &DialogueFlags, reputation: &DialogueReputation) -> bool {
        match self {
            DialogueRequirement::FlagSet(flag) => flags.is_set(flag),
            DialogueRequirement::FlagNotSet(flag) => !flags.is_set(flag),
            DialogueRequirement::ReputationGreaterThan(faction, value) => {
                reputation.get(faction) > *value
            }
            DialogueRequirement::ReputationLessThan(faction, value) => {
                reputation.get(faction) < *value
            }
            DialogueRequirement::ItemInInventory(_item, _count) => {
                // TODO: Check player inventory
                true
            }
            DialogueRequirement::QuestCompleted(_quest_id) => {
                // TODO: Check quest completion
                true
            }
        }
    }
}

/// Consequences that occur when a choice is selected
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DialogueConsequence {
    SetFlag(String),
    UnsetFlag(String),
    ChangeReputation(String, i32),
    GiveItem(String, u32),
    TakeItem(String, u32),
    StartQuest(String),
    CompleteQuest(String),
    GiveXP(u32),
    TriggerEvent(String),
}

// ============================================================================
// RESOURCES
// ============================================================================

/// Global dialogue flags that track choices and state
#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DialogueFlags {
    pub flags: HashMap<String, bool>,
}

impl DialogueFlags {
    pub fn set(&mut self, flag: &str) {
        self.flags.insert(flag.to_string(), true);
    }

    pub fn unset(&mut self, flag: &str) {
        self.flags.insert(flag.to_string(), false);
    }

    pub fn is_set(&self, flag: &str) -> bool {
        self.flags.get(flag).copied().unwrap_or(false)
    }
}

/// Reputation with different factions/characters
#[derive(Resource, Default, Debug, Clone, Serialize, Deserialize)]
pub struct DialogueReputation {
    pub reputations: HashMap<String, i32>,
}

impl DialogueReputation {
    pub fn change(&mut self, faction: &str, amount: i32) {
        let current = self.reputations.entry(faction.to_string()).or_insert(0);
        *current += amount;
    }

    pub fn get(&self, faction: &str) -> i32 {
        self.reputations.get(faction).copied().unwrap_or(0)
    }
}

/// Registry of all dialogue trees in the game
#[derive(Resource, Default)]
pub struct DialogueTreeRegistry {
    pub trees: HashMap<String, DialogueTree>,
}

impl DialogueTreeRegistry {
    pub fn register(&mut self, tree: DialogueTree) {
        self.trees.insert(tree.tree_id.clone(), tree);
    }

    pub fn get(&self, tree_id: &str) -> Option<&DialogueTree> {
        self.trees.get(tree_id)
    }
}

/// Current active dialogue state
#[derive(Resource, Default, Debug, Clone)]
pub struct ActiveDialogueState {
    pub active: bool,
    pub tree_id: Option<String>,
    pub current_node_id: Option<String>,
    pub npc_entity: Option<Entity>,
}

impl ActiveDialogueState {
    pub fn start_dialogue(&mut self, tree_id: String, npc_entity: Entity) {
        self.active = true;
        self.tree_id = Some(tree_id);
        self.current_node_id = None;
        self.npc_entity = Some(npc_entity);
    }

    pub fn navigate_to_node(&mut self, node_id: String) {
        self.current_node_id = Some(node_id);
    }

    pub fn end_dialogue(&mut self) {
        self.active = false;
        self.tree_id = None;
        self.current_node_id = None;
        self.npc_entity = None;
    }
}

// ============================================================================
// EVENTS
// ============================================================================

#[derive(Event, Debug, Clone)]
pub struct DialogueChoiceSelected {
    pub tree_id: String,
    pub node_id: String,
    pub choice_index: usize,
    pub next_node_id: String,
}

#[derive(Event, Debug, Clone)]
pub struct DialogueTreeStarted {
    pub tree_id: String,
    pub npc_entity: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct DialogueTreeEnded {
    pub tree_id: String,
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Apply consequences when a choice is selected
pub fn apply_dialogue_consequences(
    mut choice_events: EventReader<DialogueChoiceSelected>,
    mut flags: ResMut<DialogueFlags>,
    mut reputation: ResMut<DialogueReputation>,
    registry: Res<DialogueTreeRegistry>,
) {
    for event in choice_events.read() {
        if let Some(tree) = registry.get(&event.tree_id) {
            if let Some(node) = tree.get_node(&event.node_id) {
                if let Some(choice) = node.choices.get(event.choice_index) {
                    // Apply all consequences
                    for consequence in &choice.consequences {
                        apply_consequence(consequence, &mut flags, &mut reputation);
                    }

                    info!(
                        "Applied consequences for choice: {} -> {}",
                        choice.text, event.next_node_id
                    );
                }
            }
        }
    }
}

fn apply_consequence(
    consequence: &DialogueConsequence,
    flags: &mut DialogueFlags,
    reputation: &mut DialogueReputation,
) {
    match consequence {
        DialogueConsequence::SetFlag(flag) => {
            flags.set(flag);
            info!("Set dialogue flag: {}", flag);
        }
        DialogueConsequence::UnsetFlag(flag) => {
            flags.unset(flag);
            info!("Unset dialogue flag: {}", flag);
        }
        DialogueConsequence::ChangeReputation(faction, amount) => {
            reputation.change(faction, *amount);
            info!("Changed reputation with {}: {:+}", faction, amount);
        }
        DialogueConsequence::GiveItem(item, count) => {
            // TODO: Add item to inventory
            info!("Give item: {} x{}", item, count);
        }
        DialogueConsequence::TakeItem(item, count) => {
            // TODO: Remove item from inventory
            info!("Take item: {} x{}", item, count);
        }
        DialogueConsequence::StartQuest(quest_id) => {
            // TODO: Start quest
            info!("Start quest: {}", quest_id);
        }
        DialogueConsequence::CompleteQuest(quest_id) => {
            // TODO: Complete quest
            info!("Complete quest: {}", quest_id);
        }
        DialogueConsequence::GiveXP(xp) => {
            // TODO: Give XP to player
            info!("Give XP: {}", xp);
        }
        DialogueConsequence::TriggerEvent(event_name) => {
            // TODO: Trigger custom event
            info!("Trigger event: {}", event_name);
        }
    }
}

// ============================================================================
// EXAMPLE DIALOGUE TREES
// ============================================================================

/// Create the spirit choice dialogue tree for the tutorial
pub fn create_spirit_choice_tree() -> DialogueTree {
    let mut tree = DialogueTree::new(
        "spirit_choice".to_string(),
        "introduction".to_string(),
    );

    // Introduction node
    let intro_node = DialogueNode::new(
        "introduction".to_string(),
        "Head Shaman".to_string(),
        "Young one, you have passed the trials. Now you must choose a spirit companion to guide you on your journey. Each spirit offers different gifts and challenges. Choose wisely.".to_string(),
    ).with_choice(
        DialogueChoice::new(
            "Tell me about the spirits.".to_string(),
            "explain_spirits".to_string(),
        )
    );

    tree.add_node(intro_node);

    // Explanation node
    let explain_node = DialogueNode::new(
        "explain_spirits".to_string(),
        "Head Shaman".to_string(),
        "The Angelic Spirit brings healing and protection, but may struggle against pure evil. The Neutral Spirit offers balance in all things. The Chaotic Spirit grants great power, but at a cost. The Dark Spirit... walks a dangerous path, trading your humanity for strength.".to_string(),
    ).with_choice(
        DialogueChoice::new(
            "I choose the Angelic Spirit.".to_string(),
            "choice_angelic".to_string(),
        ).with_consequence(
            DialogueConsequence::SetFlag("spirit_choice_angelic".to_string())
        ).with_consequence(
            DialogueConsequence::ChangeReputation("village".to_string(), 10)
        )
    ).with_choice(
        DialogueChoice::new(
            "I choose the Neutral Spirit.".to_string(),
            "choice_neutral".to_string(),
        ).with_consequence(
            DialogueConsequence::SetFlag("spirit_choice_neutral".to_string())
        )
    ).with_choice(
        DialogueChoice::new(
            "I choose the Chaotic Spirit.".to_string(),
            "choice_chaotic".to_string(),
        ).with_consequence(
            DialogueConsequence::SetFlag("spirit_choice_chaotic".to_string())
        ).with_consequence(
            DialogueConsequence::ChangeReputation("village".to_string(), -5)
        )
    ).with_choice(
        DialogueChoice::new(
            "I choose the Dark Spirit.".to_string(),
            "choice_dark".to_string(),
        ).with_consequence(
            DialogueConsequence::SetFlag("spirit_choice_dark".to_string())
        ).with_consequence(
            DialogueConsequence::ChangeReputation("village".to_string(), -15)
        )
    );

    tree.add_node(explain_node);

    // Angelic ending
    let angelic_end = DialogueNode::new(
        "choice_angelic".to_string(),
        "Head Shaman".to_string(),
        "A noble choice. The Angelic Spirit will serve you well, bringing light to the darkness. May your path be blessed.".to_string(),
    ).as_end_node();
    tree.add_node(angelic_end);

    // Neutral ending
    let neutral_end = DialogueNode::new(
        "choice_neutral".to_string(),
        "Head Shaman".to_string(),
        "Balance is the way of wisdom. The Neutral Spirit will help you see all sides of every conflict. Walk carefully.".to_string(),
    ).as_end_node();
    tree.add_node(neutral_end);

    // Chaotic ending
    let chaotic_end = DialogueNode::new(
        "choice_chaotic".to_string(),
        "Head Shaman".to_string(),
        "The Chaotic Spirit is powerful but unpredictable. Control it, or it will control you. May chaos serve your purpose.".to_string(),
    ).as_end_node();
    tree.add_node(chaotic_end);

    // Dark ending
    let dark_end = DialogueNode::new(
        "choice_dark".to_string(),
        "Head Shaman".to_string(),
        "...I see. The Dark Spirit is a heavy burden. Remember who you are, lest you lose yourself to the shadows.".to_string(),
    ).as_end_node();
    tree.add_node(dark_end);

    tree
}
