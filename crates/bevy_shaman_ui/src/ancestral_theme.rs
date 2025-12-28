/// Ancestral African aesthetic theme constants and helper functions
/// Implements the "Ancestral Legacy" design philosophy with earth tones,
/// handcrafted materials, and organic textures.

use bevy::prelude::*;

// ============================================================================
// COLOR PALETTE - Earth Tones and Natural Dyes
// ============================================================================

/// Primary wood tones - deep carved ebony and mahogany
pub mod wood {
    use super::*;

    /// Deep ebony - almost black with brown undertones
    pub const EBONY: Color = Color::srgb(0.10, 0.08, 0.06);

    /// Rich mahogany
    pub const MAHOGANY: Color = Color::srgb(0.25, 0.15, 0.10);

    /// Lighter carved wood
    pub const CARVED_LIGHT: Color = Color::srgb(0.35, 0.25, 0.18);

    /// Wood grain highlight
    pub const GRAIN_HIGHLIGHT: Color = Color::srgb(0.45, 0.35, 0.25);
}

/// Metallic tones - hammered bronze, copper, and gold
pub mod metal {
    use super::*;

    /// Hammered bronze base
    pub const BRONZE: Color = Color::srgb(0.80, 0.50, 0.20);

    /// Bronze with patina (darker, greenish)
    pub const BRONZE_PATINA: Color = Color::srgb(0.40, 0.35, 0.25);

    /// Copper shine
    pub const COPPER: Color = Color::srgb(0.72, 0.45, 0.20);

    /// Gold accent
    pub const GOLD: Color = Color::srgb(0.85, 0.65, 0.13);

    /// Gold highlight
    pub const GOLD_SHINE: Color = Color::srgb(0.95, 0.80, 0.30);
}

/// Earth and clay tones
pub mod earth {
    use super::*;

    /// Ochre red
    pub const OCHRE_RED: Color = Color::srgb(0.64, 0.27, 0.15);

    /// Burnt orange
    pub const BURNT_ORANGE: Color = Color::srgb(0.80, 0.35, 0.10);

    /// Rich soil brown
    pub const SOIL_BROWN: Color = Color::srgb(0.30, 0.20, 0.12);

    /// Charcoal grey
    pub const CHARCOAL: Color = Color::srgb(0.15, 0.15, 0.15);

    /// Terracotta
    pub const TERRACOTTA: Color = Color::srgb(0.71, 0.40, 0.28);
}

/// Natural dye accents
pub mod dye {
    use super::*;

    /// Indigo blue - deep and rich
    pub const INDIGO: Color = Color::srgb(0.18, 0.21, 0.48);

    /// Forest green
    pub const FOREST_GREEN: Color = Color::srgb(0.13, 0.33, 0.13);

    /// Turmeric yellow (vibrant)
    pub const TURMERIC: Color = Color::srgb(0.95, 0.77, 0.15);

    /// Deep red ochre (for health/blood)
    pub const RED_OCHRE: Color = Color::srgb(0.70, 0.13, 0.13);

    /// Blood red (for danger/corruption)
    pub const BLOOD_RED: Color = Color::srgb(0.85, 0.10, 0.10);
}

// ============================================================================
// PUBLIC RE-EXPORTS - For convenience in UI components
// ============================================================================

// Wood tones
pub use wood::{EBONY, MAHOGANY, CARVED_LIGHT, GRAIN_HIGHLIGHT};

// Metal tones
pub use metal::{BRONZE, BRONZE_PATINA, COPPER, GOLD, GOLD_SHINE};

// Earth tones
pub use earth::{OCHRE_RED, BURNT_ORANGE, SOIL_BROWN, CHARCOAL, TERRACOTTA};

// Natural dyes
pub use dye::{INDIGO, FOREST_GREEN, TURMERIC, RED_OCHRE, BLOOD_RED};

// Bone tones
pub use bone::{IVORY, AGED_BONE, BONE_SHADOW};

// Fabric tones
pub use fabric::{LEATHER, LEATHER_TOOLED, WOVEN_DARK, MUD_CLOTH};

/// Ivory and bone tones
pub mod bone {
    use super::*;

    /// Polished ivory
    pub const IVORY: Color = Color::srgb(0.95, 0.93, 0.84);

    /// Aged bone
    pub const AGED_BONE: Color = Color::srgb(0.88, 0.84, 0.72);

    /// Bone shadow
    pub const BONE_SHADOW: Color = Color::srgb(0.75, 0.70, 0.60);
}

/// Fabric and leather tones
pub mod fabric {
    use super::*;

    /// Tanned leather base
    pub const LEATHER: Color = Color::srgb(0.55, 0.38, 0.24);

    /// Leather highlight (tooled areas)
    pub const LEATHER_TOOLED: Color = Color::srgb(0.65, 0.45, 0.30);

    /// Woven fabric (kente-style)
    pub const WOVEN_DARK: Color = Color::srgb(0.25, 0.18, 0.12);

    /// Mud cloth pattern
    pub const MUD_CLOTH: Color = Color::srgb(0.35, 0.28, 0.20);
}

// ============================================================================
// UI DIMENSIONS - Based on handcrafted proportions
// ============================================================================

/// Standard spacing units (avoiding perfectly digital spacing)
pub const SPACING_SMALL: f32 = 7.0;   // Slightly irregular
pub const SPACING_MEDIUM: f32 = 13.0;
pub const SPACING_LARGE: f32 = 21.0;
pub const SPACING_XLARGE: f32 = 34.0;

/// Border widths
pub const BORDER_THIN: f32 = 2.0;
pub const BORDER_MEDIUM: f32 = 4.0;
pub const BORDER_THICK: f32 = 6.0;
pub const BORDER_CARVED: f32 = 8.0;

/// Slot sizes (for Mancala-style pits and gear slots)
pub const SLOT_SMALL: f32 = 48.0;
pub const SLOT_MEDIUM: f32 = 64.0;
pub const SLOT_LARGE: f32 = 80.0;

/// Portrait and icon sizes
pub const PORTRAIT_SIZE: f32 = 96.0;
pub const ICON_SMALL: f32 = 24.0;
pub const ICON_MEDIUM: f32 = 32.0;
pub const ICON_LARGE: f32 = 48.0;

// ============================================================================
// TEXTURE HELPERS
// ============================================================================

/// Creates a glossy border effect by layering colors
pub fn create_glossy_border(base_color: Color) -> [Color; 4] {
    // Returns: [top_highlight, right_shadow, bottom_shadow, left_highlight]
    let highlight = Color::srgb(
        (base_color.to_srgba().red + 0.2).min(1.0),
        (base_color.to_srgba().green + 0.2).min(1.0),
        (base_color.to_srgba().blue + 0.2).min(1.0),
    );

    let shadow = Color::srgb(
        (base_color.to_srgba().red * 0.6),
        (base_color.to_srgba().green * 0.6),
        (base_color.to_srgba().blue * 0.6),
    );

    [highlight, shadow, shadow, highlight]
}

/// Creates a wood grain texture variation
pub fn wood_grain_color(base: Color, variation: f32) -> Color {
    let vary = variation * 0.15; // 15% max variation
    Color::srgb(
        (base.to_srgba().red + vary).max(0.0).min(1.0),
        (base.to_srgba().green + vary).max(0.0).min(1.0),
        (base.to_srgba().blue + vary).max(0.0).min(1.0),
    )
}

/// Creates a hammered metal texture variation
pub fn hammered_metal_color(base: Color, hammer_intensity: f32) -> Color {
    let intensity = hammer_intensity * 0.2;
    Color::srgb(
        (base.to_srgba().red + intensity).max(0.0).min(1.0),
        (base.to_srgba().green + intensity).max(0.0).min(1.0),
        (base.to_srgba().blue + intensity).max(0.0).min(1.0),
    )
}

// ============================================================================
// UI COMPONENT STYLES
// ============================================================================

/// Style for a carved wooden panel
pub fn carved_wood_panel() -> (Color, f32) {
    (wood::MAHOGANY, BORDER_CARVED)
}

/// Style for a bronze-framed element
pub fn bronze_frame() -> (Color, f32) {
    (metal::BRONZE, BORDER_THICK)
}

/// Style for a leather-bound element
pub fn leather_binding() -> (Color, f32) {
    (fabric::LEATHER, BORDER_MEDIUM)
}

/// Style for ivory/bone inlay
pub fn bone_inlay() -> (Color, f32) {
    (bone::IVORY, BORDER_THIN)
}

// ============================================================================
// ANIMATION CURVES - Organic, not digital
// ============================================================================

/// Ease out curve for organic feel (mimics natural settling)
pub fn ease_out_organic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3) // Cubic ease-out
}

/// Pulse animation for glowing elements (like spirit energy)
pub fn spirit_pulse(time: f32, frequency: f32) -> f32 {
    ((time * frequency).sin() + 1.0) * 0.5
}

/// Breathing animation for health bars
pub fn health_breath(time: f32) -> f32 {
    ((time * 0.5).sin() * 0.03) + 1.0 // Subtle 3% breathing
}
