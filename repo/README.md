# BriefFlow

A full-stack Rust web application built with Rocket (backend), Dioxus (frontend), and MySQL.

## Quick Start (Docker — recommended)

```bash
cd repo
docker compose up --build
```

- Frontend: http://localhost:8080
- Backend API: http://localhost:8000
- Health check: http://localhost:8000/health/live

## Running Tests

```bash
cd repo
./run_tests.sh
```

---

## Manual Setup (without Docker)

### Prerequisites

- Rust toolchain (`rustup`, stable channel)
- MySQL 8.x
- `dx` CLI for Dioxus frontend: `cargo install dioxus-cli`

## Environment Variables

| Variable | Default (dev) | Description |
|---|---|---|
| `DATABASE_URL` | `mysql://root:root@localhost/brewflow` | MySQL connection string |
| `COOKIE_SECRET` | `brewflow-dev-cookie-secret` | HMAC-SHA256 secret for session cookies (32 bytes hex) |
| `ENCRYPTION_KEY` | `brewflow-dev-encryption-key` | AES-256-GCM key for voucher codes (32 bytes hex) |
| `ALLOWED_ORIGINS` | `http://localhost:8080` | Comma-separated CORS origins |
| `SITEMAP_BASE_URL` | `http://localhost:8080` | Base URL for sitemap.xml |

## Database Setup

```bash
# Create the database and run all migrations in order
mysql -u root -p -e "CREATE DATABASE IF NOT EXISTS brewflow CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"

for f in database/migrations/*.sql; do
  mysql -u root -p brewflow < "$f"
done
```

## Running the Backend

```bash
cd backend
cargo run
# Server listens on http://localhost:8000 by default (see Rocket.toml)
```

## Running the Frontend

```bash
# From the repo root
dx serve
# Dev server proxies API requests to the backend
```

## Running the Unit Tests

The backend includes an inline unit test suite covering the core pure-logic services.

```bash
cd backend
cargo test
```

Expected output: all tests pass with `test result: ok`.

To run a specific module's tests:

```bash
cargo test services::pricing
cargo test services::session
cargo test services::fulfillment
cargo test services::crypto
```

## Session and Auth Policy

- Authentication uses **rotating HMAC-signed session cookies** (`brewflow_session`).
- Sessions have a **30-minute idle timeout**; each request resets the timer.
- Session IDs rotate every **5 minutes** to prevent fixation.
- WASM / API clients that cannot auto-send cross-origin cookies must replay the `session_cookie` value from the login response as `Cookie: brewflow_session=<value>` on every request.

## Project Structure

```
backend/    Rocket HTTP server (routes, DB layer, services)
frontend/   Dioxus WASM app
shared/     DTOs and models shared between frontend and backend
database/   SQL migration files (apply in numeric order)
```
