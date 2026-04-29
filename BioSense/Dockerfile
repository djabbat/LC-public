FROM rust:1.86-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY biosense-simulator ./biosense-simulator
COPY biosense-backend ./biosense-backend
COPY datasets ./datasets
RUN cargo build --release -p biosense-backend

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/biosense-backend /usr/local/bin/biosense-backend
EXPOSE 4101
ENV RUST_LOG=info
CMD ["biosense-backend"]
