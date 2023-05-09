use tuple_list::TupleList;

use super::Lenable;

pub trait MeanErrorSquareComparable {
    fn error_from(&self, other: &Self) -> f64;
    fn error_sum(&self, other: &Self) -> f64;
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
