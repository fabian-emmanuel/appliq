# Build stage
FROM rust:1.76 as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libpq-dev

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/job-application-tracker .

# Expose the Axum port
EXPOSE 3000

CMD ["./job-application-tracker"]
