use tuple_list::tuple_list;
use tuple_list::tuple_list_type;

use crate::calculations::core::{FanCurve, InterpolableFanCurve, OperatingPoint};
use crate::calculations::traits::{indexing, ScalesTo, ScalesWith};
use crate::calculations::units::{
    BrakeHorsepower, FanDiameter, FanSpeed, InletAirflow, StaticPressure,
};

// #[derive(Debug, Clone, PartialEq)]
pub type A1OperatingPoint =
    OperatingPoint<tuple_list_type!(FanSpeed, InletAirflow, StaticPressure, BrakeHorsepower)>;

impl A1OperatingPoint {
    pub fn new(fs: FanSpeed, ia: InletAirflow, sp: StaticPressure, bhp: BrakeHorsepower) -> Self {
        OperatingPoint(tuple_list!(fs, ia, sp, bhp))
    }
}

impl AsRef<FanSpeed> for A1OperatingPoint {
    fn as_ref(&self) -> &FanSpeed {
        indexing::first(&self.0)
    }
}
impl AsRef<InletAirflow> for A1OperatingPoint {
    fn as_ref(&self) -> &InletAirflow {
        indexing::second(&self.0)
    }
}
impl AsRef<StaticPressure> for A1OperatingPoint {
    fn as_ref(&self) -> &StaticPressure {
        indexing::third(&self.0)
    }
}
impl AsRef<BrakeHorsepower> for A1OperatingPoint {
    fn as_ref(&self) -> &BrakeHorsepower {
        indexing::fourth(&self.0)
    }
}

pub type A1InterpolationPoint = OperatingPoint<tuple_list_type!(FanSpeed, BrakeHorsepower)>;

impl A1InterpolationPoint {
    pub fn new(fs: FanSpeed, bhp: BrakeHorsepower) -> Self {
        OperatingPoint(tuple_list!(fs, bhp))
    }
}

impl From<A1OperatingPoint> for (StaticPressure, A1InterpolationPoint) {
    fn from(OperatingPoint((fs, (_ia, (sp, (bhp, ()))))): A1OperatingPoint) -> Self {
        (sp, A1InterpolationPoint::new(fs, bhp))
    }
}

pub trait CanFindA1OperatingPoint
where
    Self: Clone,
    Self: Into<FanCurve<A1OperatingPoint>>,
    Self: Into<FanDiameter>,
    FanCurve<A1OperatingPoint>: ScalesWith<FanDiameter>,
    FanCurve<A1OperatingPoint>: ScalesTo<InletAirflow>,
    FanCurve<A1OperatingPoint>: InterpolableFanCurve<StaticPressure, A1InterpolationPoint>,
{
    fn a1_operating_point_for(
        self,
        fan_diameter: &FanDiameter,
        inlet_airflow: &InletAirflow,
        static_pressure: &StaticPressure,
    ) -> Result<A1InterpolationPoint, String> {
        let reference_fan_diameter: FanDiameter = self.clone().into();

        let fan_curve: FanCurve<A1OperatingPoint> = self.into();
        fan_curve
            .scale(&reference_fan_diameter, fan_diameter)
            .scale_to(inlet_airflow)
            .interpolate(static_pressure)
    }
}
