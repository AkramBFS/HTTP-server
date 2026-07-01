mod client;
mod config;
mod errors;
mod middleware;
mod proxy;
mod routes;

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::signal;
use tower::ServiceBuilder;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;
use tracing::{error, info};
use tracing_subscriber::{fmt, EnvFilter};

use crate::config::GatewayConfig;
use crate::middleware::logging::LoggingLayer;
use crate::proxy::GatewayState;

#[tokio::main]
async fn main() {
    // Step 1: Load configuration from environment.
    let gateway_config = match GatewayConfig::load() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!(
                "Initialization halted. Configuration loading failed: {}",
                err
            );
            std::process::exit(1);
        }
    };

    // Step 2: Initialize structured logging.
    let level_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("dev_gateway=debug,tower_http=debug,axum=info"));

    fmt().with_env_filter(level_filter).init();

    info!(
        "Starting dev-gateway on {}:{}",
        gateway_config.bind_host, gateway_config.bind_port
    );
    info!(
        "Loaded {} route(s); retry attempts: {}, backoff: {}ms",
        gateway_config.routes.len(),
        gateway_config.retry.attempts,
        gateway_config.retry.backoff_ms
    );
    for route in &gateway_config.routes {
        info!(
            "Routing {} -> {} (strip_prefix: {})",
            route.path, route.target, route.strip_prefix
        );
    }

    // Step 3: Build the shared upstream client.
    let http_client = Arc::new(client::build_client());

    // Step 4: Build shared gateway state.
    let state = GatewayState {
        config: Arc::new(gateway_config.clone()),
        client: http_client,
    };

    // Step 5: Build the router with layered middleware.
    //   Outermost to innermost: body limit -> timeout -> logging -> handler.
    let app = axum::Router::new()
        .fallback(proxy::proxy_handler)
        .layer(
            ServiceBuilder::new()
                .layer(RequestBodyLimitLayer::new(gateway_config.max_body_bytes))
                .layer(TimeoutLayer::new(Duration::from_secs(
                    gateway_config.request_timeout_secs,
                )))
                .layer(LoggingLayer),
        )
        .with_state(state);

    // Step 6: Bind TCP listener.
    let socket_address: SocketAddr =
        format!("{}:{}", gateway_config.bind_host, gateway_config.bind_port)
            .parse()
            .unwrap_or_else(|e| {
                eprintln!("Invalid bind address: {}", e);
                std::process::exit(1);
            });

    let tcp_listener = match tokio::net::TcpListener::bind(socket_address).await {
        Ok(listener) => listener,
        Err(err) => {
            error!("Failed to bind TCP listener on {}: {}", socket_address, err);
            std::process::exit(1);
        }
    };

    info!("Dev Gateway listening on http://{}", socket_address);

    // Step 7: Launch server with graceful shutdown.
    if let Err(err) = axum::serve(
        tcp_listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(register_shutdown_signals())
    .await
    {
        error!("Runtime error encountered: {}", err);
    }
}

/// Listens for standard termination signals from the operating system.
async fn register_shutdown_signals() {
    let ctrl_c_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to initialize SIGINT listener");
    };

    #[cfg(unix)]
    let terminate_signal = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to initialize SIGTERM listener")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate_signal = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c_signal => {},
        _ = terminate_signal => {},
    }

    info!("Initiating graceful gateway shutdown...");
}
