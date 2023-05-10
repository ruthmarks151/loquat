use std::str::FromStr;

use loquat_common::models::{FanSeries, FanSize, FanType};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

pub struct Db<T>(pub T);

impl FromRow<'_, PgRow> for Db<FanSeries<()>> {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Db(FanSeries {
            id: row.try_get("fan_series_id")?,
            fan_type: FanType::from_str(row.try_get("fan_type")?).map_err(|_| {
                sqlx::Error::TypeNotFound {
                    type_name: "fan_type".to_owned(),
                }
            })?,
            fan_sizes: (),
        }))
    }
}

// impl FromRow<'_, PgRow> for Db<FanSeries<Vec<FanSize<()>>>> {
//     fn from_row(row: &PgRow) -> sqlx::Result<Self> {
//         todo!();
//         // Ok(Db(FanSeries {
//         //     id: row.try_get("fan_series_id")?,
//         //     fan_type: FanType::from_str(row.try_get("fan_type")?).map_err(|_| {
//         //         sqlx::Error::TypeNotFound {
//         //             type_name: "fan_type".to_owned(),
//         //         }
//         //     })?,
//         //     fan_sizes: FanSize {
//         //         id: row.try_get("fan_size_id")?,
//         //         fan_series_id: row.try_get("fan_series_id")?,
//         //         fan_series: (),
//         //         diameter: row.try_get("diameter")?,
//         //     },
//         // }))
//     }
// }

impl FromRow<'_, PgRow> for Db<FanSize<()>> {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Db(FanSize {
            id: row.try_get("fan_size_id")?,
            fan_series_id: row.try_get("fan_series_id")?,
            fan_series: (),
            diameter: row.try_get("diameter")?,
        }))
    }
}

impl FromRow<'_, PgRow> for Db<FanSize<FanSeries<()>>> {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        println!("{:#?}", row.columns());
        Ok(Db(FanSize {
            id: row.try_get("fan_size_id")?,
            fan_series_id: row.try_get("fan_series_id")?,
            fan_series: FanSeries {
                id: row.try_get("fan_series_id")?,
                fan_type: FanType::from_str(row.try_get("fan_type")?).map_err(|_| {
                    sqlx::Error::TypeNotFound {
                        type_name: "fan_type".to_owned(),
                    }
                })?,
                fan_sizes: (),
            },
            diameter: row.try_get("diameter")?,
        }))
    }
}
