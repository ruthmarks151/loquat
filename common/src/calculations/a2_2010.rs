use tuple_list::tuple_list;
use tuple_list::tuple_list_type;

use crate::calculations::test_units::{
    fan_curve::FanCurve, fan_diameter::FanDiameter, inlet_airflow::InletAirflow,
    static_pressure::StaticPressure,
};

use super::a1_operating_point::A1OperatingPoint;
use super::a2_operating_point::A2OperatingPoint;
use super::test_units::brake_horsepower::BrakeHorsepower;
use super::test_units::fan_curve::{FanCurveScalesWith, InterpolableFanCurve};
use super::test_units::fan_speed::FanSpeed;
use super::test_units::operating_point::OperatingPoint;
use super::test_units::outlet_airflow::OutletAirflow;

// #[derive(Debug, Clone)]
pub type A1A2OperatingPoint = OperatingPoint<
    tuple_list_type!(
        FanSpeed,
        InletAirflow,
        OutletAirflow,
        StaticPressure,
        BrakeHorsepower,
    ),
>;

impl A1A2OperatingPoint {
    fn new(
        fs: FanSpeed,
        ia: InletAirflow,
        oa: OutletAirflow,
        ap: StaticPressure,
        bhp: BrakeHorsepower,
    ) -> Self {
        OperatingPoint(tuple_list!(fs, ia, oa, ap, bhp))
    }

    pub fn induced_ratio(&self) -> f64 {
        let outlet: &OutletAirflow = self.as_ref();
        let inlet: &InletAirflow = self.as_ref();
        outlet.cfm() / inlet.cfm()
    }
}

impl AsRef<FanSpeed> for A1A2OperatingPoint {
    fn as_ref(&self) -> &FanSpeed {
        &self.0 .0
    }
}
impl AsRef<InletAirflow> for A1A2OperatingPoint {
    fn as_ref(&self) -> &InletAirflow {
        &self.0 .1 .0
    }
}
impl AsRef<OutletAirflow> for A1A2OperatingPoint {
    fn as_ref(&self) -> &OutletAirflow {
        &self.0 .1 .1 .0
    }
}
impl AsRef<StaticPressure> for A1A2OperatingPoint {
    fn as_ref(&self) -> &StaticPressure {
        &self.0 .1 .1 .1 .0
    }
}

impl AsRef<BrakeHorsepower> for A1A2OperatingPoint {
    fn as_ref(&self) -> &BrakeHorsepower {
        &self.0 .1 .1 .1 .1 .0
    }
}

fn augment(
    a1: A1OperatingPoint,
    a2: &FanCurve<A2OperatingPoint>,
) -> Result<A1A2OperatingPoint, String> {
    let sp: &StaticPressure = a1.as_ref();

    let corresponding_a2: OutletAirflow = a2.interpolate(sp)?;

    Ok(A1A2OperatingPoint::new(
        *(a1.as_ref() as &FanSpeed),
        *(a1.as_ref() as &InletAirflow),
        corresponding_a2.clone(),
        *(a1.as_ref() as &StaticPressure),
        *(a1.as_ref() as &BrakeHorsepower),
    ))
}

pub trait CanProduceA1A2Curve
where
    Self: FanCurveScalesWith<FanDiameter, A2OperatingPoint>
        + FanCurveScalesWith<FanDiameter, A1OperatingPoint>,
{
    fn a1_a2_fan_curve(
        &self,
        fan_diameter: &FanDiameter,
    ) -> Result<FanCurve<A1A2OperatingPoint>, String> {
        let a2_points: FanCurve<A2OperatingPoint> = self.fan_curve_for_value(fan_diameter);

        let diameter_adjusted: FanCurve<A1OperatingPoint> =
            self.fan_curve_for_value(fan_diameter);

        let a1_a2_points: FanCurve<A1A2OperatingPoint> = diameter_adjusted
            .into_iter()
            .filter_map(|a1: A1OperatingPoint| augment(a1, &a2_points).ok())
            .collect();

        Ok(a1_a2_points)
    }
}
