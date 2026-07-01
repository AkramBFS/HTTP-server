use axum::http::{HeaderMap, HeaderName, HeaderValue, Request};

/// Hop-by-hop headers that must be stripped from proxied requests/responses.
/// `Upgrade` is handled separately during WebSocket handshakes.
const HOP_BY_HOP_HEADERS: &[&str] = &[
    "connection",
    "keep-alive",
    "proxy-authenticate",
    "proxy-authorization",
    "te",
    "trailers",
    "transfer-encoding",
    "upgrade",
];

/// Strips hop-by-hop headers from the given header map.
/// If `preserve_upgrade` is true, the `Upgrade` header is kept (for WebSocket handshakes).
pub fn strip_hop_by_hop(headers: &mut HeaderMap, preserve_upgrade: bool) {
    for name in HOP_BY_HOP_HEADERS {
        if preserve_upgrade && *name == "upgrade" {
            continue;
        }
        headers.remove(*name);
    }
}

/// Injects `X-Forwarded-*` headers onto an outbound proxy request.
pub fn inject_forwarded_headers<B>(
    req: &mut Request<B>,
    client_ip: &str,
) {
    let headers = req.headers_mut();

    // X-Forwarded-For: client IP
    if let Ok(val) = HeaderValue::from_str(client_ip) {
        headers.insert(
            HeaderName::from_static("x-forwarded-for"),
            val,
        );
    }

    // X-Forwarded-Host: original Host header value
    if let Some(host) = headers.get("host").cloned() {
        headers.insert(
            HeaderName::from_static("x-forwarded-host"),
            host,
        );
    }

    // X-Forwarded-Proto: always "http" for this local dev gateway
    headers.insert(
        HeaderName::from_static("x-forwarded-proto"),
        HeaderValue::from_static("http"),
    );
}
