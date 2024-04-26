use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::AppState;
use crate::models::{CreateTodo, PatchTodo, Todo, UpdateTodo};

pub async fn list_todos(State(state): State<AppState>) -> impl IntoResponse {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await;

    match todos {
        Ok(todos) => Json(todos).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, description, completed) VALUES (?, ?, ?) RETURNING *",
    )
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.completed)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await;

    match result {
        Ok(Some(todo)) => Json(todo).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateTodo>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET title = ?, description = ?, completed = ?, updated_at = datetime('now') WHERE id = ? RETURNING *",
    )
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.completed)
    .bind(id)
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(todo)) => Json(todo).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn patch_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<PatchTodo>,
) -> impl IntoResponse {
    let existing = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await;

    let existing = match existing {
        Ok(Some(todo)) => todo,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let title = input.title.unwrap_or(existing.title);
    let description = input.description.unwrap_or(existing.description);
    let completed = input.completed.unwrap_or(existing.completed);

    let result = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET title = ?, description = ?, completed = ?, updated_at = datetime('now') WHERE id = ? RETURNING *",
    )
    .bind(&title)
    .bind(&description)
    .bind(completed)
    .bind(id)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(todo) => Json(todo).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
