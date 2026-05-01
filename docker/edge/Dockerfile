FROM rust:slim-bookworm

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY rust /app

RUN cargo build --release --bin edge

CMD ["./target/release/edge"]
