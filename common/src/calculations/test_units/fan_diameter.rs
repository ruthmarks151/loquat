use std::ops::Div;

#[derive(Clone, PartialEq, Debug, Copy, PartialOrd)]
pub struct FanDiameter(f64);

impl FanDiameter {
    pub fn from_inches(inches: f64) -> Self {
        FanDiameter(inches)
    }

    pub fn inches(&self) -> f64 {
        self.0
    }
}

impl Div for FanDiameter {
    type Output = f64;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}
