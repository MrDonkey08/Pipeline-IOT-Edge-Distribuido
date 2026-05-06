FROM rust:1-slim-bookworm AS builder

WORKDIR /app

COPY rust/Cargo.toml rust/Cargo.lock ./
COPY rust/common/Cargo.toml common/Cargo.toml
COPY rust/coordinator/Cargo.toml coordinator/Cargo.toml

# Patch the workspace manifest to drop members irrelevant to this build.
# Cargo errors if a declared member directory is missing, so we excise them
# before it ever tries to resolve the graph.
RUN sed -i 's/, *"edge"//; s/, *"sensor"//' Cargo.toml

# PERFORMANCE: Cache dependency compilation separately from application source.
# By building a dummy main.rs first, Docker can cache the layer containing all
# compiled dependencies. Subsequent rebuilds only recompile your src/ changes.
# See: https://www.youtube.com/watch?v=5Wfpzfniu4I
RUN mkdir -p common/src coordinator/src \
	&& echo '' > common/src/lib.rs \
	&& echo 'fn main() {}' > coordinator/src/main.rs \
	&& cargo build --release -p coordinator

COPY rust/common/src common/src
COPY rust/coordinator/src coordinator/src
RUN touch common/src/lib.rs coordinator/src/main.rs \
	&& cargo build --release -p coordinator

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/coordinator .
ENTRYPOINT ["./coordinator"]
