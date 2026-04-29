# Ze subproject — Rust simulator + axum backend (port 4001 inside container).
# Dual-process container: Rust backend only. Phoenix frontend has its own Dockerfile.
FROM rust:1.86-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY ze-simulator ./ze-simulator
COPY ze-backend ./ze-backend
RUN cargo build --release -p ze-backend

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/ze-backend /usr/local/bin/ze-backend
EXPOSE 4001
ENV RUST_LOG=info
CMD ["ze-backend"]
