# Ultra-optimized Dockerfile - simplified approach
FROM rustlang/rust:nightly AS planner
WORKDIR /app
COPY Cargo.toml  ./
RUN cargo install cargo-chef && cargo chef prepare --recipe-path recipe.json

FROM rustlang/rust:nightly AS dependencies
WORKDIR /app
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y pkg-config libpq-dev libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=planner /app/recipe.json recipe.json
ENV CARGO_BUILD_JOBS=8
RUN cargo chef cook --release --recipe-path recipe.json

FROM rustlang/rust:nightly AS builder
WORKDIR /app
COPY --from=dependencies /app/target target
COPY --from=dependencies /usr/local/cargo /usr/local/cargo
RUN apt-get update && apt-get install -y pkg-config libpq-dev libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY src ./src
COPY db ./db
COPY resources ./resources

ENV CARGO_BUILD_JOBS=8
RUN cargo build --release

# Simple runtime stage with shell support
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 libpq5 ca-certificates curl && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -m -u 1001 appuser

WORKDIR /app

# Debug: List what we have
RUN echo "Contents of builder release directory:"
COPY --from=builder /app/target/release/ /tmp/release/
RUN ls -la /tmp/release/

# Find and copy the main executable
RUN find /tmp/release -maxdepth 1 -type f -executable \
    ! -name "build-script-*" \
    ! -name ".*" \
    ! -name "deps" \
    | head -1 | xargs -I {} cp {} /app/app && \
    chmod +x /app/app

# Copy runtime files
COPY --from=builder /app/db ./db
COPY --from=builder /app/resources ./resources
RUN chown -R appuser:appuser /app

USER appuser
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=15s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

ENTRYPOINT ["/app/app"]