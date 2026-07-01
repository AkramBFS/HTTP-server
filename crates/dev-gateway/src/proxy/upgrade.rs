use axum::body::Body;
use hyper::upgrade::OnUpgrade;
use hyper_util::rt::TokioIo;
use tokio::io::copy_bidirectional;
use tracing::{error, info};

use crate::errors::GatewayError;

/// Handles a WebSocket (or any HTTP Upgrade) tunnel.
///
/// Lifecycle:
/// 1. The caller has already extracted the client's OnUpgrade future from the inbound request
///    *before* dispatching it upstream.
/// 2. The caller dispatched the sanitized request and received a 101 Switching Protocols response.
/// 3. This function extracts the upstream's OnUpgrade future from that response, resolves both
///    upgrade futures, and bridges the two streams with bidirectional copy.
///
/// Returns the 101 response (with body stripped) so the caller can send it back to the client,
/// and spawns the bidirectional tunnel as a background task.
pub fn handle_upgrade(
    client_upgrade: OnUpgrade,
    mut response: hyper::Response<Body>,
) -> Result<hyper::Response<Body>, GatewayError> {
    // Extract the upstream's upgrade future from the response extensions.
    let server_upgrade = response
        .extensions_mut()
        .remove::<OnUpgrade>()
        .ok_or_else(|| {
            GatewayError::BadGateway(
                "Upstream returned 101 but no upgrade future available".to_string(),
            )
        })?;

    // Spawn a dedicated task for the bidirectional tunnel.
    tokio::spawn(async move {
        match tokio::try_join!(client_upgrade, server_upgrade) {
            Ok((upgraded_client, upgraded_server)) => {
                // Wrap hyper Upgraded streams in TokioIo adapters so they
                // implement tokio::io::AsyncRead + AsyncWrite.
                let mut client_io = TokioIo::new(upgraded_client);
                let mut server_io = TokioIo::new(upgraded_server);

                match copy_bidirectional(&mut client_io, &mut server_io).await {
                    Ok((client_to_server, server_to_client)) => {
                        info!(
                            "WebSocket tunnel closed: client->server {} bytes, server->client {} bytes",
                            client_to_server, server_to_client
                        );
                    }
                    Err(e) => {
                        error!("WebSocket tunnel I/O error: {}", e);
                    }
                }
                // Both streams are dropped here, closing both halves immediately.
            }
            Err(e) => {
                error!("WebSocket upgrade resolution failed: {}", e);
            }
        }
    });

    Ok(response)
}
