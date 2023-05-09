// Implimentations for
// use std::ops::{Add, Div, Mul, Neg, Sub};
// use crate::calculations::MeanErrorSquareComparable;

#[macro_export]
macro_rules! impl_UnitMath {
    ($T:ident) => {
        impl Neg for &$T {
            type Output = $T;

            fn neg(self) -> Self::Output {
                $T::new(-self.0)
            }
        }

        impl Neg for $T {
            type Output = $T;

            fn neg(self) -> Self::Output {
                $T::new(-self.0)
            }
        }

        impl Add for &$T {
            type Output = $T;

            fn add(self, rhs: Self) -> Self::Output {
                $T::new(self.0 + rhs.0)
            }
        }

        impl Add for $T {
            type Output = $T;

            fn add(self, rhs: Self) -> Self::Output {
                $T::new(self.0 + rhs.0)
            }
        }

        impl Sub for &$T {
            type Output = $T;

            fn sub(self, rhs: Self) -> Self::Output {
                $T::new(self.0 - rhs.0)
            }
        }

        impl Sub for $T {
            type Output = $T;

            fn sub(self, rhs: Self) -> Self::Output {
                $T::new(self.0 - rhs.0)
            }
        }

        impl Mul<f64> for &$T {
            type Output = $T;

            fn mul(self, rhs: f64) -> Self::Output {
                $T::new(self.0 * rhs)
            }
        }

        impl Mul<f64> for $T {
            type Output = $T;

            fn mul(self, rhs: f64) -> Self::Output {
                $T::new(self.0 * rhs)
            }
        }

        impl Div<f64> for &$T {
            type Output = $T;

            fn div(self, rhs: f64) -> Self::Output {
                $T::new(self.0 / rhs)
            }
        }

        impl Div<f64> for $T {
            type Output = $T;

            fn div(self, rhs: f64) -> Self::Output {
                $T::new(self.0 / rhs)
            }
        }

        impl Div for &$T {
            type Output = f64;

            fn div(self, rhs: Self) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl Div for $T {
            type Output = f64;

            fn div(self, rhs: Self) -> Self::Output {
                self.0 / rhs.0
            }
        }

        impl MeanErrorSquareComparable for $T {
            fn error_from(&self, other: &Self) -> f64 {
                (&(self - other) / other).powi(2)
            }

            fn error_sum(&self, other: &Self) -> f64 {
                (&(self - other) / other).powi(2)
            }
        }
    };
}
