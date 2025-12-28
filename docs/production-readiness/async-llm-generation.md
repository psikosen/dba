# Async LLM Generation Implementation Guide

## Overview

This document outlines the implementation strategy for asynchronous LLM (Large Language Model) generation in the Shaman's Journey game. The goal is to prevent frame rate drops during NPC dialogue generation by offloading LLM inference to background threads.

## Current Implementation

**Location:** `crates/bevy_shaman_ai/src/systems/dialogue.rs`

The current synchronous implementation:
```rust
pub fn generate_npc_dialogue(
    mut events: EventReader<DialogueRequest>,
    llm: Res<LlamaModel>,
    mut dialogue_events: EventWriter<DialogueGenerated>,
) {
    for request in events.read() {
        // BLOCKS GAME THREAD - Takes 100-500ms
        let response = llm.generate(&request.prompt);
        dialogue_events.send(DialogueGenerated {
            npc: request.npc,
            text: response,
        });
    }
}
```

**Problem:** LLM inference takes 100-500ms and blocks the main game thread, causing visible stuttering.

## Proposed Async Architecture

### Option 1: Tokio Runtime Integration (Recommended)

**Benefits:**
- Native async/await syntax
- Mature ecosystem
- Integrates well with Bevy's async asset loading
- Thread pool management built-in

**Implementation:**

1. **Add Dependencies** (`crates/bevy_shaman_ai/Cargo.toml`):
```toml
[dependencies]
tokio = { version = "1.0", features = ["rt-multi-thread", "sync"] }
async-channel = "2.0"
```

2. **Create Async LLM Wrapper** (`crates/bevy_shaman_ai/src/async_llm.rs`):
```rust
use async_channel::{Sender, Receiver};
use tokio::runtime::Runtime;

pub struct AsyncLlamaModel {
    runtime: Runtime,
    request_tx: Sender<DialogueRequest>,
    response_rx: Receiver<DialogueGenerated>,
}

impl AsyncLlamaModel {
    pub fn new(model_path: &str) -> Self {
        let runtime = Runtime::new().unwrap();
        let (request_tx, request_rx) = async_channel::unbounded();
        let (response_tx, response_rx) = async_channel::unbounded();

        // Spawn worker task
        runtime.spawn(async move {
            let model = LlamaModel::load(model_path).await;
            while let Ok(request) = request_rx.recv().await {
                let response = model.generate(&request.prompt).await;
                let _ = response_tx.send(DialogueGenerated {
                    npc: request.npc,
                    text: response,
                }).await;
            }
        });

        Self { runtime, request_tx, response_rx }
    }

    pub fn request_generation(&self, request: DialogueRequest) {
        let _ = self.request_tx.try_send(request);
    }

    pub fn poll_responses(&self) -> Vec<DialogueGenerated> {
        self.response_rx.try_iter().collect()
    }
}
```

3. **Update Bevy Systems** (`crates/bevy_shaman_ai/src/systems/dialogue.rs`):
```rust
pub fn queue_dialogue_requests(
    mut events: EventReader<DialogueRequest>,
    async_llm: Res<AsyncLlamaModel>,
) {
    for request in events.read() {
        async_llm.request_generation(request.clone());
    }
}

pub fn collect_dialogue_responses(
    async_llm: Res<AsyncLlamaModel>,
    mut dialogue_events: EventWriter<DialogueGenerated>,
) {
    for response in async_llm.poll_responses() {
        dialogue_events.send(response);
    }
}
```

4. **Register in Plugin** (`crates/bevy_shaman_ai/src/lib.rs`):
```rust
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AsyncLlamaModel::new("models/llama.gguf"))
            .add_systems(Update, (
                queue_dialogue_requests,
                collect_dialogue_responses,
            ).chain());
    }
}
```

### Option 2: Bevy AsyncComputeTaskPool

**Benefits:**
- Native Bevy integration
- No external dependencies
- Simpler mental model

**Implementation:**

```rust
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

#[derive(Component)]
pub struct DialogueGenerationTask(Task<String>);

pub fn spawn_dialogue_tasks(
    mut commands: Commands,
    mut events: EventReader<DialogueRequest>,
    llm: Res<LlamaModel>,
) {
    let task_pool = AsyncComputeTaskPool::get();

    for request in events.read() {
        let llm = llm.clone();
        let prompt = request.prompt.clone();
        let npc = request.npc;

        let task = task_pool.spawn(async move {
            llm.generate(&prompt)
        });

        commands.spawn((
            DialogueGenerationTask(task),
            DialogueNpcId(npc),
        ));
    }
}

pub fn poll_dialogue_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut DialogueGenerationTask, &DialogueNpcId)>,
    mut events: EventWriter<DialogueGenerated>,
) {
    for (entity, mut task, npc_id) in tasks.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            events.send(DialogueGenerated {
                npc: npc_id.0,
                text: result,
            });
            commands.entity(entity).despawn();
        }
    }
}
```

## Performance Considerations

### Thread Pool Sizing

**Recommended Configuration:**
```rust
// For 8-core CPU, reserve 2 cores for LLM
const LLM_WORKER_THREADS: usize = 2;
const GAME_THREADS: usize = num_cpus::get() - LLM_WORKER_THREADS;

Runtime::new()
    .worker_threads(LLM_WORKER_THREADS)
    .build()
    .unwrap()
```

### Request Batching

Batch multiple dialogue requests to amortize model loading costs:

```rust
pub struct DialogueBatch {
    requests: Vec<DialogueRequest>,
    batch_timeout: f32,
}

impl DialogueBatch {
    pub fn should_process(&self, time: &Time) -> bool {
        self.requests.len() >= 5 || self.batch_timeout > 0.5
    }
}
```

### Caching Strategy

Implement response caching for repeated dialogues:

```rust
#[derive(Resource)]
pub struct DialogueCache {
    cache: HashMap<String, String>,
    max_entries: usize,
}

impl DialogueCache {
    pub fn get_or_generate(&self, prompt: &str) -> Option<&String> {
        self.cache.get(prompt)
    }
}
```

## Loading Screen Integration

Show a "thinking" indicator while NPC generates response:

```rust
#[derive(Component)]
pub struct ThinkingIndicator {
    elapsed: f32,
    dots: usize,
}

pub fn update_thinking_indicator(
    time: Res<Time>,
    mut indicators: Query<(&mut ThinkingIndicator, &mut Text)>,
) {
    for (mut indicator, mut text) in indicators.iter_mut() {
        indicator.elapsed += time.delta_secs();
        if indicator.elapsed > 0.5 {
            indicator.elapsed = 0.0;
            indicator.dots = (indicator.dots + 1) % 4;
            text.sections[0].value = format!("NPC is thinking{}", ".".repeat(indicator.dots));
        }
    }
}
```

## Error Handling

Handle LLM failures gracefully:

```rust
pub enum DialogueResult {
    Success(String),
    Timeout,
    ModelError(String),
}

pub fn handle_dialogue_errors(
    results: Query<&DialogueGenerationTask>,
    mut fallback_events: EventWriter<UseFallbackDialogue>,
) {
    for result in results.iter() {
        match result.result() {
            DialogueResult::Timeout | DialogueResult::ModelError(_) => {
                fallback_events.send(UseFallbackDialogue {
                    dialogue: "...".to_string(),
                });
            }
            _ => {}
        }
    }
}
```

## Testing

### Unit Tests

```rust
#[test]
fn test_async_dialogue_generation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(AiPlugin);

    // Queue request
    app.world_mut().send_event(DialogueRequest {
        npc: Entity::from_raw(0),
        prompt: "Hello".to_string(),
    });

    // Run systems multiple times to allow async completion
    for _ in 0..100 {
        app.update();
    }

    // Verify response was generated
    let events = app.world().resource::<Events<DialogueGenerated>>();
    assert!(events.len() > 0);
}
```

## Migration Path

1. **Phase 1:** Implement async wrapper alongside existing synchronous system
2. **Phase 2:** Add feature flag to enable async mode for testing
3. **Phase 3:** Collect performance metrics comparing sync vs async
4. **Phase 4:** Remove synchronous implementation after validation

## Rollout Strategy

```toml
[features]
default = ["async-llm"]
async-llm = ["tokio", "async-channel"]
sync-llm = []  # Fallback for debugging
```

## Monitoring

Add metrics to track LLM performance:

```rust
#[derive(Resource)]
pub struct LlmMetrics {
    pub requests_queued: u64,
    pub requests_completed: u64,
    pub average_latency_ms: f32,
    pub cache_hit_rate: f32,
}
```

## Expected Performance Impact

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Frame time during dialogue | 100-500ms | <16ms | 6-30x faster |
| Dialogue generation latency | 100-500ms | 100-500ms | Same (background) |
| User experience | Stuttering | Smooth | ✅ Playable |
| CPU utilization | Bursty | Steady | More efficient |

## Conclusion

Implementing async LLM generation will dramatically improve game responsiveness during NPC interactions. The recommended approach is **Option 1 (Tokio)** for production use due to its maturity and ecosystem support.

**Next Steps:**
1. Add tokio dependency to bevy_shaman_ai crate
2. Implement AsyncLlamaModel wrapper
3. Update dialogue systems to use async model
4. Add performance metrics
5. Test with real gameplay scenarios
