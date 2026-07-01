# Dev Gateway Reverse Proxy

A local development reverse proxy gateway written in Rust and Axum. It sits in
front of multiple backend/frontend services and provides boundary-aware routing,
streaming proxying, WebSocket tunneling, request limits, timeouts, retries, and
structured request logging.

## Workspace

This repository is a Cargo workspace with two binaries:

- `todo-api`: a small Axum CRUD API used as a stand-in backend.
- `dev-gateway`: the reverse proxy gateway.

## Configuration

The gateway can run from a `gateway.toml` file for generic local service routing:

```toml
[server]
host = "127.0.0.1"
port = 8080

[limits]
max_body_bytes = 10485760
request_timeout_secs = 30

[retries]
attempts = 3
backoff_ms = 50

[[routes]]
path = "/api"
target = "http://127.0.0.1:8000"
strip_prefix = true

[[routes]]
path = "/"
target = "http://127.0.0.1:3000"
strip_prefix = false
```

Copy `gateway.toml.example` to `gateway.toml`, or set `GATEWAY_CONFIG` to a
custom file path. If no config file is present, the gateway falls back to the
legacy environment variables from `crates/dev-gateway/.env.example`.

Routes are matched with path boundaries, and the longest matching route wins.
That means `/api` matches `/api` and `/api/v1/todos`, but not `/apiary`.

## Resilience

Bodyless idempotent requests are retried when the upstream connection fails.
Retries default to 3 attempts with a 50ms backoff and can be configured in TOML
or with `RETRY_ATTEMPTS` and `RETRY_BACKOFF_MS` when using env fallback.

Streaming request bodies are not retried because replaying them would require
buffering or spooling the body, which would break the gateway's no-buffering
proxy model.

## Forwarded Headers

The gateway strips hop-by-hop headers and manages forwarding headers:

- `X-Forwarded-For` appends the client IP to any existing chain.
- `X-Forwarded-Host` preserves the original `Host` header.
- `X-Forwarded-Proto` is set to `http` for local development.

## Running Locally

```powershell
$env:SERVER_PORT = "8000"
cargo run -p todo-api
```

```powershell
cargo run -p dev-gateway
```

```powershell
curl http://127.0.0.1:8080/api/v1/todos
```

## Verification

```powershell
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

## Performance

Simple local smoke benchmark on Windows using debug binaries and 200 sequential
requests from .NET `HttpClient`:

| Path | Average latency | Requests/sec |
| --- | ---: | ---: |
| Direct `todo-api` | 1.193ms | 838.19 |
| Through `dev-gateway` | 2.723ms | 367.24 |

Observed gateway overhead in that run was 1.530ms per request. For release-grade
numbers, run the same comparison with optimized binaries and a load tool such as
`hey` or `wrk`.
