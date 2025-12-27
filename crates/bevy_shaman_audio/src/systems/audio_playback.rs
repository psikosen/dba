use bevy::prelude::*;
use crate::resources::{ActiveSong, BeatClock, SongDB};

/// Resource holding audio assets
#[derive(Resource, Default)]
pub struct AudioAssets {
    pub dawn_hymn: Handle<AudioSource>,
    pub war_chant: Handle<AudioSource>,
    pub purification_rite: Handle<AudioSource>,
    pub hit_sound: Handle<AudioSource>,
    pub menu_click: Handle<AudioSource>,
    pub typewriter_beep: Handle<AudioSource>,
    pub level_up: Handle<AudioSource>,
    pub quest_complete: Handle<AudioSource>,
}

/// Music player entity marker
#[derive(Component)]
pub struct MusicPlayer;

/// Sound effect type
#[derive(Event)]
pub enum PlaySoundEffect {
    Hit,
    MenuClick,
    TypewriterBeep,
    LevelUp,
    QuestComplete,
}

/// Load audio assets at startup
pub fn load_audio_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Check if assets exist, otherwise log warnings
    info!("Loading audio assets...");

    let assets = AudioAssets {
        dawn_hymn: asset_server.load("audio/music/dawn_hymn.ogg"),
        war_chant: asset_server.load("audio/music/war_chant.ogg"),
        purification_rite: asset_server.load("audio/music/purification_rite.ogg"),
        hit_sound: asset_server.load("audio/sfx/hit.ogg"),
        menu_click: asset_server.load("audio/sfx/menu_click.ogg"),
        typewriter_beep: asset_server.load("audio/sfx/typewriter_beep.ogg"),
        level_up: asset_server.load("audio/sfx/level_up.ogg"),
        quest_complete: asset_server.load("audio/sfx/quest_complete.ogg"),
    };

    commands.insert_resource(assets);
}

/// Play music based on active song
pub fn play_music(
    mut commands: Commands,
    active_song: Res<ActiveSong>,
    song_db: Res<SongDB>,
    audio_assets: Res<AudioAssets>,
    mut beat_clock: ResMut<BeatClock>,
    music_player: Query<Entity, With<MusicPlayer>>,
) {
    // Skip if no active song
    let Some(ref song_id) = active_song.current_song_id else {
        // Stop music if there's no active song
        if !music_player.is_empty() {
            for entity in music_player.iter() {
                commands.entity(entity).despawn();
            }
            beat_clock.is_playing = false;
        }
        return;
    };

    // Get song data
    let Some(song) = song_db.get(song_id) else {
        warn!("Song not found in database: {}", song_id);
        return;
    };

    // Update beat clock BPM
    beat_clock.set_bpm(song.bpm);
    beat_clock.is_playing = true;

    // If music player doesn't exist, spawn it
    if music_player.is_empty() {
        let audio_source = match song_id.as_str() {
            "dawn_hymn" => audio_assets.dawn_hymn.clone(),
            "war_chant" => audio_assets.war_chant.clone(),
            "purification_rite" => audio_assets.purification_rite.clone(),
            _ => {
                warn!("Unknown song ID: {}", song_id);
                return;
            }
        };

        commands.spawn((
            MusicPlayer,
            AudioPlayer::<AudioSource>(audio_source),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::new(0.7),
                speed: 1.0,
                paused: false,
                spatial: false,
                spatial_scale: None,
            },
        ));

        info!("Started playing: {}", song.display_name);
    }
}

/// Play sound effects
pub fn play_sound_effects(
    mut commands: Commands,
    mut sfx_events: EventReader<PlaySoundEffect>,
    audio_assets: Res<AudioAssets>,
) {
    for event in sfx_events.read() {
        let audio_source = match event {
            PlaySoundEffect::Hit => audio_assets.hit_sound.clone(),
            PlaySoundEffect::MenuClick => audio_assets.menu_click.clone(),
            PlaySoundEffect::TypewriterBeep => audio_assets.typewriter_beep.clone(),
            PlaySoundEffect::LevelUp => audio_assets.level_up.clone(),
            PlaySoundEffect::QuestComplete => audio_assets.quest_complete.clone(),
        };

        commands.spawn((
            AudioPlayer::<AudioSource>(audio_source),
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::new(0.5),
                speed: 1.0,
                paused: false,
                spatial: false,
                spatial_scale: None,
            },
        ));
    }
}

/// Trigger sound effects on combat hits
pub fn play_hit_sounds(
    mut sfx_events: EventWriter<PlaySoundEffect>,
    mut hit_events: EventReader<bevy_shaman_combat::systems::events::HitLanded>,
) {
    for _event in hit_events.read() {
        sfx_events.send(PlaySoundEffect::Hit);
    }
}

/// Trigger sound effect on level up
pub fn play_level_up_sound(
    player_level: Res<bevy_shaman_core::resources::PlayerLevel>,
    mut sfx_events: EventWriter<PlaySoundEffect>,
    mut last_level: Local<u8>,
) {
    if player_level.current > *last_level {
        sfx_events.send(PlaySoundEffect::LevelUp);
        *last_level = player_level.current;
    }
}

/// Trigger sound effect on quest completion
pub fn play_quest_complete_sound(
    mut sfx_events: EventWriter<PlaySoundEffect>,
    mut quest_events: EventReader<bevy_shaman_story::systems::quest_system::QuestCompleted>,
) {
    for _event in quest_events.read() {
        sfx_events.send(PlaySoundEffect::QuestComplete);
    }
}
