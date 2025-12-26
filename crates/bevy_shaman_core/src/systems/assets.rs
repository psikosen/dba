use bevy::prelude::*;

/// Resource to track asset loading progress
#[derive(Resource, Default)]
pub struct AssetLoadingState {
    pub loaded: bool,
}

/// Marker component for the player sprite
#[derive(Component)]
pub struct PlayerSprite;

/// Marker component for world tile sprites
#[derive(Component)]
pub struct TileSprite;

/// System to load all game assets (core assets only - monsters register their own sprites)
pub fn load_game_assets(
    mut commands: Commands,
    mut loading_state: ResMut<AssetLoadingState>,
    mut images: ResMut<Assets<Image>>,
) {
    if loading_state.loaded {
        return;
    }

    info!("Loading core game assets...");

    // Create placeholder sprites (32x32 colored squares)
    // Player sprite (blue)
    let player_image = create_colored_sprite(32, 32, Color::srgb(0.3, 0.5, 1.0));
    let player_handle = images.add(player_image);
    commands.insert_resource(PlayerSpriteHandle(player_handle));

    // World tile sprites
    let grass_tile = create_colored_sprite(32, 32, Color::srgb(0.2, 0.8, 0.2));
    let forest_tile = create_colored_sprite(32, 32, Color::srgb(0.1, 0.5, 0.1));
    let mountain_tile = create_colored_sprite(32, 32, Color::srgb(0.5, 0.5, 0.5));
    let village_tile = create_colored_sprite(32, 32, Color::srgb(0.6, 0.4, 0.2));
    let corrupted_tile = create_colored_sprite(32, 32, Color::srgb(0.5, 0.1, 0.5));

    commands.insert_resource(TileSpriteHandles {
        grass: images.add(grass_tile),
        forest: images.add(forest_tile),
        mountain: images.add(mountain_tile),
        village: images.add(village_tile),
        corrupted: images.add(corrupted_tile),
    });

    // Item sprites
    let health_potion = create_colored_sprite(16, 16, Color::srgb(1.0, 0.0, 0.0));
    let spirit_orb = create_colored_sprite(16, 16, Color::srgb(0.3, 0.8, 1.0));
    let drum_item = create_colored_sprite(16, 16, Color::srgb(0.6, 0.3, 0.1));

    commands.insert_resource(ItemSpriteHandles {
        health_potion: images.add(health_potion),
        spirit_orb: images.add(spirit_orb),
        drum: images.add(drum_item),
    });

    // Monster sprites will be registered by the monsters plugin
    commands.insert_resource(MonsterSpriteHandles {
        forest_spirit: images.add(create_colored_sprite(32, 32, Color::srgb(0.4, 0.9, 0.4))),
        chaos_hound: images.add(create_colored_sprite(32, 32, Color::srgb(0.9, 0.2, 0.2))),
        corrupt_shade: images.add(create_colored_sprite(32, 32, Color::srgb(0.5, 0.1, 0.5))),
        shadow_beast: images.add(create_colored_sprite(32, 32, Color::srgb(0.2, 0.2, 0.2))),
        spirit_wisp: images.add(create_colored_sprite(32, 32, Color::srgb(0.9, 0.9, 1.0))),
        rock_golem: images.add(create_colored_sprite(32, 32, Color::srgb(0.6, 0.5, 0.4))),
        flame_wraith: images.add(create_colored_sprite(32, 32, Color::srgb(1.0, 0.5, 0.1))),
        void_stalker: images.add(create_colored_sprite(32, 32, Color::srgb(0.1, 0.0, 0.2))),
    });

    loading_state.loaded = true;
    info!("Core asset loading complete!");
}

/// Helper function to create a colored sprite
fn create_colored_sprite(width: u32, height: u32, color: Color) -> Image {
    let size = bevy::render::render_resource::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    let image = Image::new_fill(
        size,
        bevy::render::render_resource::TextureDimension::D2,
        &[
            (color.to_srgba().red * 255.0) as u8,
            (color.to_srgba().green * 255.0) as u8,
            (color.to_srgba().blue * 255.0) as u8,
            (color.to_srgba().alpha * 255.0) as u8,
        ],
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    image
}

/// Resource to hold the player sprite handle
#[derive(Resource)]
pub struct PlayerSpriteHandle(pub Handle<Image>);

/// Resource to hold tile sprite handles
#[derive(Resource)]
pub struct TileSpriteHandles {
    pub grass: Handle<Image>,
    pub forest: Handle<Image>,
    pub mountain: Handle<Image>,
    pub village: Handle<Image>,
    pub corrupted: Handle<Image>,
}

/// Resource to hold item sprite handles
#[derive(Resource)]
pub struct ItemSpriteHandles {
    pub health_potion: Handle<Image>,
    pub spirit_orb: Handle<Image>,
    pub drum: Handle<Image>,
}

/// Resource to hold monster sprite handles
#[derive(Resource)]
pub struct MonsterSpriteHandles {
    pub forest_spirit: Handle<Image>,
    pub chaos_hound: Handle<Image>,
    pub corrupt_shade: Handle<Image>,
    pub shadow_beast: Handle<Image>,
    pub spirit_wisp: Handle<Image>,
    pub rock_golem: Handle<Image>,
    pub flame_wraith: Handle<Image>,
    pub void_stalker: Handle<Image>,
}
