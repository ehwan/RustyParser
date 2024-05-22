// Merge Single Value into Tuple's front/back
// implement Maximum Tuple Size: 12
// TODO: any elegant way to merge tuple?

pub trait AppendValueToTuple<T> {
    type FrontOutput;
    type BackOutput;
    fn append_front(self, val: T) -> Self::FrontOutput;
    fn append_back(self, val: T) -> Self::BackOutput;
}

impl<T, U0> AppendValueToTuple<T> for (U0,) {
    type FrontOutput = (T, U0);
    type BackOutput = (U0, T);
    fn append_front(self, val: T) -> Self::FrontOutput {
        (val, self.0)
    }
    fn append_back(self, val: T) -> Self::BackOutput {
        (self.0, val)
    }
}
impl<T, U0, U1> AppendValueToTuple<T> for (U0, U1) {
    type FrontOutput = (T, U0, U1);
    type BackOutput = (U0, U1, T);
    fn append_front(self, val: T) -> Self::FrontOutput {
        (val, self.0, self.1)
    }
    fn append_back(self, val: T) -> Self::BackOutput {
        (self.0, self.1, val)
    }
}
impl<T, U0, U1, U2> AppendValueToTuple<T> for (U0, U1, U2) {
    type FrontOutput = (T, U0, U1, U2);
    type BackOutput = (U0, U1, U2, T);
    fn append_front(self, val: T) -> Self::FrontOutput {
        (val, self.0, self.1, self.2)
    }
    fn append_back(self, val: T) -> Self::BackOutput {
        (self.0, self.1, self.2, val)
    }
}

impl<T, U0, U1, U2, U3> AppendValueToTuple<T> for (U0, U1, U2, U3) {
    type FrontOutput = (T, U0, U1, U2, U3);
    type BackOutput = (U0, U1, U2, U3, T);
    fn append_front(self, val: T) -> Self::FrontOutput {
        (val, self.0, self.1, self.2, self.3)
    }
    fn append_back(self, val: T) -> Self::BackOutput {
        (self.0, self.1, self.2, self.3, val)
    }
}

impl<T, U0, U1, U2, U3, U4> AppendValueToTuple<T> for (U0, U1, U2, U3, U4) {
    type FrontOutput = (T, U0, U1, U2, U3, U4);
    type BackOutput = (U0, U1, U2, U3, U4, T);
    fn append_front(self, val: T) -> Self::FrontOutput {
        (val, self.0, self.1, self.2, self.3, self.4)
    }
    fn append_back(self, val: T) -> Self::BackOutput {
        (self.0, self.1, self.2, self.3, self.4, val)
    }
}

impl<T, U0, U1, U2, U3, U4, U5> AppendValueToTuple<T> for (U0, U1, U2, U3, U4, U5) {
    type FrontOutput = (T, U0, U1, U2, U3, U4, U5);
    type BackOutput = (U0, U1, U2, U3, U4, U5, T);
    fn append_front(self, val: T) -> Self::FrontOutput {
        (val, self.0, self.1, self.2, self.3, self.4, self.5)
    }
    fn append_back(self, val: T) -> Self::BackOutput {
        (self.0, self.1, self.2, self.3, self.4, self.5, val)
    }
}

impl<T, U0, U1, U2, U3, U4, U5, U6> AppendValueToTuple<T> for (U0, U1, U2, U3, U4, U5, U6) {
    type FrontOutput = (T, U0, U1, U2, U3, U4, U5, U6);
    type BackOutput = (U0, U1, U2, U3, U4, U5, U6, T);
    fn append_front(self, val: T) -> Self::FrontOutput {
        (val, self.0, self.1, self.2, self.3, self.4, self.5, self.6)
    }
    fn append_back(self, val: T) -> Self::BackOutput {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, val)
    }
}
impl<T, U0, U1, U2, U3, U4, U5, U6, U7> AppendValueToTuple<T> for (U0, U1, U2, U3, U4, U5, U6, U7) {
    type FrontOutput = (T, U0, U1, U2, U3, U4, U5, U6, U7);
    type BackOutput = (U0, U1, U2, U3, U4, U5, U6, U7, T);
    fn append_front(self, val: T) -> Self::FrontOutput {
        (
            val, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7,
        )
    }
    fn append_back(self, val: T) -> Self::BackOutput {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, val,
        )
    }
}
impl<T, U0, U1, U2, U3, U4, U5, U6, U7, U8> AppendValueToTuple<T>
    for (U0, U1, U2, U3, U4, U5, U6, U7, U8)
{
    type FrontOutput = (T, U0, U1, U2, U3, U4, U5, U6, U7, U8);
    type BackOutput = (U0, U1, U2, U3, U4, U5, U6, U7, U8, T);
    fn append_front(self, val: T) -> Self::FrontOutput {
        (
            val, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8,
        )
    }
    fn append_back(self, val: T) -> Self::BackOutput {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, val,
        )
    }
}

impl<T, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9> AppendValueToTuple<T>
    for (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)
{
    type FrontOutput = (T, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9);
    type BackOutput = (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, T);
    fn append_front(self, val: T) -> Self::FrontOutput {
        (
            val, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
        )
    }
    fn append_back(self, val: T) -> Self::BackOutput {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, val,
        )
    }
}

impl<T, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10> AppendValueToTuple<T>
    for (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)
{
    type FrontOutput = (T, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10);
    type BackOutput = (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, T);
    fn append_front(self, val: T) -> Self::FrontOutput {
        (
            val, self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            self.10,
        )
    }
    fn append_back(self, val: T) -> Self::BackOutput {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            self.10, val,
        )
    }
}
