FROM rust:1.84-slim AS builder
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/server /usr/local/bin/longevitycommon-server
COPY --from=builder /app/migrations /app/migrations
EXPOSE 8080
ENV RUST_LOG=info LONGEVITYCOMMON_VERSION=v5.6
CMD ["longevitycommon-server"]
