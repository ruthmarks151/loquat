use tuple_list::TupleList;

pub mod a1_2010;
pub mod a1_operating_point;
pub mod a2_2010;
pub mod a2_operating_point;
pub mod s1_2010;
pub mod test_units;
pub trait ScalesWith<Context> {
    fn scale(self, from: &Context, to: &Context) -> Self;
}

impl<T> ScalesWith<T> for () {
    fn scale(self, _from: &T, _to: &T) -> Self {
        ()
    }
}

pub trait ScalesTo<Context> {
    fn scale_to(self, to: &Context) -> Self;
}
pub trait Interpolable<X, Y>
where
    X: Clone,
    Self: Clone,
{
    fn interpolate_between(low: (X, Y), high: (X, Y), target: &X) -> Y;
}

impl<X> Interpolable<X, ()> for ()
where
    X: Clone,
{
    fn interpolate_between(_low: (X, ()), _high: (X, ()), _target: &X) -> () {
        ()
    }
}

impl<X, Head, Tail> Interpolable<X, (Head, Tail)> for (Head, Tail)
where
    X: Clone,
    Head: Interpolable<X, Head>,
    Tail: Interpolable<X, Tail> + TupleList,
{
    fn interpolate_between(
        (low_static_pressure, (low_head, low_tail)): (X, (Head, Tail)),
        (high_static_pressure, (high_head, high_tail)): (X, (Head, Tail)),
        target_static_pressure: &X,
    ) -> (Head, Tail) {
        (
            Head::interpolate_between(
                (low_static_pressure.clone(), low_head),
                (high_static_pressure.clone(), high_head),
                target_static_pressure,
            ),
            Tail::interpolate_between(
                (low_static_pressure, low_tail),
                (high_static_pressure, high_tail),
                target_static_pressure,
            ),
        )
    }
}

pub trait MeanErrorSquareComparable {
    fn error_from(&self, other: &Self) -> f64;
    fn error_sum(&self, other: &Self) -> f64;
}

// impl<T> MeanErrorSquareComparable for T
// where
//     T: Sub<T, Output = T> + Div<T, Output = f64> + Clone, // &T: Sub<Output = impl Div<&T, Output = f64>>,
// {
//     fn error_from(&self, other: &Self) -> f64 {
//         ((self.clone() - other.clone()) / other.clone()).powi(2)
//     }
// }
