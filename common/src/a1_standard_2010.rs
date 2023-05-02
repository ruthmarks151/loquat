#[derive(Clone, Debug)]
pub struct A1Standard2010Parameters{
    rpm: f64,
}

#[derive(Clone, Debug)]
pub struct A1Standard2010Determination {
    cfm: f64,
    static_pressure: f64,
    brake_horsepower: f64 
}

#[derive(Clone, Debug)]
pub struct A1Standard2010TestEvent {
    fan_size: FanSize,
    fan_series: FanSeries,
    parameters: A1Standard2010Parameters,
    determinations: [A1Standard2010Determination; 10]
}

impl TestEvent<A1Standard2010Parameters, A1Standard2010Determination> for A1Standard2010TestEvent {
    fn standard_id(&self) -> &'static str {
        "A1-2010"
    }

    fn determinations(&self) -> Vec<A1Standard2010Determination> {
        self.determinations.to_vec()
    }
}

fn test () {
  // Test
  let testPoints: Vec<A1Standard2010Determination> = [
    [1823, 0.001, 0.723],
    [1637, 0.668, 0.785],
    [1459, 1.326, 0.831],
    [1281, 1.911, 0.850],
    [1100, 2.452, 0.845],
    [912, 2.452, 0.829],
    [740, 3.064, 0.782],
    [548, 3.115, 0.715],
    [294, 3.152, 0.623],
    [0, 3.376, 0.512],
  ].iter().map(|(cfm, staticPressure, brakeHorsepower)| A1Standard2010Determination {cfm, staticPressure, brakeHorsepower}).collect();

  A1Standard2010TestEvent {

    fan_size: FanSize {
      id: "SKYPLUME G1-ELLV-18 DMF-150",
      diameter: 18.25,
    },
    fan_series: FanSeries{
      id: "SKYPLUME G1-ELLV DMF",
    },
    parameters: A1Standard2010Parameters {
      rpm: 1750
    },
    determinations: testPoints
  }
}