
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
    fan_size: FanSize,
    fan_series: FanSeries,
    parameters: S1Standard2010Parameters,
    determinations: [S1Standard2010Determination; 10],
}

impl TestEvent<S1Standard2010Parameters, S1Standard2010Determination> for S1Standard2010TestEvent {
    fn standard_id(&self) -> &'static str {
        "S1-2010"
    }

    fn determinations(&self) -> Vec<S1Standard2010Determination> {
        self.determinations.to_vec()
    }
}
