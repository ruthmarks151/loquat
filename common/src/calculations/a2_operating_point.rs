use crate::calculations::{
    test_units::{
        brake_horsepower::BrakeHorsepower, fan_speed::FanSpeed, outlet_airflow::OutletAirflow,
        static_pressure::StaticPressure,
    },
    Interpolable, ScalesTo, ScalesWith,
};

use super::{test_units::inlet_airflow::InletAirflow, MeanErrorSquareComparable};

#[derive(Debug, Clone, PartialEq)]
pub struct A2OperatingPoint {
    rpm: FanSpeed,
    cfm: OutletAirflow,
    static_pressure: StaticPressure,
}

impl MeanErrorSquareComparable for A2OperatingPoint {
    fn error_from(&self, other: &Self) -> f64 {
        ((&(&self.rpm - &other.rpm) / &other.rpm).powi(2)
            + (&(&self.cfm - &other.cfm) / &other.cfm).powi(2)
            + (&(&self.static_pressure - &other.static_pressure) / &other.static_pressure)
                .powi(2)
                .powi(2))
            / 3.0
    }
}

// A2 Operating point, can determine inlet airflow from A1 results
// Calculate an induced ratio, varies over region of interest
// Plot an induced ratio at each A2 Operating point
// Outlet airflow at each A1 point, as "augmented A1"
// #[derive(Debug, Clone, PartialEq)]
// pub struct A2OperatingPoint {
//     rpm: FanSpeed,
//     cfm: OutletAirflow,
//     static_pressure: StaticPressure,
//     brake_horsepower: BrakeHorsepower,
// }

impl A2OperatingPoint {
    pub fn new(rpm: FanSpeed, cfm: OutletAirflow, static_pressure: StaticPressure) -> Self {
        A2OperatingPoint {
            rpm,
            cfm,
            static_pressure,
        }
    }
}

impl AsRef<FanSpeed> for A2OperatingPoint {
    fn as_ref(&self) -> &FanSpeed {
        &self.rpm
    }
}
impl AsRef<OutletAirflow> for A2OperatingPoint {
    fn as_ref(&self) -> &OutletAirflow {
        &self.cfm
    }
}
impl AsRef<StaticPressure> for A2OperatingPoint {
    fn as_ref(&self) -> &StaticPressure {
        &self.static_pressure
    }
}

impl<T> ScalesWith<T> for A2OperatingPoint
where
    FanSpeed: ScalesWith<T>,
    OutletAirflow: ScalesWith<T>,
    StaticPressure: ScalesWith<T>,
{
    fn scale(self, from: &T, to: &T) -> Self {
        A2OperatingPoint::new(
            self.rpm.scale(from, to),
            self.cfm.scale(from, to),
            self.static_pressure.scale(from, to),
        )
    }
}

impl<T> ScalesTo<T> for A2OperatingPoint
where
    T: Clone,
    A2OperatingPoint: AsRef<T> + ScalesWith<T>,
{
    fn scale_to(self, new_measure: &T) -> Self {
        let current_measure: &T = &self.as_ref().clone();
        self.scale(current_measure, new_measure)
    }
}

impl Interpolable<StaticPressure> for A2OperatingPoint {
    fn interpolate_between(
        (low_static_pressure, low_op): (StaticPressure, A2OperatingPoint),
        (high_static_pressure, high_op): (StaticPressure, A2OperatingPoint),
        target_static_pressure: &StaticPressure,
    ) -> A2OperatingPoint {
        if low_op.rpm != high_op.rpm {
            panic!("Interpolating with non-constant RPM");
        }

        if &low_static_pressure > target_static_pressure
            || &high_static_pressure < target_static_pressure
        {
            panic!("interpolating out of bounds")
        }

        A2OperatingPoint::new(
            low_op.rpm,
            OutletAirflow::interpolate_between(
                (low_static_pressure.clone(), low_op.cfm),
                (high_static_pressure.clone(), high_op.cfm),
                target_static_pressure,
            ),
            target_static_pressure.clone(),
        )
    }
}
