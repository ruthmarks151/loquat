use std::{error::Error, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FanType {
    #[serde(rename = "centrifugal")]
    Centrifugal,
    #[serde(rename = "mixed_flow")]
    MixedFlow,
    #[serde(rename = "axial")]
    Axial,
    #[serde(rename = "induced_flow")]
    InducedFlow,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseFanTypeError;

impl FromStr for FanType {
    type Err = ParseFanTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "centrifugal" => Ok(Self::Centrifugal),
            "mixed_flow" => Ok(Self::MixedFlow),
            "axial"=> Ok(Self::Axial),
            "induced_flow" => Ok(Self::InducedFlow),
            _ => Err(ParseFanTypeError),
        }
    }
}

impl ToString for FanType {
    fn to_string(&self) -> String {
        match self {
            Self::Centrifugal => "centrifugal",
            Self::MixedFlow => "mixed_flow",
            Self::Axial => "axial",
            Self::InducedFlow => "induced_flow",
        }.to_string()
    }
}


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FanSeries {
    pub id: String,
    pub fan_type: FanType,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FanSize {
    pub id: String,
    pub fan_series_id: String,
    pub diameter: f64,
}

pub trait TestEvent<Parameters, Determination> {
    fn standard_id(&self) -> &'static str;

    fn determinations(&self) -> Vec<Determination>;
}

#[derive(Clone, Debug)]
pub struct A2Standard2010Parameters {
    rpm: f64,
}

#[derive(Clone, Debug)]
pub struct A2Standard2010Determination {
    cfm: f64,
    static_pressure: f64,
    brake_horsepower: f64,
}

#[derive(Clone, Debug)]
pub struct A2Standard2010TestEvent {
    fan_size: FanSize,
    fan_series: FanSeries,
    parameters: A2Standard2010Parameters,
    determinations: [A2Standard2010Determination; 10],
}

impl TestEvent<A2Standard2010Parameters, A2Standard2010Determination> for A2Standard2010TestEvent {
    fn standard_id(&self) -> &'static str {
        "A2-2010"
    }

    fn determinations(&self) -> Vec<A2Standard2010Determination> {
        self.determinations.to_vec()
    }
}

#[derive(Clone, Debug)]
pub struct S1Standard2010Parameters {}

#[derive(Clone, Debug)]
pub struct S1Standard2010Determination {
    rpm: f64,
    cfm: f64,
    static_pressure: f64,
    brake_horsepower: f64,
}

#[derive(Clone, Debug)]
pub struct S1Standard2010TestEvent {
    fan_size: FanSize,
    fan_series: FanSeries,
    parameters: S1Standard2010Parameters,
    determinations: [S1Standard2010Determination; 10],
}

impl TestEvent<S1Standard2010Parameters, S1Standard2010Determination> for S1Standard2010TestEvent {
    fn standard_id(&self) -> &'static str {
        "S1-2010"
    }

    fn determinations(&self) -> Vec<S1Standard2010Determination> {
        self.determinations.to_vec()
    }
}
