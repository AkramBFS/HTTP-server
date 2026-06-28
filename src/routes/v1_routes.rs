use crate::handlers::{create_todo, delete_todo, get_todo, list_todos, update_todo};
use crate::routes::AppState;
use axum::{routing::get, Router};

/// Initializes the V1 API sub-router using the shared Application State.
/// Note that Axum 0.7 uses the ":" parameter matching syntax.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/:id",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
}
