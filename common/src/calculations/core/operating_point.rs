use tuple_list::TupleList;

use crate::calculations::traits::{
    Interpolable, Lenable, MeanErrorSquareComparable, ScalesTo, ScalesWith,
};

#[derive(Clone)]
pub struct OperatingPoint<Tup: TupleList>(pub Tup);

impl<Context, Tup> ScalesWith<Context> for OperatingPoint<Tup>
where
    Tup: TupleList + ScalesWith<Context>,
{
    fn scale(self, from: &Context, to: &Context) -> Self {
        OperatingPoint(self.0.scale(from, to))
    }
}

impl<Context, Tup: TupleList> ScalesTo<Context> for OperatingPoint<Tup>
where
    Context: Clone,
    Self: AsRef<Context> + ScalesWith<Context>,
{
    fn scale_to(self, to: &Context) -> Self {
        let from: Context = (self.as_ref() as &Context).clone();
        self.scale(&from, to)
    }
}

impl<X, Tup> Interpolable<X, OperatingPoint<Tup>> for OperatingPoint<Tup>
where
    X: Clone,
    Tup: Interpolable<X, Tup> + TupleList,
{
    fn interpolate_between(
        (low_x, OperatingPoint(low_tup)): (X, OperatingPoint<Tup>),
        (high_x, OperatingPoint(high_tup)): (X, OperatingPoint<Tup>),
        target: &X,
    ) -> OperatingPoint<Tup> {
        OperatingPoint(Tup::interpolate_between(
            (low_x, low_tup),
            (high_x, high_tup),
            target,
        ))
    }
}

impl<Tup: TupleList + Lenable + MeanErrorSquareComparable> MeanErrorSquareComparable
    for OperatingPoint<Tup>
{
    fn error_from(&self, other: &Self) -> f64 {
        self.0.error_from(&other.0)
    }

    fn error_sum(&self, other: &Self) -> f64 {
        self.0.error_sum(&other.0)
    }
}
