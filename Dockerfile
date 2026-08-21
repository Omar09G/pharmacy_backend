# ── Stage 1: build ───────────────────────────────────────────────────────────
# Compiles the release binary inside Docker so builds are reproducible and
# CI-friendly (no dependency on a host-side `cargo build --release`).
FROM rust:1-slim AS builder

WORKDIR /build

# pkg-config/libssl-dev only needed if a dependency links against system OpenSSL;
# the project uses rustls, but keep them for safety with future deps.
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first to maximize layer caching of dependencies
COPY Cargo.toml Cargo.lock* ./
COPY schemas ./schemas
COPY migration ./migration

# Fake main.rs so dependencies can be cached before copying real sources
RUN mkdir -p src && echo "fn main() {}" > src/main.rs \
    && cargo build --release --bin pharmacy_backend || true

# Now copy the real sources and build the actual binary
COPY src ./src
RUN touch src/main.rs && cargo build --release --bin pharmacy_backend

# ── Stage 2: runtime ────────────────────────────────────────────────────────
FROM debian:stable-slim

# Minimal runtime dependencies (curl is used by the HEALTHCHECK)
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home appuser

WORKDIR /app

ENV PORT=8081 \
    SERVER_ADDR=0.0.0.0 \
    LOG_LEVEL=info \
    RUST_LOG=info \
    LOG_DIR=/app/logs

COPY --from=builder /build/target/release/pharmacy_backend /app/pharmacy_backend

RUN mkdir -p /app/logs /app/pem \
    && chown -R appuser:appuser /app

USER appuser

EXPOSE 8081

HEALTHCHECK --interval=30s --timeout=5s --start-period=20s --retries=3 \
    CMD curl -fsS http://127.0.0.1:${PORT}/v1/api/health || exit 1

ENTRYPOINT ["/app/pharmacy_backend"]
