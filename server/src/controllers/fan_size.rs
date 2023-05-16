use axum::{extract::Path, Extension, Json};
use sqlx::PgPool;

use loquat_common::{
    api::fan_size::{GetResponse, IndexResponse},
    models::{FanSeries, FanSize},
};

pub async fn index(Extension(pool): Extension<PgPool>) -> Result<Json<IndexResponse>, String> {
    let fan_sizes = sqlx::query!("SELECT * FROM fan_sizes LIMIT 50")
        .fetch_all(&pool)
        .await
        .map_err(|err| err.to_string())
        .map(|records| {
            records
                .into_iter()
                .map(|record| FanSize {
                    id: record.id,
                    fan_series_id: record.fan_series_id,
                    fan_series: (),
                    diameter: record.diameter,
                    outlet_area: record.outlet_area,
                })
                .collect()
        })?;
    Ok(Json(fan_sizes))
}

pub async fn get(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<GetResponse>, String> {
    let fan_size = sqlx::query!(
        "SELECT fan_sizes.id as fan_size_id, fan_series_id, fan_type, diameter, outlet_area
             FROM fan_sizes 
             JOIN fan_serieses ON fan_series_id = fan_serieses.id 
             WHERE fan_sizes.id = $1",
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|err| err.to_string())
    .map(|record| FanSize {
        id: record.fan_size_id,
        fan_series_id: record.fan_series_id.clone(),
        fan_series: FanSeries {
            id: record.fan_series_id,
            fan_type: record.fan_type[..]
                .try_into()
                .map_err(|err| format!("Could not parse fan type: '{:?}'", err))
                .unwrap(),
            fan_sizes: (),
        },
        diameter: record.diameter,
        outlet_area: record.outlet_area,
    })?;

    Ok(Json(fan_size))
}
