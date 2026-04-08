FROM rust:1.87-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY shared/Cargo.toml      shared/Cargo.toml
COPY backend/Cargo.toml     backend/Cargo.toml
COPY frontend/Cargo.toml    frontend/Cargo.toml

RUN mkdir -p shared/src backend/src frontend/src \
 && echo "pub fn _stub() {}" > shared/src/lib.rs \
 && echo "fn main() {}"      > backend/src/main.rs \
 && echo "fn main() {}"      > frontend/src/main.rs

RUN cargo fetch
RUN cargo build --release --bin backend 2>&1 || true
RUN find target/release -name "backend*" -delete 2>/dev/null; true

COPY shared/   shared/
COPY backend/  backend/

RUN cargo build --release --bin backend

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
        curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/backend /app/backend
COPY --from=builder /app/Rocket.toml            /app/Rocket.toml

EXPOSE 8000

CMD ["/app/backend"]
