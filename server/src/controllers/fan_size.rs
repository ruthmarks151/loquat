use axum::{extract::Path, Extension, Json};
use sqlx::PgPool;

use loquat_common::{
    api::fan_size::{GetResponse, IndexResponse},
    models::{FanSeries, FanSize},
};

use crate::db::Db;

pub async fn index(Extension(pool): Extension<PgPool>) -> Result<Json<IndexResponse>, String> {
    let fan_sizes = sqlx::query_as(
            "SELECT * FROM fan_sizes LIMIT 50"
        )
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
    let fan_size = sqlx::query_as(
        "SELECT fan_sizes.id as fan_size_id, fan_series_id, fan_type, diameter
             FROM fan_sizes 
             JOIN fan_serieses ON fan_series_id = fan_serieses.id 
             WHERE fan_sizes.id = $1",
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|err| err.to_string())
    .map(|Db(data): Db<FanSize<FanSeries<()>>>| data)?;

    Ok(Json(fan_size))
}
