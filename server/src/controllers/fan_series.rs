use std::str::FromStr;

use axum::{extract::Path, Extension, Json};
use itertools::Itertools;
use sqlx::{FromRow, PgPool, Row};

use loquat_common::{
    api::fan_series::{GetResponse, IndexResponse},
    models::{FanSeries, FanSize, FanType},
};

use crate::db::Db;

pub async fn index(Extension(pool): Extension<PgPool>) -> Result<Json<IndexResponse>, String> {
    sqlx::query_as("SELECT id as fan_series_id, fan_type FROM fan_serieses LIMIT 50")
        .fetch_all(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|data: Vec<Db<FanSeries<()>>>| Json(data.into_iter().map(|Db(fan)| fan).collect()))
}

pub async fn get(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<GetResponse>, String> {
    let rows = sqlx::query(
        "SELECT fan_sizes.id as fan_size_id, diameter, fan_serieses.id as fan_series_id, fan_type
            FROM fan_serieses
            LEFT JOIN fan_sizes
            ON fan_sizes.fan_series_id = fan_serieses.id
            WHERE fan_serieses.id = $1",
    )
    .bind(&id)
    .fetch_all(&pool)
    .await
    .map_err(|err| err.to_string())?;

    let groups = rows
        .into_iter()
        .group_by(|row| (row.get("fan_series_id"), row.get("fan_type")) as (String, String));

    let fan_series_group = groups
        .into_iter()
        .find(|((fan_series_id, _), _)| fan_series_id == &id);

    if let Some(((fan_series_id, fan_type), rows)) = fan_series_group {
        let fan_sizes: Vec<FanSize<()>> = rows
            .map(|row| {
                Db::from_row(&row)
                    .map(|Db(fan_size)| fan_size)
                    .map_err(|err| format!("Could not parse FanSize {}", err).to_string())
            })
            .collect::<Result<_, _>>()?;
        let fan_type: FanType =
            FanType::from_str(&fan_type).map_err(|_| "Could not parase FanType".to_string())?;
        Ok(Json(FanSeries {
            id: fan_series_id,
            fan_type,
            fan_sizes: fan_sizes,
        }))
    } else {
        Err("Could not find Fan Series".to_string())
    }
}
