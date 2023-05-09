use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct FanSize<FanSeriesRepr>
where
// A Generic for which repesentation of fan series is included
// FanSeriesRepr: Deserialize + Serialize
{
    pub id: String,
    pub fan_series_id: String,
    pub fan_series: FanSeriesRepr,
    pub diameter: f64,
}

impl<FanSeriesRepr: Eq + PartialEq> Eq for FanSize<FanSeriesRepr> {}

impl<FanSeriesRepr> FanSize<FanSeriesRepr> {
    pub fn flatten(self) -> (FanSize<()>, FanSeriesRepr) {
        let FanSize {
            id,
            fan_series_id,
            fan_series,
            diameter,
        } = self;
        (
            FanSize {
                id,
                fan_series_id,
                fan_series: (),
                diameter,
            },
            fan_series,
        )
    }
}
