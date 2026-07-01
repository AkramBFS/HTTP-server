mod config;
mod errors;
mod handlers;
mod models;
mod routes;

use std::sync::{Arc, RwLock};
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber::{fmt, EnvFilter};

use crate::config::AppConfig;
use crate::routes::{create_router, AppState};

#[tokio::main]
async fn main() {
    // Attempt configuration loading from environment definitions.
    let system_config = match AppConfig::load() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!(
                "Initialization halted. Configuration loading failed: {}",
                err
            );
            std::process::exit(1);
        }
    };

    // Initialize structured logging and diagnostic frameworks.
    let level_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let trace_level = if system_config.environment == "development" {
            "todo_api=debug,tower_http=debug,axum=info"
        } else {
            "todo_api=info,tower_http=info,axum=warn"
        };
        EnvFilter::new(trace_level)
    });

    fmt().with_env_filter(level_filter).init();

    info!(
        "Bootstrapping server environment: {}",
        system_config.environment
    );

    // Initialize the shared application state.
    let shared_state = AppState {
        config: system_config.clone(),
        db: Arc::new(RwLock::new(Vec::new())),
    };

    // Construct the primary application router.
    let application_router = create_router(shared_state);

    // Bind to the configured TCP socket.
    let socket_address_str = format!(
        "{}:{}",
        system_config.server_host, system_config.server_port
    );
    let tcp_listener = match tokio::net::TcpListener::bind(&socket_address_str).await {
        Ok(listener) => listener,
        Err(err) => {
            error!(
                "Failed to bind TCP listener on {}: {}",
                socket_address_str, err
            );
            std::process::exit(1);
        }
    };

    info!("HTTP Server listening on http://{}", socket_address_str);

    // Launch the server with graceful shutdown orchestration.
    if let Err(err) = axum::serve(tcp_listener, application_router)
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

    info!("Initiating graceful server shutdown...");
}
