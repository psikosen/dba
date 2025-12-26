use bevy::prelude::*;
use crate::components::CameraTarget;

const CAMERA_FOLLOW_SPEED: f32 = 5.0;

/// Camera follows CameraTarget entity with smooth interpolation
pub fn follow_player(
    time: Res<Time>,
    target: Query<&Transform, (With<CameraTarget>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let Ok(target_transform) = target.get_single() else {
        return;
    };

    let Ok(mut camera_transform) = camera.get_single_mut() else {
        return;
    };

    let target_pos = target_transform.translation;
    let current_pos = camera_transform.translation;

    camera_transform.translation = current_pos.lerp(
        Vec3::new(target_pos.x, target_pos.y, current_pos.z),
        CAMERA_FOLLOW_SPEED * time.delta_secs(),
    );
}
