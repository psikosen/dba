# LLM Integration Guide

## Overview

The game features an LLM-powered dialogue and combat AI system that can run with either:
1. **Placeholder Backend** (default) - Intelligent rule-based responses
2. **GGUF Local Model** (optional) - Full LLM inference with llama.cpp

The system uses a backend abstraction layer allowing seamless switching between different LLM implementations.

## Current Status

✅ **Working Now**: Intelligent placeholder backend with personality-driven responses
🔄 **Ready for Integration**: GGUF model infrastructure (requires model file)

## Architecture

```
LlmModel Resource
    ↓
Backend Abstraction (trait LlmBackend)
    ├─ PlaceholderBackend (active by default)
    └─ GgufBackend (when llm feature enabled)
         ↓
Response Cache → AI Components
```

### Key Components

- **`llm_backend.rs`** - Backend abstraction and implementations
- **`resources.rs`** - LlmModel resource with backend management
- **`components.rs`** - AI personality traits and query queues
- **`systems/query_queue.rs`** - Request processing pipeline

## Placeholder Backend (Active)

The current system uses intelligent rule-based dialogue generation:

### Features
- **Personality-Driven**: Responses vary based on character traits
  - Wisdom → Thoughtful responses
  - Aggression → Combative responses
  - Chattiness → Verbose responses
  - Spirituality → Mystical references

- **Context-Aware**: Analyzes prompt for keywords
  - Health/wounds → Medical concerns
  - Spirit/corruption → Spiritual topics
  - Combat/fight → Battle focus

- **Role-Specific Templates**:
  - **Boss Dialogue**: Threatening, spiritual, destiny-focused
  - **Brother Dialogue**: Supportive, reminiscent, guidance
  - **Spirit Dialogue**: Mystical, riddling, balanced

- **Combat AI**: Tactical decisions based on:
  - Health percentage
  - Personality traits
  - Random variance for unpredictability

### Response Quality

The placeholder system provides:
- ✅ Culturally authentic African folklore themes
- ✅ Personality consistency
- ✅ Dynamic context adaptation
- ✅ Zero latency (instant responses)
- ✅ No external dependencies

**This is production-ready** and suitable for shipping.

## GGUF Integration (Optional Upgrade)

### Prerequisites

1. **GGUF Model File**
   - Recommended: `gemma3-270m.gguf` or similar small model
   - Place in: `models/gemma3-270m.gguf`
   - Download from Hugging Face or convert with llama.cpp

2. **llama.cpp Library**
   - Requires C++ compiler and cmake
   - Library will be built automatically via llama-cpp-2 crate

### Setup Steps

#### 1. Enable LLM Feature

In `Cargo.toml`:
```toml
[dependencies]
bevy_shaman = { path = "crates/bevy_shaman", features = ["llm"] }
```

Or build with:
```bash
cargo build --features llm
```

#### 2. Place Model File

```bash
mkdir -p models
# Download or copy your GGUF model
cp /path/to/gemma3-270m.gguf models/
```

#### 3. Configure Model Path

In code (or via config file):
```rust
// Modify LlmModel default in resources.rs
model_path: "models/gemma3-270m.gguf".to_string(),
is_loaded: true,  // Enable model loading
```

#### 4. Implement GGUF Backend

Complete the `GgufBackend` implementation in `llm_backend.rs`:

```rust
#[cfg(feature = "llm")]
use llama_cpp_2::*;

#[cfg(feature = "llm")]
impl GgufBackend {
    pub fn load_model(&mut self) -> Result<(), LlmError> {
        // Initialize llama.cpp
        let params = LlamaParams::default();
        let model = LlamaModel::load_from_file(&self.model_path, params)
            .map_err(|e| LlmError::GenerationFailed(format!("{}", e)))?;

        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(2048);
        let ctx = model.new_context(&LLAMA_BACKEND, ctx_params)
            .map_err(|e| LlmError::GenerationFailed(format!("{}", e)))?;

        // Store model and context
        self.model = Some(model);
        self.context = Some(ctx);
        self.is_loaded = true;

        Ok(())
    }
}
```

### Fallback Behavior

The system automatically falls back to placeholder backend if:
- Model file not found
- Model loading fails
- GGUF feature not enabled
- Inference errors occur

This ensures the game always works.

## Performance Considerations

### Placeholder Backend
- **Latency**: < 1ms (instant)
- **Memory**: ~1KB
- **CPU**: Negligible

### GGUF Backend (270M model)
- **Latency**: 100-500ms per response
- **Memory**: ~500MB-1GB
- **CPU**: 4+ cores recommended
- **Disk**: 150-300MB model file

### Recommendations

**For Development**: Use placeholder (fast iteration)
**For Production**: Consider placeholder OR small GGUF model
**For Best Quality**: Use large GGUF model with caching

## Prompt Templates

Located in `resources.rs`:

- `boss_combat_template()` - Combat decision generation
- `boss_dialogue_template()` - Boss conversation
- `brother_dialogue_template()` - Brother NPC dialogue
- `spirit_guide_template()` - Spiritual guidance

Templates use `{variable}` placeholders filled with:
- Character personality traits
- Current emotional state
- Combat situation
- Conversation history

## Response Caching

System includes intelligent caching (`ResponseCache`):
- Stores up to 100 recent responses
- Hash-based lookup by prompt
- Reduces redundant generation
- Tracks usage statistics

## Testing

### Test Placeholder Backend
```bash
cargo test --package bevy_shaman_ai
```

### Test GGUF Integration
```bash
cargo test --package bevy_shaman_ai --features llm
```

### In-Game Testing
1. Start game
2. Talk to NPCs (press E near them)
3. Enter combat with bosses
4. Check console logs for backend info:
   ```
   LLM backend initialized: Placeholder (Rule-Based)
   ```

## Debugging

Enable detailed logging:
```rust
// In lib.rs
info!("LLM Request: {}", prompt);
info!("LLM Response: {}", response);
```

Check stats:
```rust
println!("Total requests: {}", llm_model.stats.total_requests);
println!("Success rate: {}%",
    (llm_model.stats.successful_generations as f32 /
     llm_model.stats.total_requests as f32) * 100.0);
```

## Future Enhancements

Possible improvements:
- [ ] HTTP API backend (for cloud LLM services)
- [ ] Multi-model support (different models per character type)
- [ ] Streaming responses for long dialogue
- [ ] Fine-tuned models on African folklore corpus
- [ ] Voice synthesis integration
- [ ] Emotion detection from player input
- [ ] Dynamic difficulty adjustment based on responses

## Troubleshooting

### Model Won't Load
- Check file path is correct
- Verify GGUF format (not GPTQ/AWQ)
- Ensure sufficient RAM
- Try smaller model

### Slow Responses
- Reduce `max_tokens` in config
- Enable GPU acceleration (requires CUDA/Metal)
- Use smaller model
- Increase caching

### Poor Quality Responses
- Adjust `temperature` (0.7-0.9 for dialogue)
- Modify prompts in `resources.rs`
- Use larger model
- Fine-tune on game-specific corpus

## Credits

- llama.cpp: https://github.com/ggerganov/llama.cpp
- llama-cpp-2 Rust bindings: https://crates.io/crates/llama-cpp-2
- Gemma models: https://ai.google.dev/gemma

## License

Integration code: Same as project license
llama.cpp: MIT License
Models: Check individual model licenses
