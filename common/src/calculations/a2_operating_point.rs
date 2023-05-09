use tuple_list::tuple_list;
use tuple_list::tuple_list_type;

use crate::calculations::test_units::{
    fan_speed::FanSpeed, outlet_airflow::OutletAirflow, static_pressure::StaticPressure,
};

use super::test_units::operating_point::OperatingPoint;

pub type A2OperatingPoint =
    OperatingPoint<tuple_list_type!(FanSpeed, OutletAirflow, StaticPressure)>;

impl A2OperatingPoint {
    pub fn new(fs: FanSpeed, oa: OutletAirflow, sp: StaticPressure) -> A2OperatingPoint {
        OperatingPoint(tuple_list!(fs, oa, sp))
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

impl AsRef<FanSpeed> for A2OperatingPoint {
    fn as_ref(&self) -> &FanSpeed {
        &self.0 .0
    }
}
impl AsRef<OutletAirflow> for A2OperatingPoint {
    fn as_ref(&self) -> &OutletAirflow {
        &self.0 .1 .0
    }
}
impl AsRef<StaticPressure> for A2OperatingPoint {
    fn as_ref(&self) -> &StaticPressure {
        &self.0 .1 .1 .0
    }
}

impl From<A2OperatingPoint> for OutletAirflow {
    fn from(value: A2OperatingPoint) -> Self {
        value.0 .1 .0
    }
}
