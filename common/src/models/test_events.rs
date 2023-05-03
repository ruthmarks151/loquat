use super::{fan_series::FanSeries, fan_size::FanSize};

pub trait TestEvent<Parameters, Determination> {
    fn standard_id(&self) -> &'static str;

    // fn fan_size(&self) -> &FanSize;

    // fn fan_series(&self) -> &FanSeries;

    // fn determinations(&self) -> Vec<Determination>;

    // fn parameters(&self) -> &Parameters;
}

trait Airflowed {
    fn cfm(&self) -> f64;
}

trait StaticPressured {
    fn static_pressure(&self) -> f64;
}

trait BrakeHorsepowererd {
    fn brake_horsepower(&self) -> f64;
}

pub mod a1_2010;
pub mod a2_2010;
pub mod s1_2010;
