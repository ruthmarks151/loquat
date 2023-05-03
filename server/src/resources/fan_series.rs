use axum::{extract::Path, Extension, Json};
use sqlx::PgPool;

use loquat_common::api::fan_series::{GetResponse, IndexResponse};

use crate::db::{DbFanSeries, DbFanSize};

pub async fn index(Extension(pool): Extension<PgPool>) -> Result<Json<IndexResponse>, String> {
    sqlx::query_as("SELECT * FROM fan_serieses LIMIT 50")
        .fetch_all(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|data: Vec<DbFanSeries>| Json(data.into_iter().map(|DbFanSeries(fan)| fan).collect()))
}

pub async fn get(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<GetResponse>, String> {
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

    Ok(Json(GetResponse {
        fan_series,
        fan_sizes,
    }))
}
