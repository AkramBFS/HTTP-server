pub mod v1_routes;

use crate::config::AppConfig;
use crate::handlers::health::health_check;
use crate::models::Todo;
use axum::{routing::get, Router};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)]
    pub config: AppConfig,
    pub db: Arc<RwLock<Vec<Todo>>>,
}

/// Builds the root router, registers endpoints, and injects global state.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", v1_routes::routes())
        .with_state(state)
}
