use axum::{extract::Path, Extension, Json};
use serde::Serialize;
use serde_json::value::Serializer;
use sqlx::PgPool;

use loquat_common::{
    api::a1_2010_report::{GetResponse, UpdateBody},
    models::{
        A1Standard2010Determination, A1Standard2010Parameters, A1Standard2010Report, FanSeries,
        FanSize,
    },
};

pub async fn get(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<GetResponse>, String> {
    let report = sqlx::query!(
      "SELECT a1_2010_report_id, fan_sizes.fan_size_id, fan_sizes.fan_series_id, fan_type, diameter, outlet_area, rpm, determinations
           FROM a1_2010_reports
           JOIN fan_sizes ON a1_2010_reports.fan_size_id = fan_sizes.fan_size_id
           JOIN fan_serieses ON fan_sizes.fan_series_id = fan_serieses.fan_series_id
           WHERE a1_2010_reports.a1_2010_report_id = $1",
      id
  )
  .fetch_one(&pool)
  .await
  .map_err(|err| err.to_string())
  .map(|record|
     A1Standard2010Report {
        id: record.a1_2010_report_id,
        fan_size_id: record.fan_size_id.clone(),
        fan_size: FanSize {
          id: record.fan_size_id,
          fan_series_id: record.fan_series_id.clone(),
          fan_series: FanSeries {
              id: record.fan_series_id,
              fan_type: record.fan_type[..]
                  .try_into()
                  .map_err(|err| format!("Could not parse fan type: '{:?}'", err))
                  .expect("Could not parse fan type from DB"),
              fan_sizes: (),
          },
          diameter: record.diameter,
          outlet_area: record.outlet_area,
      },
        parameters: A1Standard2010Parameters {
          rpm: record.rpm,
        },
        determinations: record.determinations.as_array().expect("Determinations JSONB was not an array!").into_iter().map(|el| { let obj = el.as_object().expect("Element of determinants was not an object!"); A1Standard2010Determination{
            cfm: obj.get("cfm").and_then(|num| num.as_f64()).expect("Object has no CFM"),
            static_pressure: obj.get("static_pressure").and_then(|num| num.as_f64()).expect("Object has no static_pressure"),
            brake_horsepower: obj.get("brake_horsepower").and_then(|num| num.as_f64()).expect("Object has no brake_horsepower"),
        } }).collect()
    })?;

    Ok(Json(report))
}

pub async fn post(
    Extension(pool): Extension<PgPool>,
    Json(UpdateBody{
        id,
        fan_rpm,
        fan_size_id,
        determinations,
    }): Json<UpdateBody>,
) -> Result<Json<GetResponse>, String> {
    let record = sqlx::query!(
        "
    INSERT INTO a1_2010_reports (a1_2010_report_id, fan_size_id,rpm, determinations) VALUES
      ($1,$2,$3,$4) ON CONFLICT DO NOTHING RETURNING a1_2010_report_id;;",
        id,
        fan_size_id,
        fan_rpm,
        determinations
            .serialize(Serializer)
            .map_err(|e| e.to_string())?
    ).fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;
    get(Path(record.a1_2010_report_id), Extension(pool)).await
}

pub async fn put(
    Path(id): Path<String>,
    Extension(pool): Extension<PgPool>,
    Json(UpdateBody{
        id: new_id,
        fan_rpm,
        fan_size_id,
        determinations,
    }): Json<UpdateBody>,
) -> Result<Json<GetResponse>, String> {
    sqlx::query!(
        "
      UPDATE a1_2010_reports SET
        a1_2010_report_id = $1,
        fan_size_id = $2, 
        rpm = $3,
        determinations = $4 
        WHERE a1_2010_report_id = $5",
        new_id,
        fan_size_id,
        fan_rpm,
        determinations
            .serialize(Serializer)
            .map_err(|e| e.to_string())?,
        id,
    )
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;
    get(Path(id), Extension(pool)).await
}
