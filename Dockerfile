# Minimal runtime image
FROM debian:bookworm-slim

# Install only the runtime dependencies
RUN apt-get update && apt-get install -y libssl3 libpq5 ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1001 appuser

WORKDIR /app

# Copy the prebuilt binary and assets
COPY app /app/app
COPY db ./db
COPY resources ./resources

RUN chmod +x /app/app && chown -R appuser:appuser /app

USER appuser
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=15s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

ENTRYPOINT ["/app/app"]
