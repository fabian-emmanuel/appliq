FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy pre-built binary and resources
COPY app /usr/local/bin/app
COPY db ./db
COPY resources ./resources

# Make binary executable
RUN chmod +x /usr/local/bin/app

# Expose port
EXPOSE 8000

# Run the binary
CMD ["/usr/local/bin/app"]
