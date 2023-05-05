use crate::{
    calculations::{
        a1_2010::A1Tested,
        a1_operating_point::A1OperatingPoint,
        test_units::{
            brake_horsepower::BrakeHorsepower, fan_curve::FanCurve, fan_diameter::FanDiameter,
            fan_speed::FanSpeed, inlet_airflow::InletAirflow, static_pressure::StaticPressure,
        },
    },
    models::{fan_series::FanSeries, fan_size::FanSize},
};

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

#[derive(Clone, Debug)]
pub struct A1Standard2010Report {
    fan_size: FanSize,
    fan_series: FanSeries,
    parameters: A1Standard2010Parameters,
    determinations: [A1Standard2010Determination; 10],
}

impl A1Tested for A1Standard2010Report {
    fn fan_curve(&self) -> FanCurve<A1OperatingPoint> {
        self.determinations
            .iter()
            .map(|op| {
                A1OperatingPoint::new(
                    FanSpeed::from_rpm(self.parameters.rpm),
                    InletAirflow::from_cfm(op.cfm),
                    StaticPressure::from_inches(op.static_pressure),
                    BrakeHorsepower::from_hp(op.brake_horsepower),
                )
            })
            .collect()
    }

    fn fan_diameter(&self) -> FanDiameter {
        FanDiameter::from_inches(self.fan_size.diameter)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        calculations::{
            a1_operating_point::A1OperatingPoint,
            test_units::{
                brake_horsepower::BrakeHorsepower, fan_diameter::FanDiameter, fan_speed::FanSpeed,
                inlet_airflow::InletAirflow, static_pressure::StaticPressure,
            },
            MeanErrorSquareComparable,
        },
        models::{fan_series::FanSeries, fan_size::FanSize, fan_type::FanType},
    };

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
        let test_event = A1Standard2010Report {
            fan_size: FanSize {
                id: "SKYPLUME G1-ELLV-18 DMF-150".to_string(),
                fan_series_id: fan_series_id.clone(),
                diameter: 18.25,
            },
            fan_series: FanSeries {
                id: fan_series_id.clone(),
                fan_type: FanType::Axial,
            },
            parameters: A1Standard2010Parameters { rpm: 1750.0 },
            determinations: test_points,
        };

        // rpm 1750            cfm 1281.0,   static 1.911,  BHP 0.850),

        let op_res = test_event.operating_point_for(
            &FanDiameter::from_inches(18.25),
            &InletAirflow::from_cfm(1281.0),
            &StaticPressure::from_inches(1.91),
        );

        // let op = find_a1_operating_point(&te, 20.0, 5000.0, 4.0);
        // dbg!(op);
        assert!(op_res.is_ok());
        if let Ok(point) = op_res {
            // Ensure mean squared error is less than .1% ^ 2
            let allowable_percent_error = (0.1_f64 / 100.0).powi(2);
            let percent_square_error = point.error_from(&A1OperatingPoint::new(
                FanSpeed::from_rpm(1750.0),
                InletAirflow::from_cfm(1281.0),
                StaticPressure::from_inches(1.911),
                BrakeHorsepower::from_hp(0.85),
            ));
            println!("Error Amount: {}", percent_square_error);
            assert!(
                percent_square_error < allowable_percent_error
            ); 
        } else {
            assert!(op_res.is_ok());
        }
        
    }
}
