use std::path::PathBuf;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Extension, Json, Router,
};
use sqlx::types::Uuid;
use sqlx::{postgres::PgRow, Executor, FromRow, PgPool, Row};
use tower_http::services::{ServeDir, ServeFile};

use loquat_common::models::fan::Fan;
struct DbFan(Fan);

impl FromRow<'_, PgRow> for DbFan {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let uuid: Uuid = row.try_get("id")?;
        Ok(DbFan(Fan {
            id: uuid.to_string(),
            name: row.try_get("name")?,
        }))
    }
}

async fn get_fan(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Fan>, String> {
    let uuid = Uuid::parse_str(&id).map_err(|_err| "ID was not a valid UUID".to_string())?;

    sqlx::query_as("SELECT * FROM fans WHERE id = $1")
        .bind(uuid)
        .fetch_one(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|data: DbFan| Json(data.0))
}

async fn get_fans(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<Fan>>, String> {
    sqlx::query_as("SELECT * FROM fans LIMIT 50")
        .fetch_all(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|data: Vec<DbFan>| Json(data.into_iter().map(|DbFan(fan)| fan).collect()))
}

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
        .route("/api/fans", get(get_fans))
        .route("/api/fans/:id", get(get_fan))
        .fallback_service(serve_dir)
        .layer(Extension(pool));
    Ok(router.into())
}
