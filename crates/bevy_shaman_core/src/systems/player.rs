use crate::components::*;
use crate::systems::assets::PlayerSpriteHandle;
use bevy::prelude::*;

/// Marker to ensure player spawns only once
#[derive(Resource, Default)]
pub struct PlayerSpawned(pub bool);

/// Spawn the player entity with all required components
pub fn spawn_player(
    mut commands: Commands,
    mut spawned: ResMut<PlayerSpawned>,
    sprite_handle: Res<PlayerSpriteHandle>,
    loading_from_save: Option<Res<crate::resources::LoadingFromSave>>,
) {
    if spawned.0 {
        return;
    }

    // Don't spawn player if we're loading from a save
    if let Some(loading) = loading_from_save {
        if loading.is_loading {
            info!("Skipping player spawn - loading from save");
            return;
        }
    }

    info!("Spawning player entity...");

    commands.spawn((
        // Player marker
        Player,
        // Position components
        GridPosition { x: 10, y: 10 },
        Transform::from_xyz(320.0, 320.0, 10.0), // 10 * 32 pixel tile size
        // Visual components
        Sprite {
            image: sprite_handle.0.clone(),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        // Stats
        Health::new(100.0),
        Spirit::new(100.0),
        Stamina::new(100.0),
        // Gameplay components
        MovementQueue::default(),
        BlocksMovement,
        CameraTarget,
        // Progression components
        bevy_shaman_combat::components::WeaponEnhancement::new(),
        bevy_shaman_combat::components::EquippedWeapon::default(),
        bevy_shaman_combat::systems::skill_tree::SkillTree::new(),
        // Collider for grid occupancy
        GlobalTransform::default(),
        Visibility::default(),
    ));

    spawned.0 = true;
    info!("Player spawned at (10, 10)");
}

/// Spawn the main camera
pub fn spawn_camera(mut commands: Commands, cameras: Query<Entity, With<Camera>>) {
    if !cameras.is_empty() {
        return;
    }

    info!("Spawning main camera...");

    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.1, 0.1, 0.15)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 999.9),
    ));
}
