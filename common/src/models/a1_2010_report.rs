use crate::{
    calculations::{
        core::FanCurve,
        standards::{A1OperatingPoint, CanFindA1OperatingPoint},
        units::{BrakeHorsepower, FanDiameter, FanSpeed, InletAirflow, StaticPressure},
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

impl From<A1Standard2010Report> for FanCurve<A1OperatingPoint> {
    fn from(value: A1Standard2010Report) -> Self {
        value
            .determinations
            .iter()
            .map(|op| {
                A1OperatingPoint::new(
                    FanSpeed::from_rpm(value.parameters.rpm),
                    InletAirflow::from_cfm(op.cfm),
                    StaticPressure::from_inches(op.static_pressure),
                    BrakeHorsepower::from_hp(op.brake_horsepower),
                )
            })
            .collect()
    }
}
impl From<A1Standard2010Report> for FanDiameter {
    fn from(value: A1Standard2010Report) -> Self {
        FanDiameter::from_inches(value.fan_size.diameter)
    }
}

impl CanFindA1OperatingPoint for A1Standard2010Report {}

#[cfg(test)]
mod tests {
    use crate::{
        calculations::{
            standards::{A1InterpolationPoint, CanFindA1OperatingPoint},
            traits::MeanErrorSquareComparable,
            units::{FanDiameter, InletAirflow, StaticPressure},
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
            let actual_point = A1InterpolationPoint::new(
                FanSpeed::from_rpm(1750.0),
                BrakeHorsepower::from_hp(7.481),
            );
            let percent_square_error = point.error_from(&actual_point);
            println!("Error Amount: {}", percent_square_error);
            assert!(percent_square_error < allowable_percent_error);
        } else {
            assert!(op_res.is_ok());
        }
    }
}
