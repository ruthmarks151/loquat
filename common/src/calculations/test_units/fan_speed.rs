use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{
    calculations::{Interpolable, ScalesWith},
    impl_UnitMath,
};

use super::{
    fan_diameter::FanDiameter, inlet_airflow::InletAirflow, static_pressure::StaticPressure,
};

#[derive(Clone, PartialEq, Debug)]
pub struct FanSpeed(f64);
impl_UnitMath!(FanSpeed);

impl FanSpeed {
    pub fn new(rpm: f64) -> FanSpeed {
        FanSpeed(rpm)
    }
    pub fn from_rpm(rpm: f64) -> FanSpeed {
        FanSpeed(rpm)
    }
    pub fn rpm(&self) -> f64 {
        self.0
    }
}

impl ScalesWith<InletAirflow> for FanSpeed {
    fn scale(self, &from_airflow: &InletAirflow, &to_airflow: &InletAirflow) -> Self {
        Self(self.0 * (to_airflow / from_airflow))
    }
}

impl ScalesWith<FanDiameter> for FanSpeed {
    fn scale(self, _: &FanDiameter, _: &FanDiameter) -> Self {
        self
    }
}

impl Interpolable<StaticPressure> for FanSpeed {
    fn interpolate_between(
        (low_static_pressure, low_speed): (StaticPressure, FanSpeed),
        (high_static_pressure, high_speed): (StaticPressure, FanSpeed),
        target: &StaticPressure,
    ) -> FanSpeed {
        let low_rpm = low_speed.rpm();
        let high_rpm = high_speed.rpm();
        //absoulte nonsense interpolation quadratic - just let it be
        let interval = high_rpm - low_rpm;
        let a = ((&low_static_pressure / low_rpm) - (&high_static_pressure / high_rpm)).inches();
        let b = ((&high_static_pressure * low_rpm / high_rpm)
            - (low_static_pressure * high_rpm / low_rpm))
            .inches();
        let c = (target * interval).inches();
        // (-b - Math.sqrt(Math.pow(b,2) - 4 * a * c)) / (2 * a)
        FanSpeed::from_rpm((-b - (b.powi(2) - a * c * 4.0).sqrt()) / (2.0 * a))
    }
}
