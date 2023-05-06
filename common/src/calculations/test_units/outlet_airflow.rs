use crate::{
    calculations::{Interpolable, ScalesWith},
    impl_UnitMath,
};
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::{fan_diameter::FanDiameter, static_pressure::StaticPressure};

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct OutletAirflow(f64);
impl_UnitMath!(OutletAirflow);

impl OutletAirflow {
    // inlet airflow - A, outlet airlow - F, Induced flow -D
    pub fn new(cfm: f64) -> Self {
        Self(cfm)
    }

    pub fn from_cfm(cfm: f64) -> Self {
        Self(cfm)
    }

    pub fn cfm(&self) -> f64 {
        self.0
    }
}

impl ScalesWith<FanDiameter> for OutletAirflow {
    fn scale(self, &from: &FanDiameter, &to: &FanDiameter) -> Self {
        Self(self.0 * (to / from).powi(3))
    }
}

impl Interpolable<StaticPressure> for OutletAirflow {
    fn interpolate_between(
        (low_pressure, low_oaf): (StaticPressure, Self),
        (high_pressure, high_oaf): (StaticPressure, Self),
        required_pressure: &StaticPressure,
    ) -> Self {
        if &low_pressure > required_pressure || &high_pressure < required_pressure {
            panic!("interpolating out of bounds")
        }
        // TODO, is this linear?
        let interval_fraction =
            (required_pressure - &low_pressure) / (high_pressure - low_pressure);
        low_oaf + (high_oaf - low_oaf) * interval_fraction
    }
}
