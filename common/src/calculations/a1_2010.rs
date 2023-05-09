use tuple_list::tuple_list;
use tuple_list::tuple_list_type;

use crate::calculations::test_units::{
    fan_curve::FanCurve, fan_curve::InterpolableFanCurve, fan_diameter::FanDiameter,
    inlet_airflow::InletAirflow, static_pressure::StaticPressure,
};
use crate::calculations::ScalesTo;

use super::a1_operating_point::A1OperatingPoint;
use super::test_units::brake_horsepower::BrakeHorsepower;
use super::test_units::fan_curve::FanCurveScalesWith;
use super::test_units::fan_speed::FanSpeed;
use super::test_units::operating_point::OperatingPoint;

pub type A1InterpolationPoint = OperatingPoint<tuple_list_type!(FanSpeed, BrakeHorsepower)>;

impl A1InterpolationPoint {
    pub fn new(fs: FanSpeed, bhp: BrakeHorsepower) -> Self {
        OperatingPoint(tuple_list!(fs, bhp))
    }
}

impl From<A1OperatingPoint> for A1InterpolationPoint {
    fn from(value: A1OperatingPoint) -> A1InterpolationPoint {
        let fs: &FanSpeed = value.as_ref();
        let bhp: &BrakeHorsepower = value.as_ref();
        A1InterpolationPoint::new(fs.clone(), bhp.clone())
    }
}

pub trait CanFindA1OperatingPoint
where
    FanCurve<A1OperatingPoint>: ScalesTo<InletAirflow>, //+ InterpolableFanCurve<StaticPressure, A1Interpolation, A1OperatingPointTuple>,
    Self: FanCurveScalesWith<FanDiameter, A1OperatingPoint>,
{
    fn a1_operating_point_for(
        &self,
        fan_diameter: &FanDiameter,
        inlet_airflow: &InletAirflow,
        static_pressure: &StaticPressure,
    ) -> Result<A1InterpolationPoint, String> {
        let a = self.fan_curve_for_value(fan_diameter);

        let scaled: FanCurve<A1OperatingPoint> = a.scale_to(inlet_airflow);
        scaled.interpolate(static_pressure)
    }
}

// impl<X> InterpolableFanCurve<X, OutletAirflow, A1OperatingPointTuple> for FanCurve<A1OperatingPointTuple>
// where
//     X: PartialOrd + Clone,
//     A1OperatingPointTuple: Interpolable<X, OutletAirflow> + AsRef<X>,
// {
// }
