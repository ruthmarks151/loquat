use tuple_list::TupleList;

pub trait ScalesWith<Context> {
    fn scale(self, from: &Context, to: &Context) -> Self;
}

impl<T> ScalesWith<T> for () {
    fn scale(self, _from: &T, _to: &T) -> Self {}
}

impl<ScaledValue, Head, Tail> ScalesWith<ScaledValue> for (Head, Tail)
where
    Head: ScalesWith<ScaledValue>,
    Tail: ScalesWith<ScaledValue> + TupleList,
{
    fn scale(self, from: &ScaledValue, to: &ScaledValue) -> Self {
        let (head, tail) = self;
        (head.scale(from, to), tail.scale(from, to))
    }
}

pub trait ScalesTo<Context> {
    fn scale_to(self, to: &Context) -> Self;
}

impl<Context> ScalesTo<Context> for () {
    fn scale_to(self, _to: &Context) -> Self {
        ()
    }
}

impl<Context, Head, Tail> ScalesTo<Context> for (Head, Tail)
where
    Head: ScalesTo<Context>,
    Tail: ScalesTo<Context> + TupleList,
{
    fn scale_to(self, to: &Context) -> Self {
        let (head, tail) = self;
        (head.scale_to(to), tail.scale_to(to))
    }
}
