use tuple_list::TupleList;

pub trait Lenable {
    fn len(&self) -> i32;

    fn is_empty(&self) -> bool;
}

impl Lenable for () {
    fn len(&self) -> i32 {
        0
    }

    fn is_empty(&self) -> bool{
        true
    }
}

impl<Head, Tail: TupleList + Lenable> Lenable for (Head, Tail) {
    fn len(&self) -> i32 {
        1 + self.1.len()
    }

    fn is_empty(&self) -> bool{
        false
    }
}
