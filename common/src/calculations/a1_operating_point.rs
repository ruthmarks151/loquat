use crate::calculations::{
    test_units::{
        brake_horsepower::BrakeHorsepower, fan_speed::FanSpeed, inlet_airflow::InletAirflow,
        static_pressure::StaticPressure,
    },
    Interpolable, ScalesTo, ScalesWith,
};

use super::MeanErrorSquareComparable;

#[derive(Debug, Clone, PartialEq)]
pub struct A1OperatingPoint {
    rpm: FanSpeed,
    cfm: InletAirflow,
    static_pressure: StaticPressure,
    brake_horsepower: BrakeHorsepower,
}

trait OperatingPointOn<T1, T2, T3, T4>
where
    Self: Sized,
    T1: From<Self>,
    T2: From<Self>,
    T3: From<Self>,
    T4: From<Self>,
{
}

impl MeanErrorSquareComparable for A1OperatingPoint {
    fn error_from(&self, other: &Self) -> f64 {
        (self.rpm.error_from(&other.rpm)
            + self.cfm.error_from(&other.cfm)
            + self.static_pressure.error_from(&other.static_pressure)
            + self.brake_horsepower.error_from(&other.brake_horsepower))
            / 4.0
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

impl A1OperatingPoint {
    pub fn new(
        rpm: FanSpeed,
        cfm: InletAirflow,
        static_pressure: StaticPressure,
        brake_horsepower: BrakeHorsepower,
    ) -> Self {
        A1OperatingPoint {
            rpm,
            cfm,
            static_pressure,
            brake_horsepower,
        }
    }
}

impl AsRef<FanSpeed> for A1OperatingPoint {
    fn as_ref(&self) -> &FanSpeed {
        &self.rpm
    }
}
impl AsRef<InletAirflow> for A1OperatingPoint {
    fn as_ref(&self) -> &InletAirflow {
        &self.cfm
    }
}
impl AsRef<StaticPressure> for A1OperatingPoint {
    fn as_ref(&self) -> &StaticPressure {
        &self.static_pressure
    }
}
impl AsRef<BrakeHorsepower> for A1OperatingPoint {
    fn as_ref(&self) -> &BrakeHorsepower {
        &self.brake_horsepower
    }
}

impl<T> ScalesWith<T> for A1OperatingPoint
where
    FanSpeed: ScalesWith<T>,
    InletAirflow: ScalesWith<T>,
    StaticPressure: ScalesWith<T>,
    BrakeHorsepower: ScalesWith<T>,
{
    fn scale(self, from: &T, to: &T) -> Self {
        A1OperatingPoint::new(
            self.rpm.scale(from, to),
            self.cfm.scale(from, to),
            self.static_pressure.scale(from, to),
            self.brake_horsepower.scale(from, to),
        )
    }
}

impl<T> ScalesTo<T> for A1OperatingPoint
where
    T: Clone,
    A1OperatingPoint: AsRef<T> + ScalesWith<T>,
{
    fn scale_to(self, new_measure: &T) -> Self {
        let current_measure: &T = &self.as_ref().clone();
        self.scale(current_measure, new_measure)
    }
}

impl Interpolable<StaticPressure> for A1OperatingPoint {
    fn interpolate_between(
        (low_static_pressure, low_op): (StaticPressure, A1OperatingPoint),
        (high_static_pressure, high_op): (StaticPressure, A1OperatingPoint),
        target_static_pressure: &StaticPressure,
    ) -> A1OperatingPoint {
        if low_op.cfm != high_op.cfm {
            panic!("Interpolating with non-constant CFMs");
        }

        if &low_static_pressure > target_static_pressure
            || &high_static_pressure < target_static_pressure
        {
            panic!("interpolating out of bounds")
        }

        A1OperatingPoint::new(
            FanSpeed::interpolate_between(
                (low_static_pressure, low_op.rpm),
                (high_static_pressure, high_op.rpm),
                target_static_pressure,
            ),
            low_op.cfm,
            *target_static_pressure,
            BrakeHorsepower::interpolate_between(
                (low_static_pressure, low_op.brake_horsepower),
                (high_static_pressure, high_op.brake_horsepower),
                target_static_pressure,
            ),
        )
    }
}
