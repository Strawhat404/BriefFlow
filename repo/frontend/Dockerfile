# ──────────────────────────────────────────────────────────────────────────────
# Stage 1 — Build the Dioxus WASM app
# ──────────────────────────────────────────────────────────────────────────────
FROM rust:1.82-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        curl \
    && rm -rf /var/lib/apt/lists/*

# Add the WASM compilation target
RUN rustup target add wasm32-unknown-unknown

# Install the Dioxus CLI (matches the dioxus 0.6 workspace dependency)
RUN cargo install dioxus-cli --version "^0.6" --locked

# ── Copy workspace files ───────────────────────────────────────────────────────
# The Dioxus.toml and workspace Cargo.toml must be at the root so `dx build`
# resolves paths correctly.
COPY Cargo.toml Cargo.lock ./
COPY Dioxus.toml           ./
COPY shared/               shared/
COPY frontend/             frontend/
COPY backend/Cargo.toml    backend/Cargo.toml

# Provide a stub backend/src so the workspace is valid without backend deps
RUN mkdir -p backend/src && echo "fn main() {}" > backend/src/main.rs

# Build the Dioxus web app; outputs to ./dist/
RUN dx build --release

# ──────────────────────────────────────────────────────────────────────────────
# Stage 2 — Serve with nginx
# ──────────────────────────────────────────────────────────────────────────────
FROM nginx:1.27-alpine

COPY --from=builder /app/dist              /usr/share/nginx/html
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
