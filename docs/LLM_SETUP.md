# LLM Integration Setup Guide

This guide explains how to enable and configure the LLM (Large Language Model) integration for dynamic AI behavior in Shaman's Journey.

## Overview

The game supports two LLM backends:
- **Placeholder Backend** (default): Rule-based responses, no model required
- **GGUF Backend** (optional): Local LLM inference via llama.cpp for truly dynamic AI

## Quick Start

### Using Placeholder (Default - No Setup Required)

The game works out of the box with intelligent rule-based responses:

```bash
cargo run --release
```

The placeholder backend provides:
- Context-aware dialogue templates
- Personality-driven boss combat decisions
- No external dependencies
- Instant response times

### Enabling GGUF Local LLM

#### Step 1: Install System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get install clang cmake build-essential
```

**macOS:**
```bash
brew install cmake
```

**Windows:**
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
- Install [CMake](https://cmake.org/download/)

#### Step 2: Download a GGUF Model

Download a quantized GGUF model (recommended: 7B parameter model with Q4_K_M quantization):

**Option A: Llama 2 (Recommended for testing)**
```bash
# Create models directory
mkdir -p models

# Download Llama 2 7B Q4 quantized model (~3.8GB)
wget https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGUF/resolve/main/llama-2-7b-chat.Q4_K_M.gguf \
     -O models/llama-2-7b-chat.Q4_K_M.gguf
```

**Option B: Other Models**
Browse models at: https://huggingface.co/models?search=gguf

Popular options:
- **Llama 3** - Latest from Meta (8B or 70B)
- **Mistral** - Fast and efficient (7B)
- **Zephyr** - Instruction-tuned (7B)
- **Phi-3** - Compact and capable (3.8B)

**Size Guidelines:**
- 3-4GB: Small models (Phi-3, quantized 7B) - Good for testing
- 4-5GB: Standard 7B Q4 - Recommended balance
- 6-8GB: 7B Q5/Q6 - Better quality
- 10GB+: 13B or larger - Best quality, requires more RAM

#### Step 3: Build with LLM Feature

```bash
cargo build --release --features llm
```

#### Step 4: Configure Model Path

Edit your game configuration to point to the model:

**Option A: Environment Variable**
```bash
export LLAMA_MODEL_PATH="models/llama-2-7b-chat.Q4_K_M.gguf"
cargo run --release --features llm
```

**Option B: Code Configuration**
Edit `crates/bevy_shaman_ai/src/resources.rs`:

```rust
impl Default for LlmModel {
    fn default() -> Self {
        Self {
            model_path: "models/llama-2-7b-chat.Q4_K_M.gguf".to_string(),
            is_loaded: true, // Set to true to enable
            config: ModelConfig::default(),
        }
    }
}
```

#### Step 5: Run the Game

```bash
cargo run --release --features llm
```

You should see:
```
INFO Loading GGUF model from: models/llama-2-7b-chat.Q4_K_M.gguf
INFO GGUF model loaded successfully
INFO Using GGUF Local Model backend
```

## Configuration

### Model Parameters

Edit `ModelConfig` in `crates/bevy_shaman_ai/src/resources.rs`:

```rust
pub struct ModelConfig {
    pub temperature: f32,      // 0.0 = deterministic, 1.0+ = creative
    pub max_tokens: Option<usize>,  // Max response length
    pub top_p: Option<f32>,    // Nucleus sampling (0.9 recommended)
    pub seed: Option<u32>,     // Random seed for reproducibility
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            temperature: 0.7,     // Balanced creativity
            max_tokens: Some(256), // Moderate responses
            top_p: Some(0.9),      // Focused sampling
            seed: None,            // Random each time
        }
    }
}
```

**Parameter Tuning:**

- **Boss Combat** (aggressive, tactical):
  ```rust
  temperature: 0.8  // More varied actions
  max_tokens: Some(128)  // Quick decisions
  ```

- **Dialogue** (wise, thoughtful):
  ```rust
  temperature: 0.6  // More consistent
  max_tokens: Some(512)  // Longer responses
  ```

- **Spirit Guide** (mystical, poetic):
  ```rust
  temperature: 0.9  // Creative phrasing
  max_tokens: Some(384)  // Moderate length
  ```

### Context Window

Adjust context size in `GgufBackend::new()`:

```rust
n_ctx: 2048,  // Standard - handles most dialogue
n_ctx: 4096,  // Large - for complex boss AI (more RAM)
n_ctx: 8192,  // Huge - for epic battles (lots of RAM)
```

**RAM Requirements:**
- 2048 context: ~2-3GB RAM
- 4096 context: ~4-5GB RAM
- 8192 context: ~8-10GB RAM

## Performance Optimization

### 1. Model Selection

**Fastest (Recommended for Gameplay):**
- Phi-3 Mini (3.8B) - 2-3 tokens/sec on CPU
- Llama 2 7B Q4_K_M - 5-8 tokens/sec on CPU

**Best Quality:**
- Llama 3 8B Q5_K_M - 3-5 tokens/sec on CPU
- Mistral 7B Q8_0 - 2-4 tokens/sec on CPU

### 2. Quantization Levels

- **Q2_K**: Smallest, fastest, lowest quality
- **Q4_K_M**: ⭐ Recommended - Best balance
- **Q5_K_M**: Better quality, slower
- **Q8_0**: Highest quality, slowest, largest

### 3. Response Caching

The game automatically caches LLM responses:

```rust
// In crates/bevy_shaman_ai/src/resources.rs
pub struct ResponseCache {
    cache: HashMap<String, CachedResponse>,
    max_size: usize,  // Default: 100 responses
}
```

This means repeated boss encounters or dialogue paths won't regenerate.

### 4. Async Generation (Planned)

Future versions will support async generation to avoid frame drops during inference.

## Troubleshooting

### "Model file not found"

**Solution:** Check the path is correct and file exists:
```bash
ls -lh models/llama-2-7b-chat.Q4_K_M.gguf
```

### "Backend init failed: clang not found"

**Solution:** Install clang (see Step 1 dependencies)

### "Out of memory" during load

**Solutions:**
1. Use a smaller model (Q2_K or Q4_K_S)
2. Reduce context window (`n_ctx: 1024`)
3. Close other applications
4. Add swap space

### Slow inference (< 1 token/sec)

**Solutions:**
1. Use smaller quantization (Q4_K_M instead of Q8)
2. Reduce context window
3. Use smaller model (7B instead of 13B)
4. Enable CPU optimizations:
   ```bash
   export OMP_NUM_THREADS=8  # Match your CPU cores
   ```

### Game freezes during generation

**Current Limitation:** Inference is synchronous and blocks the game thread.

**Workarounds:**
1. Use shorter `max_tokens` (128 instead of 512)
2. Use faster model quantization
3. Stick with placeholder backend for now

**Planned Fix:** Async inference system (see roadmap)

## Testing LLM Integration

### Basic Test

1. Start the game with llm feature
2. Trigger boss encounter
3. Watch console for:
   ```
   INFO Tokenized prompt: 156 tokens
   INFO Generated response: 84 tokens
   ```

### Dialogue Test

1. Talk to NPC
2. Check dialogue is contextual and varied
3. Compare with placeholder responses

### Combat AI Test

1. Fight boss with LLM enabled
2. Note combat decisions in console
3. Boss should use varied tactics based on health/state

## Prompt Engineering

The game uses specific prompts for different scenarios:

### Boss Combat Prompt Template
```
You are [boss_name], a powerful corrupted spirit.
Health: [health_percentage]%
Player threat: [player_power]
Choose your next action from:
- BasicAttack
- SpecialAttack: [ability_name]
- SummonMinion: [minion_type]
- ChangeStance: [stance]

Respond with JSON: {"action": "...", "taunt": "..."}
```

### NPC Dialogue Prompt Template
```
You are [npc_name], a [role] in the village.
Personality: [traits]
Context: [player_action]

Respond in character with 1-2 sentences.
```

You can customize these in `crates/bevy_shaman_ai/src/systems/*.rs`

## Architecture

```
┌─────────────────┐
│  Game Systems   │
└────────┬────────┘
         │
    ┌────▼────┐
    │ LlmModel│ (Resource)
    └────┬────┘
         │
  ┌──────▼──────┐
  │ Backend     │ (Trait)
  │ Factory     │
  └──────┬──────┘
         │
    ┌────┴────┐
    │         │
┌───▼──┐  ┌──▼──────┐
│Placeholder│ GgufBackend│
│Backend │  │(llama-cpp-2)│
└───────┘  └─────────┘
```

## Development Tips

### Enable Verbose Logging

```bash
RUST_LOG=bevy_shaman_ai=debug,llama_cpp_2=debug cargo run --features llm
```

### Benchmark Model Speed

```bash
time cargo run --release --features llm -- --test-llm
```

### Profile Memory Usage

```bash
/usr/bin/time -v cargo run --release --features llm
```

## Future Enhancements

Planned improvements (see PRODUCTION_ROADMAP.md):

1. ✨ **Async Generation** - Non-blocking inference
2. 🎯 **Streaming Responses** - Token-by-token output
3. 🧠 **Multi-Model Support** - Different models per use case
4. 💾 **Persistent Memory** - Boss remembers past encounters
5. 🔧 **Runtime Model Switching** - Change models without restart
6. ⚡ **GPU Acceleration** - CUDA/Metal support
7. 🌐 **Remote LLM Support** - OpenAI API, Anthropic, etc.

## Resources

- **llama-cpp-2 Documentation**: https://docs.rs/llama-cpp-2
- **GGUF Model Hub**: https://huggingface.co/models?search=gguf
- **llama.cpp GitHub**: https://github.com/ggerganov/llama.cpp
- **Prompt Engineering Guide**: https://www.promptingguide.ai/

## Support

If you encounter issues:

1. Check this documentation
2. Enable debug logging (`RUST_LOG=debug`)
3. Test with placeholder backend first
4. Open issue with:
   - Model name and size
   - System specs (CPU, RAM)
   - Console output
   - `cargo --version` and `rustc --version`

---

**Note**: LLM integration is experimental. The placeholder backend is production-ready and provides excellent gameplay. Only enable GGUF if you want to experiment with truly dynamic AI behavior.
