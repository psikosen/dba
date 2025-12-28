# Services Architecture

## Overview

This document describes the DragonflyDB and RabbitMQ integration for Shaman's Journey, providing caching and async processing capabilities.

## Architecture Components

### 1. DragonflyDB (Cache Layer)

DragonflyDB is a modern, high-performance Redis alternative that provides:
- **Lower latency** than Redis
- **Higher throughput** with better memory efficiency
- **Full Redis compatibility** - drop-in replacement
- **Built-in persistence** with snapshots

#### Cache Services

##### LLM Response Cache
**Purpose**: Reduce LLM generation load by caching responses

**Features**:
- Hash-based cache keys (prompt + context)
- Configurable TTL (default: 1 hour)
- Automatic serialization/deserialization
- Cache hit/miss tracking

**Use Cases**:
- Repeated NPC dialogue requests
- Common quest text generation
- Spirit guide interactions

**Example**:
```rust
use bevy_shaman_services::{LlmResponseCache, LlmResponse};

// Get cached response
if let Some(response) = llm_cache.get_response(prompt, context).await? {
    // Use cached response
    println!("Using cached LLM response");
} else {
    // Generate new response
    let response = generate_llm_response(prompt, context).await?;
    llm_cache.cache_response(prompt, context, &response).await?;
}
```

##### Dungeon Seed Cache
**Purpose**: Enable faster level loading by caching dungeon generation

**Features**:
- Caches complete dungeon layouts
- Stores monster and item placements
- Supports multiple difficulty levels
- Preloading capabilities

**Use Cases**:
- Quick dungeon re-entry
- Consistent dungeon layouts
- Performance optimization for complex dungeons

**Example**:
```rust
use bevy_shaman_services::{DungeonSeedCache, DungeonSeed};

// Check cache first
if let Some(seed) = dungeon_cache.get_seed("forest_temple", 1, "normal").await? {
    // Use cached dungeon layout
    load_dungeon_from_seed(seed);
} else {
    // Generate new dungeon
    let seed = generate_dungeon("forest_temple", 1, "normal");
    dungeon_cache.cache_seed(&seed).await?;
}
```

##### Session State Cache
**Purpose**: Fast session state management with automatic expiration

**Features**:
- Player location and stats snapshot
- Active quest tracking
- Inventory snapshot
- Sliding expiration (extends on access)
- Automatic cleanup of inactive sessions

**Use Cases**:
- Quick session restoration
- Cross-system state sharing
- Temporary state storage

**Example**:
```rust
use bevy_shaman_services::{SessionStateCache, SessionState};

// Load session
if let Some(state) = session_cache.get_session(session_id).await? {
    // Restore player state
    restore_player_state(state);
} else {
    // Create new session
    let state = SessionState::new(session_id, player_id);
    session_cache.save_session(session_id, &state).await?;
}
```

### 2. RabbitMQ (Message Queue Layer)

RabbitMQ provides reliable message queuing for async operations:
- **Durable queues** - Messages survive restarts
- **Topic-based routing** - Flexible event distribution
- **Dead letter queues** - Handle failed messages
- **Management UI** - Monitor queues at http://localhost:15672

#### Queue Services

##### Background Job Queue
**Purpose**: Process long-running tasks asynchronously

**Job Types**:
- Dungeon simulations (balance testing)
- Player analytics (stats, playtime)
- Game statistics aggregation
- AI content generation (bulk)
- Data exports
- Session cleanup
- Achievement processing

**Example**:
```rust
use bevy_shaman_services::{BackgroundJobQueue, BackgroundJob, JobType, JobPayload};

// Submit a background job
let job = BackgroundJob::new(
    JobType::DungeonSimulation,
    JobPayload::DungeonSimulation {
        dungeon_id: "forest_temple".to_string(),
        level: 1,
        iterations: 1000,
    }
)
.with_priority(8);

let job_id = background_jobs.submit_job(job).await?;
println!("Submitted job: {}", job_id);
```

##### Event Bus
**Purpose**: Event-driven architecture for game events

**Event Types**:
- Player actions (movement, interactions)
- Combat events (attacks, damage)
- Item events (acquired, used, dropped)
- Dungeon events (entered, completed)
- Quest events (started, completed)
- Achievements unlocked
- Level ups
- Session lifecycle

**Routing Pattern**: Topic-based routing with patterns like:
- `game.player.*` - All player events
- `game.combat.*` - All combat events
- `game.*.event` - All events

**Example**:
```rust
use bevy_shaman_services::{EventBus, GameEvent, EventType, EventPayload};

// Publish an event
let event = GameEvent::new(
    EventType::AchievementUnlocked,
    EventPayload::Achievement {
        achievement_id: "first_kill".to_string(),
        player_id: "player123".to_string(),
        name: "First Blood".to_string(),
    },
    "achievement_system".to_string(),
);

event_bus.publish_event(event).await?;

// Subscribe to events
event_bus.subscribe("analytics_queue", "game.player.*").await?;
```

##### Offline Processor
**Purpose**: Handle long-running offline tasks with scheduling

**Job Types**:
- Report generation
- Data migration
- Complex simulations
- Bulk AI generation
- Behavior analysis
- Dungeon generation (bulk)
- Achievement calculations

**Features**:
- Delayed execution (scheduling)
- Configurable timeouts
- Batch submission

**Example**:
```rust
use bevy_shaman_services::{OfflineProcessor, OfflineJob, OfflineJobType, OfflineJobPayload};

// Submit offline job
let job = OfflineJob::new(
    OfflineJobType::Simulation,
    OfflineJobPayload::Simulation {
        simulation_type: "combat_balance".to_string(),
        iterations: 10000,
        parameters: HashMap::new(),
    }
)
.with_timeout(7200); // 2 hours

let job_id = offline_processor.submit_job(job).await?;
```

## Configuration

### Environment Variables

```bash
# DragonflyDB
DRAGONFLY_URL=redis://:password@localhost:6379
DRAGONFLY_PASSWORD=changeme
LLM_CACHE_TTL=3600          # 1 hour
DUNGEON_CACHE_TTL=86400     # 24 hours
SESSION_CACHE_TTL=1800      # 30 minutes

# RabbitMQ
RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f
RABBITMQ_USER=guest
RABBITMQ_PASSWORD=guest
```

### Docker Compose

Start all services:
```bash
docker-compose up -d
```

Access RabbitMQ Management UI:
- URL: http://localhost:15672
- Username: guest (or from RABBITMQ_USER)
- Password: guest (or from RABBITMQ_PASSWORD)

## Service Initialization

### In Rust Code

```rust
use bevy_shaman_services::{initialize_services, ServiceConfig};

// Initialize all services
let config = ServiceConfig::from_env();
let services = initialize_services(config).await?;

// Use services
let response = services.llm_cache.get_response(prompt, context).await?;
services.event_bus.publish_event(event).await?;
services.background_jobs.submit_job(job).await?;

// Health check
let health = services.health_check().await?;
if health.overall {
    println!("All services healthy");
}
```

## Performance Considerations

### DragonflyDB

**Memory Management**:
- Default max memory: 512MB (configurable)
- Automatic eviction of expired keys
- Memory-efficient data structures

**Persistence**:
- Snapshot frequency: Every minute (configurable)
- Data survives container restarts

**Monitoring**:
- Use `services.dragonfly.get_stats()` for cache metrics
- Monitor hit/miss ratios
- Track memory usage

### RabbitMQ

**Queue Configuration**:
- Durable queues (survives restarts)
- Message persistence enabled
- Dead letter queues for failures

**Consumer Patterns**:
- Worker pools for parallel processing
- Prefetch limits to prevent overload
- Acknowledgment-based delivery

**Monitoring**:
- Management UI at port 15672
- Queue depth monitoring
- Consumer status tracking

## Error Handling

### Cache Failures

```rust
// Graceful degradation
match llm_cache.get_response(prompt, context).await {
    Ok(Some(response)) => use_cached(response),
    Ok(None) => generate_fresh(),
    Err(e) => {
        warn!("Cache error: {}", e);
        generate_fresh() // Fallback to generation
    }
}
```

### Queue Failures

```rust
// Retry logic
match background_jobs.submit_job(job).await {
    Ok(job_id) => println!("Job submitted: {}", job_id),
    Err(e) => {
        error!("Failed to submit job: {}", e);
        // Store locally for retry or alert
    }
}
```

## Best Practices

1. **Cache Keys**: Use descriptive, hierarchical keys (e.g., `dungeon:seed:forest_temple:1:normal`)

2. **TTL Settings**: Balance freshness vs performance
   - Frequently changing data: Short TTL (minutes)
   - Static content: Long TTL (hours/days)
   - Session data: Medium TTL with sliding expiration

3. **Event Publishing**: Publish events at appropriate granularity
   - Too many events: Queue overflow
   - Too few events: Lost insights

4. **Job Sizing**: Break large jobs into smaller chunks
   - Easier error recovery
   - Better resource utilization
   - Progress tracking

5. **Monitoring**: Regularly check service health
   - Implement health check endpoints
   - Alert on service degradation
   - Monitor resource usage

## Troubleshooting

### DragonflyDB Connection Issues

```bash
# Test connection
redis-cli -h localhost -p 6379 -a changeme ping

# Check logs
docker logs shaman-dragonfly
```

### RabbitMQ Connection Issues

```bash
# Check RabbitMQ status
docker exec shaman-rabbitmq rabbitmq-diagnostics status

# View logs
docker logs shaman-rabbitmq

# Check queues
docker exec shaman-rabbitmq rabbitmqctl list_queues
```

### Common Issues

1. **Connection refused**: Ensure services are running (`docker-compose ps`)
2. **Authentication failed**: Check credentials in .env file
3. **Out of memory**: Adjust DragonflyDB maxmemory setting
4. **Queue backlog**: Scale up consumer workers

## Migration Guide

### From No Caching to DragonflyDB

1. Add service initialization to your startup code
2. Wrap expensive operations with cache checks
3. Monitor cache hit rates
4. Adjust TTLs based on usage patterns

### From Synchronous to Async Processing

1. Identify long-running operations
2. Convert to background jobs
3. Add job status tracking
4. Implement job result handling

## Future Enhancements

- [ ] Cache warming on startup
- [ ] Distributed cache invalidation
- [ ] Job scheduling with cron-like syntax
- [ ] Event replay capabilities
- [ ] Metrics integration with Prometheus
- [ ] Automatic job retry with exponential backoff
- [ ] Rate limiting for API-like operations
- [ ] Cache compression for large values
