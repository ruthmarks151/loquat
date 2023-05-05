use crate::{impl_UnitMath, models::test_events::ScalesWith};
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::fan_diameter::FanDiameter;

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Airflow(f64);
impl_UnitMath!(Airflow);

impl Airflow {
    pub fn new(cfm: f64) -> Airflow {
        Airflow(cfm)
    }

    pub fn from_cfm(cfm: f64) -> Airflow {
        Airflow(cfm)
    }

    fn cfm(&self) -> f64 {
        self.0
    }
}

impl ScalesWith<FanDiameter> for Airflow {
    fn scale(self, &from: &FanDiameter, &to: &FanDiameter) -> Self {
        Self(self.0 * (to / from).powi(3))
    }
}
