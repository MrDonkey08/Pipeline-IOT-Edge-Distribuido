FROM rust:slim-bookworm

WORKDIR /app

COPY rust /app

RUN cargo build --release --bin edge

CMD ["./target/release/edge"]
