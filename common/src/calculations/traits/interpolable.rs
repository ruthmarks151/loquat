use tuple_list::TupleList;

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
    fn interpolate_between(_low: (X, ()), _high: (X, ()), _target: &X) {}
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
