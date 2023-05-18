use core::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::models::{A1Standard2010Determination, A1Standard2010Report, FanSeries, FanSize};

pub type GetResponse = A1Standard2010Report<FanSize<FanSeries<()>>>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateBody {
    pub id: String,
    pub fan_rpm: f64,
    pub fan_size_id: String,
    pub determinations: Vec<A1Standard2010Determination>,
}

impl Eq for UpdateBody {}

impl Hash for UpdateBody {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.fan_rpm.to_bits().hash(state);
        self.fan_size_id.hash(state);
        self.determinations.hash(state);
    }
}
