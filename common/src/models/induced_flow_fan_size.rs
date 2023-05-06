use serde::{Deserialize, Serialize};

use super::{fan_size::FanSize, nozzle::Nozzle};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct InducedFlowFanSize {
    pub id: String,
    pub fan_size_id: String,
    pub nozzle_id: String,
}
