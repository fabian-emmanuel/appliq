# Build stage
FROM rustlang/rust:nightly AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libpq-dev

# Copy dependencies manifest
COPY Cargo.toml ./

# Create a new empty project to cache dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
# Build dependencies
RUN cargo build --release

# Remove the dummy project files that were built
RUN rm -f target/release/deps/appliq*

# Copy the actual source code and build
COPY ./src ./src
COPY ./db ./db
COPY ./resources ./resources
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
# Using Debian Bookworm (stable) instead of Bullseye for newer GLIBC
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/appliq .
# Copy migrations to the final stage as well
COPY --from=builder /app/db ./db
# Copy email templates to the final stage as well
COPY --from=builder /app/resources ./resources

# Expose the Axum port
EXPOSE 80

CMD ["./appliq"]
