pub fn first<T1, Tup>((t1, _rest): &(T1, Tup)) -> &T1 {
    t1
}

pub fn second<T1, T2, Tup>((_t1, (t2, _rest)): &(T1, (T2, Tup))) -> &T2 {
    t2
}

pub fn third<T1, T2, T3, Tup>((_t1, (_t2, (t3, _rest))): &(T1, (T2, (T3, Tup)))) -> &T3 {
    t3
}

pub fn fourth<T1, T2, T3, T4, Tup>(
    (_t1, (_t2, (_t3, (t4, _rest)))): &(T1, (T2, (T3, (T4, Tup)))),
) -> &T4 {
    t4
}

pub fn fifth<T1, T2, T3, T4, T5, Tup>(
    (_t1, (_t2, (_t3, (_t4, (t5, _rest))))): &(T1, (T2, (T3, (T4, (T5, Tup))))),
) -> &T5 {
    t5
}
