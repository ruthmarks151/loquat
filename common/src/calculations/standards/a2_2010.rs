use tuple_list::tuple_list;
use tuple_list::tuple_list_type;

use super::a1_2010::A1OperatingPoint;
use crate::calculations::core::{FanCurve, InterpolableFanCurve, OperatingPoint};
use crate::calculations::traits::{indexing, ScalesWith};
use crate::calculations::units::{
    BrakeHorsepower, FanDiameter, FanSpeed, InletAirflow, OutletAirflow, StaticPressure,
};

// A2 Operating point, can determine inlet airflow from A1 results
// Calculate an induced ratio, varies over region of interest
// Plot an induced ratio at each A2 Operating point
// Outlet airflow at each A1 point, as "augmented A1"

pub type A2OperatingPoint =
    OperatingPoint<tuple_list_type!(FanSpeed, OutletAirflow, StaticPressure)>;

impl A2OperatingPoint {
    pub fn new(fs: FanSpeed, oa: OutletAirflow, sp: StaticPressure) -> A2OperatingPoint {
        OperatingPoint(tuple_list!(fs, oa, sp))
    }
}

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

impl From<A2OperatingPoint> for (StaticPressure, OutletAirflow) {
    fn from(OperatingPoint((_fs, (oa, (sp, ())))): A2OperatingPoint) -> Self {
        (sp, oa)
    }
}

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

impl AsRef<InletAirflow> for A1A2OperatingPoint {
    fn as_ref(&self) -> &InletAirflow {
        indexing::second(&self.0)
    }
}
impl AsRef<OutletAirflow> for A1A2OperatingPoint {
    fn as_ref(&self) -> &OutletAirflow {
        indexing::third(&self.0)
    }
}

fn augment_with_outlet_airflow(
    a1: A1OperatingPoint,
    a2: &FanCurve<A2OperatingPoint>,
) -> Result<A1A2OperatingPoint, String> {
    let OperatingPoint((fs, (ia, (sp, (bhp, ()))))) = a1;

    let corresponding_a2: OutletAirflow = a2.interpolate(&sp)?;

    Ok(A1A2OperatingPoint::new(
        fs,
        ia,
        corresponding_a2.clone(),
        sp,
        bhp,
    ))
}

pub trait CanProduceA1A2Curve
where
    Self: Clone,
    Self: Into<FanDiameter>,
    FanCurve<A1OperatingPoint>: From<Self> + ScalesWith<FanDiameter>,
    FanCurve<A2OperatingPoint>: From<Self> + ScalesWith<FanDiameter>,
{
    fn a1_a2_fan_curve(
        self,
        fan_diameter: &FanDiameter,
    ) -> Result<FanCurve<A1A2OperatingPoint>, String> {
        let a1_points: FanCurve<A1OperatingPoint> = FanCurve::from(self.clone());
        let a2_points: FanCurve<A2OperatingPoint> = FanCurve::from(self.clone());

        let reference_fan_diameter: FanDiameter = self.into();
        let scaled_a1: FanCurve<A1OperatingPoint> =
            a1_points.scale(&reference_fan_diameter, fan_diameter);
        let scaled_a2 = a2_points.scale(&reference_fan_diameter, fan_diameter);

        let a1_a2_points: FanCurve<A1A2OperatingPoint> = scaled_a1
            .into_iter()
            .filter_map(|a1: A1OperatingPoint| augment_with_outlet_airflow(a1, &scaled_a2).ok())
            .collect();

        Ok(a1_a2_points)
    }
}
