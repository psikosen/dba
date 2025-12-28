# bevy_shaman_services

Service integrations for Shaman's Journey, providing caching and async processing capabilities.

## Features

### DragonflyDB Cache Layer
- **LLM Response Cache**: Reduce generation load by caching LLM responses
- **Dungeon Seed Cache**: Faster level loading with cached dungeon layouts
- **Session State Cache**: Fast session management with automatic expiration

### RabbitMQ Queue Layer
- **Background Job Queue**: Async processing for simulations and analytics
- **Event Bus**: Event-driven architecture for game events
- **Offline Processor**: Long-running tasks with scheduling support

## Usage

```rust
use bevy_shaman_services::{initialize_services, ServiceConfig};

// Initialize all services
let config = ServiceConfig::from_env();
let services = initialize_services(config).await?;

// Use caching
if let Some(response) = services.llm_cache.get_response(prompt, context).await? {
    // Use cached response
} else {
    // Generate and cache new response
}

// Publish events
services.event_bus.publish_event(event).await?;

// Submit background jobs
services.background_jobs.submit_job(job).await?;
```

## Documentation

- [SERVICES_ARCHITECTURE.md](../../SERVICES_ARCHITECTURE.md) - Detailed architecture documentation
- [SERVICES_QUICKSTART.md](../../SERVICES_QUICKSTART.md) - Quick start guide

## Dependencies

- `redis` - DragonflyDB client (Redis-compatible)
- `lapin` - RabbitMQ client
- `tokio` - Async runtime
- `serde` - Serialization

## License

MIT OR Apache-2.0
