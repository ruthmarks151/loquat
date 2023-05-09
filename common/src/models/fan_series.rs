use serde::{Deserialize, Serialize};

use super::fan_type::FanType;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct FanSeries<FanSizesRepr> {
    pub id: String,
    pub fan_type: FanType,
    pub fan_sizes: FanSizesRepr,
}

impl<FanSizesRepr> FanSeries<FanSizesRepr> {
    pub fn flatten(self) -> (FanSeries<()>, FanSizesRepr) {
        let FanSeries {
            id,
            fan_type,
            fan_sizes,
        } = self;
        (
            FanSeries {
                id,
                fan_type,
                fan_sizes: (),
            },
            fan_sizes,
        )
    }
}
