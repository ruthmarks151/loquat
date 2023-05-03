use std::{path::PathBuf, str::FromStr};

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Extension, Json, Router,
};
use sqlx::{postgres::PgRow, Executor, FromRow, PgPool, Row};
use tower_http::services::{ServeDir, ServeFile};

use loquat_common::{
    api::{GetFanSeriesResponse, GetFanSizeResponse},
    models::{fan_series::FanSeries, fan_size::FanSize, fan_type::FanType},
};
struct DbFanSeries(FanSeries);
struct DbFanSize(FanSize);

impl FromRow<'_, PgRow> for DbFanSeries {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(DbFanSeries(FanSeries {
            id: row.try_get("id")?,
            fan_type: FanType::from_str(row.try_get("fan_type")?).map_err(|_| {
                sqlx::Error::TypeNotFound {
                    type_name: "fan_type".to_owned(),
                }
            })?,
        }))
    }
}

impl FromRow<'_, PgRow> for DbFanSize {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(DbFanSize(FanSize {
            id: row.try_get("id")?,
            fan_series_id: row.try_get("fan_series_id")?,
            diameter: row.try_get("diameter")?,
        }))
    }
}

async fn get_fan_series(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<GetFanSeriesResponse>, String> {
    let fan_series = sqlx::query_as("SELECT * FROM fan_serieses WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|data: DbFanSeries| data.0)?;

    let fan_sizes = sqlx::query_as("SELECT * FROM fan_sizes WHERE fan_series_id = $1")
        .bind(fan_series.id.clone())
        .fetch_all(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|data: Vec<DbFanSize>| data.into_iter().map(|DbFanSize(size)| size).collect())?;

    Ok(Json(GetFanSeriesResponse {
        fan_series,
        fan_sizes,
    }))
}

async fn get_fan_serieses(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<FanSeries>>, String> {
    sqlx::query_as("SELECT * FROM fan_serieses LIMIT 50")
        .fetch_all(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|data: Vec<DbFanSeries>| Json(data.into_iter().map(|DbFanSeries(fan)| fan).collect()))
}

async fn get_fan_size(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<GetFanSizeResponse>, String> {
    let fan_size = sqlx::query_as("SELECT * FROM fan_sizes WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|data: DbFanSize| data.0)?;

    let fan_series = sqlx::query_as("SELECT * FROM fan_serieses WHERE id = $1")
        .bind(fan_size.fan_series_id.clone())
        .fetch_one(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|DbFanSeries(series)| series)?;

    Ok(Json(GetFanSizeResponse {
        fan_series,
        fan_size,
    }))
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
        .route("/api/fan_series", get(get_fan_serieses))
        .route("/api/fan_series/:id", get(get_fan_series))
        .route("/api/fan_sizes/:id", get(get_fan_size))
        .fallback_service(serve_dir)
        .layer(Extension(pool));
    Ok(router.into())
}
