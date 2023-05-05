use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{calculations::ScalesWith, impl_UnitMath};

use super::{fan_diameter::FanDiameter, inlet_airflow::InletAirflow};

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub struct StaticPressure(f64);
impl_UnitMath!(StaticPressure);

impl StaticPressure {
    pub fn new(inches: f64) -> Self {
        Self(inches)
    }

    pub fn from_inches(inches: f64) -> Self {
        Self(inches)
    }

    pub fn inches(&self) -> f64 {
        self.0
    }
}

impl ScalesWith<FanDiameter> for StaticPressure {
    fn scale(self, from: &FanDiameter, to: &FanDiameter) -> Self {
        Self(self.0 * to.div(*from).powi(2))
    }
}

impl ScalesWith<InletAirflow> for StaticPressure {
    fn scale(self, from_airflow: &InletAirflow, to_airflow: &InletAirflow) -> Self {
        Self(self.0 * (*to_airflow / *from_airflow).powi(2))
    }
}
