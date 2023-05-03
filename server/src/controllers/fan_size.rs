use axum::{extract::Path, Extension, Json};
use sqlx::PgPool;

use loquat_common::api::fan_size::{GetResponse, IndexResponse};

use crate::db::{DbFanSeries, DbFanSize};

pub async fn index(Extension(pool): Extension<PgPool>) -> Result<Json<IndexResponse>, String> {
    let fan_sizes = sqlx::query_as("SELECT * FROM fan_sizes LIMIT 50")
        .fetch_all(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|data: Vec<DbFanSize>| data.into_iter().map(|db_fan| db_fan.0).collect())?;
    Ok(Json(fan_sizes))
}

pub async fn get(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<GetResponse>, String> {
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

    Ok(Json(GetResponse {
        fan_series,
        fan_size,
    }))
}