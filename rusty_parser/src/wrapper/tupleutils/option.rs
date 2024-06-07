use std::option::Option;

/// this tarit is used to construct Option< Output > for Optional Parser
/// if Output is (T0,), new Output will be Option<T0>
/// otherwise, new Output will be Option<Output>
pub trait OptionOutputSpecialize {
    type Output;

    fn make_some(self) -> Self::Output;
    fn make_none() -> Self::Output;
}

// () -> Option<()>
impl OptionOutputSpecialize for () {
    type Output = Option<()>;

    fn make_some(self) -> Self::Output {
        Some(())
    }
    fn make_none() -> Self::Output {
        None
    }
}
// (T0,) -> Option<T0,>
impl<T0> OptionOutputSpecialize for (T0,) {
    type Output = Option<T0>;

    fn make_some(self) -> Self::Output {
        Some(self.0)
    }
    fn make_none() -> Self::Output {
        None
    }
}

impl<T0, T1> OptionOutputSpecialize for (T0, T1) {
    type Output = Option<(T0, T1)>;

    fn make_some(self) -> Self::Output {
        Some(self)
    }
    fn make_none() -> Self::Output {
        None
    }
}

impl<T0, T1, T2> OptionOutputSpecialize for (T0, T1, T2) {
    type Output = Option<(T0, T1, T2)>;

    fn make_some(self) -> Self::Output {
        Some(self)
    }
    fn make_none() -> Self::Output {
        None
    }
}

impl<T0, T1, T2, T3> OptionOutputSpecialize for (T0, T1, T2, T3) {
    type Output = Option<(T0, T1, T2, T3)>;

    fn make_some(self) -> Self::Output {
        Some(self)
    }
    fn make_none() -> Self::Output {
        None
    }
}

impl<T0, T1, T2, T3, T4> OptionOutputSpecialize for (T0, T1, T2, T3, T4) {
    type Output = Option<(T0, T1, T2, T3, T4)>;

    fn make_some(self) -> Self::Output {
        Some(self)
    }
    fn make_none() -> Self::Output {
        None
    }
}

impl<T0, T1, T2, T3, T4, T5> OptionOutputSpecialize for (T0, T1, T2, T3, T4, T5) {
    type Output = Option<(T0, T1, T2, T3, T4, T5)>;

    fn make_some(self) -> Self::Output {
        Some(self)
    }
    fn make_none() -> Self::Output {
        None
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> OptionOutputSpecialize for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = Option<(T0, T1, T2, T3, T4, T5, T6)>;

    fn make_some(self) -> Self::Output {
        Some(self)
    }
    fn make_none() -> Self::Output {
        None
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> OptionOutputSpecialize for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Output = Option<(T0, T1, T2, T3, T4, T5, T6, T7)>;

    fn make_some(self) -> Self::Output {
        Some(self)
    }
    fn make_none() -> Self::Output {
        None
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> OptionOutputSpecialize
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
{
    type Output = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>;

    fn make_some(self) -> Self::Output {
        Some(self)
    }
    fn make_none() -> Self::Output {
        None
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> OptionOutputSpecialize
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    type Output = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>;

    fn make_some(self) -> Self::Output {
        Some(self)
    }
    fn make_none() -> Self::Output {
        None
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> OptionOutputSpecialize
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    type Output = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>;

    fn make_some(self) -> Self::Output {
        Some(self)
    }
    fn make_none() -> Self::Output {
        None
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> OptionOutputSpecialize
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    type Output = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>;

    fn make_some(self) -> Self::Output {
        Some(self)
    }
    fn make_none() -> Self::Output {
        None
    }
}
