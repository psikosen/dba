use bevy::prelude::*;
use crate::components::SpriteAnimation;

/// Updates sprite animations based on frame timers
pub fn update_sprite_animations(
    time: Res<Time>,
    mut animators: Query<(&mut SpriteAnimation, &mut Sprite)>,
) {
    for (mut anim, mut sprite) in animators.iter_mut() {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished() {
            anim.current_frame += 1;

            if anim.current_frame >= anim.frame_count {
                if anim.looping {
                    anim.current_frame = 0;
                } else {
                    anim.current_frame = anim.frame_count - 1;
                }
            }

            // Update sprite rect (assumes horizontal sprite sheet)
            if let Some(mut rect) = sprite.rect {
                rect.min.x = anim.current_frame as f32 * rect.width();
                rect.max.x = rect.min.x + rect.width();
                sprite.rect = Some(rect);
            }
        }
    }
}
