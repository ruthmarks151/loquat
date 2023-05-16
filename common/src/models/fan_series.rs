use serde::{Deserialize, Serialize};

use super::fan_type::FanType;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct FanSeries<FanSizesRepr> {
    pub id: String,
    pub fan_type: FanType,
    pub fan_sizes: FanSizesRepr,
}

impl<FanSizesRepr> From<FanSeries<FanSizesRepr>> for (FanSeries<()>, FanSizesRepr) {
    fn from(value: FanSeries<FanSizesRepr>) -> Self {
        let FanSeries {
            id,
            fan_type,
            fan_sizes,
        } = value;
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
// Jemar's first rust, it belongs in a museum
// fn hi(fs: FanSeries<i32>) -> (FanSeries<()>, i32) {
//     return fs.into();
// }

impl<FanSizesRepr> From<(FanSeries<()>, FanSizesRepr)> for FanSeries<FanSizesRepr> {
    fn from(value: (FanSeries<()>, FanSizesRepr)) -> Self {
        let (
            FanSeries {
                id,
                fan_type,
                fan_sizes: _,
            },
            fan_sizes,
        ) = value;
        FanSeries {
            id,
            fan_type,
            fan_sizes,
        }
    }
}
