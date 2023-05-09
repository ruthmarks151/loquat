use crate::models::{FanSeries, FanSize};

#[derive(Clone, Debug)]
pub struct S1Standard2010Parameters {}

#[derive(Clone, Debug)]
pub struct S1Standard2010Determination {
    rpm: f64,
    cfm: f64,
    static_pressure: f64,
    brake_horsepower: f64,
}

#[derive(Clone, Debug)]
pub struct S1Standard2010TestEvent {
    fan_size: FanSize<FanSeries<()>>,
    parameters: S1Standard2010Parameters,
    determinations: [S1Standard2010Determination; 10],
}
