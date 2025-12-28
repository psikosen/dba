use bevy::prelude::*;
use sentry::{ClientOptions, IntoDsn};

/// Initialize Sentry error tracking
pub fn init_sentry(dsn: &str) {
    let _guard = sentry::init((
        dsn.into_dsn().unwrap(),
        ClientOptions {
            release: sentry::release_name!(),
            environment: Some(
                std::env::var("ENVIRONMENT")
                    .unwrap_or_else(|_| "development".to_string())
                    .into(),
            ),
            attach_stacktrace: true,
            send_default_pii: false,
            ..Default::default()
        },
    ));

    info!("Sentry error tracking initialized");

    // Set up panic hook
    std::panic::set_hook(Box::new(|panic_info| {
        sentry::capture_message(&format!("Panic: {:?}", panic_info), sentry::Level::Error);
        eprintln!("{}", panic_info);
    }));
}

/// Capture a message to Sentry
pub fn capture_message(msg: &str, level: SentryLevel) {
    let sentry_level = match level {
        SentryLevel::Debug => sentry::Level::Debug,
        SentryLevel::Info => sentry::Level::Info,
        SentryLevel::Warning => sentry::Level::Warning,
        SentryLevel::Error => sentry::Level::Error,
        SentryLevel::Fatal => sentry::Level::Fatal,
    };
    sentry::capture_message(msg, sentry_level);
}

/// Capture an error to Sentry
pub fn capture_error(err: &dyn std::error::Error) {
    sentry::capture_error(err);
}

/// Add breadcrumb for tracking user actions
pub fn add_breadcrumb(category: &str, message: &str, level: SentryLevel) {
    let sentry_level = match level {
        SentryLevel::Debug => sentry::Level::Debug,
        SentryLevel::Info => sentry::Level::Info,
        SentryLevel::Warning => sentry::Level::Warning,
        SentryLevel::Error => sentry::Level::Error,
        SentryLevel::Fatal => sentry::Level::Fatal,
    };

    sentry::add_breadcrumb(sentry::Breadcrumb {
        category: Some(category.into()),
        message: Some(message.into()),
        level: sentry_level,
        ..Default::default()
    });
}

/// Sentry log levels
#[derive(Debug, Clone, Copy)]
pub enum SentryLevel {
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

/// Set user context for Sentry
pub fn set_user_context(user_id: Option<&str>, username: Option<&str>) {
    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            id: user_id.map(|s| s.to_string()),
            username: username.map(|s| s.to_string()),
            ..Default::default()
        }));
    });
}

/// Set custom tag for Sentry events
pub fn set_tag(key: &str, value: &str) {
    sentry::configure_scope(|scope| {
        scope.set_tag(key, value);
    });
}

/// Set custom context for Sentry events
pub fn set_context(key: &str, context: sentry::protocol::Context) {
    sentry::configure_scope(|scope| {
        scope.set_context(key, context);
    });
}
