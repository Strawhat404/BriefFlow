# ──────────────────────────────────────────────────────────────────────────────
# Stage 1 — Build the Rocket backend binary
# ──────────────────────────────────────────────────────────────────────────────
FROM rust:1.82-slim AS builder

WORKDIR /app

# System libs needed by sqlx (TLS) and proc-macro crates
RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# ── Layer-cache: resolve workspace dependencies before copying source ──────────
# Copy the workspace manifest and every crate's Cargo.toml so `cargo fetch`
# can pull all crates.io deps.  Stub out `src/` files so the workspace is
# valid but nothing expensive is compiled yet.
COPY Cargo.toml Cargo.lock ./

COPY shared/Cargo.toml      shared/Cargo.toml
COPY backend/Cargo.toml     backend/Cargo.toml
COPY frontend/Cargo.toml    frontend/Cargo.toml

# Create minimal stub source trees so Cargo doesn't complain about missing files
RUN mkdir -p shared/src backend/src frontend/src \
 && echo "pub fn _stub() {}" > shared/src/lib.rs \
 && echo "fn main() {}"      > backend/src/main.rs \
 && echo "fn main() {}"      > frontend/src/main.rs

# Pre-fetch & pre-build all *dependencies* (cached layer; invalidated only
# when Cargo.toml / Cargo.lock change)
RUN cargo build --release --bin backend 2>&1 || true
RUN find target/release -name "backend*" -delete 2>/dev/null; true

# ── Full build with real source ──────────────────────────────────────────────
COPY shared/   shared/
COPY backend/  backend/

RUN cargo build --release --bin backend

# ──────────────────────────────────────────────────────────────────────────────
# Stage 2 — Minimal runtime image
# ──────────────────────────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend
COPY --from=builder /app/Rocket.toml            /app/Rocket.toml

EXPOSE 8000

CMD ["/app/backend"]
