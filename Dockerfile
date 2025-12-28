# Dockerfile for Shaman's Journey - Multi-stage build for optimized production images
# Build Stage: Compile the Rust binary
FROM rust:1.91-slim as builder

# Install system dependencies required for building Bevy applications
RUN apt-get update && apt-get install -y \
    libasound2-dev \
    libudev-dev \
    pkg-config \
    build-essential \
    libx11-dev \
    libxi-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libxcursor-dev \
    libxinerama-dev \
    libxrandr-dev \
    libxfixes-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy workspace configuration files first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY crates/bevy_shaman/Cargo.toml ./crates/bevy_shaman/
COPY crates/bevy_shaman_core/Cargo.toml ./crates/bevy_shaman_core/
COPY crates/bevy_shaman_combat/Cargo.toml ./crates/bevy_shaman_combat/
COPY crates/bevy_shaman_audio/Cargo.toml ./crates/bevy_shaman_audio/
COPY crates/bevy_shaman_monsters/Cargo.toml ./crates/bevy_shaman_monsters/
COPY crates/bevy_shaman_minions/Cargo.toml ./crates/bevy_shaman_minions/
COPY crates/bevy_shaman_world/Cargo.toml ./crates/bevy_shaman_world/
COPY crates/bevy_shaman_dungeons/Cargo.toml ./crates/bevy_shaman_dungeons/
COPY crates/bevy_shaman_items/Cargo.toml ./crates/bevy_shaman_items/
COPY crates/bevy_shaman_ui/Cargo.toml ./crates/bevy_shaman_ui/
COPY crates/bevy_shaman_story/Cargo.toml ./crates/bevy_shaman_story/
COPY crates/bevy_shaman_save/Cargo.toml ./crates/bevy_shaman_save/
COPY crates/bevy_shaman_ai/Cargo.toml ./crates/bevy_shaman_ai/
COPY crates/bevy_shaman_shop/Cargo.toml ./crates/bevy_shaman_shop/
COPY crates/bevy_shaman_tutorial/Cargo.toml ./crates/bevy_shaman_tutorial/

# Create dummy source files to cache dependencies
RUN mkdir -p crates/bevy_shaman/src && \
    mkdir -p crates/bevy_shaman_core/src && \
    mkdir -p crates/bevy_shaman_combat/src && \
    mkdir -p crates/bevy_shaman_audio/src && \
    mkdir -p crates/bevy_shaman_monsters/src && \
    mkdir -p crates/bevy_shaman_minions/src && \
    mkdir -p crates/bevy_shaman_world/src && \
    mkdir -p crates/bevy_shaman_dungeons/src && \
    mkdir -p crates/bevy_shaman_items/src && \
    mkdir -p crates/bevy_shaman_ui/src && \
    mkdir -p crates/bevy_shaman_story/src && \
    mkdir -p crates/bevy_shaman_save/src && \
    mkdir -p crates/bevy_shaman_ai/src && \
    mkdir -p crates/bevy_shaman_shop/src && \
    mkdir -p crates/bevy_shaman_tutorial/src && \
    echo "fn main() {}" > crates/bevy_shaman/src/main.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_core/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_combat/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_audio/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_monsters/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_minions/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_world/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_dungeons/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_items/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_ui/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_story/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_save/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_ai/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_shop/src/lib.rs && \
    echo "pub fn dummy() {}" > crates/bevy_shaman_tutorial/src/lib.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release --workspace --exclude bevy_shaman_audio && \
    rm -rf crates/*/src

# Copy actual source code
COPY crates ./crates
COPY tests ./tests

# Build the actual application with release optimizations
RUN cargo build --release --workspace --exclude bevy_shaman_audio

# Strip debug symbols to reduce binary size
RUN strip /app/target/release/bevy_shaman

# Runtime Stage: Minimal runtime environment
FROM debian:bookworm-slim

# Install only runtime dependencies (not build dependencies)
RUN apt-get update && apt-get install -y \
    libasound2 \
    libudev1 \
    libx11-6 \
    libxi6 \
    libgl1 \
    libxcursor1 \
    libxinerama1 \
    libxrandr2 \
    libxfixes3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1000 shaman && \
    mkdir -p /app/saves /app/assets && \
    chown -R shaman:shaman /app

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/bevy_shaman /app/bevy_shaman

# Copy assets directory (even if empty, for future use)
COPY --chown=shaman:shaman assets ./assets

# Set permissions
RUN chmod +x /app/bevy_shaman

# Switch to non-root user
USER shaman

# Expose port if needed for future networking features
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Health check (optional - checks if process is running)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD pgrep -x bevy_shaman || exit 1

# Run the game
CMD ["/app/bevy_shaman"]
