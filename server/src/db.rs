use std::str::FromStr;

use loquat_common::api;
use loquat_common::models::{fan_series::FanSeries, fan_size::FanSize, fan_type::FanType};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

pub struct DbFanSeries(pub FanSeries);

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

pub struct DbFanSize(pub FanSize);

impl FromRow<'_, PgRow> for DbFanSize {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(DbFanSize(FanSize {
            id: row.try_get("id")?,
            fan_series_id: row.try_get("fan_series_id")?,
            diameter: row.try_get("diameter")?,
        }))
    }
}
