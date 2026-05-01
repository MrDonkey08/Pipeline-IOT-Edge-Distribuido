FROM rust:1.85-slim-bookworm

WORKDIR /app
COPY rust /app

RUN cargo build --release --bin edge

CMD ["cargo", "run", "--release", "--bin", "edge"]
