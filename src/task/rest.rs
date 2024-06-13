use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

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

async fn tasks_create(State(_pool): State<PgPool>) -> impl IntoResponse {
    "Create a task"
}

async fn tasks_get(Path(_id): Path<u64>, State(_pool): State<PgPool>) -> impl IntoResponse {
    "Get a task"
}

async fn tasks_update(Path(_id): Path<u64>, State(_pool): State<PgPool>) -> impl IntoResponse {
    "Update a task"
}

async fn tasks_delete(Path(_id): Path<u64>, State(_pool): State<PgPool>) -> impl IntoResponse {
    "Delete a task"
}

async fn tasks_close(Path(_id): Path<u64>, State(_pool): State<PgPool>) -> impl IntoResponse {
    "Close a task"
}

async fn tasks_reopen(Path(_id): Path<u64>, State(_pool): State<PgPool>) -> impl IntoResponse {
    "Reopen a task"
}
