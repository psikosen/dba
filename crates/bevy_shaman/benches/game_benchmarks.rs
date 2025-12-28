use bevy::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Benchmark ECS query performance
fn benchmark_ecs_queries(c: &mut Criterion) {
    let mut group = c.benchmark_group("ecs_queries");

    // Setup a test app with entities
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Spawn test entities with components
    for i in 0..1000 {
        app.world_mut().spawn((
            Transform::from_xyz(i as f32, 0.0, 0.0),
            Visibility::default(),
        ));
    }

    group.bench_function("query_1000_entities", |b| {
        b.iter(|| {
            let mut query = app.world_mut().query::<&Transform>();
            let count = query.iter(app.world()).count();
            black_box(count);
        });
    });

    group.finish();
}

/// Benchmark state machine transitions
fn benchmark_state_transitions(c: &mut Criterion) {
    use bevy_shaman_core::states::GameState;

    let mut group = c.benchmark_group("state_transitions");

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.init_state::<GameState>();

    group.bench_function("state_transition", |b| {
        b.iter(|| {
            app.world_mut()
                .resource_mut::<NextState<GameState>>()
                .set(GameState::Playing);
            app.update();
            app.world_mut()
                .resource_mut::<NextState<GameState>>()
                .set(GameState::Paused);
            app.update();
        });
    });

    group.finish();
}

/// Benchmark entity spawning
fn benchmark_entity_spawning(c: &mut Criterion) {
    let mut group = c.benchmark_group("entity_spawning");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins);

            b.iter(|| {
                for i in 0..size {
                    app.world_mut().spawn((
                        Transform::from_xyz(i as f32, 0.0, 0.0),
                        Visibility::default(),
                    ));
                }
            });
        });
    }

    group.finish();
}

/// Benchmark plugin initialization
fn benchmark_plugin_initialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("plugin_init");

    group.bench_function("core_plugin", |b| {
        b.iter(|| {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins);
            app.add_plugins(bevy_shaman_core::CorePlugin);
            app.update();
        });
    });

    group.bench_function("combat_plugin", |b| {
        b.iter(|| {
            let mut app = App::new();
            app.add_plugins(MinimalPlugins);
            app.add_plugins(bevy_shaman_combat::CombatPlugin);
            app.update();
        });
    });

    group.finish();
}

/// Benchmark grid operations (if applicable to your game)
fn benchmark_grid_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("grid_operations");

    // Example benchmark for grid-based calculations
    group.bench_function("grid_distance_calc", |b| {
        b.iter(|| {
            let mut distances = Vec::new();
            for x in 0..100 {
                for y in 0..100 {
                    let dist = ((x * x + y * y) as f32).sqrt();
                    distances.push(dist);
                }
            }
            black_box(distances);
        });
    });

    group.finish();
}

/// Benchmark serialization performance
fn benchmark_serialization(c: &mut Criterion) {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    struct GameData {
        player_level: u32,
        monsters_defeated: u32,
        corruption_level: f32,
        inventory: Vec<String>,
    }

    let mut group = c.benchmark_group("serialization");

    let test_data = GameData {
        player_level: 10,
        monsters_defeated: 150,
        corruption_level: 0.75,
        inventory: vec![
            "item1".to_string(),
            "item2".to_string(),
            "item3".to_string(),
        ],
    };

    group.bench_function("serialize_json", |b| {
        b.iter(|| {
            let serialized = serde_json::to_string(&test_data).unwrap();
            black_box(serialized);
        });
    });

    group.bench_function("deserialize_json", |b| {
        let json = serde_json::to_string(&test_data).unwrap();
        b.iter(|| {
            let deserialized: GameData = serde_json::from_str(&json).unwrap();
            black_box(deserialized);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_ecs_queries,
    benchmark_state_transitions,
    benchmark_entity_spawning,
    benchmark_plugin_initialization,
    benchmark_grid_operations,
    benchmark_serialization
);

criterion_main!(benches);
