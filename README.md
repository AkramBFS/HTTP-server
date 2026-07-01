# Dev Gateway Reverse Proxy (Rust / Axum)

A **generic local development reverse proxy gateway** written in **Rust**, built on **Axum + Hyper**.

The Dev Gateway is designed to sit in front of any number of upstream services and behave like a **production-grade reverse proxy**, even when used only for local development.

It focuses on correctness, streaming safety, and explicit configuration rather than framework-specific assumptions.

---

## Features

* Declarative routing via `gateway.toml` (with environment variable fallback)
* Fully streaming HTTP proxying (no request or response buffering)
* WebSocket tunneling
* Retry & backoff for safe, idempotent requests
* Request size limits and request timeouts
* Correct `X-Forwarded-*` header handling
* Structured, low-noise logging
* Deterministic, boundary-safe route matching

---

## Workspace Overview

This repository is a **Cargo workspace** containing two independent binaries:

* **`dev-gateway`** — the reverse proxy gateway
* **`todo-api`** — a **simple Axum-based CRUD API used only as a testing backend**

> **Important:**
> The `todo-api` crate exists solely as a **stand-in backend for development and testing**.
> It is **not** intended to be a production service.

---

## Project Structure

```
http-server/
├── Cargo.toml                   # Workspace root
├── gateway.toml.example         # Declarative routing configuration
├── crates/
│   ├── todo-api/                # Test-only CRUD backend
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── config.rs
│   │       ├── errors.rs
│   │       ├── handlers/
│   │       ├── models/
│   │       └── routes/
│   └── dev-gateway/             # Reverse proxy gateway
│       ├── Cargo.toml
│       ├── .env.example
│       └── src/
│           ├── main.rs          # Server bootstrap + middleware
│           ├── config.rs        # GatewayConfig (env + gateway.toml)
│           ├── errors.rs        # GatewayError → HTTP mapping
│           ├── client.rs        # Shared Hyper connection pool
│           ├── routes/
│           │   └── mod.rs       # Config-driven route matcher
│           ├── proxy/
│           │   ├── mod.rs       # GatewayState + proxy handler
│           │   ├── headers.rs   # Header sanitization + forwarding logic
│           │   ├── forward.rs   # Streaming forwarding + retries
│           │   └── upgrade.rs   # WebSocket tunneling
│           └── middleware/
│               └── logging.rs   # Single-line request logger
```

---

## Services & Default Ports

| Service     | Default Port |
| ----------- | ------------ |
| dev-gateway | 8080         |
| todo-api    | 3001         |
| example API | 8000         |

All ports are configurable via **environment variables** and/or **`gateway.toml`**.

---

## Routing Model

### Declarative, Config-Driven Routing

The gateway does **not** assume any framework, backend role, or technology stack.

All routing is defined through:

1. `gateway.toml` (preferred)
2. Environment variables (fallback)
3. Hardcoded defaults (last resort)

### Example `gateway.toml`

```toml
[[routes]]
name = "api"
match = "/api"
strip_prefix = true
upstream = "http://127.0.0.1:8000"

[[routes]]
name = "frontend"
match = "/"
strip_prefix = false
upstream = "http://127.0.0.1:3000"
```

### Matching Rules

A route matches if:

* `path == match`
* `path.starts_with(match + "/")`

This prevents unsafe overlaps such as:

* `/apiary`
* `/api-websocket`

---

## What the Dev Gateway Does

### 1. Boundary-Safe Routing

* Deterministic route resolution
* Explicit prefix stripping
* Safe against accidental overlaps
* Fully generic (no hardcoded backend roles)

Suitable for:

* Microservice setups
* Monorepos
* Frontend + API stacks
* Multiple APIs behind a single port

---

### 2. Streaming Proxying (No Buffering)

* Request and response bodies are streamed end-to-end
* No buffering into `Vec<u8>`
* Uses `http_body_util` to adapt Hyper bodies into Axum

**Why this matters**

* Large uploads
* Slow clients
* Backpressure-sensitive workloads
* Predictable memory usage

---

### 3. Retry & Backoff (Safe by Design)

Retries are only enabled for requests that are:

* Idempotent (`GET`, `HEAD`, etc.)
* Bodyless

Examples:

```
✔ GET /health        → retryable
✖ POST /upload      → not retryable
```

This preserves the no-buffering guarantee while improving resilience against transient upstream failures.

---

### 4. WebSocket Tunneling

* Transparent WebSocket proxying
* Uses `hyper::upgrade`
* Bidirectional streaming via `tokio::io::copy_bidirectional`
* Clean EOF handling to prevent file descriptor leaks

---

### 5. Correct Header Handling

**Hop-by-hop headers** are stripped:

* `Connection`
* `Transfer-Encoding`
* and others

**Forwarded headers**:

* `X-Forwarded-For` (appended, not overwritten)
* `X-Forwarded-Host`
* `X-Forwarded-Proto`

Fully compliant with proxy chaining expectations and covered by unit tests.

---

### 6. Middleware Guarantees

| Feature         | Default            |
| --------------- | ------------------ |
| Max body size   | 10 MB              |
| Request timeout | 30 sec             |
| Logging         | 1 line per request |
| Error mapping   | JSON + HTTP status |

**Example log line**

```
2026-07-01T00:00:00Z GET /api/v1/todos
→ http://127.0.0.1:8000/v1/todos
STATUS: 200 | LATENCY: 3.20ms
```

---

## Configuration

### `.env.example` (dev-gateway)

```
GATEWAY_HOST=127.0.0.1
GATEWAY_PORT=8080

GATEWAY_CONFIG_PATH=./gateway.toml

MAX_BODY_BYTES=10485760
REQUEST_TIMEOUT_SECS=30
```

### Configuration Precedence

1. Environment variables
2. `gateway.toml`
3. Hardcoded defaults

All configuration values are:

* Loaded via `dotenvy`
* Parsed with full type safety
* Validated at startup

---

## Error Mapping

| Condition              | HTTP Status               |
| ---------------------- | ------------------------- |
| Upstream unreachable   | 502 Bad Gateway           |
| Upstream timeout       | 504 Gateway Timeout       |
| Request body too large | 413 Payload Too Large     |
| Internal gateway error | 500 Internal Server Error |

**Example error response**

```json
{
  "error": "BAD_GATEWAY",
  "message": "Connection refused"
}
```

---

## Running Locally

### 1. Start a backend (test API)

```bash
SERVER_PORT=8000 cargo run -p todo-api
```

> This uses the **test-only `todo-api`** as a backend stand-in.

### 2. Start the gateway

```bash
cargo run -p dev-gateway
```

### 3. Test via the gateway

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

**502 Bad Gateway**

* Point a route upstream at a closed port
* Start the gateway and issue a request

**413 Payload Too Large**

```bash
curl -X POST http://127.0.0.1:8080/api/test -d @large_file.bin
```
