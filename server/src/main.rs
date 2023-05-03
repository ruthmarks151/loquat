use std::{path::PathBuf};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Extension, Router,
};
use loquat_server::resources;
use sqlx::{Executor, PgPool};
use tower_http::services::{ServeDir, ServeFile};

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    pool.execute(include_str!("../schema.sql"))
        .await
        .expect("Schema didn't execute");

    // In production, serve from the root static folder
    // In dev, this is empty and the frontend proxies to anything /api to this server
    let serve_dir = get_service(
        ServeDir::new(static_folder.clone())
            .fallback(ServeFile::new(static_folder.join("index.html"))),
    )
    .handle_error(handle_error);

    let router = Router::new()
        .route("/api/fan_series", get(resources::fan_series::index))
        .route("/api/fan_series/:id", get(resources::fan_series::get))
        .route("/api/fan_sizes", get(resources::fan_size::index))
        .route("/api/fan_sizes/:id", get(resources::fan_size::get))
        .fallback_service(serve_dir)
        .layer(Extension(pool));
    Ok(router.into())
}
