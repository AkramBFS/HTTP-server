use axum::body::Body;
use hyper::Request;
use http_body_util::BodyExt;

use crate::client::HttpClient;
use crate::errors::GatewayError;

/// Dispatches a request through the shared connection pool and streams the
/// response back without buffering the entire body into memory.
pub async fn forward_request(
    client: &HttpClient,
    req: Request<Body>,
) -> Result<hyper::Response<Body>, GatewayError> {
    let response = client
        .request(req)
        .await
        .map_err(classify_upstream_error)?;

    // Convert hyper::body::Incoming → axum::body::Body via BoxBody adapter.
    let (parts, incoming) = response.into_parts();
    let body = Body::new(incoming.map_err(|e| {
        axum::Error::new(e)
    }));

    Ok(hyper::Response::from_parts(parts, body))
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
                std::io::ErrorKind::TimedOut => {
                    GatewayError::GatewayTimeout(format!("Upstream connection timed out: {}", message))
                }
                _ => GatewayError::BadGateway(format!("Upstream I/O error: {}", message)),
            };
        }
        source = inner.source();
    }

    // Default: treat any non-I/O upstream error as a bad gateway.
    GatewayError::BadGateway(format!("Upstream error: {}", message))
}
