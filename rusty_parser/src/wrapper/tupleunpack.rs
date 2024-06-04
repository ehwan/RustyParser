pub trait TupleUnpack<Tup> {
    type Output;

    fn map(&self, args: Tup) -> Self::Output;
}

impl<Closure, Output> TupleUnpack<()> for Closure
where
    Closure: Fn() -> Output,
{
    type Output = Output;

    fn map(&self, _args: ()) -> Self::Output {
        (self)()
    }
}
impl<Closure, Output, T0> TupleUnpack<(T0,)> for Closure
where
    Closure: Fn(T0) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0,)) -> Self::Output {
        (self)(args.0)
    }
}

impl<Closure, Output, T0, T1> TupleUnpack<(T0, T1)> for Closure
where
    Closure: Fn(T0, T1) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0, T1)) -> Self::Output {
        (self)(args.0, args.1)
    }
}

impl<Closure, Output, T0, T1, T2> TupleUnpack<(T0, T1, T2)> for Closure
where
    Closure: Fn(T0, T1, T2) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0, T1, T2)) -> Self::Output {
        (self)(args.0, args.1, args.2)
    }
}

impl<Closure, Output, T0, T1, T2, T3> TupleUnpack<(T0, T1, T2, T3)> for Closure
where
    Closure: Fn(T0, T1, T2, T3) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0, T1, T2, T3)) -> Self::Output {
        (self)(args.0, args.1, args.2, args.3)
    }
}

impl<Closure, Output, T0, T1, T2, T3, T4> TupleUnpack<(T0, T1, T2, T3, T4)> for Closure
where
    Closure: Fn(T0, T1, T2, T3, T4) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0, T1, T2, T3, T4)) -> Self::Output {
        (self)(args.0, args.1, args.2, args.3, args.4)
    }
}

impl<Closure, Output, T0, T1, T2, T3, T4, T5> TupleUnpack<(T0, T1, T2, T3, T4, T5)> for Closure
where
    Closure: Fn(T0, T1, T2, T3, T4, T5) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0, T1, T2, T3, T4, T5)) -> Self::Output {
        (self)(args.0, args.1, args.2, args.3, args.4, args.5)
    }
}

impl<Closure, Output, T0, T1, T2, T3, T4, T5, T6> TupleUnpack<(T0, T1, T2, T3, T4, T5, T6)>
    for Closure
where
    Closure: Fn(T0, T1, T2, T3, T4, T5, T6) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        (self)(args.0, args.1, args.2, args.3, args.4, args.5, args.6)
    }
}

impl<Closure, Output, T0, T1, T2, T3, T4, T5, T6, T7> TupleUnpack<(T0, T1, T2, T3, T4, T5, T6, T7)>
    for Closure
where
    Closure: Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        (self)(
            args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7,
        )
    }
}

impl<Closure, Output, T0, T1, T2, T3, T4, T5, T6, T7, T8>
    TupleUnpack<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> for Closure
where
    Closure: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        (self)(
            args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8,
        )
    }
}

impl<Closure, Output, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>
    TupleUnpack<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> for Closure
where
    Closure: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        (self)(
            args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8, args.9,
        )
    }
}

impl<Closure, Output, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
    TupleUnpack<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> for Closure
where
    Closure: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        (self)(
            args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8, args.9, args.10,
        )
    }
}

impl<Closure, Output, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
    TupleUnpack<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> for Closure
where
    Closure: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Output,
{
    type Output = Output;

    fn map(&self, args: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        (self)(
            args.0, args.1, args.2, args.3, args.4, args.5, args.6, args.7, args.8, args.9,
            args.10, args.11,
        )
    }
}
