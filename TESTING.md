# Testing & Code Coverage

This document describes the testing infrastructure and code coverage setup for the Shaman's Journey project.

## Overview

The project uses:
- **Unit Tests**: Standard Rust unit tests for all crates
- **QA Tests**: World generation and game systems validation tests
- **Performance Tests**: Criterion benchmarks for critical paths
- **Code Coverage**: cargo-tarpaulin with lcov output, targeting 90% coverage

## Running Tests

### Run All Tests
```bash
cargo test --workspace
```

### Run Tests for Specific Crate
```bash
cargo test -p bevy_shaman_world
cargo test -p bevy_shaman_items
cargo test -p bevy_shaman_dungeons
```

### Run Tests with Output
```bash
cargo test --workspace -- --nocapture
```

## Code Coverage

### Generate Coverage Report
```bash
cargo tarpaulin --config tarpaulin.toml
```

This will generate:
- HTML report in `coverage/index.html`
- LCOV report in `coverage/lcov.info`

### View Coverage
```bash
# Open HTML report
xdg-open coverage/index.html

# Or view summary
cat coverage_output.txt | grep "Coverage"
```

### Coverage Target
The project targets **90% code coverage** across all workspace crates.

## Performance Testing

### Run Benchmarks
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench world_generation

# Run benchmarks with baseline comparison
cargo bench -- --save-baseline main
```

### Benchmark Results
Benchmark results are saved in `target/criterion/` with HTML reports.

View reports:
```bash
xdg-open target/criterion/report/index.html
```

## QA Testing

### World Generation Tests
The `bevy_shaman_world` crate includes comprehensive QA tests that validate:
- World size configurations (small, medium, large)
- Ecosystem distribution and constraints
- Dungeon placement density
- Corruption type distribution
- Biome walkability expectations
- Difficulty scaling

Run QA tests:
```bash
cargo test -p bevy_shaman_world qa
```

### Dungeon System Tests
The `bevy_shaman_dungeons` crate includes QA tests for:
- Room count ranges (5-12 rooms)
- Room type distribution (60% encounters, 25% empty, 15% treasure)
- Boss placement (always final room)
- Monster spawn counts (1-4 per encounter)
- Boss stats and phase transitions
- AI aggression ranges

Run dungeon QA tests:
```bash
cargo test -p bevy_shaman_dungeons qa
```

## Test Organization

### Unit Tests
Each crate has a `tests.rs` module containing:
```rust
#[cfg(test)]
mod component_tests {
    #[test]
    fn test_component_creation() {
        // Test code
    }
}
```

### Integration Tests
Integration tests live in `tests/` directories within each crate.

### Benchmarks
Performance benchmarks are in `benches/` directories:
- `bevy_shaman_world/benches/world_generation.rs`

## Continuous Integration

Coverage reports are generated automatically in CI/CD pipelines.

### GitHub Actions
```yaml
- name: Run tests with coverage
  run: cargo tarpaulin --config tarpaulin.toml --fail-under 90
```

## Coverage Exclusions

The following are excluded from coverage:
- Test files (`*/tests.rs`)
- Benchmark files (`*/benches/*`)
- Generated code
- Cargo artifacts (`*/.cargo/*`)

## Writing Tests

### Unit Test Template
```rust
#[test]
fn test_feature_name() {
    // Arrange
    let component = Component::new();

    // Act
    let result = component.do_something();

    // Assert
    assert_eq!(result, expected);
}
```

### QA Test Template
```rust
#[test]
fn test_qa_system_constraint() {
    // Verify system-level constraints
    let config = SystemConfig::default();

    // Validate constraints
    assert!(config.min_value < config.max_value);
    assert_eq!(config.expected_behavior, actual_behavior);
}
```

### Performance Test Template
```rust
fn benchmark_system(c: &mut Criterion) {
    c.bench_function("system_name", |b| {
        b.iter(|| {
            // Code to benchmark
            black_box(system_operation());
        });
    });
}
```

## Coverage Goals by Crate

| Crate | Coverage Target | Status |
|-------|----------------|--------|
| bevy_shaman_core | 90% | ✓ |
| bevy_shaman_world | 90% | ✓ |
| bevy_shaman_dungeons | 90% | ✓ |
| bevy_shaman_items | 90% | ✓ |
| bevy_shaman_combat | 90% | ⏳ |
| bevy_shaman_monsters | 90% | ⏳ |
| bevy_shaman_ui | 90% | ⏳ |
| bevy_shaman_audio | 85% | ⏳ |
| bevy_shaman_shop | 90% | ⏳ |
| bevy_shaman_story | 90% | ⏳ |
| bevy_shaman_save | 90% | ⏳ |
| bevy_shaman_ai | 90% | ⏳ |
| bevy_shaman_minions | 90% | ⏳ |
| bevy_shaman_tutorial | 90% | ⏳ |

## Troubleshooting

### Tests Won't Compile
Ensure you have required system dependencies:
```bash
apt-get install libudev-dev pkg-config
```

### Coverage Generation Fails
Check timeout settings in `tarpaulin.toml`:
```toml
timeout = "300s"  # Increase if needed
```

### Benchmarks Show Variance
Run with `--warm-up-time` and `--measurement-time`:
```bash
cargo bench -- --warm-up-time 3 --measurement-time 10
```

## Resources

- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
