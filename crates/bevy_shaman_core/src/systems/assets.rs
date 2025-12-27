use bevy::prelude::*;

/// Resource to track asset loading progress
#[derive(Resource)]
pub struct AssetLoadingState {
    pub loaded: bool,
    pub pending_assets: Vec<UntypedHandle>,
    pub use_placeholders: bool,
}

impl Default for AssetLoadingState {
    fn default() -> Self {
        Self {
            loaded: false,
            pending_assets: Vec::new(),
            use_placeholders: false, // Use real assets (fallback to placeholders if missing)
        }
    }
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
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    if loading_state.loaded {
        return;
    }

    info!("Loading core game assets...");
    let use_placeholders = loading_state.use_placeholders;

    // Load or create player sprite
    let player_handle = if !use_placeholders {
        asset_server.load("sprites/player/player.png")
    } else {
        let player_image = create_colored_sprite(32, 32, Color::srgb(0.3, 0.5, 1.0));
        images.add(player_image)
    };
    commands.insert_resource(PlayerSpriteHandle(player_handle.clone()));

    // Load or create world tile sprites
    let tile_sprites = if !use_placeholders {
        TileSpriteHandles {
            grass: asset_server.load("sprites/tiles/grass.png"),
            forest: asset_server.load("sprites/tiles/forest.png"),
            mountain: asset_server.load("sprites/tiles/mountain.png"),
            village: asset_server.load("sprites/tiles/village.png"),
            corrupted: asset_server.load("sprites/tiles/corrupted.png"),
        }
    } else {
        TileSpriteHandles {
            grass: images.add(create_colored_sprite(32, 32, Color::srgb(0.2, 0.8, 0.2))),
            forest: images.add(create_colored_sprite(32, 32, Color::srgb(0.1, 0.5, 0.1))),
            mountain: images.add(create_colored_sprite(32, 32, Color::srgb(0.5, 0.5, 0.5))),
            village: images.add(create_colored_sprite(32, 32, Color::srgb(0.6, 0.4, 0.2))),
            corrupted: images.add(create_colored_sprite(32, 32, Color::srgb(0.5, 0.1, 0.5))),
        }
    };
    commands.insert_resource(tile_sprites);

    // Load or create item sprites
    let item_sprites = if !use_placeholders {
        ItemSpriteHandles {
            health_potion: asset_server.load("sprites/items/health_potion.png"),
            spirit_orb: asset_server.load("sprites/items/spirit_orb.png"),
            drum: asset_server.load("sprites/items/drum.png"),
        }
    } else {
        ItemSpriteHandles {
            health_potion: images.add(create_colored_sprite(16, 16, Color::srgb(1.0, 0.0, 0.0))),
            spirit_orb: images.add(create_colored_sprite(16, 16, Color::srgb(0.3, 0.8, 1.0))),
            drum: images.add(create_colored_sprite(16, 16, Color::srgb(0.6, 0.3, 0.1))),
        }
    };
    commands.insert_resource(item_sprites);

    // Load or create monster sprites
    let monster_sprites = if !use_placeholders {
        MonsterSpriteHandles {
            forest_spirit: asset_server.load("sprites/monsters/forest_spirit.png"),
            chaos_hound: asset_server.load("sprites/monsters/chaos_hound.png"),
            corrupt_shade: asset_server.load("sprites/monsters/corrupt_shade.png"),
            shadow_beast: asset_server.load("sprites/monsters/shadow_beast.png"),
            spirit_wisp: asset_server.load("sprites/monsters/spirit_wisp.png"),
            rock_golem: asset_server.load("sprites/monsters/rock_golem.png"),
            flame_wraith: asset_server.load("sprites/monsters/flame_wraith.png"),
            void_stalker: asset_server.load("sprites/monsters/void_stalker.png"),
        }
    } else {
        MonsterSpriteHandles {
            forest_spirit: images.add(create_colored_sprite(32, 32, Color::srgb(0.4, 0.9, 0.4))),
            chaos_hound: images.add(create_colored_sprite(32, 32, Color::srgb(0.9, 0.2, 0.2))),
            corrupt_shade: images.add(create_colored_sprite(32, 32, Color::srgb(0.5, 0.1, 0.5))),
            shadow_beast: images.add(create_colored_sprite(32, 32, Color::srgb(0.2, 0.2, 0.2))),
            spirit_wisp: images.add(create_colored_sprite(32, 32, Color::srgb(0.9, 0.9, 1.0))),
            rock_golem: images.add(create_colored_sprite(32, 32, Color::srgb(0.6, 0.5, 0.4))),
            flame_wraith: images.add(create_colored_sprite(32, 32, Color::srgb(1.0, 0.5, 0.1))),
            void_stalker: images.add(create_colored_sprite(32, 32, Color::srgb(0.1, 0.0, 0.2))),
        }
    };
    commands.insert_resource(monster_sprites);

    // Load or create NPC sprites
    let npc_sprites = if !use_placeholders {
        NpcSpriteHandles {
            default_npc: asset_server.load("sprites/npcs/villager.png"),
            elder: asset_server.load("sprites/npcs/elder.png"),
            merchant: asset_server.load("sprites/npcs/merchant.png"),
            farmer: asset_server.load("sprites/npcs/farmer.png"),
            hunter: asset_server.load("sprites/npcs/hunter.png"),
            child: asset_server.load("sprites/npcs/child.png"),
            head_shaman: asset_server.load("sprites/npcs/head_shaman.png"),
            brother: asset_server.load("sprites/npcs/brother.png"),
        }
    } else {
        NpcSpriteHandles {
            default_npc: images.add(create_colored_sprite(32, 32, Color::srgb(1.0, 1.0, 0.5))),
            elder: images.add(create_colored_sprite(32, 32, Color::srgb(0.7, 0.7, 0.9))),
            merchant: images.add(create_colored_sprite(32, 32, Color::srgb(0.9, 0.7, 0.3))),
            farmer: images.add(create_colored_sprite(32, 32, Color::srgb(0.5, 0.6, 0.3))),
            hunter: images.add(create_colored_sprite(32, 32, Color::srgb(0.5, 0.4, 0.3))),
            child: images.add(create_colored_sprite(32, 32, Color::srgb(1.0, 0.8, 0.6))),
            head_shaman: images.add(create_colored_sprite(32, 32, Color::srgb(0.6, 0.3, 0.9))),
            brother: images.add(create_colored_sprite(32, 32, Color::srgb(0.9, 0.3, 0.3))),
        }
    };
    commands.insert_resource(npc_sprites);

    // Note: Audio assets are loaded in the bevy_shaman_audio crate

    loading_state.loaded = true;
    info!("Core asset loading complete! (Using {})", if use_placeholders { "placeholders" } else { "real assets" });
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

/// Resource to hold NPC sprite handles
#[derive(Resource)]
pub struct NpcSpriteHandles {
    pub default_npc: Handle<Image>,
    pub elder: Handle<Image>,
    pub merchant: Handle<Image>,
    pub farmer: Handle<Image>,
    pub hunter: Handle<Image>,
    pub child: Handle<Image>,
    pub head_shaman: Handle<Image>,
    pub brother: Handle<Image>,
}

// Note: Audio assets are managed in the bevy_shaman_audio crate
