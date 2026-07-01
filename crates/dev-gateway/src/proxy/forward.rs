use axum::body::Body;
use axum::http::{HeaderMap, HeaderValue, Method, Request};
use http_body_util::BodyExt;
use std::time::Duration;
use tokio::time::sleep;
use tracing::debug;

use crate::client::HttpClient;
use crate::config::RetryConfig;
use crate::errors::GatewayError;

/// Dispatches a request through the shared connection pool and streams the
/// response back without buffering the entire body into memory.
pub async fn forward_request(
    client: &HttpClient,
    req: Request<Body>,
    retry: RetryConfig,
    allow_retry: bool,
) -> Result<hyper::Response<Body>, GatewayError> {
    if allow_retry && is_retryable_request(&req) && retry.attempts > 1 {
        return forward_retryable_request(client, req, retry).await;
    }

    let response = client.request(req).await.map_err(classify_upstream_error)?;

    // Convert hyper::body::Incoming -> axum::body::Body via BoxBody adapter.
    let (parts, incoming) = response.into_parts();
    let body = Body::new(incoming.map_err(axum::Error::new));

    Ok(hyper::Response::from_parts(parts, body))
}

async fn forward_retryable_request(
    client: &HttpClient,
    req: Request<Body>,
    retry: RetryConfig,
) -> Result<hyper::Response<Body>, GatewayError> {
    let (parts, _) = req.into_parts();
    let attempts = retry.attempts.max(1);

    for attempt in 1..=attempts {
        let request = Request::builder()
            .method(parts.method.clone())
            .uri(parts.uri.clone())
            .version(parts.version)
            .body(Body::empty())
            .map_err(|e| {
                GatewayError::InternalServerError(format!(
                    "Failed to rebuild retryable upstream request: {}",
                    e
                ))
            })?;

        let (mut rebuilt_parts, body) = request.into_parts();
        rebuilt_parts.headers = parts.headers.clone();
        rebuilt_parts.extensions = parts.extensions.clone();

        match client
            .request(Request::from_parts(rebuilt_parts, body))
            .await
        {
            Ok(response) => {
                let (parts, incoming) = response.into_parts();
                let body = Body::new(incoming.map_err(axum::Error::new));
                return Ok(hyper::Response::from_parts(parts, body));
            }
            Err(err) if attempt < attempts && is_retryable_upstream_error(&err) => {
                debug!(
                    "Upstream request failed on attempt {}/{}; retrying in {}ms: {}",
                    attempt, attempts, retry.backoff_ms, err
                );
                sleep(Duration::from_millis(retry.backoff_ms)).await;
            }
            Err(err) => return Err(classify_upstream_error(err)),
        }
    }

    Err(GatewayError::BadGateway(
        "Upstream request failed after retries".to_string(),
    ))
}

fn is_retryable_request(req: &Request<Body>) -> bool {
    matches!(
        *req.method(),
        Method::GET | Method::HEAD | Method::OPTIONS | Method::DELETE
    ) && has_no_declared_body(req.headers())
}

fn has_no_declared_body(headers: &HeaderMap<HeaderValue>) -> bool {
    headers
        .get("content-length")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0)
        == 0
}

fn is_retryable_upstream_error(err: &hyper_util::client::legacy::Error) -> bool {
    let mut source: Option<&(dyn std::error::Error + 'static)> = Some(err);
    while let Some(inner) = source {
        if let Some(io_err) = inner.downcast_ref::<std::io::Error>() {
            return matches!(
                io_err.kind(),
                std::io::ErrorKind::ConnectionRefused
                    | std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::ConnectionAborted
                    | std::io::ErrorKind::NotConnected
                    | std::io::ErrorKind::TimedOut
            );
        }
        source = inner.source();
    }

    false
}

/// Maps upstream connection failures into the correct gateway error variants.
fn classify_upstream_error(err: hyper_util::client::legacy::Error) -> GatewayError {
    let message = err.to_string();

    // Walk the error source chain looking for I/O errors.
    let mut source: Option<&(dyn std::error::Error + 'static)> = Some(&err);
    while let Some(inner) = source {
        if let Some(io_err) = inner.downcast_ref::<std::io::Error>() {
            return match io_err.kind() {
                std::io::ErrorKind::ConnectionRefused => {
                    GatewayError::BadGateway(format!("Upstream connection refused: {}", message))
                }
                std::io::ErrorKind::TimedOut => GatewayError::GatewayTimeout(format!(
                    "Upstream connection timed out: {}",
                    message
                )),
                _ => GatewayError::BadGateway(format!("Upstream I/O error: {}", message)),
            };
        }
        source = inner.source();
    }

    // Default: treat any non-I/O upstream error as a bad gateway.
    GatewayError::BadGateway(format!("Upstream error: {}", message))
}
