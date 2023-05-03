use std::{iter::Zip};

use crate::models::{fan_series::FanSeries, fan_size::FanSize};

use super::{Airflowed, BrakeHorsepowererd, StaticPressured, TestEvent};

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

impl A1Standard2010Determination {
    fn scale_diameter(&self, current: f64, to: f64) -> A1Standard2010Determination {
        let diameter_scale_factor = to / current;

        A1Standard2010Determination {
            cfm: self.cfm * diameter_scale_factor.powi(3),
            static_pressure: self.static_pressure * diameter_scale_factor.powi(2),
            brake_horsepower: self.brake_horsepower * diameter_scale_factor.powi(5),
        }
    }

    fn efficiency(&self) -> f64 {
        self.cfm * self.static_pressure / (6362.0 * self.brake_horsepower)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperatingPoint {
    parameters: A1Standard2010Parameters,
    determination: A1Standard2010Determination,
}

impl OperatingPoint {
    fn scale_cfm(&self, target_cfm: f64) -> OperatingPoint {
        let OperatingPoint {
            parameters,
            determination,
        } = self;
        let airflow_ratio = target_cfm / determination.cfm;
        dbg!(airflow_ratio);
        OperatingPoint {
            parameters: A1Standard2010Parameters {
                rpm: parameters.rpm * airflow_ratio,
            },
            determination: A1Standard2010Determination {
                cfm: target_cfm,
                static_pressure: determination.static_pressure * airflow_ratio.powi(2),
                brake_horsepower: determination.brake_horsepower * airflow_ratio.powi(3),
            },
        }
    }
}

fn interpolate_parameters(
    low: &OperatingPoint,
    high: &OperatingPoint,
    required_static: f64,
) -> A1Standard2010Parameters {
    //absoulte nonsense interpolation quadratic - just let it be
    let interval = high.parameters.rpm - low.parameters.rpm;
    let a = (low.determination.static_pressure / low.parameters.rpm) - (high.determination.static_pressure / high.parameters.rpm);
    let b = (high.determination.static_pressure * low.parameters.rpm / high.parameters.rpm)
        - (low.determination.static_pressure * high.parameters.rpm / low.parameters.rpm);
    let c = required_static * interval;
    A1Standard2010Parameters {
          // (-b - Math.sqrt(Math.pow(b,2) - 4 * a * c)) / (2 * a)
        rpm: (-b - (b.powi(2) - 4.0 * a * c).sqrt() ) / (2.0 * a),
    }
}

fn interpolate_horsepower(low: &OperatingPoint, high: &OperatingPoint, required_static: f64) -> f64 {
    //linear interpolation for horespower
    let interval_multiplier = (required_static - low.determination.static_pressure) / (high.determination.static_pressure - low.determination.static_pressure);
    return low.determination.brake_horsepower
        + (high.determination.brake_horsepower - low.determination.brake_horsepower)
            * interval_multiplier;
}

fn pairwise<T, I>(i: I) -> Zip<I, I>
where
    I: Iterator<Item = T> + Clone,
{
    let mut tail = i.clone();
    tail.next();
    let zipped = i.zip(tail);
    zipped
}

pub fn find_a1_operating_point(
    testEvent: &A1Standard2010TestEvent,
    point_diameter: f64,
    point_cfm: f64,
    point_static_pressure: f64,
) -> Result<OperatingPoint, String> {


    dbg!(testEvent.determinations.clone());
    let scaled_operating_points = testEvent
        .determinations
        .iter()
        .map(|d| d.scale_diameter(testEvent.fan_size.diameter, point_diameter))
        .map(|det| {
            OperatingPoint {
                parameters: testEvent.parameters.clone(),
                determination: det,
            }
            .scale_cfm(point_cfm)
        });

    let bounds = pairwise(scaled_operating_points)
        .find(|(lower, uppper)| uppper.determination.static_pressure >= point_static_pressure);
    //set up variables
    if let Some((store_low, store_high)) = bounds {
        dbg!(store_low.clone());
        dbg!(store_high.clone());

        //find required rotation through interpolation
        let parameters = interpolate_parameters(&store_low, &store_high, point_static_pressure);
        //find required horespower through interpolation
        let required_horsepower = interpolate_horsepower(&store_low, &store_high, point_static_pressure);
        //Calculate static efficiency
        //return operating point
        Ok(OperatingPoint {
            parameters,
            determination: A1Standard2010Determination {
                cfm: point_cfm,
                static_pressure: point_static_pressure,
                brake_horsepower: required_horsepower,
            },
        })
    } else {
        Err("Out of bounds".to_string())
    }
}

impl Airflowed for A1Standard2010Determination {
    fn cfm(&self) -> f64 {
        self.cfm
    }
}

impl StaticPressured for A1Standard2010Determination {
    fn static_pressure(&self) -> f64 {
        self.static_pressure
    }
}

impl BrakeHorsepowererd for A1Standard2010Determination {
    fn brake_horsepower(&self) -> f64 {
        self.brake_horsepower
    }
}

#[derive(Clone, Debug)]
pub struct A1Standard2010TestEvent {
    fan_size: FanSize,
    fan_series: FanSeries,
    parameters: A1Standard2010Parameters,
    determinations: [A1Standard2010Determination; 10],
}

impl TestEvent<A1Standard2010Parameters, A1Standard2010Determination> for A1Standard2010TestEvent {
    fn standard_id(&self) -> &'static str {
        "A1-2010"
    }

    // fn fan_size(&self) -> &FanSize {
    //     &self.fan_size
    // }

    // fn fan_series(&self) -> &FanSeries {
    //     &self.fan_series
    // }

    // fn determinations(&self) -> Vec<A1Standard2010Determination> {
    //     self.determinations.to_vec()
    // }

    // fn parameters(&self) -> &A1Standard2010Parameters {
    //     &self.parameters
    // }
}

#[cfg(test)]
mod tests {
    use crate::models::{fan_series::FanSeries, fan_size::FanSize, fan_type::FanType};

    use super::*;

    #[test]
    fn it_calculates() {
        // Test
        let mut raw_dets = [
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
        let te = A1Standard2010TestEvent {
            fan_size: FanSize {
                id: "SKYPLUME G1-ELLV-18 DMF-150".to_string(),
                fan_series_id: fan_series_id.clone(),
                diameter: 18.25,
            },
            fan_series: FanSeries {
                id: fan_series_id,
                fan_type: FanType::InducedFlow,
            },
            parameters: A1Standard2010Parameters { rpm: 1750.0 },
            determinations: test_points,
        };

          // rpm 1750            cfm 1281.0,   static 1.911,  BHP 0.850),

        let op = find_a1_operating_point(&te, 18.25, 1281.0, 1.911);
        // let op = find_a1_operating_point(&te, 20.0, 5000.0, 4.0);
        // dbg!(op);
        assert_eq!(op, 
            Ok(
              OperatingPoint {
                  parameters: A1Standard2010Parameters {
                      rpm: 1750.0,
                  },
                  determination: A1Standard2010Determination {
                      cfm: 1281.0,
                      static_pressure: 1.911,
                      brake_horsepower: 0.85,
                  },
              },
          )
        );
    }
}
