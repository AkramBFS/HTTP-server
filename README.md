# Dev Gateway Reverse Proxy (Rust / Axum)

A local development **reverse proxy gateway** written in Rust, designed to sit in front of multiple backend and frontend services and provide:

* Unified routing
* Streaming proxying (no buffering)
* WebSocket tunneling
* Request size limits
* Timeouts
* Structured logging

---

## Workspace Overview

This repository is organized as a **Cargo workspace** containing two binaries:

* **`todo-api`** — a simple Axum-based CRUD API (used as a stand-in backend)
* **`dev-gateway`** — the reverse proxy gateway

---

## Project Structure

```text
http-server/
├── Cargo.toml                 # Workspace root
├── .env                       # Local env (ports updated)
├── crates/
│   ├── todo-api/              # CRUD API (migrated from original http-server)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── config.rs
│   │       ├── errors.rs
│   │       ├── handlers/
│   │       ├── models/
│   │       └── routes/
│   └── dev-gateway/           # Reverse proxy gateway
│       ├── Cargo.toml
│       ├── .env.example
│       └── src/
│           ├── main.rs        # Server bootstrap + middleware
│           ├── config.rs      # GatewayConfig (env-driven)
│           ├── errors.rs      # GatewayError → HTTP mapping
│           ├── client.rs      # Shared Hyper connection pool
│           ├── routes/
│           │   └── mod.rs     # Boundary-aware route matching (+ tests)
│           ├── proxy/
│           │   ├── mod.rs     # GatewayState + proxy handler
│           │   ├── headers.rs # Hop-by-hop stripping + X-Forwarded-*
│           │   ├── forward.rs # Streaming body forwarding
│           │   └── upgrade.rs # WebSocket tunneling
│           └── middleware/
│               └── logging.rs # Single-line request logger
```

---

## Services & Ports

| Service     | Default Port |
| ----------- | ------------ |
| dev-gateway | 8080         |
| todo-api    | 3001         |
| Next.js     | 3000         |
| Django      | 8000         |

### Why this matters

The gateway intentionally avoids port collisions so it can sit in front of **frontend and backend services simultaneously** during local development.

---

## What the Dev Gateway Does

### 1. Boundary-Aware Routing

Requests are routed based on path **without false positives**.

#### Routed to Django if:

```text
path == /api
path.starts_with("/api/")
```

#### Routed to Next.js otherwise.

#### Examples

| Path             | Target  |
| ---------------- | ------- |
| `/api/v1/todos`  | Django  |
| `/api`           | Django  |
| `/api?user=1`    | Django  |
| `/apiary/v1`     | Next.js |
| `/api-websocket` | Next.js |
| `/`              | Next.js |

This prevents common proxy bugs where `/apiary` accidentally matches `/api`.

---

### 2. Streaming Proxying (No Buffering)

* Request and response bodies are streamed end-to-end
* No `Vec<u8>` buffering
* Uses `http_body_util` to adapt Hyper bodies into Axum

This makes the gateway safe for:

* Large uploads
* Slow clients
* Backpressure-sensitive workloads

---

### 3. WebSocket Tunneling

* Fully transparent WebSocket proxying
* Uses `hyper::upgrade`
* Bridges upgraded sockets using:

```rust
tokio::io::copy_bidirectional
```

* Ensures clean shutdown on EOF to avoid file descriptor leaks

---

### 4. Header Sanitization

* Strips hop-by-hop headers (`Connection`, `Transfer-Encoding`, etc.)
* Injects:

  * `X-Forwarded-For`
  * `X-Forwarded-Host`
  * `X-Forwarded-Proto`

---

### 5. Middleware Guarantees

| Feature         | Default            |
| --------------- | ------------------ |
| Max body size   | 10 MB              |
| Request timeout | 30 seconds         |
| Logging         | 1 line per request |
| Error mapping   | JSON + HTTP status |

#### Example log line

```text
2026-07-01T00:00:00Z GET /api/v1/todos -> http://127.0.0.1:8000/api/v1/todos | STATUS: 200 | LATENCY: 3.20ms
```

---

## Configuration

`dev-gateway/.env.example`

```ini
GATEWAY_HOST=127.0.0.1
GATEWAY_PORT=8080

DJANGO_BACKEND_URL=http://127.0.0.1:8000
NEXTJS_FRONTEND_URL=http://127.0.0.1:3000

API_PREFIX=/api
STRIP_API_PREFIX=true

MAX_BODY_BYTES=10485760
REQUEST_TIMEOUT_SECS=30
```

All values are:

* Loaded via `dotenvy`
* Parsed with type safety
* Validated at startup

---

## Error Mapping

| Condition              | HTTP                      |
| ---------------------- | ------------------------- |
| Upstream unreachable   | 502 Bad Gateway           |
| Upstream timeout       | 504 Gateway Timeout       |
| Request body too large | 413 Payload Too Large     |
| Internal gateway error | 500 Internal Server Error |

#### Example error response

```json
{
  "error": "BAD_GATEWAY",
  "message": "Connection refused"
}
```

---

## Running Locally

### 1. Start the backend (Todo API)

```bash
SERVER_PORT=8000 cargo run -p todo-api
```

### 2. Start the gateway

```bash
DJANGO_BACKEND_URL=http://127.0.0.1:8000 \
STRIP_API_PREFIX=false \
cargo run -p dev-gateway
```

### 3. Test through the gateway

```bash
curl http://127.0.0.1:8080/api/v1/todos
```

---

## Verification

### Automated

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

### Manual Error Scenarios

#### 502 Bad Gateway

```bash
DJANGO_BACKEND_URL=http://127.0.0.1:9999 cargo run -p dev-gateway
```

#### 413 Payload Too Large

```bash
curl -X POST http://127.0.0.1:8080/api/test -d @large_file.bin
```
