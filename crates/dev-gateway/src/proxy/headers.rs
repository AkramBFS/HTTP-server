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
pub fn inject_forwarded_headers<B>(req: &mut Request<B>, client_ip: &str) {
    let headers = req.headers_mut();

    append_x_forwarded_for(headers, client_ip);

    // X-Forwarded-Host: original Host header value
    if let Some(host) = headers.get("host").cloned() {
        headers.insert(HeaderName::from_static("x-forwarded-host"), host);
    }

    // X-Forwarded-Proto: always "http" for this local dev gateway
    headers.insert(
        HeaderName::from_static("x-forwarded-proto"),
        HeaderValue::from_static("http"),
    );
}

fn append_x_forwarded_for(headers: &mut HeaderMap, client_ip: &str) {
    let next_value = match headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
    {
        Some(existing) if !existing.trim().is_empty() => format!("{}, {}", existing, client_ip),
        _ => client_ip.to_string(),
    };

    if let Ok(value) = HeaderValue::from_str(&next_value) {
        headers.insert(HeaderName::from_static("x-forwarded-for"), value);
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{HeaderValue, Request};

    use super::inject_forwarded_headers;

    #[test]
    fn appends_x_forwarded_for_when_header_exists() {
        let mut request = Request::builder()
            .header("x-forwarded-for", "10.0.0.1")
            .body(Body::empty())
            .unwrap();

        inject_forwarded_headers(&mut request, "127.0.0.1");

        assert_eq!(
            request.headers().get("x-forwarded-for"),
            Some(&HeaderValue::from_static("10.0.0.1, 127.0.0.1"))
        );
    }
}
