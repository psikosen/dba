use bevy::prelude::*;
use bevy_shaman_core::components::GridPosition;
use bevy_shaman_world::components::{BiomeType, WorldTile};
use crate::components::{NpcName, NpcDialogue, NpcSicknessState, HeadShaman, PlayerBrother};
use crate::resources::{AfricanNamesDB, CharacterType, PortraitEmotion};
use rand::seq::SliceRandom;
use rand::Rng;

/// Resource to track if NPCs have been spawned
#[derive(Resource, Default)]
pub struct NpcsSpawned(pub bool);

/// Spawn NPCs in village tiles after world generation
pub fn spawn_village_npcs(
    mut commands: Commands,
    mut spawned: ResMut<NpcsSpawned>,
    names_db: Res<AfricanNamesDB>,
    village_tiles: Query<&GridPosition, With<crate::components::VillageMarker>>,
) {
    if spawned.0 {
        return;
    }

    // Wait for villages to be marked (this happens after world generation)
    if village_tiles.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();

    // Spawn Head Shaman (always at a specific village location)
    if let Some(first_village) = village_tiles.iter().next() {
        let head_shaman_name = get_name_by_type(&names_db, CharacterType::SpiritualLeader, &mut rng)
            .unwrap_or("Nuru".to_string());

        spawn_npc(
            &mut commands,
            head_shaman_name,
            first_village.x,
            first_village.y,
            Some(HeadShamanBundle),
            "Welcome, young shaman. The spirits are restless and corruption spreads across our land.".to_string(),
            Some("I sense... the awakening begins. Your brothers stir.".to_string()),
        );

        info!("Spawned Head Shaman at village ({}, {})", first_village.x, first_village.y);
    }

    // Spawn Player's Brothers (4 of them)
    let brother_names: Vec<String> = names_db.names.iter()
        .filter(|(_, (char_type, _, _))| *char_type == CharacterType::Brother)
        .map(|(name, _)| name.clone())
        .collect();

    for (i, brother_name) in brother_names.iter().take(4).enumerate() {
        // Distribute brothers across different villages
        if let Some(village_pos) = village_tiles.iter().nth(i % village_tiles.iter().count()) {
            spawn_brother(
                &mut commands,
                brother_name.clone(),
                village_pos.x + (i as i32 - 2),
                village_pos.y + (i as i32 - 2),
            );
            info!("Spawned Brother {} at ({}, {})", brother_name, village_pos.x, village_pos.y);
        }
    }

    // Spawn Elders (3 of them)
    spawn_npcs_by_type(
        &mut commands,
        &names_db,
        &village_tiles,
        CharacterType::Elder,
        3,
        &mut rng,
        "The wisdom of our ancestors guides us through these dark times.",
        Some("I remember... when the spirits were at peace..."),
    );

    // Spawn Merchants (6 of them)
    spawn_npcs_by_type(
        &mut commands,
        &names_db,
        &village_tiles,
        CharacterType::Merchant,
        6,
        &mut rng,
        "Welcome! I have goods to trade.",
        Some("Trade... yes... I can trade..."),
    );

    // Spawn Farmers (3 of them)
    spawn_npcs_by_type(
        &mut commands,
        &names_db,
        &village_tiles,
        CharacterType::Farmer,
        3,
        &mut rng,
        "The crops wither under this corruption. We need your help!",
        Some("Fields... dying... help..."),
    );

    // Spawn Hunters (4 of them)
    spawn_npcs_by_type(
        &mut commands,
        &names_db,
        &village_tiles,
        CharacterType::Hunter,
        4,
        &mut rng,
        "I've seen corrupted beasts in the forest. Be careful out there.",
        Some("Monsters... everywhere... hunt..."),
    );

    // Spawn Children (3 of them)
    spawn_npcs_by_type(
        &mut commands,
        &names_db,
        &village_tiles,
        CharacterType::Child,
        3,
        &mut rng,
        "Will you help save our village?",
        Some("Scared... dark..."),
    );

    // Spawn generic Villagers (10 of them)
    spawn_npcs_by_type(
        &mut commands,
        &names_db,
        &village_tiles,
        CharacterType::Villager,
        10,
        &mut rng,
        "Please, help us! The corruption is spreading!",
        Some("Help... need... help..."),
    );

    spawned.0 = true;
    info!("Village NPCs spawned successfully!");
}

/// Helper function to get a random name by character type
fn get_name_by_type(
    names_db: &AfricanNamesDB,
    char_type: CharacterType,
    rng: &mut impl Rng,
) -> Option<String> {
    let names: Vec<String> = names_db.names.iter()
        .filter(|(_, (ct, _, _))| *ct == char_type)
        .map(|(name, _)| name.clone())
        .collect();

    names.choose(rng).cloned()
}

/// Helper function to spawn multiple NPCs of a specific type
fn spawn_npcs_by_type(
    commands: &mut Commands,
    names_db: &AfricanNamesDB,
    village_tiles: &Query<&GridPosition, With<crate::components::VillageMarker>>,
    char_type: CharacterType,
    count: usize,
    rng: &mut impl Rng,
    full_dialogue: &str,
    partial_dialogue: Option<&str>,
) {
    for i in 0..count {
        if let Some(name) = get_name_by_type(names_db, char_type, rng) {
            if let Some(village_pos) = village_tiles.iter().nth(i % village_tiles.iter().count()) {
                let offset_x = rng.gen_range(-2..=2);
                let offset_y = rng.gen_range(-2..=2);

                spawn_npc(
                    commands,
                    name,
                    village_pos.x + offset_x,
                    village_pos.y + offset_y,
                    None,
                    full_dialogue.to_string(),
                    partial_dialogue.map(|s| s.to_string()),
                );
            }
        }
    }
}

/// Marker component for Head Shaman bundle
#[derive(Bundle)]
struct HeadShamanBundle {
    head_shaman: HeadShaman,
}

/// Core function to spawn an NPC entity
fn spawn_npc(
    commands: &mut Commands,
    name: String,
    x: i32,
    y: i32,
    special_bundle: Option<HeadShamanBundle>,
    full_dialogue: String,
    partial_dialogue: Option<String>,
) {
    let mut entity_commands = commands.spawn((
        NpcName {
            name: name.clone(),
            current_emotion: PortraitEmotion::Neutral,
        },
        NpcSicknessState::AsleepSick,
        NpcDialogue {
            full_dialogue,
            partial_dialogue,
            sick_dialogue: "... ... ...".to_string(),
        },
        GridPosition { x, y },
        Transform::from_xyz(x as f32 * 32.0, y as f32 * 32.0, 1.0),
        GlobalTransform::default(),
        Visibility::default(),
        Name::new(format!("NPC: {}", name)),
    ));

    // Add special component if provided
    if let Some(bundle) = special_bundle {
        entity_commands.insert(bundle);
    }
}

/// Helper function to spawn a brother NPC
fn spawn_brother(
    commands: &mut Commands,
    name: String,
    x: i32,
    y: i32,
) {
    commands.spawn((
        NpcName {
            name: name.clone(),
            current_emotion: PortraitEmotion::Angry,
        },
        PlayerBrother::default(),
        NpcSicknessState::AsleepSick,
        NpcDialogue {
            full_dialogue: format!("{}, my brother... I'm sorry for what I've become.", name),
            partial_dialogue: Some("Fight me... cleanse my soul...".to_string()),
            sick_dialogue: "... ... ...".to_string(),
        },
        GridPosition { x, y },
        Transform::from_xyz(x as f32 * 32.0, y as f32 * 32.0, 1.0),
        GlobalTransform::default(),
        Visibility::default(),
        Name::new(format!("Brother: {}", name)),
    ));
}

/// Marker component for village tiles
#[derive(Component)]
pub struct VillageMarker;

/// System to mark village tiles (runs before NPC spawning)
pub fn mark_village_tiles(
    mut commands: Commands,
    village_tiles: Query<(Entity, &WorldTile), Without<VillageMarker>>,
) {
    for (entity, tile) in village_tiles.iter() {
        if tile.biome == BiomeType::Village {
            commands.entity(entity).insert(VillageMarker);
        }
    }
}
