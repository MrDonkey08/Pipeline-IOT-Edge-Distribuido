FROM rust:1-slim-bookworm AS builder

WORKDIR /app

COPY rust/Cargo.toml rust/Cargo.lock ./
COPY rust/common/Cargo.toml common/Cargo.toml
COPY rust/sensor/Cargo.toml sensor/Cargo.toml

# Patch the workspace manifest to drop members irrelevant to this build.
# Cargo errors if a declared member directory is missing, so we excise them
# before it ever tries to resolve the graph.
RUN sed -i 's/, *"coordinator"//; s/, *"edge"//' Cargo.toml

# PERFORMANCE: Cache dependency compilation separately from application source.
# By building a dummy main.rs first, Docker can cache the layer containing all
# compiled dependencies. Subsequent rebuilds only recompile your src/ changes.
# See: https://www.youtube.com/watch?v=5Wfpzfniu4I
RUN mkdir -p common/src sensor/src \
	&& echo '' > common/src/lib.rs \
	&& echo 'fn main() {}' > sensor/src/main.rs \
	&& cargo build --release -p sensor

COPY rust/common/src common/src
COPY rust/sensor/src sensor/src
RUN touch common/src/lib.rs sensor/src/main.rs \
	&& cargo build --release -p sensor

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/sensor .
ENTRYPOINT ["./sensor"]
