/// Monster spawning systems for dev mode and gameplay
use bevy::prelude::*;
use bevy_shaman_core::components::GridPosition;
use crate::components::*;
use crate::resources::MonsterTemplateDB;

// ============================================================================
// DEV MODE TEST SPAWNING
// ============================================================================

/// Spawn a test boss monster near the player for testing combat
pub fn spawn_test_boss(
    mut commands: Commands,
    template_db: Res<MonsterTemplateDB>,
    player_query: Query<&GridPosition, With<bevy_shaman_core::components::Player>>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        // Use the chaos_hound template as a boss (aggressive, high attack)
        let boss_template = template_db.get("chaos_hound")
            .or_else(|| template_db.get("corrupt_shade"))
            .or_else(|| {
                // Fallback if templates aren't loaded yet
                warn!("DEV: No boss templates found, using default");
                None
            });

        let (monster_id, stats, affinity, default_state) = if let Some(template) = boss_template {
            (
                template.id.clone(),
                template.base_stats,
                template.affinity_profile.clone(),
                template.default_state,
            )
        } else {
            // Default boss stats
            (
                "test_boss".to_string(),
                MonsterStats {
                    attack: 20.0,
                    defense: 10.0,
                    speed: 8.0,
                    spirit_affinity: 0.3,
                },
                MusicAffinityProfile {
                    prefers_calm: 0.1,
                    prefers_aggressive: 0.9,
                    corruption_resistance: 0.3,
                    trust_level: 0.0,
                },
                StateType::Chaos,
            )
        };

        // Spawn boss near player (3 tiles away)
        let spawn_x = player_pos.x + 3;
        let spawn_y = player_pos.y;

        commands.spawn((
            MonsterId(monster_id.clone()),
            MonsterState {
                state: default_state,
                stability_meter: 0.2, // Low stability = aggressive
                corruption_meter: 0.8, // High corruption
                obedience_meter: 0.0, // Cannot be tamed
                chaos_output: 1.5, // High damage output
            },
            stats,
            affinity,
            AiBehavior {
                behavior_tree_id: "boss_aggressive".to_string(),
                aggression: 0.9,
                flee_threshold: 0.1, // Fights to the death
            },
            AiState::Aggressive,
            bevy_shaman_core::components::Health {
                current: 100.0,
                max: 100.0,
            },
            GridPosition {
                x: spawn_x,
                y: spawn_y,
            },
            Transform::from_xyz(spawn_x as f32 * 32.0, spawn_y as f32 * 32.0, 2.0),
            GlobalTransform::default(),
            Visibility::default(),
            Name::new(format!("Test Boss: {}", monster_id)),
        ));

        info!("DEV: Spawned test boss '{}' at ({}, {})", monster_id, spawn_x, spawn_y);
    } else {
        warn!("DEV: Cannot spawn test boss - player not found");
    }
}
