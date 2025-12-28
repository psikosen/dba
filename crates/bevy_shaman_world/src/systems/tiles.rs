use crate::components::{CorruptionType, TileCorruption};
use bevy::prelude::*;

/// Updates tile visual appearance based on corruption level and tier
/// Each corruption type has 4 visual tiers:
/// - Tier 1: Light corruption (25%) - subtle overlay
/// - Tier 2: Moderate corruption (50%) - noticeable effect
/// - Tier 3: Heavy corruption (75%) - strong effect
/// - Tier 4: Severe corruption (100%) - maximum intensity
pub fn update_tile_visuals(
    mut tiles: Query<(&TileCorruption, &mut Sprite), Changed<TileCorruption>>,
) {
    for (corruption, mut sprite) in tiles.iter_mut() {
        if corruption.purified {
            // Purified tiles have a holy glow
            sprite.color = Color::srgb(0.9, 0.95, 1.0);
        } else if corruption.corruption_type == CorruptionType::None {
            // Pure, uncorrupted tile
            sprite.color = Color::WHITE;
        } else {
            // Apply tier-based corruption visuals
            let tier = corruption.corruption_tier();
            sprite.color = get_tier_color(corruption.corruption_type, tier);
        }
    }
}

/// Get the appropriate color for a corruption type and tier
fn get_tier_color(corruption_type: CorruptionType, tier: u8) -> Color {
    match corruption_type {
        CorruptionType::None => Color::WHITE,
        CorruptionType::Chaos => match tier {
            1 => Color::srgb(0.95, 0.85, 0.85), // Light pink tint
            2 => Color::srgb(0.90, 0.60, 0.60), // Moderate red
            3 => Color::srgb(0.80, 0.35, 0.35), // Strong red
            _ => Color::srgb(0.70, 0.15, 0.15), // Deep crimson
        },
        CorruptionType::Decay => match tier {
            1 => Color::srgb(0.85, 0.90, 0.80), // Light sickly green
            2 => Color::srgb(0.65, 0.75, 0.55), // Moderate decay
            3 => Color::srgb(0.45, 0.60, 0.30), // Strong rot
            _ => Color::srgb(0.30, 0.45, 0.15), // Deep putrid green
        },
        CorruptionType::Void => match tier {
            1 => Color::srgb(0.80, 0.80, 0.90), // Light purple tint
            2 => Color::srgb(0.50, 0.50, 0.70), // Moderate void
            3 => Color::srgb(0.25, 0.25, 0.50), // Strong darkness
            _ => Color::srgb(0.10, 0.10, 0.30), // Absolute void
        },
        CorruptionType::Ancestral => match tier {
            1 => Color::srgb(0.90, 0.85, 0.95), // Light mystical purple
            2 => Color::srgb(0.75, 0.60, 0.85), // Moderate ancestral
            3 => Color::srgb(0.60, 0.40, 0.75), // Strong spirit energy
            _ => Color::srgb(0.50, 0.25, 0.65), // Deep ancestral power
        },
    }
}
