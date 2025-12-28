use crate::components::{
    AiRole, ConversationHistory, DialogueContext, EmotionalState, LlmAi, LlmQueryQueue,
    PersonalityTraits, QueuedResponse,
};
use crate::systems::npc_dialogue::{NpcDialogueResponse, PlayerDialogueRequest};
/// Spirit Guide System
/// Handles mystical encounters with spirit guides who provide wisdom and guidance
use bevy::prelude::*;
use bevy_shaman_core::components::{GridPosition, Player, Spirit};

// ============================================================================
// COMPONENTS
// ============================================================================

/// Marks an entity as a spirit guide
#[derive(Component)]
pub struct SpiritGuide {
    pub guide_type: SpiritGuideType,
    pub manifestation_state: ManifestationState,
    pub wisdom_level: f32, // 0.0 - 1.0
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpiritGuideType {
    Ancestor,  // Ancestral spirit offering guidance
    Nature,    // Nature spirit connected to the land
    Cosmic,    // Cosmic/celestial entity
    Trickster, // Trickster spirit with cryptic wisdom
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ManifestationState {
    Hidden,     // Not visible
    Appearing,  // Fading in
    Manifested, // Fully present
    Fading,     // Fading out
}

/// Component for spirit guide encounter zones (mystical locations)
#[derive(Component)]
pub struct SpiritEncounterZone {
    pub radius: f32,
    pub required_spirit_level: f32, // Minimum spirit required to see guide
    pub active: bool,
}

impl Default for SpiritEncounterZone {
    fn default() -> Self {
        Self {
            radius: 3.0,
            required_spirit_level: 0.5,
            active: true,
        }
    }
}

// ============================================================================
// SPIRIT GUIDE SPAWNING
// ============================================================================

/// Spawn a spirit guide at a mystical location
pub fn spawn_spirit_guide(
    commands: &mut Commands,
    position: GridPosition,
    guide_type: SpiritGuideType,
    name: String,
) -> Entity {
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

    commands
        .spawn((
            Name::new(name.clone()),
            position,
            SpiritGuide {
                guide_type,
                manifestation_state: ManifestationState::Hidden,
                wisdom_level: personality.wisdom,
            },
            LlmAi {
                character_name: name,
                role: AiRole::SpiritGuide,
                personality,
                emotional_state: EmotionalState::Calm,
                combat_stance: crate::components::CombatStance::Defensive,
            },
            LlmQueryQueue::new(5),
            ConversationHistory::new(15),
            SpiritEncounterZone::default(),
            // Visual components would go here (sprite, glow effect, etc.)
        ))
        .id()
}

// ============================================================================
// MANIFESTATION SYSTEM
// ============================================================================

/// Handle spirit guide manifestation based on player proximity and spirit level
pub fn handle_spirit_manifestation(
    mut guide_query: Query<(
        Entity,
        &GridPosition,
        &mut SpiritGuide,
        &SpiritEncounterZone,
    )>,
    player_query: Query<(&GridPosition, &Spirit), With<Player>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let Ok((player_pos, player_spirit)) = player_query.get_single() else {
        return;
    };

    let spirit_percent = player_spirit.current / player_spirit.max;

    for (guide_entity, guide_pos, mut guide, zone) in guide_query.iter_mut() {
        if !zone.active {
            continue;
        }

        let distance = calculate_distance(guide_pos, player_pos);
        let in_range = distance <= zone.radius;
        let has_spirit = spirit_percent >= zone.required_spirit_level;

        match guide.manifestation_state {
            ManifestationState::Hidden => {
                if in_range && has_spirit {
                    // Start manifesting
                    guide.manifestation_state = ManifestationState::Appearing;
                    info!("Spirit guide beginning to manifest...");

                    // Trigger visual effects
                    commands.trigger_targets(
                        SpiritManifestationEffect { appearing: true },
                        guide_entity,
                    );
                }
            }
            ManifestationState::Appearing => {
                // Complete manifestation after delay
                // (In real implementation, use a timer component)
                guide.manifestation_state = ManifestationState::Manifested;
                info!("Spirit guide fully manifested!");

                // Trigger greeting
                commands.trigger_targets(SpiritGreetingEvent, guide_entity);
            }
            ManifestationState::Manifested => {
                if !in_range || !has_spirit {
                    // Start fading
                    guide.manifestation_state = ManifestationState::Fading;
                    info!("Spirit guide beginning to fade...");

                    commands.trigger_targets(
                        SpiritManifestationEffect { appearing: false },
                        guide_entity,
                    );
                }
            }
            ManifestationState::Fading => {
                // Complete fading
                guide.manifestation_state = ManifestationState::Hidden;
                info!("Spirit guide has faded away");
            }
        }
    }
}

fn calculate_distance(pos1: &GridPosition, pos2: &GridPosition) -> f32 {
    let dx = (pos1.x - pos2.x) as f32;
    let dy = (pos1.y - pos2.y) as f32;
    (dx * dx + dy * dy).sqrt()
}

// ============================================================================
// SPIRIT GUIDANCE SYSTEM
// ============================================================================

/// Provide wisdom and guidance to player
pub fn provide_spirit_guidance(
    mut guide_query: Query<(Entity, &SpiritGuide, &LlmAi, &mut LlmQueryQueue)>,
    player_query: Query<(Entity, &bevy_shaman_core::components::Health, &Spirit), With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
    mut last_guidance: Local<f64>,
) {
    let current_time = time.elapsed_secs_f64();

    // Guidance cooldown (every 30 seconds)
    if current_time - *last_guidance < 30.0 {
        return;
    }

    let Ok((player_entity, player_health, player_spirit)) = player_query.get_single() else {
        return;
    };

    let health_percent = player_health.current / player_health.max;
    let spirit_percent = player_spirit.current / player_spirit.max;

    // Find manifested spirit guides
    for (guide_entity, spirit_guide, ai, mut queue) in guide_query.iter_mut() {
        if spirit_guide.manifestation_state != ManifestationState::Manifested {
            continue;
        }

        // Determine guidance context
        let guidance_context =
            determine_guidance_context(health_percent, spirit_percent, &spirit_guide.guide_type);

        if let Some(context) = guidance_context {
            info!(
                "Spirit guide {} providing guidance: {:?}",
                ai.character_name, context
            );

            // Add contextual wisdom to queue if not already present
            let wisdom = generate_spirit_wisdom(&spirit_guide.guide_type, &context);

            queue.dialogue_responses.push(QueuedResponse {
                text: wisdom.clone(),
                context: DialogueContext::Custom(context.clone()),
                generated_at: current_time,
            });

            // Trigger dialogue
            commands.trigger(PlayerDialogueRequest {
                player: player_entity,
                npc: guide_entity,
                player_message: Some(context),
            });

            *last_guidance = current_time;
            break; // Only one guide speaks at a time
        }
    }
}

fn determine_guidance_context(
    health_percent: f32,
    spirit_percent: f32,
    guide_type: &SpiritGuideType,
) -> Option<String> {
    match guide_type {
        SpiritGuideType::Ancestor => {
            if health_percent < 0.3 {
                Some("low_health_wisdom".to_string())
            } else if spirit_percent < 0.3 {
                Some("low_spirit_wisdom".to_string())
            } else {
                Some("ancestral_blessing".to_string())
            }
        }
        SpiritGuideType::Nature => {
            if spirit_percent < 0.5 {
                Some("commune_with_nature".to_string())
            } else {
                Some("nature_harmony".to_string())
            }
        }
        SpiritGuideType::Cosmic => Some("cosmic_perspective".to_string()),
        SpiritGuideType::Trickster => Some("cryptic_riddle".to_string()),
    }
}

fn generate_spirit_wisdom(guide_type: &SpiritGuideType, context: &str) -> String {
    match (guide_type, context) {
        (SpiritGuideType::Ancestor, "low_health_wisdom") => {
            "Your ancestors walked through fire and emerged stronger. Rest, child, and let the earth heal you.".to_string()
        }
        (SpiritGuideType::Ancestor, "low_spirit_wisdom") => {
            "The spirits grow distant when we forget to honor them. Play your instrument, offer your prayers.".to_string()
        }
        (SpiritGuideType::Ancestor, "ancestral_blessing") => {
            "You carry the strength of seven generations within you. Walk with purpose.".to_string()
        }
        (SpiritGuideType::Nature, "commune_with_nature") => {
            "The trees whisper secrets to those who listen. Find peace among the green.".to_string()
        }
        (SpiritGuideType::Nature, "nature_harmony") => {
            "You are in harmony with the land. The spirits of earth and sky smile upon you.".to_string()
        }
        (SpiritGuideType::Cosmic, "cosmic_perspective") => {
            "The stars have seen empires rise and fall. Your struggles are but a moment in eternity, yet infinitely meaningful.".to_string()
        }
        (SpiritGuideType::Trickster, "cryptic_riddle") => {
            "What flies without wings, cries without eyes, and moves without legs? Think, shaman!".to_string()
        }
        _ => {
            "The spirits watch over you, young shaman.".to_string()
        }
    }
}

// ============================================================================
// SPIRIT QUESTS
// ============================================================================

/// Event for spirit guide offering a quest or trial
#[derive(Event)]
pub struct SpiritQuestOffered {
    pub guide_entity: Entity,
    pub quest_id: String,
    pub quest_description: String,
}

/// Handle spirit guides offering mystical quests
pub fn offer_spirit_quests(
    mut guide_query: Query<(Entity, &SpiritGuide, &LlmAi), Changed<SpiritGuide>>,
    mut quest_events: EventWriter<SpiritQuestOffered>,
) {
    for (guide_entity, spirit_guide, ai) in guide_query.iter_mut() {
        // Only offer quests when first manifested
        if spirit_guide.manifestation_state == ManifestationState::Manifested {
            let quest_id = format!(
                "spirit_quest_{}",
                ai.character_name.to_lowercase().replace(" ", "_")
            );
            let quest_description = match spirit_guide.guide_type {
                SpiritGuideType::Ancestor => {
                    "Retrieve the sacred drum of your ancestors from the corrupted temple."
                        .to_string()
                }
                SpiritGuideType::Nature => {
                    "Purify the three sacred groves that have been tainted by corruption."
                        .to_string()
                }
                SpiritGuideType::Cosmic => {
                    "Align the celestial stones under the light of the full moon.".to_string()
                }
                SpiritGuideType::Trickster => {
                    "Find the three hidden symbols scattered across the land.".to_string()
                }
            };

            info!(
                "Spirit guide {} offering quest: {}",
                ai.character_name, quest_id
            );

            quest_events.send(SpiritQuestOffered {
                guide_entity,
                quest_id,
                quest_description,
            });
        }
    }
}

// ============================================================================
// VISUAL EFFECTS
// ============================================================================

#[derive(Event)]
pub struct SpiritManifestationEffect {
    pub appearing: bool,
}

#[derive(Event)]
pub struct SpiritGreetingEvent;

/// Apply visual effects for spirit manifestation
pub fn apply_spirit_visual_effects(
    mut effect_events: EventReader<SpiritManifestationEffect>,
    guide_query: Query<&SpiritGuide>,
) {
    for event in effect_events.read() {
        if event.appearing {
            info!("Playing spirit manifestation effect (appearing)");
            // Spawn particles, glow, ethereal sounds, etc.
        } else {
            info!("Playing spirit manifestation effect (fading)");
            // Fade out particles, dim glow, etc.
        }
    }
}

// ============================================================================
// BLESSING SYSTEM
// ============================================================================

#[derive(Component)]
pub struct SpiritBlessing {
    pub blessing_type: BlessingType,
    pub duration: f32,
    pub strength: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlessingType {
    Protection, // Damage reduction
    Wisdom,     // XP gain boost
    Clarity,    // Spirit regen boost
    Strength,   // Damage boost
}

/// Apply spirit blessings to player
pub fn apply_spirit_blessings(
    mut player_query: Query<Entity, With<Player>>,
    blessing_query: Query<&SpiritBlessing>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let Ok(player_entity) = player_query.get_single_mut() else {
        return;
    };

    // Apply active blessings (would integrate with buff system)
    for blessing in blessing_query.iter() {
        info!(
            "Active spirit blessing: {:?} (strength: {:.1}, duration: {:.1}s)",
            blessing.blessing_type, blessing.strength, blessing.duration
        );
    }
}
