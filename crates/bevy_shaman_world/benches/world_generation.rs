use bevy::prelude::*;
use bevy_shaman_world::systems::generation::{WorldGenConfig, WorldGenerated, WorldSeed};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn setup_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.init_resource::<WorldGenConfig>();
    app.init_resource::<WorldGenerated>();
    app.init_resource::<WorldSeed>();
    app
}

fn benchmark_world_generation_small(c: &mut Criterion) {
    c.bench_function("world_gen_small_50", |b| {
        b.iter(|| {
            let mut app = setup_test_app();
            let mut config = app.world_mut().resource_mut::<WorldGenConfig>();
            config.world_radius = black_box(50);
            config.village_radius = 10;
            config.ecosystem_count = 5;
            config.dungeons_per_ecosystem = 3;
        });
    });
}

fn benchmark_world_generation_medium(c: &mut Criterion) {
    c.bench_function("world_gen_medium_100", |b| {
        b.iter(|| {
            let mut app = setup_test_app();
            let mut config = app.world_mut().resource_mut::<WorldGenConfig>();
            config.world_radius = black_box(100);
            config.village_radius = 15;
            config.ecosystem_count = 5;
            config.dungeons_per_ecosystem = 3;
        });
    });
}

fn benchmark_world_generation_large(c: &mut Criterion) {
    c.bench_function("world_gen_large_150", |b| {
        b.iter(|| {
            let mut app = setup_test_app();
            let mut config = app.world_mut().resource_mut::<WorldGenConfig>();
            config.world_radius = black_box(150);
            config.village_radius = 20;
            config.ecosystem_count = 5;
            config.dungeons_per_ecosystem = 3;
        });
    });
}

fn benchmark_world_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("world_generation_sizes");

    for size in [25, 50, 75, 100, 150].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut app = setup_test_app();
                let mut config = app.world_mut().resource_mut::<WorldGenConfig>();
                config.world_radius = black_box(size);
                config.village_radius = size / 5;
                config.ecosystem_count = 5;
                config.dungeons_per_ecosystem = 3;
            });
        });
    }
    group.finish();
}

fn benchmark_ecosystem_counts(c: &mut Criterion) {
    let mut group = c.benchmark_group("ecosystem_counts");

    for count in [3, 5, 7, 9].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, &count| {
            b.iter(|| {
                let mut app = setup_test_app();
                let mut config = app.world_mut().resource_mut::<WorldGenConfig>();
                config.world_radius = 100;
                config.village_radius = 15;
                config.ecosystem_count = black_box(count);
                config.dungeons_per_ecosystem = 3;
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_world_generation_small,
    benchmark_world_generation_medium,
    benchmark_world_generation_large,
    benchmark_world_sizes,
    benchmark_ecosystem_counts
);
criterion_main!(benches);
