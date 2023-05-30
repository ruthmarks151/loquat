use core::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::models::{FanSeries, FanSize, FanType};

pub type IndexResponse = Vec<FanSeries<()>>;

pub type GetResponse = FanSeries<Vec<FanSize<()>>>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateBody {
    pub id: String,
    pub fan_type: FanType,
}
