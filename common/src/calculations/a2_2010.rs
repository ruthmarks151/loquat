use serde::{Deserialize, Serialize};

use crate::models::fan_series::FanSeries;
use crate::models::fan_size::FanSize;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct A2Standard2010Parameters {
    rpm: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct A2Standard2010Determination {
    cfm: f64,
    static_pressure: f64,
    brake_horsepower: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct A2Standard2010TestEvent {
    fan_size: FanSize,
    fan_series: FanSeries,
    parameters: A2Standard2010Parameters,
    determinations: [A2Standard2010Determination; 10],
}