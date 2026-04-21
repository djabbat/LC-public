FROM rust:1.75-slim AS builder

WORKDIR /usr/src/ze_backend

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build for release
RUN cargo build --release

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /usr/src/ze_backend/target/release/ze_backend /usr/local/bin/

# Copy migrations
COPY --from=builder /usr/src/ze_backend/migrations ./migrations

# Create non-root user
RUN useradd -m -u 1000 ze
USER ze

# Set environment variables
ENV PORT=3009
ENV RUST_LOG=ze_backend=info,tower_http=debug

EXPOSE 3009

CMD ["ze_backend"]