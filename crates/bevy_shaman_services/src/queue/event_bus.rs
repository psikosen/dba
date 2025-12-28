use super::rabbitmq::RabbitMqClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Event-driven architecture bus using RabbitMQ
pub struct EventBus {
    client: RabbitMqClient,
    exchange_name: String,
}

impl EventBus {
    /// Create a new event bus
    pub async fn new(client: RabbitMqClient, exchange_name: String) -> Result<Self> {
        // Declare a topic exchange for flexible routing
        client.declare_exchange(&exchange_name, "topic", true).await?;

        Ok(Self {
            client,
            exchange_name,
        })
    }

    /// Publish a game event
    pub async fn publish_event(&self, event: GameEvent) -> Result<String> {
        let event_id = uuid::Uuid::new_v4().to_string();
        let mut event_with_id = event;
        event_with_id.event_id = event_id.clone();

        let routing_key = event_with_id.routing_key();
        let message = serde_json::to_vec(&event_with_id)?;

        self.client
            .publish_to_exchange(&self.exchange_name, &routing_key, &message)
            .await?;

        debug!("Published event: {} (type: {})", event_id, routing_key);
        Ok(event_id)
    }

    /// Subscribe to events with a specific pattern
    pub async fn subscribe(&self, queue_name: &str, routing_pattern: &str) -> Result<()> {
        // Declare a queue for the subscriber
        self.client.declare_queue(queue_name, true).await?;

        // Bind the queue to the exchange with the routing pattern
        self.client
            .bind_queue(queue_name, &self.exchange_name, routing_pattern)
            .await?;

        info!(
            "Subscribed queue '{}' to pattern '{}'",
            queue_name, routing_pattern
        );
        Ok(())
    }

    /// Get the exchange name
    pub fn exchange_name(&self) -> &str {
        &self.exchange_name
    }
}

/// Game event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    pub event_id: String,
    pub event_type: EventType,
    pub payload: EventPayload,
    pub timestamp: u64,
    pub source: String,
}

impl GameEvent {
    pub fn new(event_type: EventType, payload: EventPayload, source: String) -> Self {
        Self {
            event_id: String::new(), // Will be set when published
            event_type,
            payload,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            source,
        }
    }

    /// Generate routing key based on event type
    pub fn routing_key(&self) -> String {
        match &self.event_type {
            EventType::PlayerAction => "game.player.action",
            EventType::CombatEvent => "game.combat.event",
            EventType::ItemEvent => "game.item.event",
            EventType::DungeonEvent => "game.dungeon.event",
            EventType::QuestEvent => "game.quest.event",
            EventType::AchievementUnlocked => "game.achievement.unlocked",
            EventType::LevelUp => "game.player.levelup",
            EventType::SessionStart => "game.session.start",
            EventType::SessionEnd => "game.session.end",
            EventType::Error => "game.error",
        }
        .to_string()
    }
}

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    PlayerAction,
    CombatEvent,
    ItemEvent,
    DungeonEvent,
    QuestEvent,
    AchievementUnlocked,
    LevelUp,
    SessionStart,
    SessionEnd,
    Error,
}

/// Event payload data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    PlayerAction {
        player_id: String,
        action: String,
        details: std::collections::HashMap<String, String>,
    },
    Combat {
        combat_id: String,
        attacker: String,
        defender: String,
        damage: f32,
        outcome: String,
    },
    Item {
        player_id: String,
        item_id: String,
        action: String, // "acquired", "used", "dropped"
        quantity: u32,
    },
    Dungeon {
        dungeon_id: String,
        level: u32,
        event_type: String, // "entered", "completed", "failed"
        player_id: String,
    },
    Quest {
        quest_id: String,
        player_id: String,
        status: String, // "started", "completed", "failed"
    },
    Achievement {
        achievement_id: String,
        player_id: String,
        name: String,
    },
    LevelUp {
        player_id: String,
        old_level: u32,
        new_level: u32,
        stats_gained: std::collections::HashMap<String, f32>,
    },
    Session {
        session_id: String,
        player_id: String,
    },
    Error {
        error_type: String,
        message: String,
        context: std::collections::HashMap<String, String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_event_routing_key() {
        let event = GameEvent::new(
            EventType::PlayerAction,
            EventPayload::PlayerAction {
                player_id: "player123".to_string(),
                action: "move".to_string(),
                details: std::collections::HashMap::new(),
            },
            "game_server".to_string(),
        );

        assert_eq!(event.routing_key(), "game.player.action");
    }

    #[test]
    fn test_combat_event_routing_key() {
        let event = GameEvent::new(
            EventType::CombatEvent,
            EventPayload::Combat {
                combat_id: "combat123".to_string(),
                attacker: "player1".to_string(),
                defender: "monster1".to_string(),
                damage: 50.0,
                outcome: "hit".to_string(),
            },
            "combat_system".to_string(),
        );

        assert_eq!(event.routing_key(), "game.combat.event");
    }
}
