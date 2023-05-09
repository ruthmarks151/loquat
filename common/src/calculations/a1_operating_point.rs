use crate::calculations::{
    test_units::{
        brake_horsepower::BrakeHorsepower, fan_speed::FanSpeed, inlet_airflow::InletAirflow,
        static_pressure::StaticPressure,
    },
    ScalesWith,
};

use super::test_units::operating_point::OperatingPoint;

use tuple_list::tuple_list;
use tuple_list::{tuple_list_type, TupleList};

// #[derive(Debug, Clone, PartialEq)]
pub type A1OperatingPoint =
    OperatingPoint<tuple_list_type!(FanSpeed, InletAirflow, StaticPressure, BrakeHorsepower)>;

impl A1OperatingPoint {
    pub fn new(fs: FanSpeed, ia: InletAirflow, sp: StaticPressure, bhp: BrakeHorsepower) -> Self {
        OperatingPoint(tuple_list!(fs, ia, sp, bhp))
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

impl AsRef<FanSpeed> for A1OperatingPoint {
    fn as_ref(&self) -> &FanSpeed {
        &self.0 .0
    }
}
impl AsRef<InletAirflow> for A1OperatingPoint {
    fn as_ref(&self) -> &InletAirflow {
        &self.0 .1 .0
    }
}
impl AsRef<StaticPressure> for A1OperatingPoint {
    fn as_ref(&self) -> &StaticPressure {
        &self.0 .1 .1 .0
    }
}
impl AsRef<BrakeHorsepower> for A1OperatingPoint {
    fn as_ref(&self) -> &BrakeHorsepower {
        &self.0 .1 .1 .1 .0
    }
}

impl<ScaledValue, Head, Tail> ScalesWith<ScaledValue> for (Head, Tail)
where
    Head: ScalesWith<ScaledValue>,
    Tail: ScalesWith<ScaledValue> + TupleList,
{
    fn scale(self, from: &ScaledValue, to: &ScaledValue) -> Self {
        (self.0.scale(from, to), self.1.scale(from, to))
    }
}
