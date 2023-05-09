use crate::{
    calculations::{
        a1_operating_point::A1OperatingPoint,
        a2_2010::CanProduceA1A2Curve,
        a2_operating_point::A2OperatingPoint,
        test_units::{
            fan_curve::{FanCurve, FanCurveScalesWith},
            fan_diameter::FanDiameter,
            fan_speed::FanSpeed,
            outlet_airflow::OutletAirflow,
            static_pressure::StaticPressure,
        },
    },
    models::{fan_series::FanSeries, fan_size::FanSize},
};

use super::{
    a1_2010_report::A1Standard2010Report, induced_flow_fan_size::InducedFlowFanSize, nozzle::Nozzle,
};

#[derive(Clone, Debug, PartialEq)]
pub struct A2Standard2010Parameters {
    rpm: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct A2Standard2010Determination {
    cfm: f64,
    static_pressure: f64,
}

#[derive(Clone, Debug)]
pub struct A2Standard2010Report {
    a1_report: A1Standard2010Report,
    induced_flow_fan_size: InducedFlowFanSize,
    nozzle: Nozzle,
    fan_size: FanSize,
    fan_series: FanSeries,
    parameters: A2Standard2010Parameters,
    determinations: [A2Standard2010Determination; 10],
}

impl FanCurveScalesWith<FanDiameter, A2OperatingPoint> for A2Standard2010Report {
    fn fan_curve(&self) -> FanCurve<A2OperatingPoint> {
        self.determinations
            .iter()
            .map(|op| {
                A2OperatingPoint::new(
                    FanSpeed::from_rpm(self.parameters.rpm),
                    OutletAirflow::from_cfm(op.cfm),
                    StaticPressure::from_inches(op.static_pressure),
                )
            })
            .collect()
    }

    fn current_value(&self) -> FanDiameter {
        FanDiameter::from_inches(self.fan_size.diameter)
    }
}

impl FanCurveScalesWith<FanDiameter, A1OperatingPoint> for A2Standard2010Report {
    fn current_value(&self) -> FanDiameter {
        FanDiameter::from_inches(self.fan_size.diameter)
    }

    fn fan_curve(&self) -> FanCurve<A1OperatingPoint> {
        self.a1_report.fan_curve()
    }
}

impl CanProduceA1A2Curve for A2Standard2010Report {}

#[cfg(test)]
mod tests {

    use crate::{
        calculations::{
            test_units::{
                fan_curve::InterpolableFanCurve, fan_diameter::FanDiameter,
                static_pressure::StaticPressure,
            },
            Interpolable,
        },
        models::{
            a1_2010_report::{A1Standard2010Determination, A1Standard2010Parameters},
            fan_series::FanSeries,
            fan_size::FanSize,
            fan_type::FanType,
        },
    };

    use super::*;

    #[test]
    fn it_calculates() {
        // Test

        let raw_a1_dets = [
            // SP   CFM      BHP
            (0.001, 11077.0, 6.320),
            (1.184, 9981.0, 6.632),
            (2.593, 8884.0, 7.243),
            (3.789, 7749.0, 7.481),
            (4.608, 6659.0, 7.416),
            (5.158, 5524.0, 7.079),
            (5.532, 4436.0, 6.606),
            (5.795, 3311.0, 6.171),
            (6.054, 1549.0, 6.419),
            (6.839, 0.0, 7.204),
        ];
        // raw_dets.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let a1_determinations: [A1Standard2010Determination; 10] = raw_a1_dets
            .into_iter()
            .map(
                |(static_pressure, cfm, brake_horsepower)| A1Standard2010Determination {
                    cfm,
                    static_pressure,
                    brake_horsepower,
                },
            )
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let fan_series_id = "SKYPLUME G1-ELLV DMF".to_string();

        let fan_series = FanSeries {
            id: fan_series_id.clone(),
            fan_type: FanType::Axial,
        };

        let fan_size = FanSize {
            id: "SKYPLUME G1-ELLV-18 DMF-150".to_string(),
            fan_series_id: fan_series_id.clone(),
            diameter: 27.0,
        };

        let a1_test_event = A1Standard2010Report {
            fan_size: fan_size.clone(),
            fan_series: fan_series.clone(),
            parameters: A1Standard2010Parameters { rpm: 1750.0 },
            determinations: a1_determinations,
        };

        let raw_a2_dets = [
            // SP   CFM
            (0.040, 1530.0),
            (1.227, 13816.0),
            (2.537, 12386.0),
            (3.738, 10815.0),
            (4.543, 9316.0),
            (5.078, 7736.0),
            (5.448, 6419.0),
            (5.787, 5035.0),
            (5.966, 2350.0),
            (6.646, 0.0),
        ];
        // raw_dets.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let a2_determinations: [A2Standard2010Determination; 10] = raw_a2_dets
            .into_iter()
            .map(|(static_pressure, cfm)| A2Standard2010Determination {
                cfm,
                static_pressure,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let a2_test_event = A2Standard2010Report {
            a1_report: a1_test_event,
            induced_flow_fan_size: InducedFlowFanSize {
                id: "ID".to_string(),
                fan_size_id: fan_size.id.clone(),
                nozzle_id: "ID".to_string(),
            },
            nozzle: Nozzle {
                id: "ID".to_string(),
            },
            fan_size: fan_size.clone(),
            fan_series: fan_series.clone(),
            parameters: A2Standard2010Parameters { rpm: 1750.0 },
            determinations: a2_determinations,
        };

        let a2_curve: FanCurve<A2OperatingPoint> = a2_test_event.fan_curve();
        let interpolated_outlet: Result<OutletAirflow, String> =
            a2_curve.interpolate(&StaticPressure::from_inches(2.593));

        assert_eq!(
            interpolated_outlet.ok(),
            Some(OutletAirflow::from_cfm(12312.747710241465))
        );

        let pairwise_interpolated_outlet = OutletAirflow::interpolate_between(
            (
                StaticPressure::from_inches(2.537),
                OutletAirflow::from_cfm(12386.0),
            ),
            (
                StaticPressure::from_inches(3.738),
                OutletAirflow::from_cfm(10815.0),
            ),
            &StaticPressure::from_inches(2.593),
        );
        assert_eq!(
            pairwise_interpolated_outlet,
            OutletAirflow::from_cfm(12312.747710241465)
        );

        // rpm 1750            cfm 1281.0,   static 1.911,  BHP 0.850),

        let op_res = a2_test_event.a1_a2_fan_curve(&FanDiameter::from_inches(27.0));

        // let op = find_a1_operating_point(&te, 20.0, 5000.0, 4.0);
        // dbg!(op);
        assert!(op_res.is_ok());
        if let Ok(curve) = op_res {
            // println!(
            //     "Curve: {:#?}",
            //     curve
            //         .into_iter()
            //         .map(|e| format!(
            //             "{:?} {:?} {:?} {}%",
            //             (e.as_ref() as &StaticPressure),
            //             (e.as_ref() as &InletAirflow),
            //             (e.as_ref() as &OutletAirflow),
            //             e.induced_ratio() * 100.0
            //         ))
            //         .collect::<Vec<_>>()
            // );
            assert_eq!(
                curve
                    .into_iter()
                    .map(|e| e.induced_ratio())
                    .collect::<Vec<_>>(),
                vec![
                    1.3396383203908824,
                    1.3859463879155183,
                    1.3834084782728928,
                    1.3701813159900245,
                    1.3488854531577197,
                    1.3697163992115997,
                    1.484445786771368,
                    1.3207762123571183
                ]
            );
        } else {
            assert!(op_res.is_ok());
        }
    }
}
