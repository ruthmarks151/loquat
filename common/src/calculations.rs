pub mod a1_2010;
pub mod a1_operating_point;
pub mod a2_2010;
pub mod s1_2010;
pub mod test_units;

pub trait ScalesWith<Context> {
    fn scale(self, from: &Context, to: &Context) -> Self;
}

impl<T: PartialEq + Clone> ScalesWith<T> for T {
    fn scale(self, from: &T, to: &T) -> Self {
        if self != *from {
            panic!("Tried to scale an value to an one of its own type, but not from itself")
        }
        (*to).clone()
    }
}

pub trait ScalesTo<Context> {
    fn scale_to(self, to: &Context) -> Self;
}
pub trait Interpolable<X>
where
    X: Clone,
    Self: Clone,
{
    fn interpolate_between(low: (X, Self), high: (X, Self), target: &X) -> Self;
}

pub trait MeanErrorSquareComparable {
    fn error_from(&self, other: &Self) -> f64;
}
