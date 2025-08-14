# Migration Dockerfile for running SQLx migrations
FROM rust:1.88-slim AS builder

# Limit cargo/rustc parallelism to reduce memory during sqlx-cli build
ENV CARGO_BUILD_JOBS=2
# Use sparse registry and git CLI for lower memory footprint
ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_NET_GIT_FETCH_WITH_CLI=true \
    CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Install build dependencies
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libpq-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install SQLx CLI (restrict features to postgres; limit jobs)
RUN cargo install -j 2 sqlx-cli --no-default-features --features postgres

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy SQLx CLI from builder
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# Set working directory
WORKDIR /app

# Copy migration files
COPY db ./db

# Default command to run migrations
CMD ["sqlx", "migrate", "run"]