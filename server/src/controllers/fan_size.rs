use axum::{extract::Path, Extension, Json};
use sqlx::PgPool;

use loquat_common::{api::fan_size::{GetResponse, IndexResponse}, models::FanSize};

use crate::db::Db;

pub async fn index(Extension(pool): Extension<PgPool>) -> Result<Json<IndexResponse>, String> {
    let fan_sizes = sqlx::query_as("SELECT * FROM fan_sizes LIMIT 50")
        .fetch_all(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|data: Vec<Db<FanSize<()>>>| data.into_iter().map(|db_fan| db_fan.0).collect())?;
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
        .map(|Db(data): Db<FanSize<()>>| data)?;

    let fan_series = sqlx::query_as("SELECT * FROM fan_serieses WHERE id = $1")
        .bind(fan_size.fan_series_id.clone())
        .fetch_one(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|Db(series)| series)?;

    Ok(Json(FanSize {
            id: fan_size.id,
            fan_series_id: fan_size.fan_series_id,
            fan_series: fan_series,
            diameter: fan_size.diameter,
        },
    ))
}
