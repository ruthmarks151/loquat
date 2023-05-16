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

impl<FanSeriesRepr> From<FanSize<FanSeriesRepr>> for (FanSize<()>, FanSeriesRepr) {
    fn from(value: FanSize<FanSeriesRepr>) -> Self {
        let FanSize {
            id,
            fan_series_id,
            fan_series,
            diameter,
        } = value;
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

impl<FanSeriesRepr> From<(FanSize<()>, FanSeriesRepr)> for FanSize<FanSeriesRepr> {
    fn from(value: (FanSize<()>, FanSeriesRepr)) -> Self {
        let (
            FanSize {
                id,
                fan_series_id,
                fan_series: _,
                diameter,
            },
            fan_series,
        ) = value;
        FanSize {
            id,
            fan_series_id,
            fan_series,
            diameter,
        }
    }
}
