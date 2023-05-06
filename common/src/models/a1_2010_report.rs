use crate::{
    calculations::{
        a1_2010::CanFindA1OperatingPoint,
        a1_operating_point::A1OperatingPoint,
        test_units::{
            brake_horsepower::BrakeHorsepower,
            fan_curve::{FanCurve, FanCurveScalesWith},
            fan_diameter::FanDiameter,
            fan_speed::FanSpeed,
            inlet_airflow::InletAirflow,
            static_pressure::StaticPressure,
        },
    },
    models::{fan_series::FanSeries, fan_size::FanSize},
};

#[derive(Clone, Debug, PartialEq)]
pub struct A1Standard2010Parameters {
    pub rpm: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct A1Standard2010Determination {
    pub cfm: f64,
    pub static_pressure: f64,
    pub brake_horsepower: f64,
}

#[derive(Clone, Debug)]
pub struct A1Standard2010Report {
    pub fan_size: FanSize,
    pub fan_series: FanSeries,
    pub parameters: A1Standard2010Parameters,
    pub determinations: [A1Standard2010Determination; 10],
}

impl FanCurveScalesWith<FanDiameter, A1OperatingPoint> for A1Standard2010Report {
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

    fn current_value(&self) -> FanDiameter {
        FanDiameter::from_inches(self.fan_size.diameter)
    }
}

impl CanFindA1OperatingPoint for A1Standard2010Report {}

#[cfg(test)]
mod tests {
    use crate::{
        calculations::{
            a1_2010::CanFindA1OperatingPoint,
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
        let test_points: [A1Standard2010Determination; 10] = raw_dets
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
        let test_event = A1Standard2010Report {
            fan_size: FanSize {
                id: "SKYPLUME G1-ELLV-18 DMF-150".to_string(),
                fan_series_id: fan_series_id.clone(),
                diameter: 27.0,
            },
            fan_series: FanSeries {
                id: fan_series_id.clone(),
                fan_type: FanType::Axial,
            },
            parameters: A1Standard2010Parameters { rpm: 1750.0 },
            determinations: test_points,
        };


        let op_res = test_event.a1_operating_point_for(
            &FanDiameter::from_inches(27.0),
            &InletAirflow::from_cfm(7749.0),
            &StaticPressure::from_inches(3.789),
        );

        assert!(op_res.is_ok());
        if let Ok(point) = op_res {
            // Ensure mean squared error is less than .1% ^ 2
            let allowable_percent_error = (0.1_f64 / 100.0).powi(2);
            let percent_square_error = point.error_from(&A1OperatingPoint::new(
                FanSpeed::from_rpm(1750.0),
                InletAirflow::from_cfm(7749.0),
                StaticPressure::from_inches(3.789),
                BrakeHorsepower::from_hp(7.481),
            ));
            println!("Error Amount: {}", percent_square_error);
            assert!(percent_square_error < allowable_percent_error);
        } else {
            assert!(op_res.is_ok());
        }
    }
}
