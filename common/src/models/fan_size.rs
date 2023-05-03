use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct FanSize {
    pub id: String,
    pub fan_series_id: String,
    pub diameter: f64,
}

impl Eq for FanSize {}