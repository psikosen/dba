use crate::components::{VesselMutation, WorldSpiritType};
use bevy::prelude::*;

// ============================================================================
// PRIME VESSEL EVENTS
// ============================================================================

/// Fired when the Prime Vessel absorbs a spirit
#[derive(Event, Debug, Clone)]
pub struct VesselAbsorbedSpirit {
    pub spirit_entity: Entity,
    pub spirit_type: WorldSpiritType,
    pub power_gained: f32,
    pub vessel_new_power: f32,
}

/// Fired when the Prime Vessel sheds a Lesser Self
#[derive(Event, Debug, Clone)]
pub struct VesselShedLesserSelf {
    pub lesser_self_entity: Entity,
    pub origin_tier: u8,
    pub power_level: f32,
    pub mutations: Vec<VesselMutation>,
    pub position: (i32, i32),
}

/// Fired when the Prime Vessel evolves to a new tier
#[derive(Event, Debug, Clone)]
pub struct VesselEvolved {
    pub new_tier: u8,
    pub new_power: f32,
    pub new_mutation: Option<VesselMutation>,
}

/// Fired when the Prime Vessel loses power due to Metabolic Decay
#[derive(Event, Debug, Clone)]
pub struct VesselDecayProcessed {
    pub power_lost: f32,
    pub spirits_decayed: u32,
    pub new_power: f32,
}

/// Fired when the Prime Vessel is defeated
#[derive(Event, Debug, Clone)]
pub struct VesselDefeated {
    pub defeated_by_player: bool,
    pub final_tier: u8,
    pub final_power: f32,
    pub lesser_selves_remaining: u32,
}

/// Fired when the Prime Vessel is resurrected via ritual
#[derive(Event, Debug, Clone)]
pub struct VesselResurrected {
    pub starting_power: f32,
    pub starting_tier: u8,
}

// ============================================================================
// CORRUPTION INDEX EVENTS
// ============================================================================

/// Fired when the Global Corruption Index changes significantly
#[derive(Event, Debug, Clone)]
pub struct CorruptionIndexChanged {
    pub old_percentage: f32,
    pub new_percentage: f32,
    pub old_danger_level: u8,
    pub new_danger_level: u8,
}

/// Fired when the world enters Total Collapse (Zero-Point)
#[derive(Event, Debug, Clone)]
pub struct TotalCollapseTriggered {
    pub spirits_consumed_by_vessel: u32,
    pub spirits_consumed_by_player: u32,
    pub player_contribution_percentage: f32,
}

/// Fired when the Corruption Index UI is revealed
#[derive(Event, Debug, Clone)]
pub struct CorruptionIndexRevealed {
    pub current_percentage: f32,
    pub spirits_remaining: u32,
}

// ============================================================================
// SPIRIT EVENTS
// ============================================================================

/// Fired when a world spirit is spawned
#[derive(Event, Debug, Clone)]
pub struct WorldSpiritSpawned {
    pub entity: Entity,
    pub spirit_type: WorldSpiritType,
    pub power: f32,
    pub position: (i32, i32),
}

/// Fired when a spirit is absorbed (by vessel or player)
#[derive(Event, Debug, Clone)]
pub struct SpiritAbsorbed {
    pub spirit_entity: Entity,
    pub absorber_entity: Entity,
    pub by_vessel: bool,
    pub power: f32,
}

/// Fired when a spirit is purified by the player
#[derive(Event, Debug, Clone)]
pub struct SpiritPurified {
    pub spirit_entity: Entity,
    pub power: f32,
}

/// Fired when a trapped spirit is freed from a defeated entity
#[derive(Event, Debug, Clone)]
pub struct SpiritFreed {
    pub spirit_entity: Entity,
    pub freed_from: Entity,
    pub power: f32,
}

// ============================================================================
// LESSER SELF EVENTS
// ============================================================================

/// Fired when a Lesser Self is encountered by the player
#[derive(Event, Debug, Clone)]
pub struct LesserSelfEncountered {
    pub lesser_self_entity: Entity,
    pub generation: u32,
    pub power_level: f32,
}

/// Fired when a Lesser Self is defeated
#[derive(Event, Debug, Clone)]
pub struct LesserSelfDefeated {
    pub lesser_self_entity: Entity,
    pub generation: u32,
    pub spirits_freed: u32,
}

// ============================================================================
// SOUL ALIGNMENT EVENTS
// ============================================================================

/// Fired when an entity's soul alignment shifts due to corruption
#[derive(Event, Debug, Clone)]
pub struct SoulAlignmentShifted {
    pub entity: Entity,
    pub old_alignment: f32,
    pub new_alignment: f32,
    pub became_hostile: bool,
}

/// Fired when an entity is chaos-locked during Total Collapse
#[derive(Event, Debug, Clone)]
pub struct EntityChaosLocked {
    pub entity: Entity,
    pub entity_type: String,
}
