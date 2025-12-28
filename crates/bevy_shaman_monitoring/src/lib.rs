pub mod metrics;
pub mod sentry_integration;
pub mod tracing_setup;

use bevy::prelude::*;

/// Plugin for monitoring, metrics, and error tracking
pub struct MonitoringPlugin;

impl Plugin for MonitoringPlugin {
    fn build(&self, app: &mut App) {
        // Initialize tracing
        tracing_setup::init_tracing();

        // Initialize Sentry if DSN is provided
        if let Ok(dsn) = std::env::var("SENTRY_DSN") {
            sentry_integration::init_sentry(&dsn);
        }

        // Initialize Prometheus metrics
        metrics::init_metrics();

        // Add monitoring systems
        app.add_systems(Update, (
            metrics::update_performance_metrics,
            metrics::update_game_metrics,
        ));
    }
}
