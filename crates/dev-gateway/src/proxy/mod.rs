pub mod forward;
pub mod headers;
pub mod upgrade;

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{uri::Uri, StatusCode},
    response::{IntoResponse, Response},
};
use hyper::{upgrade::OnUpgrade, Request};
use tracing::debug;

use crate::client::HttpClient;
use crate::config::GatewayConfig;
use crate::errors::GatewayError;
use crate::routes;

/// Shared state passed to the proxy handler via Axum's `State` extractor.
#[derive(Clone)]
pub struct GatewayState {
    pub config: Arc<GatewayConfig>,
    pub client: Arc<HttpClient>,
}

/// The single catch-all proxy handler.
///
/// 1. Resolves target upstream from the configured route table.
/// 2. Rewrites the path if prefix stripping is enabled.
/// 3. Sanitizes headers and injects X-Forwarded-* headers.
/// 4. If an Upgrade header is present, hands off to the WebSocket tunnel.
/// 5. Otherwise, streams the proxied response back to the client.
pub async fn proxy_handler(
    State(state): State<GatewayState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    mut req: Request<Body>,
) -> Result<Response, GatewayError> {
    let original_path = req.uri().path().to_string();
    let query = req.uri().query().map(|q| q.to_string());

    // Step 1: Route resolution.
    let route = routes::resolve_route(&original_path, &state.config.routes).ok_or_else(|| {
        GatewayError::BadGateway(format!("No route matched path {}", original_path))
    })?;
    let upstream_base = &route.target;

    // Step 2: Path rewriting.
    let rewritten_path = routes::rewrite_path(&original_path, &route.path, route.strip_prefix);

    // Reconstruct the upstream URI, preserving query string.
    let upstream_uri = match query {
        Some(q) if !q.is_empty() => format!("{}{}{}{}", upstream_base, rewritten_path, "?", q),
        _ => format!("{}{}", upstream_base, rewritten_path),
    };

    debug!(
        "{} {} -> {} (route: {})",
        req.method(),
        req.uri(),
        upstream_uri,
        route.path
    );

    // Step 3: Detect upgrade intent *before* modifying the request.
    let is_upgrade = req.headers().get("upgrade").is_some();

    // Step 4: Extract client upgrade future before dispatching (only for upgrades).
    let client_upgrade: Option<OnUpgrade> = if is_upgrade {
        req.extensions_mut().remove::<OnUpgrade>()
    } else {
        None
    };

    // Step 5: Sanitize headers, preserving Upgrade during WebSocket handshakes.
    headers::strip_hop_by_hop(req.headers_mut(), is_upgrade);
    headers::inject_forwarded_headers(&mut req, &addr.ip().to_string());

    // Step 6: Rewrite the request URI to the upstream target.
    *req.uri_mut() = upstream_uri.parse::<Uri>().map_err(|e| {
        GatewayError::InternalServerError(format!("Failed to parse upstream URI: {}", e))
    })?;

    // Step 7: Dispatch.
    let mut response =
        forward::forward_request(&state.client, req, state.config.retry, !is_upgrade).await?;

    // Step 8: Handle WebSocket upgrade if upstream responded with 101.
    if is_upgrade && response.status() == StatusCode::SWITCHING_PROTOCOLS {
        if let Some(client_up) = client_upgrade {
            response = upgrade::handle_upgrade(client_up, response)?;
            return Ok(response.into_response());
        }
    }

    // Step 9: Sanitize response headers.
    headers::strip_hop_by_hop(response.headers_mut(), false);

    Ok(response.into_response())
}
