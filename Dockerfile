# Build stage
FROM rustlang/rust:nightly as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libpq-dev

# Copy the project files
COPY Cargo.toml ./
COPY ./src ./src
# Copy the migrations directory
COPY ./db ./db

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/job-application-tracker .
# Copy migrations to the final stage as well
COPY ./db ./db

# Expose the Axum port
EXPOSE 3000

CMD ["./job-application-tracker"]