use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct InducedFlowFanSize {
    pub id: String,
    pub fan_size_id: String,
    pub nozzle_id: String,
}
