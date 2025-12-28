# Services Quick Start Guide

Quick guide to get DragonflyDB and RabbitMQ running with Shaman's Journey.

## Prerequisites

- Docker and Docker Compose
- Rust toolchain (for development)

## Start Services

### 1. Configure Environment

Copy the example environment file:
```bash
cp .env.example .env
```

Edit `.env` and set secure passwords:
```bash
# DragonflyDB
DRAGONFLY_PASSWORD=your_secure_password_here

# RabbitMQ
RABBITMQ_USER=admin
RABBITMQ_PASSWORD=your_secure_password_here
```

### 2. Start All Services

Start DragonflyDB, RabbitMQ, and the game:
```bash
docker-compose up -d
```

Or start services individually:
```bash
# Start only cache and queue
docker-compose up -d dragonfly rabbitmq

# Start the game
docker-compose up -d shaman-journey
```

### 3. Verify Services

Check service status:
```bash
docker-compose ps
```

All services should show as "healthy":
```
NAME                  STATUS
shaman-dragonfly      Up (healthy)
shaman-rabbitmq       Up (healthy)
shaman-journey-game   Up (healthy)
```

### 4. Access Management UIs

**RabbitMQ Management Console**:
- URL: http://localhost:15672
- Username: `guest` (or your RABBITMQ_USER)
- Password: `guest` (or your RABBITMQ_PASSWORD)

**DragonflyDB** (via redis-cli):
```bash
docker exec -it shaman-dragonfly redis-cli -a your_password
```

## Usage Examples

### Check Cache Stats

Using redis-cli:
```bash
docker exec -it shaman-dragonfly redis-cli -a changeme

# Inside redis-cli
INFO stats
KEYS *
```

### Monitor Queues

Using RabbitMQ CLI:
```bash
# List queues
docker exec shaman-rabbitmq rabbitmqctl list_queues

# List exchanges
docker exec shaman-rabbitmq rabbitmqctl list_exchanges

# List bindings
docker exec shaman-rabbitmq rabbitmqctl list_bindings
```

Or use the web UI at http://localhost:15672

### View Logs

```bash
# DragonflyDB logs
docker logs shaman-dragonfly

# RabbitMQ logs
docker logs shaman-rabbitmq

# Game logs
docker logs shaman-journey-game
```

## Development Workflow

### Local Development (Without Docker)

1. Start DragonflyDB locally:
```bash
docker run -d \
  -p 6379:6379 \
  -v dragonfly-data:/data \
  --name dragonfly \
  docker.dragonflydb.io/dragonflydb/dragonfly:latest
```

2. Start RabbitMQ locally:
```bash
docker run -d \
  -p 5672:5672 \
  -p 15672:15672 \
  --name rabbitmq \
  rabbitmq:3.13-management-alpine
```

3. Set environment variables:
```bash
export DRAGONFLY_URL=redis://localhost:6379
export RABBITMQ_URL=amqp://guest:guest@localhost:5672/%2f
```

4. Run the game:
```bash
cargo run --release
```

### Testing Services

Run service tests:
```bash
# Test the services crate
cargo test -p bevy_shaman_services

# Run all tests
cargo test
```

## Common Tasks

### Clear All Caches

```bash
docker exec -it shaman-dragonfly redis-cli -a changeme FLUSHALL
```

### Purge All Queues

```bash
docker exec shaman-rabbitmq rabbitmqctl purge_queue background_jobs
docker exec shaman-rabbitmq rabbitmqctl purge_queue offline_jobs
```

### Restart Services

```bash
# Restart all
docker-compose restart

# Restart individual services
docker-compose restart dragonfly
docker-compose restart rabbitmq
```

### Stop Services

```bash
# Stop all
docker-compose down

# Stop and remove volumes (WARNING: deletes data)
docker-compose down -v
```

## Troubleshooting

### Service Won't Start

Check logs:
```bash
docker-compose logs dragonfly
docker-compose logs rabbitmq
```

Common issues:
- Port already in use (6379, 5672, 15672)
- Insufficient permissions
- Out of disk space

### Connection Errors

1. Verify services are running:
```bash
docker-compose ps
```

2. Check network connectivity:
```bash
docker network inspect dba_shaman-network
```

3. Test connection:
```bash
# DragonflyDB
docker exec shaman-dragonfly redis-cli -a changeme PING

# RabbitMQ
docker exec shaman-rabbitmq rabbitmq-diagnostics ping
```

### High Memory Usage

Check DragonflyDB memory:
```bash
docker exec -it shaman-dragonfly redis-cli -a changeme INFO memory
```

Adjust max memory in `docker-compose.yml`:
```yaml
command: >
  dragonfly
  --requirepass ${DRAGONFLY_PASSWORD:-changeme}
  --maxmemory 1gb  # Increase from 512mb
```

### Queue Backlog

Check queue depth:
```bash
docker exec shaman-rabbitmq rabbitmqctl list_queues name messages
```

Solutions:
- Scale up consumer workers
- Increase processing timeout
- Check for stuck consumers

## Performance Tuning

### DragonflyDB

Increase memory limit:
```yaml
# In docker-compose.yml
command: >
  dragonfly
  --requirepass ${DRAGONFLY_PASSWORD:-changeme}
  --maxmemory 2gb
  --save_schedule "0:30"  # Snapshot every 30 minutes
```

### RabbitMQ

Add resource limits:
```yaml
# In docker-compose.yml
rabbitmq:
  environment:
    - RABBITMQ_VM_MEMORY_HIGH_WATERMARK=1GB
    - RABBITMQ_DISK_FREE_LIMIT=2GB
```

## Integration Examples

### Add to Existing Bevy System

```rust
use bevy::prelude::*;
use bevy_shaman_services::{Services, ServiceConfig, initialize_services};

#[derive(Resource)]
struct GameServices(Services);

fn setup_services(mut commands: Commands) {
    // Initialize services
    let config = ServiceConfig::from_env();

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            match initialize_services(config).await {
                Ok(services) => {
                    commands.insert_resource(GameServices(services));
                    info!("Services initialized");
                }
                Err(e) => {
                    error!("Failed to initialize services: {}", e);
                }
            }
        });
}

fn use_cache_system(services: Res<GameServices>) {
    // Use services in your systems
    let cache = &services.0.llm_cache;
    // ...
}
```

## Next Steps

1. Review [SERVICES_ARCHITECTURE.md](SERVICES_ARCHITECTURE.md) for detailed documentation
2. Check example integrations in the codebase
3. Monitor service health and performance
4. Adjust cache TTLs based on your needs
5. Set up monitoring and alerting for production

## Support

For issues or questions:
- Check the main [README.md](README.md)
- Review service logs
- See [SERVICES_ARCHITECTURE.md](SERVICES_ARCHITECTURE.md) for troubleshooting
