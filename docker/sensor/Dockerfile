FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app
COPY rust /app/rust
COPY rust/Cargo.toml /app/Cargo.toml

RUN cargo build --release --bin sensor

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/sensor /app/sensor

ENV EDGE_HOST=10.5.5.1
ENV EDGE_PORT=3001
ENV SENSOR_ID=sensor
ENV INTERVAL_MS=1000

ENTRYPOINT ["/app/sensor"]
