pub mod health;

use crate::errors::AppError;
use crate::models::{CreateTodo, Todo, UpdateTodo};
use crate::routes::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

/// Creates a new task and inserts it into the thread-safe state repository.
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<impl IntoResponse, AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Task title must contain valid characters".to_string(),
        ));
    }

    let mut database = state.db.write().map_err(|_| {
        AppError::InternalServerError("Database write lock acquisition poisoned".to_string())
    })?;

    let next_id = database.iter().map(|item| item.id).max().unwrap_or(0) + 1;
    let fresh_todo = Todo {
        id: next_id,
        title: payload.title,
        completed: false,
    };
    database.push(fresh_todo.clone());

    Ok((StatusCode::CREATED, Json(fresh_todo)))
}

/// Retrieves all tasks currently stored in memory.
pub async fn list_todos(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let database = state.db.read().map_err(|_| {
        AppError::InternalServerError("Database read lock acquisition poisoned".to_string())
    })?;

    let items: Vec<Todo> = database.clone();
    Ok(Json(items))
}

/// Retrieves a single task by its unique ID.
pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let database = state.db.read().map_err(|_| {
        AppError::InternalServerError("Database read lock acquisition poisoned".to_string())
    })?;

    let item = database
        .iter()
        .find(|t| t.id == id)
        .cloned()
        .ok_or_else(|| AppError::NotFound(format!("Todo record with ID {} not found", id)))?;

    Ok(Json(item))
}

/// Modifies an existing task based on the provided fields.
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, AppError> {
    let mut database = state.db.write().map_err(|_| {
        AppError::InternalServerError("Database write lock acquisition poisoned".to_string())
    })?;

    let target_todo = database
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Todo record with ID {} not found", id)))?;

    if let Some(ref title) = payload.title {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest(
                "Updated title must not be empty".to_string(),
            ));
        }
        target_todo.title = title.clone();
    }

    if let Some(completed) = payload.completed {
        target_todo.completed = completed;
    }

    Ok(Json(target_todo.clone()))
}

/// Removes a task from the in-memory repository.
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, AppError> {
    let mut database = state.db.write().map_err(|_| {
        AppError::InternalServerError("Database write lock acquisition poisoned".to_string())
    })?;

    let position = database
        .iter()
        .position(|t| t.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Todo record with ID {} not found", id)))?;

    database.remove(position);

    Ok(StatusCode::NO_CONTENT)
}
