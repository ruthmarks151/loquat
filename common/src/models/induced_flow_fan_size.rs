use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct InducedFlowFanSize<FanSizeRepr, NozzleRepr> {
    pub id: String,
    pub fan_size_id: String,
    pub fan_size: FanSizeRepr,
    pub nozzle_id: String,
    pub nozzle: NozzleRepr,
}

impl <FanSizeRepr, NozzleRepr>InducedFlowFanSize<FanSizeRepr, NozzleRepr> {
    pub fn flatten(self) -> (InducedFlowFanSize<(), ()>, FanSizeRepr, NozzleRepr) {
        let InducedFlowFanSize{ id, fan_size_id, fan_size, nozzle_id, nozzle } = self;
        (
            InducedFlowFanSize{
                id, fan_size_id, fan_size: (), nozzle_id, nozzle: ()
            },
            fan_size,
            nozzle,
        )
    }
}