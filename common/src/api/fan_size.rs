use crate::models::{fan_series::FanSeries, fan_size::FanSize};
use serde::{Deserialize, Serialize};

pub type IndexResponse = Vec<FanSize>;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GetResponse {
    pub fan_series: FanSeries,
    pub fan_size: FanSize,
}
