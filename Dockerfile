# Dockerfile for pharmacy_backend
FROM debian:stable-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Runtime environment defaults
ENV PORT=8088
ENV SERVER_ADDR=0.0.0.0
ENV LOG_LEVEL=info
ENV RUST_LOG=info

# Copy the release binary produced by cargo build --release
COPY target/release/pharmacy_backend /app/pharmacy_backend

# Copy environment validation script
COPY validate_env.sh /usr/local/bin/validate_env.sh
RUN chmod +x /usr/local/bin/validate_env.sh

# Create logs dir
RUN mkdir -p /app/logs

EXPOSE 8088

ENTRYPOINT ["/usr/local/bin/validate_env.sh"]
CMD ["/app/pharmacy_backend"]
