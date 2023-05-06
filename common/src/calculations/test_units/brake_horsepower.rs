use crate::{
    calculations::{Interpolable, ScalesWith},
    impl_UnitMath,
};
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{
    fan_diameter::FanDiameter, inlet_airflow::InletAirflow, static_pressure::StaticPressure,
};

#[derive(Clone, PartialEq, Debug)]
pub struct BrakeHorsepower(f64);

impl BrakeHorsepower {
    pub fn new(hp: f64) -> Self {
        BrakeHorsepower(hp)
    }

    pub fn from_hp(hp: f64) -> Self {
        BrakeHorsepower(hp)
    }

    pub fn hp(&self) -> f64 {
        self.0
    }
}
impl_UnitMath!(BrakeHorsepower);

impl ScalesWith<FanDiameter> for BrakeHorsepower {
    fn scale(self, &from: &FanDiameter, &to: &FanDiameter) -> Self {
        Self(self.0 * (to / from).powi(5))
    }
}

impl ScalesWith<InletAirflow> for BrakeHorsepower {
    fn scale(self, &from_airflow: &InletAirflow, &to_airflow: &InletAirflow) -> Self {
        Self(self.0 * (to_airflow / from_airflow).powi(3))
    }
}

impl Interpolable<StaticPressure> for BrakeHorsepower {
    fn interpolate_between(
        (low_pressure, low_bhp): (StaticPressure, Self),
        (high_pressure, high_bhp): (StaticPressure, Self),
        required_static: &StaticPressure,
    ) -> Self {
        //linear interpolation for horespower

        if &low_pressure > required_static || &high_pressure < required_static {
            panic!("interpolating out of bounds")
        }

        let interval_fraction =
            (required_static - &low_pressure) / (&high_pressure - &low_pressure);
        &low_bhp + &((&high_bhp - &low_bhp) * interval_fraction)
    }
}
