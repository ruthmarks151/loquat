use crate::calculations::test_units::{
    fan_curve::FanCurve, fan_curve::InterpolableFanCurve, fan_diameter::FanDiameter,
    inlet_airflow::InletAirflow, static_pressure::StaticPressure,
};
use crate::calculations::{ScalesTo, ScalesWith};

use super::a1_2010::CanFindA1OperatingPoint;
use super::a1_operating_point::A1OperatingPoint;
use super::test_units::brake_horsepower::BrakeHorsepower;
use super::test_units::fan_curve::FanCurveScalesWith;
use super::test_units::fan_speed::FanSpeed;
use super::test_units::outlet_airflow::OutletAirflow;
use super::{a2_operating_point::A2OperatingPoint, Interpolable};

#[derive(Debug, Clone)]
pub struct A1A2OperatingPoint {
    rpm: FanSpeed,
    inlet: InletAirflow,
    outlet: OutletAirflow,
    static_pressure: StaticPressure,
    brake_horsepower: BrakeHorsepower,
}

impl AsRef<StaticPressure> for A1A2OperatingPoint {
    fn as_ref(&self) -> &StaticPressure {
        &self.static_pressure
    }
}

impl AsRef<InletAirflow> for A1A2OperatingPoint {
    fn as_ref(&self) -> &InletAirflow {
        &self.inlet
    }
}

impl AsRef<OutletAirflow> for A1A2OperatingPoint {
    fn as_ref(&self) -> &OutletAirflow {
        &self.outlet
    }
}

impl A1A2OperatingPoint {
    pub fn induced_ratio(&self) -> f64 {
        self.outlet.cfm() / self.inlet.cfm()
    }
}

fn augment(
    a1: A1OperatingPoint,
    a2: &FanCurve<A2OperatingPoint>,
) -> Result<A1A2OperatingPoint, String> {
    let sp: &StaticPressure = a1.as_ref();

    let corresponding_a2 = a2.interpolate(sp)?;

    Ok(A1A2OperatingPoint {
        rpm: (a1.as_ref() as &FanSpeed).clone(),
        inlet: (a1.as_ref() as &InletAirflow).clone(),
        outlet: *corresponding_a2.as_ref(),
        static_pressure: (a1.as_ref() as &StaticPressure).clone(),
        brake_horsepower: (a1.as_ref() as &BrakeHorsepower).clone(),
    })
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

        let a1_a2_points: FanCurve<A1A2OperatingPoint> = self
            .fan_curve_for_value(fan_diameter)
            .into_iter()
            .filter_map(|a1| augment(a1, &a2_points).ok())
            .collect();

        // let a2_point =self.fan_curve_for_value(fan_diameter).scale_to(inlet_airflow).interpolate(inlet_airflow);

        // self.a1_operating_point_for(fan_diameter, inlet_airflow, );

        Ok(a1_a2_points)
    }
}

impl<U> InterpolableFanCurve<U, A2OperatingPoint> for FanCurve<A2OperatingPoint>
where
    U: PartialOrd + Clone,
    A2OperatingPoint: Interpolable<U> + AsRef<U>,
{
}
