use serde::{Deserialize, Serialize};

use crate::models::{fan_series::FanSeries, fan_size::FanSize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetFanSeriesResponse {
    pub fan_series: FanSeries,
    pub fan_sizes: Vec<FanSize>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]

pub struct GetFanSizeResponse {
    pub fan_series: FanSeries,
    pub fan_size: FanSize,
}
