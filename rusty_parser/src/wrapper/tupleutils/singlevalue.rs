/// For special treatment of single-value-output
///
/// `T` will be wrapped in a tuple if it is not already a tuple
pub trait SingleValueAutoTuple<T> {
    type Output;

    fn wrap(self) -> Self::Output;
}

impl<T> SingleValueAutoTuple<T> for T {
    type Output = T;

    fn wrap(self) -> Self::Output {
        self
    }
}

impl<T> SingleValueAutoTuple<(T,)> for T {
    type Output = (T,);

    fn wrap(self) -> Self::Output {
        (self,)
    }
}
