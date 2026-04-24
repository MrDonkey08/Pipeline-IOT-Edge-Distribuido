FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app
COPY rust /app/rust
COPY rust/Cargo.toml /app/Cargo.toml

RUN cargo build --release --bin coordinator

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/coordinator /app/coordinator

ENV LISTEN_PORT=8002
ENV HTTP_PORT=8080

EXPOSE 8080

ENTRYPOINT ["/app/coordinator"]
