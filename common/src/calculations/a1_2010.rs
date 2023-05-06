use crate::calculations::test_units::{
    fan_curve::FanCurve, fan_curve::InterpolableFanCurve, fan_diameter::FanDiameter,
    inlet_airflow::InletAirflow, static_pressure::StaticPressure,
};
use crate::calculations::ScalesTo;

use super::test_units::fan_curve::FanCurveScalesWith;
use super::{a1_operating_point::A1OperatingPoint, Interpolable};

pub trait CanFindA1OperatingPoint
where
    Self: FanCurveScalesWith<FanDiameter, A1OperatingPoint>,
{
    fn a1_operating_point_for(
        &self,
        fan_diameter: &FanDiameter,
        inlet_airflow: &InletAirflow,
        static_pressure: &StaticPressure,
    ) -> Result<A1OperatingPoint, String> {
        self.fan_curve_for_value(fan_diameter)
            .scale_to(inlet_airflow)
            .interpolate(static_pressure)
    }
}

impl<U> InterpolableFanCurve<U, A1OperatingPoint> for FanCurve<A1OperatingPoint>
where
    U: PartialOrd + Clone,
    A1OperatingPoint: Interpolable<U> + AsRef<U>,
{
}
