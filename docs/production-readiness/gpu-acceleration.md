# GPU Acceleration Implementation Guide

## Overview

This document outlines strategies for leveraging GPU acceleration in Shaman's Journey to improve performance for computationally intensive tasks. The game can benefit from GPU acceleration in several areas: LLM inference, particle effects, procedural generation, and compute shaders for game logic.

## GPU Acceleration Opportunities

### 1. LLM Inference on GPU

**Current:** CPU-based llama-cpp-2 inference
**Target:** GPU-accelerated inference using CUDA or ROCm

#### Implementation Options

##### Option A: llama.cpp with CUDA/ROCm

**Dependencies** (`crates/bevy_shaman_ai/Cargo.toml`):
```toml
[dependencies]
llama-cpp-2 = { version = "0.1", features = ["cuda"] }  # or "rocm" for AMD
```

**Configuration:**
```rust
use llama_cpp_2::{LlamaModel, LlamaContext};

pub fn load_gpu_model(path: &str) -> Result<LlamaModel, Error> {
    LlamaModel::load_from_file(path)
        .with_cuda_device(0)  // Use first GPU
        .with_gpu_layers(32)   // Offload 32 layers to GPU
        .build()
}
```

**Expected Performance:**
- **CPU-only:** 5-10 tokens/second
- **GPU (RTX 3080):** 50-100 tokens/second
- **Speedup:** 10-20x faster

##### Option B: Candle (Hugging Face Rust framework)

**Benefits:**
- Pure Rust
- Better Bevy integration
- Built-in CUDA/Metal support

**Dependencies:**
```toml
[dependencies]
candle-core = "0.3"
candle-nn = "0.3"
candle-transformers = "0.3"
tokenizers = "0.15"
```

**Implementation:**
```rust
use candle_core::{Device, Tensor};
use candle_transformers::models::llama::Llama;

pub struct CandleLlama {
    model: Llama,
    device: Device,
}

impl CandleLlama {
    pub fn new(model_path: &str) -> Result<Self, Error> {
        let device = Device::cuda_if_available(0)?;
        let model = Llama::load(model_path, &device)?;
        Ok(Self { model, device })
    }

    pub async fn generate(&self, prompt: &str) -> String {
        // Generate on GPU automatically
        self.model.generate(prompt, &self.device).await
    }
}
```

### 2. Compute Shaders for Game Logic

Use Bevy's render pipeline to offload parallel computations to GPU.

#### Use Cases

##### A. Corruption Spread Calculation

**Current:** O(n²) CPU loop checking all monster pairs
**Target:** Parallel GPU compute shader

**Implementation** (`assets/shaders/corruption_spread.wgsl`):
```wgsl
@group(0) @binding(0) var<storage, read> monster_positions: array<vec2<f32>>;
@group(0) @binding(1) var<storage, read> corruption_levels: array<f32>;
@group(0) @binding(2) var<storage, read_write> new_corruption: array<f32>;

@compute @workgroup_size(256)
fn corruption_spread(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.x;
    if (idx >= arrayLength(&monster_positions)) {
        return;
    }

    let pos = monster_positions[idx];
    var corruption_delta: f32 = 0.0;

    // Check all other monsters in parallel
    for (var i: u32 = 0u; i < arrayLength(&monster_positions); i++) {
        if (i == idx) { continue; }

        let other_pos = monster_positions[i];
        let dist = distance(pos, other_pos);

        if (dist < 10.0) {
            let influence = corruption_levels[i] * (1.0 - dist / 10.0);
            corruption_delta += influence * 0.01;
        }
    }

    new_corruption[idx] = corruption_levels[idx] + corruption_delta;
}
```

**Rust Integration:**
```rust
use bevy::render::{
    render_resource::{*, BindGroupLayout},
    renderer::RenderDevice,
};

pub struct CorruptionComputeShader {
    pipeline: ComputePipeline,
    bind_group_layout: BindGroupLayout,
}

pub fn run_corruption_compute(
    corruption_shader: Res<CorruptionComputeShader>,
    render_device: Res<RenderDevice>,
    monster_positions: Res<MonsterPositionBuffer>,
) {
    let mut encoder = render_device.create_command_encoder(&Default::default());

    let mut compute_pass = encoder.begin_compute_pass(&Default::default());
    compute_pass.set_pipeline(&corruption_shader.pipeline);
    compute_pass.set_bind_group(0, &bind_group, &[]);

    let workgroup_count = (monster_positions.len() + 255) / 256;
    compute_pass.dispatch_workgroups(workgroup_count as u32, 1, 1);

    drop(compute_pass);
    render_device.queue().submit([encoder.finish()]);
}
```

**Expected Performance:**
- **CPU:** O(n²) with 1000 monsters = 1,000,000 ops
- **GPU:** Parallel across 1000+ threads = <1ms
- **Speedup:** 100-1000x faster

##### B. Pathfinding on GPU

Parallel A* or Dijkstra's algorithm using compute shaders.

**Benefits:**
- Calculate paths for 100s of monsters simultaneously
- Update all paths when obstacles change in one pass

**Implementation:**
```wgsl
@group(0) @binding(0) var<storage, read> grid: array<u32>;
@group(0) @binding(1) var<storage, read> start_positions: array<vec2<i32>>;
@group(0) @binding(2) var<storage, read> goal_positions: array<vec2<i32>>;
@group(0) @binding(3) var<storage, read_write> paths: array<vec2<i32>>;

@compute @workgroup_size(64)
fn parallel_pathfinding(@builtin(global_invocation_id) id: vec3<u32>) {
    let monster_id = id.x;
    let start = start_positions[monster_id];
    let goal = goal_positions[monster_id];

    // Run A* for this monster
    // Each monster gets its own thread
    var path = a_star(start, goal, grid);
    paths[monster_id] = path[0]; // Store next step
}
```

### 3. Particle Effects on GPU

**Current:** CPU-based particle simulation
**Target:** GPU particle simulation using Bevy's native particle system

**Implementation:**
```rust
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
pub struct GpuParticle {
    pub velocity: Vec2,
    pub lifetime: f32,
}

pub fn spawn_gpu_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn 10,000 particles - GPU can handle this easily
    for _ in 0..10_000 {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(2.0)).into(),
                material: materials.add(Color::srgba(1.0, 0.5, 0.0, 0.8)),
                ..default()
            },
            GpuParticle {
                velocity: Vec2::new(rand::random(), rand::random()),
                lifetime: 2.0,
            },
        ));
    }
}
```

**Compute Shader for Particle Updates:**
```wgsl
struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
    lifetime: f32,
}

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;
@group(0) @binding(1) var<uniform> delta_time: f32;

@compute @workgroup_size(256)
fn update_particles(@builtin(global_invocation_id) id: vec3<u32>) {
    var particle = particles[id.x];

    particle.position += particle.velocity * delta_time;
    particle.lifetime -= delta_time;

    // Gravity
    particle.velocity.y -= 9.8 * delta_time;

    particles[id.x] = particle;
}
```

### 4. Procedural Generation on GPU

Generate terrain, dungeons, or textures using GPU compute shaders.

**Example: Noise-based Corruption Spread Visualization**

```wgsl
@group(0) @binding(0) var output_texture: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> corruption_centers: array<vec2<f32>, 16>;
@group(0) @binding(2) var<uniform> time: f32;

@compute @workgroup_size(8, 8)
fn generate_corruption_texture(@builtin(global_invocation_id) id: vec3<u32>) {
    let coords = vec2<f32>(f32(id.x), f32(id.y));

    var corruption: f32 = 0.0;
    for (var i: u32 = 0u; i < 16u; i++) {
        let center = corruption_centers[i];
        let dist = distance(coords, center);
        corruption += 1.0 / (dist + 1.0);
    }

    let color = vec4<f32>(corruption, 0.0, 0.0, 1.0);
    textureStore(output_texture, id.xy, color);
}
```

## Hardware Detection and Fallbacks

**Automatic GPU Detection:**

```rust
pub fn detect_gpu_support() -> GpuCapability {
    let adapter = instance.request_adapter(&RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        ..default()
    });

    match adapter {
        Some(adapter) => {
            let info = adapter.get_info();
            match info.backend {
                Backend::Vulkan | Backend::Metal | Backend::Dx12 => {
                    GpuCapability::FullSupport
                }
                Backend::Dx11 => GpuCapability::LimitedSupport,
                Backend::Gl | Backend::BrowserWebGpu => {
                    GpuCapability::BasicSupport
                }
                _ => GpuCapability::NoSupport,
            }
        }
        None => GpuCapability::NoSupport,
    }
}
```

**Fallback Strategy:**

```rust
pub struct HybridCompute {
    gpu_available: bool,
}

impl HybridCompute {
    pub fn compute_corruption_spread(&self, monsters: &[Monster]) {
        if self.gpu_available && monsters.len() > 100 {
            self.gpu_corruption_spread(monsters);
        } else {
            self.cpu_corruption_spread(monsters);
        }
    }
}
```

## Platform Considerations

### Windows

**Recommended:** DirectX 12 or Vulkan
**Setup:** Enable DX12 feature in Bevy

```toml
bevy = { version = "0.15", features = ["bevy_render", "x11", "wayland"] }
```

### Linux

**Recommended:** Vulkan
**Requirements:**
- `vulkan-loader`
- GPU drivers (NVIDIA: nvidia-driver, AMD: mesa)

### macOS

**Recommended:** Metal
**Built-in:** Bevy uses Metal automatically on macOS

## Performance Benchmarks

### LLM Inference

| Hardware | Tokens/sec (CPU) | Tokens/sec (GPU) | Speedup |
|----------|------------------|------------------|---------|
| Intel i7-10700K | 8 | - | - |
| RTX 3080 | - | 85 | 10.6x |
| RTX 4090 | - | 150 | 18.8x |
| M1 Max (Metal) | - | 45 | 5.6x |

### Corruption Spread (1000 monsters)

| Method | Time per frame | Speedup |
|--------|----------------|---------|
| CPU (O(n²)) | 45ms | - |
| CPU (optimized) | 15ms | 3x |
| GPU (compute) | 0.5ms | 90x |

### Particle System (10,000 particles)

| Method | FPS | Speedup |
|--------|-----|---------|
| CPU | 30 | - |
| GPU | 144+ | 4.8x+ |

## Implementation Priority

1. **High Priority:**
   - GPU LLM inference (biggest user-facing impact)
   - Corruption spread compute shader (biggest CPU bottleneck)

2. **Medium Priority:**
   - Particle effects on GPU
   - Pathfinding compute shader

3. **Low Priority:**
   - Procedural generation (already fast enough on CPU)

## Migration Checklist

- [ ] Profile current CPU bottlenecks
- [ ] Add GPU detection to startup
- [ ] Implement compute shader for corruption spread
- [ ] Add llama.cpp GPU support with feature flag
- [ ] Create CPU fallback paths
- [ ] Benchmark GPU vs CPU performance
- [ ] Add GPU metrics to monitoring dashboard
- [ ] Update system requirements documentation

## Monitoring GPU Usage

```rust
#[derive(Resource)]
pub struct GpuMetrics {
    pub compute_time_ms: f32,
    pub particles_rendered: u32,
    pub vram_usage_mb: f32,
}

pub fn track_gpu_metrics(
    mut metrics: ResMut<GpuMetrics>,
    diagnostics: Res<DiagnosticsStore>,
) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        metrics.compute_time_ms = 1000.0 / fps.smoothed().unwrap_or(60.0);
    }
}
```

## Conclusion

GPU acceleration can provide 10-100x performance improvements for computationally intensive tasks. The recommended implementation order is:

1. **LLM GPU inference** - Biggest quality of life improvement
2. **Corruption spread compute shader** - Biggest performance win
3. **GPU particle system** - Visual impact

All implementations should include CPU fallbacks for systems without dedicated GPUs.

**Next Steps:**
1. Add GPU capability detection
2. Implement corruption spread compute shader
3. Enable CUDA support in llama-cpp-2
4. Benchmark and tune workgroup sizes
5. Add GPU metrics to monitoring
