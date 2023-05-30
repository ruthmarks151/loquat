use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FanType {
    #[serde(rename = "centrifugal")]
    Centrifugal,
    #[serde(rename = "mixed_flow")]
    MixedFlow,
    #[serde(rename = "axial")]
    Axial,
}

impl FanType {
    pub fn all_options() -> Vec<Self> {
        vec![Self::Centrifugal, Self::MixedFlow, Self::Axial]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFanTypeError;

impl TryFrom<&str> for FanType {
    type Error = ParseFanTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "centrifugal" => Ok(Self::Centrifugal),
            "mixed_flow" => Ok(Self::MixedFlow),
            "axial" => Ok(Self::Axial),
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
        }
        .to_string()
    }
}
