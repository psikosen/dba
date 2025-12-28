.PHONY: test coverage bench clean help

# Default target
help:
	@echo "Available targets:"
	@echo "  make test        - Run all unit tests"
	@echo "  make coverage    - Generate code coverage report (target: 90%)"
	@echo "  make bench       - Run performance benchmarks"
	@echo "  make clean       - Clean build artifacts and coverage reports"
	@echo "  make test-world  - Run world generation tests"
	@echo "  make test-qa     - Run all QA tests"
	@echo "  make view-coverage - Open coverage report in browser"

# Run all tests
test:
	cargo test --workspace

# Run tests for specific components
test-world:
	cargo test -p bevy_shaman_world

test-items:
	cargo test -p bevy_shaman_items

test-dungeons:
	cargo test -p bevy_shaman_dungeons

test-qa:
	cargo test --workspace -- qa

# Generate code coverage report with lcov
coverage:
	@echo "Generating code coverage report..."
	cargo tarpaulin --config tarpaulin.toml
	@echo ""
	@echo "Coverage report generated!"
	@echo "HTML: coverage/index.html"
	@echo "LCOV: coverage/lcov.info"

# View coverage report
view-coverage:
	@if [ -f coverage/index.html ]; then \
		xdg-open coverage/index.html 2>/dev/null || open coverage/index.html 2>/dev/null || echo "Coverage report: coverage/index.html"; \
	else \
		echo "Coverage report not found. Run 'make coverage' first."; \
	fi

# Run performance benchmarks
bench:
	cargo bench --workspace

# Run specific benchmark
bench-world:
	cargo bench --bench world_generation

# Clean build artifacts and coverage
clean:
	cargo clean
	rm -rf coverage/ target/criterion/
	rm -f coverage_output.txt

# Development build (fast compile)
dev:
	cargo build

# Release build
release:
	cargo build --release

# Check all crates without building
check:
	cargo check --workspace

# Format code
fmt:
	cargo fmt --all

# Run linter
lint:
	cargo clippy --workspace -- -D warnings

# Run all CI checks
ci: fmt lint test coverage
	@echo "All CI checks passed!"
