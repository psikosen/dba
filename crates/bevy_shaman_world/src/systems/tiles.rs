use bevy::prelude::*;
use crate::components::TileCorruption;

/// Updates tile visual appearance based on corruption level
pub fn update_tile_visuals(
    mut tiles: Query<(&TileCorruption, &mut Sprite), Changed<TileCorruption>>,
) {
    for (corruption, mut sprite) in tiles.iter_mut() {
        if corruption.purified {
            sprite.color = Color::srgb(0.9, 0.9, 1.0); // Pure blue-white
        } else if corruption.is_corrupt() {
            let base_color = corruption.corruption_color();
            let intensity = corruption.level;
            sprite.color = base_color.with_alpha(intensity);
        } else {
            sprite.color = Color::WHITE;
        }
    }
}
