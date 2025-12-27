use bevy::prelude::*;

/// Event triggered when player wants to interact with an NPC
#[derive(Event)]
pub struct DialogueRequested {
    pub npc_entity: Entity,
    pub player_entity: Entity,
}

/// Event triggered when player entity dies
#[derive(Event)]
pub struct PlayerDied {
    pub player_entity: Entity,
}

/// Event triggered when an entity (monster or player) dies
#[derive(Event)]
pub struct EntityDied {
    pub entity: Entity,
    pub was_player: bool,
}

/// Event triggered when a hit lands in combat
#[derive(Event, Debug, Clone)]
pub struct HitLanded {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: f32,
}

/// Event triggered when a quest is completed
#[derive(Event, Debug, Clone)]
pub struct QuestCompleted {
    pub quest_id: String,
}
