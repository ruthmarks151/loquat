use crate::models::{fan_series::FanSeries, fan_size::FanSize};

use super::{
    test_units::{
        airflow::Airflow, brake_horsepower::BrakeHorsepower, fan_curve::InterpolableFanCurve,
        fan_diameter::FanDiameter, fan_speed::FanSpeed, static_pressure::StaticPressure,
    },
    Interpolable, ScalesTo, ScalesWith,
};
use crate::models::test_events::test_units::fan_curve::FanCurve;

pub mod operating_point;
use operating_point::OperatingPoint;

#[derive(Clone, Debug, PartialEq)]
pub struct A1Standard2010Parameters {
    rpm: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct A1Standard2010Determination {
    cfm: f64,
    static_pressure: f64,
    brake_horsepower: f64,
}

impl InterpolableFanCurve<StaticPressure, OperatingPoint> for FanCurve<OperatingPoint> {}

impl A1Standard2010TestEvent {
    fn fan_curve(&self) -> FanCurve<OperatingPoint> {
        self.determinations
            .iter()
            .map(|op| {
                OperatingPoint::new(
                    FanSpeed::from_rpm(self.parameters.rpm),
                    Airflow::from_cfm(op.cfm),
                    StaticPressure::from_inches(op.static_pressure),
                    BrakeHorsepower::from_hp(op.brake_horsepower),
                )
            })
            .collect()
    }

    fn fan_curve_for_size(&self, new_fan_size: &FanSize) -> FanCurve<OperatingPoint> {
        if new_fan_size.fan_series_id != self.fan_series.id {
            panic!("Fans cannot be substituted with different fan_series_id");
        }

        self.fan_curve().clone().scale(
            &FanDiameter::from_inches(self.fan_size.diameter),
            &FanDiameter::from_inches(new_fan_size.diameter),
        )
    }
}
#[derive(Clone, Debug)]
pub struct A1Standard2010TestEvent {
    fan_size: FanSize,
    fan_series: FanSeries,
    parameters: A1Standard2010Parameters,
    determinations: [A1Standard2010Determination; 10],
}

#[cfg(test)]
mod tests {
    use crate::models::{fan_series::FanSeries, fan_size::FanSize, fan_type::FanType};

    use super::*;

    #[test]
    fn it_calculates() {
        // Test
        let raw_dets = [
            (1823.0, 0.001, 0.723),
            (1637.0, 0.668, 0.785),
            (1459.0, 1.326, 0.831),
            (1281.0, 1.911, 0.850),
            (1100.0, 2.452, 0.845),
            (912.0, 2.452, 0.829),
            (740.0, 3.064, 0.782),
            (548.0, 3.115, 0.715),
            (294.0, 3.152, 0.623),
            (0.0, 3.376, 0.512),
        ];
        // raw_dets.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let test_points: [A1Standard2010Determination; 10] = raw_dets
            .into_iter()
            .map(
                |(cfm, static_pressure, brake_horsepower)| A1Standard2010Determination {
                    cfm,
                    static_pressure,
                    brake_horsepower,
                },
            )
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let fan_series_id = "SKYPLUME G1-ELLV DMF".to_string();
        let test_event = A1Standard2010TestEvent {
            fan_size: FanSize {
                id: "SKYPLUME G1-ELLV-18 DMF-150".to_string(),
                fan_series_id: fan_series_id.clone(),
                diameter: 18.25,
            },
            fan_series: FanSeries {
                id: fan_series_id.clone(),
                fan_type: FanType::InducedFlow,
            },
            parameters: A1Standard2010Parameters { rpm: 1750.0 },
            determinations: test_points,
        };

        // rpm 1750            cfm 1281.0,   static 1.911,  BHP 0.850),

        let op = test_event
            .fan_curve_for_size(&FanSize {
                id: "SKYPLUME G1-ELLV-18 DMF-150".to_string(),
                fan_series_id: fan_series_id,
                diameter: 18.25,
            })
            .scale_to(&Airflow::from_cfm(1281.0))
            .interpolate(&StaticPressure::from_inches(1.911));

        // let op = find_a1_operating_point(&te, 20.0, 5000.0, 4.0);
        // dbg!(op);
        assert_eq!(
            op,
            Ok(OperatingPoint::new(
                FanSpeed::from_rpm(1750.0),
                Airflow::from_cfm(1281.0),
                StaticPressure::from_inches(1.911),
                BrakeHorsepower::from_hp(0.85),
            ),)
        );
    }
}
