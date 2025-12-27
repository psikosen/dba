use bevy::prelude::*;
use bevy_shaman_core::components::Player;
use crate::components::{BeatPrompt, BeatPromptManager};
use crate::resources::BeatClock;
use crate::systems::events::RhythmInputEvaluated;

/// Resource to hold the beat prompt sprite handle
#[derive(Resource)]
pub struct BeatPromptSprite(pub Handle<Image>);

/// Spawns beat prompts above the player
pub fn spawn_beat_prompts(
    mut commands: Commands,
    mut manager: ResMut<BeatPromptManager>,
    clock: Res<BeatClock>,
    player_query: Query<&Transform, With<Player>>,
    prompt_sprite: Option<Res<BeatPromptSprite>>,
) {
    if !clock.is_playing {
        return;
    }

    // Wait for sprite to be loaded
    let Some(sprite_res) = prompt_sprite else {
        return;
    };

    // Get player position
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    // Clean up old prompts that have passed
    manager.active_prompts.retain(|&beat| beat > clock.current_beat);

    // Calculate how many prompts we need to spawn
    let prompts_needed = manager.max_prompts.saturating_sub(manager.active_prompts.len());

    if prompts_needed == 0 {
        return;
    }

    // Spawn new prompts for upcoming beats
    for i in 0..prompts_needed {
        let target_beat = clock.current_beat + manager.lookahead_beats + i as u32;

        // Skip if we already have a prompt for this beat
        if manager.active_prompts.contains(&target_beat) {
            continue;
        }

        // Calculate time until this beat
        let beats_ahead = (target_beat - clock.current_beat) as f32;
        let time_until_hit = (beats_ahead * clock.beat_duration) - clock.time_in_beat;

        // Starting position above player (higher up based on time until hit)
        let start_y_offset = 80.0; // Pixels above player

        // Horizontal spacing for multiple prompts
        let x_offset = if i == 0 { -20.0 } else { 20.0 };

        // Spawn the prompt entity
        commands.spawn((
            BeatPrompt {
                target_beat,
                time_until_hit,
                consumed: false,
                start_y_offset,
            },
            Sprite {
                image: sprite_res.0.clone(),
                custom_size: Some(Vec2::new(24.0, 24.0)),
                color: Color::srgba(1.0, 1.0, 0.0, 0.8), // Yellow with transparency
                ..default()
            },
            Transform::from_xyz(
                player_transform.translation.x + x_offset,
                player_transform.translation.y + start_y_offset,
                20.0, // Z layer above player
            ),
            GlobalTransform::default(),
            Visibility::default(),
        ));

        manager.active_prompts.push(target_beat);
    }
}

/// Updates beat prompt positions and lifecycle
pub fn update_beat_prompts(
    mut commands: Commands,
    time: Res<Time>,
    clock: Res<BeatClock>,
    player_query: Query<&Transform, With<Player>>,
    mut prompt_query: Query<(Entity, &mut BeatPrompt, &mut Transform, &mut Sprite), Without<Player>>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    for (entity, mut prompt, mut transform, mut sprite) in prompt_query.iter_mut() {
        if prompt.consumed {
            continue;
        }

        // Update time remaining
        prompt.time_until_hit -= time.delta_secs();

        // Calculate progress (0.0 = just spawned, 1.0 = time to hit)
        let total_time = (prompt.target_beat - clock.current_beat + 1) as f32 * clock.beat_duration;
        let progress = 1.0 - (prompt.time_until_hit / total_time).max(0.0).min(1.0);

        // Move prompt down towards player as beat approaches
        let current_y_offset = prompt.start_y_offset * (1.0 - progress);
        transform.translation.y = player_transform.translation.y + current_y_offset;

        // Keep x position relative to player (follow player horizontally)
        let x_offset = if prompt.target_beat % 2 == 0 { -20.0 } else { 20.0 };
        transform.translation.x = player_transform.translation.x + x_offset;

        // Pulse effect based on proximity to beat
        let pulse_scale = 1.0 + (progress * 0.3); // Grows up to 30% larger
        transform.scale = Vec3::splat(pulse_scale);

        // Color shift: Yellow -> Green as it gets closer
        let green_amount = progress;
        sprite.color = Color::srgba(1.0 - green_amount * 0.5, 1.0, green_amount * 0.5, 0.8);

        // Remove prompts that have passed
        if prompt.time_until_hit <= -clock.beat_duration {
            commands.entity(entity).despawn();
        }
    }
}

/// Handles consuming prompts when player hits rhythm input
pub fn consume_beat_prompts(
    mut commands: Commands,
    mut prompt_query: Query<(Entity, &mut BeatPrompt, &mut Sprite)>,
    mut rhythm_events: EventReader<RhythmInputEvaluated>,
    clock: Res<BeatClock>,
) {
    for event in rhythm_events.read() {
        // Find the prompt closest to the current beat
        let mut closest_prompt_entity: Option<Entity> = None;
        let mut closest_distance = f32::MAX;

        for (entity, prompt, _sprite) in prompt_query.iter() {
            if prompt.consumed {
                continue;
            }

            let distance = (prompt.target_beat as i32 - clock.current_beat as i32).abs() as f32;
            if distance < closest_distance {
                closest_distance = distance;
                closest_prompt_entity = Some(entity);
            }
        }

        // Consume the closest prompt
        if let Some(entity) = closest_prompt_entity {
            if let Ok((_, mut prompt, mut sprite)) = prompt_query.get_mut(entity) {
            prompt.consumed = true;

            // Visual feedback based on timing quality
            sprite.color = match event.quality {
                crate::resources::TimingQuality::Perfect => Color::srgb(0.0, 1.0, 0.0), // Green
                crate::resources::TimingQuality::Great => Color::srgb(0.5, 1.0, 0.5),   // Light green
                crate::resources::TimingQuality::Good => Color::srgb(1.0, 1.0, 0.0),    // Yellow
                crate::resources::TimingQuality::Miss => Color::srgb(1.0, 0.0, 0.0),    // Red
            };

            // Despawn after a short delay (show feedback then remove)
            commands.entity(entity).despawn();
            }
        }
    }
}

/// Initialize the beat prompt manager with default settings
pub fn init_beat_prompt_manager(
    mut commands: Commands,
) {
    commands.insert_resource(BeatPromptManager {
        active_prompts: Vec::new(),
        max_prompts: 2,
        lookahead_beats: 4, // Spawn prompts 4 beats ahead
    });
}

/// Creates the beat prompt sprite (placeholder)
pub fn create_beat_prompt_sprite(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // Create a circular-ish sprite for the beat prompt
    let size = bevy::render::render_resource::Extent3d {
        width: 24,
        height: 24,
        depth_or_array_layers: 1,
    };

    // Create a yellow/orange circle sprite
    let color = Color::srgb(1.0, 0.8, 0.0);
    let image = Image::new_fill(
        size,
        bevy::render::render_resource::TextureDimension::D2,
        &[
            (color.to_srgba().red * 255.0) as u8,
            (color.to_srgba().green * 255.0) as u8,
            (color.to_srgba().blue * 255.0) as u8,
            255, // Full opacity
        ],
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    let handle = images.add(image);
    commands.insert_resource(BeatPromptSprite(handle));
}
