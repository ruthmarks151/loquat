use crate::calculations::test_units::{
    fan_curve::FanCurve, fan_curve::InterpolableFanCurve, fan_diameter::FanDiameter,
    inlet_airflow::InletAirflow, static_pressure::StaticPressure,
};
use crate::calculations::{ScalesTo, ScalesWith};

use super::{a1_operating_point::A1OperatingPoint, Interpolable};

pub trait A1Tested {
    fn fan_diameter(&self) -> FanDiameter;

    fn fan_curve(&self) -> FanCurve<A1OperatingPoint>;

    fn fan_curve_for_size(&self, new_fan_size: &FanDiameter) -> FanCurve<A1OperatingPoint> {
        self.fan_curve()
            .clone()
            .scale(&self.fan_diameter(), new_fan_size)
    }

    fn operating_point_for(
        &self,
        fan_diameter: &FanDiameter,
        inlet_airflow: &InletAirflow,
        static_pressure: &StaticPressure,
    ) -> Result<A1OperatingPoint, String> {
        self.fan_curve_for_size(fan_diameter)
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
