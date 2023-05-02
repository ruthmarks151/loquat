use std::str::FromStr;

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
            "axial" => Ok(Self::Axial),
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
        }
        .to_string()
    }
}
