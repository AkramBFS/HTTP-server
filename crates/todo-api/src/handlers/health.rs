use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

/// Responds with standard execution metadata to confirm service health.
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
