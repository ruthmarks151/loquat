use std::iter::Zip;

pub fn pairwise<T>(i: Vec<T>) -> Zip<std::vec::IntoIter<T>, std::vec::IntoIter<T>>
where
    T: Clone,
{
    let mut tail = i.clone().into_iter();
    tail.next();
    i.into_iter().zip(tail)
}
