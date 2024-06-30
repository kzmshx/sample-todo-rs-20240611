use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use sqlx::PgPool;

use super::models::{CreateTaskInput, UpdateTaskInput};

pub(crate) fn router(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/tasks", get(tasks_list).post(tasks_create))
        .route(
            "/tasks/:id",
            get(tasks_get).post(tasks_update).delete(tasks_delete),
        )
        .route("/tasks/:id/close", post(tasks_close))
        .route("/tasks/:id/reopen", post(tasks_reopen))
        .with_state(pool)
}

async fn tasks_list(State(_pool): State<PgPool>) -> impl IntoResponse {
    "List tasks"
}

async fn tasks_create(
    State(_pool): State<PgPool>,
    Json(_payload): Json<CreateTaskInput>,
) -> impl IntoResponse {
    "Create a task"
}

async fn tasks_get(State(_pool): State<PgPool>, Path(_id): Path<u64>) -> impl IntoResponse {
    "Get a task"
}

async fn tasks_update(
    State(_pool): State<PgPool>,
    Path(_id): Path<u64>,
    Json(_payload): Json<UpdateTaskInput>,
) -> impl IntoResponse {
    "Update a task"
}

async fn tasks_delete(State(_pool): State<PgPool>, Path(_id): Path<u64>) -> impl IntoResponse {
    "Delete a task"
}

async fn tasks_close(State(_pool): State<PgPool>, Path(_id): Path<u64>) -> impl IntoResponse {
    "Close a task"
}

async fn tasks_reopen(State(_pool): State<PgPool>, Path(_id): Path<u64>) -> impl IntoResponse {
    "Reopen a task"
}
