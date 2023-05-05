use std::process::Output;

use crate::models::test_events::{
    test_units::{
        airflow::Airflow, brake_horsepower::BrakeHorsepower, fan_speed::FanSpeed,
        static_pressure::StaticPressure,
    },
    Interpolable, ScalesTo, ScalesWith,
};

#[derive(Debug, Clone, PartialEq)]
pub struct OperatingPoint {
    rpm: FanSpeed,
    cfm: Airflow,
    static_pressure: StaticPressure,
    brake_horsepower: BrakeHorsepower,
}

impl OperatingPoint {
    pub fn new(
        rpm: FanSpeed,
        cfm: Airflow,
        static_pressure: StaticPressure,
        brake_horsepower: BrakeHorsepower,
    ) -> Self {
        OperatingPoint {
            rpm,
            cfm,
            static_pressure,
            brake_horsepower,
        }
    }
}

impl AsRef<FanSpeed> for OperatingPoint {
    fn as_ref(&self) -> &FanSpeed {
        &self.rpm
    }
}
impl AsRef<Airflow> for OperatingPoint {
    fn as_ref(&self) -> &Airflow {
        &self.cfm
    }
}
impl AsRef<StaticPressure> for OperatingPoint {
    fn as_ref(&self) -> &StaticPressure {
        &self.static_pressure
    }
}
impl AsRef<BrakeHorsepower> for OperatingPoint {
    fn as_ref(&self) -> &BrakeHorsepower {
        &self.brake_horsepower
    }
}

impl<T> ScalesWith<T> for OperatingPoint
where
    FanSpeed: ScalesWith<T>,
    Airflow: ScalesWith<T>,
    StaticPressure: ScalesWith<T>,
    BrakeHorsepower: ScalesWith<T>,
{
    fn scale(self, from: &T, to: &T) -> Self {
        OperatingPoint::new(
            self.rpm.scale(from, to),
            self.cfm.scale(from, to),
            self.static_pressure.scale(from, to),
            self.brake_horsepower.scale(from, to),
        )
    }
}

impl<T> ScalesTo<T> for OperatingPoint
where
    T: Clone,
    OperatingPoint: AsRef<T> + ScalesWith<T>,
{
    fn scale_to(self, new_measure: &T) -> Self {
        let current_measure: &T = &self.as_ref().clone();
        self.scale(current_measure, new_measure)
    }
}

impl Interpolable<StaticPressure> for OperatingPoint {
    fn interpolate_between(
        (low_static_pressure, low_op): (StaticPressure, OperatingPoint),
        (high_static_pressure, high_op): (StaticPressure, OperatingPoint),
        target_static_pressure: &StaticPressure,
    ) -> OperatingPoint {
        if low_op.cfm != high_op.cfm {
            panic!("Interpolating with non-constant CFMs");
        }

        OperatingPoint::new(
            FanSpeed::interpolate_between(
                (low_static_pressure.clone(), low_op.rpm),
                (high_static_pressure.clone(), high_op.rpm),
                target_static_pressure,
            ),
            low_op.cfm.clone(),
            target_static_pressure.clone(),
            BrakeHorsepower::interpolate_between(
                (low_static_pressure, low_op.brake_horsepower),
                (high_static_pressure, high_op.brake_horsepower),
                target_static_pressure,
            ),
        )
    }
}
