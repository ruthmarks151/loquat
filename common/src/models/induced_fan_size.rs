



use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct InducedFanSize();


#[serde(rename = "induced_flow")]
struct InducedFlow(String, FanSize, Nozzle);