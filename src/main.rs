use axum::{routing::get, Router};
use shuttle_runtime::CustomError;

mod task;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: sqlx::PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let router = Router::new()
        .route("/hello", get(hello))
        .merge(task::rest::router(pool));

    Ok(router.into())
}

async fn hello() -> &'static str {
    "Hello, world!"
}
