use bevy::prelude::*;
use prometheus::{Gauge, Histogram, HistogramOpts, IntCounter, IntGauge, Opts, Registry};
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

    // Internal initialization that can fail
    if let Err(e) = try_init_metrics() {
        error!(
            "Failed to initialize Prometheus metrics: {} - metrics disabled",
            e
        );
    }
}

/// Internal function that performs the actual initialization with error handling
fn try_init_metrics() -> Result<(), Box<dyn std::error::Error>> {
    let registry = Registry::new();

    // Register performance metrics
    let frame_time = Histogram::with_opts(
        HistogramOpts::new("bevy_shaman_frame_time_seconds", "Frame rendering time").buckets(vec![
            0.001, 0.005, 0.010, 0.016, 0.033, 0.050, 0.100, 0.250, 0.500, 1.0,
        ]),
    )?;
    registry.register(Box::new(frame_time.clone()))?;

    let fps = Gauge::with_opts(Opts::new("bevy_shaman_fps", "Current frames per second"))?;
    registry.register(Box::new(fps.clone()))?;

    let entity_count =
        IntGauge::with_opts(Opts::new("bevy_shaman_entity_count", "Total entity count"))?;
    registry.register(Box::new(entity_count.clone()))?;

    let system_count =
        IntGauge::with_opts(Opts::new("bevy_shaman_system_count", "Active system count"))?;
    registry.register(Box::new(system_count.clone()))?;

    // Register game metrics
    let monsters_spawned = IntCounter::with_opts(Opts::new(
        "bevy_shaman_monsters_spawned_total",
        "Total monsters spawned",
    ))?;
    registry.register(Box::new(monsters_spawned.clone()))?;

    let monsters_defeated = IntCounter::with_opts(Opts::new(
        "bevy_shaman_monsters_defeated_total",
        "Total monsters defeated",
    ))?;
    registry.register(Box::new(monsters_defeated.clone()))?;

    let player_deaths = IntCounter::with_opts(Opts::new(
        "bevy_shaman_player_deaths_total",
        "Total player deaths",
    ))?;
    registry.register(Box::new(player_deaths.clone()))?;

    let corruption_level = Gauge::with_opts(Opts::new(
        "bevy_shaman_corruption_level",
        "Current world corruption level",
    ))?;
    registry.register(Box::new(corruption_level.clone()))?;

    let purification_count = IntCounter::with_opts(Opts::new(
        "bevy_shaman_purification_total",
        "Total purifications performed",
    ))?;
    registry.register(Box::new(purification_count.clone()))?;

    let combat_encounters = IntCounter::with_opts(Opts::new(
        "bevy_shaman_combat_encounters_total",
        "Total combat encounters",
    ))?;
    registry.register(Box::new(combat_encounters.clone()))?;

    let boss_encounters = IntCounter::with_opts(Opts::new(
        "bevy_shaman_boss_encounters_total",
        "Total boss encounters",
    ))?;
    registry.register(Box::new(boss_encounters.clone()))?;

    // Set registry (thread-safe one-time initialization)
    REGISTRY
        .set(Arc::new(registry))
        .map_err(|_| "Registry already initialized")?;

    info!("Prometheus metrics initialized");
    Ok(())
}

/// Get the Prometheus registry
pub fn get_registry() -> Option<Arc<Registry>> {
    REGISTRY.get().cloned()
}

/// System to update performance metrics
pub fn update_performance_metrics(
    _time: Res<Time>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
) {
    if let Some(_registry) = get_registry() {
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

        // Encode metrics, gracefully handling errors
        if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
            error!("Failed to encode Prometheus metrics: {}", e);
            return String::new();
        }

        // Convert to UTF-8, gracefully handling errors
        String::from_utf8(buffer).unwrap_or_else(|e| {
            error!("Failed to convert metrics to UTF-8: {}", e);
            String::new()
        })
    } else {
        String::new()
    }
}
