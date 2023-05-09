use tuple_list::TupleList;

use crate::calculations::{Interpolable, MeanErrorSquareComparable, ScalesTo, ScalesWith};

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

trait Lenable {
    fn len(&self) -> i32;
}

impl Lenable for () {
    fn len(&self) -> i32 {
        0
    }
}

impl<Head, Tail: TupleList + Lenable> Lenable for (Head, Tail) {
    fn len(&self) -> i32 {
        1 + self.1.len()
    }
}

impl MeanErrorSquareComparable for () {
    fn error_from(&self, _other: &Self) -> f64 {
        0.0
    }

    fn error_sum(&self, _other: &Self) -> f64 {
        0.0
    }
}

impl<Head, Tail> MeanErrorSquareComparable for (Head, Tail)
where
    Head: MeanErrorSquareComparable,
    Tail: MeanErrorSquareComparable + TupleList + Lenable,
    (Head, Tail): TupleList,
{
    fn error_from(&self, other: &Self) -> f64 {
        let dimension_count: f64 = (self.len() + 1).into();
        self.error_sum(other) / dimension_count
    }

    fn error_sum(&self, (other_head, other_tail): &Self) -> f64 {
        let (head, tail) = self;
        head.error_sum(other_head) + tail.error_sum(other_tail)
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
