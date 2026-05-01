FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app
COPY rust /app/rust
COPY rust/Cargo.toml /app/Cargo.toml

RUN cargo build --release --bin edge

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/edge /app/edge

ENV COORDINATOR_HOST=10.5.5.8
ENV COORDINATOR_PORT=8002
ENV EDGE_ID=edge
ENV HTTP_PORT=3001

EXPOSE 3001

ENTRYPOINT ["/app/edge"]
