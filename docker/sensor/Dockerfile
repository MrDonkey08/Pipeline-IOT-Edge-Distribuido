FROM rust:1-slim-bookworm AS builder

WORKDIR /app

# PERFORMANCE: Cache dependency compilation separately from application source.
# By building a dummy main.rs first, Docker can cache the layer containing all
# compiled dependencies. Subsequent rebuilds only recompile your src/ changes.
# See: https://www.youtube.com/watch?v=5Wfpzfniu4I
COPY Cargo.toml ./
RUN mkdir -p src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --release

# Overwrite application source and touch artifacts to bust Cargo's mtime cache,
# forcing a relink against the real entry point instead of the dummy one.
COPY ./src ./src
RUN touch src/main.rs \
    && cargo build --release

# Final image — no Rust toolchain, no build artifacts, just the binary.
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
COPY --from=builder /app/target/release/sensor .
CMD ["./sensor"]
