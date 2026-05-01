FROM rust:1.85-slim-bookworm

WORKDIR /app

# Copiar TODO el contenido de rust/ a /app
COPY rust /app

# Compilar
RUN cargo build --release --bin edge

CMD ["./target/release/edge"]
