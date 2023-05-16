use axum::{extract::Path, Extension, Json};
use itertools::Itertools;
use sqlx::PgPool;

use loquat_common::{
    api::fan_series::{GetResponse, IndexResponse},
    models::{FanSeries, FanSize, FanType},
};

pub async fn index(Extension(pool): Extension<PgPool>) -> Result<Json<IndexResponse>, String> {
    sqlx::query!("SELECT id as fan_series_id, fan_type FROM fan_serieses LIMIT 50")
        .fetch_all(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|records| {
            Json(
                records
                    .into_iter()
                    .map(|record| FanSeries {
                        id: record.fan_series_id,
                        fan_type: record.fan_type[..].try_into().unwrap(),
                        fan_sizes: (),
                    })
                    .collect(),
            )
        })
}

pub async fn get(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<GetResponse>, String> {
    let rows = sqlx::query_file!("src/queries/fan_series/get_fan_series.sql", id.clone())
        .fetch_all(&pool)
        .await
        .map_err(|err| err.to_string())?;

    let groups = rows
        .into_iter()
        .into_group_map_by(|row| (row.fan_series_id.clone(), row.fan_type.clone()));

    let fan_series_group = groups
        .into_iter()
        .find(|((fan_series_id, _), _)| fan_series_id == &id);

    if let Some(((fan_series_id, fan_type), rows)) = fan_series_group {
        let fan_sizes: Vec<FanSize<()>> = rows
            .into_iter()
            .map(|row| FanSize {
                id: row.fan_size_id,
                fan_series_id: row.fan_series_id,
                fan_series: (),
                diameter: row.diameter,
                outlet_area: row.outlet_area,
            })
            .collect();
        let fan_type: FanType = fan_type[..]
            .try_into()
            .map_err(|_| "Could not parase FanType".to_string())?;
        Ok(Json(FanSeries {
            id: fan_series_id,
            fan_type,
            fan_sizes,
        }))
    } else {
        Err("Could not find Fan Series".to_string())
    }
}
