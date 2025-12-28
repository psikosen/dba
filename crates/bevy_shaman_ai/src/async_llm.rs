/// Async LLM Generation Implementation
///
/// This module provides async/await support for LLM generation to prevent
/// blocking the game loop during dialogue generation.
///
/// PERFORMANCE OPTIMIZATION: Prevents 100-500ms frame drops during NPC dialogue
/// by offloading LLM inference to background threads.

use async_channel::{Receiver, Sender};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

use crate::components::*;
use crate::llm_backend::{LlmBackend, LlmError};
use crate::resources::*;

// ============================================================================
// ASYNC LLM REQUEST/RESPONSE
// ============================================================================

#[derive(Debug, Clone)]
pub struct AsyncDialogueRequest {
    pub npc_entity: Entity,
    pub prompt: String,
    pub priority: RequestPriority,
    pub requested_at: f64,
}

#[derive(Debug, Clone)]
pub struct AsyncDialogueResponse {
    pub npc_entity: Entity,
    pub text: String,
    pub generation_time_ms: f32,
}

// ============================================================================
// ASYNC LLM MODEL WRAPPER
// ============================================================================

/// Async wrapper around LLM backend that processes requests in background threads
#[derive(Resource)]
pub struct AsyncLlmModel {
    request_tx: Sender<AsyncDialogueRequest>,
    response_rx: Receiver<AsyncDialogueResponse>,
    pending_requests: usize,
}

impl AsyncLlmModel {
    /// Create new async LLM model
    ///
    /// Note: This spawns a background worker thread that processes LLM requests
    pub fn new(backend: Box<dyn LlmBackend>, config: ModelConfig) -> Self {
        let (request_tx, request_rx) = async_channel::unbounded::<AsyncDialogueRequest>();
        let (response_tx, response_rx) = async_channel::unbounded::<AsyncDialogueResponse>();

        // Spawn background worker thread
        std::thread::spawn(move || {
            // Make backend thread-safe
            let backend = Arc::new(Mutex::new(backend));

            // Use tokio runtime for async processing
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2) // Limit to 2 threads for LLM processing
                .thread_name("llm-worker")
                .build()
                .expect("Failed to create tokio runtime");

            rt.block_on(async {
                while let Ok(request) = request_rx.recv().await {
                    let start_time = std::time::Instant::now();

                    // Process request in background
                    let mut backend_guard = backend.lock().unwrap();

                    let result = backend_guard.generate(&request.prompt, &config);
                    drop(backend_guard); // Release lock ASAP

                    let generation_time = start_time.elapsed().as_secs_f32() * 1000.0;

                    match result {
                        Ok(text) => {
                            let _ = response_tx.send(AsyncDialogueResponse {
                                npc_entity: request.npc_entity,
                                text,
                                generation_time_ms: generation_time,
                            }).await;

                            debug!(
                                "LLM dialogue generated in {:.1}ms (background thread)",
                                generation_time
                            );
                        }
                        Err(err) => {
                            warn!("LLM generation failed: {}", err);
                            // Send fallback response
                            let _ = response_tx.send(AsyncDialogueResponse {
                                npc_entity: request.npc_entity,
                                text: "...".to_string(),
                                generation_time_ms: generation_time,
                            }).await;
                        }
                    }
                }
            });
        });

        Self {
            request_tx,
            response_rx,
            pending_requests: 0,
        }
    }

    /// Queue a dialogue generation request (non-blocking)
    pub fn request_generation(&mut self, request: AsyncDialogueRequest) {
        if self.request_tx.try_send(request).is_ok() {
            self.pending_requests += 1;
        } else {
            warn!("Failed to queue LLM request - channel full");
        }
    }

    /// Poll for completed responses (non-blocking)
    /// Returns all responses available since last poll
    pub fn poll_responses(&mut self) -> Vec<AsyncDialogueResponse> {
        let responses: Vec<_> = self.response_rx.try_iter().collect();
        self.pending_requests = self.pending_requests.saturating_sub(responses.len());
        responses
    }

    /// Get number of pending requests
    pub fn pending_count(&self) -> usize {
        self.pending_requests
    }
}

// ============================================================================
// BEVY SYSTEMS FOR ASYNC LLM
// ============================================================================

/// Queue dialogue requests for async processing
pub fn queue_async_dialogue_requests(
    mut async_llm: ResMut<AsyncLlmModel>,
    mut request_events: EventReader<crate::systems::npc_dialogue::PlayerDialogueRequest>,
    time: Res<Time>,
) {
    for request in request_events.read() {
        async_llm.request_generation(AsyncDialogueRequest {
            npc_entity: request.npc_entity,
            prompt: request.prompt.clone(),
            priority: RequestPriority::Normal,
            requested_at: time.elapsed_secs_f64(),
        });

        info!("Queued async dialogue request (pending: {})", async_llm.pending_count());
    }
}

/// Collect completed dialogue responses
pub fn collect_async_dialogue_responses(
    mut async_llm: ResMut<AsyncLlmModel>,
    mut response_events: EventWriter<crate::systems::npc_dialogue::NpcDialogueResponse>,
) {
    for response in async_llm.poll_responses() {
        response_events.send(crate::systems::npc_dialogue::NpcDialogueResponse {
            npc_entity: response.npc_entity,
            text: response.text.clone(),
        });

        debug!(
            "Received async dialogue response ({:.1}ms generation time)",
            response.generation_time_ms
        );
    }
}

// ============================================================================
// COMPONENT FOR TRACKING GENERATION STATE
// ============================================================================

/// Component to show "thinking..." indicator while dialogue generates
#[derive(Component)]
pub struct DialogueGenerating {
    pub elapsed: f32,
    pub dots: usize,
}

impl Default for DialogueGenerating {
    fn default() -> Self {
        Self {
            elapsed: 0.0,
            dots: 0,
        }
    }
}

/// Update thinking indicators
pub fn update_thinking_indicators(
    time: Res<Time>,
    mut query: Query<&mut DialogueGenerating>,
) {
    for mut indicator in query.iter_mut() {
        indicator.elapsed += time.delta_secs();

        if indicator.elapsed > 0.5 {
            indicator.elapsed = 0.0;
            indicator.dots = (indicator.dots + 1) % 4;
        }
    }
}

// ============================================================================
// METRICS
// ============================================================================

/// Metrics for tracking async LLM performance
#[derive(Resource, Default)]
pub struct AsyncLlmMetrics {
    pub requests_queued: u64,
    pub requests_completed: u64,
    pub total_generation_time_ms: f32,
    pub max_generation_time_ms: f32,
    pub min_generation_time_ms: f32,
}

impl AsyncLlmMetrics {
    pub fn record_request(&mut self) {
        self.requests_queued += 1;
    }

    pub fn record_completion(&mut self, generation_time_ms: f32) {
        self.requests_completed += 1;
        self.total_generation_time_ms += generation_time_ms;

        if self.requests_completed == 1 {
            self.max_generation_time_ms = generation_time_ms;
            self.min_generation_time_ms = generation_time_ms;
        } else {
            self.max_generation_time_ms = self.max_generation_time_ms.max(generation_time_ms);
            self.min_generation_time_ms = self.min_generation_time_ms.min(generation_time_ms);
        }
    }

    pub fn average_generation_time_ms(&self) -> f32 {
        if self.requests_completed > 0 {
            self.total_generation_time_ms / self.requests_completed as f32
        } else {
            0.0
        }
    }
}
