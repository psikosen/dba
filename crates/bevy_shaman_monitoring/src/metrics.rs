use bevy::prelude::*;
use prometheus::{Counter, Gauge, Histogram, HistogramOpts, IntCounter, IntGauge, Opts, Registry};
use std::sync::{Arc, OnceLock};

/// Global Prometheus registry (thread-safe one-time initialization)
static REGISTRY: OnceLock<Arc<Registry>> = OnceLock::new();

/// Performance metrics
pub struct PerformanceMetrics {
    pub frame_time: Histogram,
    pub fps: Gauge,
    pub entity_count: IntGauge,
    pub system_count: IntGauge,
}

/// Game-specific metrics
pub struct GameMetrics {
    pub monsters_spawned: IntCounter,
    pub monsters_defeated: IntCounter,
    pub player_deaths: IntCounter,
    pub corruption_level: Gauge,
    pub purification_count: IntCounter,
    pub combat_encounters: IntCounter,
    pub boss_encounters: IntCounter,
}

/// Initialize Prometheus metrics
pub fn init_metrics() {
    // Skip if already initialized
    if REGISTRY.get().is_some() {
        warn!("Prometheus metrics already initialized");
        return;
    }

    let registry = Registry::new();

    // Register performance metrics
    let frame_time = Histogram::with_opts(
        HistogramOpts::new("bevy_shaman_frame_time_seconds", "Frame rendering time").buckets(vec![
            0.001, 0.005, 0.010, 0.016, 0.033, 0.050, 0.100, 0.250, 0.500, 1.0,
        ]),
    )
    .unwrap();
    registry.register(Box::new(frame_time.clone())).unwrap();

    let fps = Gauge::with_opts(Opts::new("bevy_shaman_fps", "Current frames per second")).unwrap();
    registry.register(Box::new(fps.clone())).unwrap();

    let entity_count =
        IntGauge::with_opts(Opts::new("bevy_shaman_entity_count", "Total entity count")).unwrap();
    registry.register(Box::new(entity_count.clone())).unwrap();

    let system_count =
        IntGauge::with_opts(Opts::new("bevy_shaman_system_count", "Active system count")).unwrap();
    registry.register(Box::new(system_count.clone())).unwrap();

    // Register game metrics
    let monsters_spawned = IntCounter::with_opts(Opts::new(
        "bevy_shaman_monsters_spawned_total",
        "Total monsters spawned",
    ))
    .unwrap();
    registry
        .register(Box::new(monsters_spawned.clone()))
        .unwrap();

    let monsters_defeated = IntCounter::with_opts(Opts::new(
        "bevy_shaman_monsters_defeated_total",
        "Total monsters defeated",
    ))
    .unwrap();
    registry
        .register(Box::new(monsters_defeated.clone()))
        .unwrap();

    let player_deaths = IntCounter::with_opts(Opts::new(
        "bevy_shaman_player_deaths_total",
        "Total player deaths",
    ))
    .unwrap();
    registry.register(Box::new(player_deaths.clone())).unwrap();

    let corruption_level = Gauge::with_opts(Opts::new(
        "bevy_shaman_corruption_level",
        "Current world corruption level",
    ))
    .unwrap();
    registry
        .register(Box::new(corruption_level.clone()))
        .unwrap();

    let purification_count = IntCounter::with_opts(Opts::new(
        "bevy_shaman_purification_total",
        "Total purifications performed",
    ))
    .unwrap();
    registry
        .register(Box::new(purification_count.clone()))
        .unwrap();

    let combat_encounters = IntCounter::with_opts(Opts::new(
        "bevy_shaman_combat_encounters_total",
        "Total combat encounters",
    ))
    .unwrap();
    registry
        .register(Box::new(combat_encounters.clone()))
        .unwrap();

    let boss_encounters = IntCounter::with_opts(Opts::new(
        "bevy_shaman_boss_encounters_total",
        "Total boss encounters",
    ))
    .unwrap();
    registry
        .register(Box::new(boss_encounters.clone()))
        .unwrap();

    // Set registry (thread-safe one-time initialization)
    if REGISTRY.set(Arc::new(registry)).is_err() {
        error!("Failed to initialize Prometheus registry - already set");
        return;
    }

    info!("Prometheus metrics initialized");
}

/// Get the Prometheus registry
pub fn get_registry() -> Option<Arc<Registry>> {
    REGISTRY.get().cloned()
}

/// System to update performance metrics
pub fn update_performance_metrics(
    time: Res<Time>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
) {
    if let Some(registry) = get_registry() {
        // Update frame time
        if let Some(frame_time_diagnostic) =
            diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FRAME_TIME)
        {
            if let Some(value) = frame_time_diagnostic.smoothed() {
                // Frame time is stored but we can't easily update the histogram here
                // This would require storing the histogram in a resource
                trace!("Frame time: {:.3}ms", value);
            }
        }

        // Update FPS
        if let Some(fps_diagnostic) =
            diagnostics.get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        {
            if let Some(value) = fps_diagnostic.smoothed() {
                trace!("FPS: {:.1}", value);
            }
        }
    }
}

/// System to update game-specific metrics
pub fn update_game_metrics(// Add your game-specific queries here
    // For example:
    // monsters: Query<&Monster>,
    // corruption: Res<WorldCorruption>,
) {
    // Update game metrics based on game state
    // This is a placeholder - you'll need to add actual queries
    trace!("Updating game metrics");
}

/// Export metrics in Prometheus format
pub fn export_metrics() -> String {
    if let Some(registry) = get_registry() {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();
        let metric_families = registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    } else {
        String::new()
    }
}
