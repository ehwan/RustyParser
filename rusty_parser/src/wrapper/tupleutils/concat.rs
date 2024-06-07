/// Concatenate Tuple
/// Maximum Tuple Size: 12
/// TODO: any elegant way to merge tuple?

pub trait AppendTupleToTuple<Tup> {
    type Output;
    fn append_back(self, val: Tup) -> Self::Output;
}

impl AppendTupleToTuple<()> for () {
    type Output = ();
    fn append_back(self, _val: ()) -> Self::Output {}
}

impl<U0> AppendTupleToTuple<(U0,)> for () {
    type Output = (U0,);
    fn append_back(self, val: (U0,)) -> Self::Output {
        val
    }
}

impl<U0, U1> AppendTupleToTuple<(U0, U1)> for () {
    type Output = (U0, U1);
    fn append_back(self, val: (U0, U1)) -> Self::Output {
        val
    }
}

impl<U0, U1, U2> AppendTupleToTuple<(U0, U1, U2)> for () {
    type Output = (U0, U1, U2);
    fn append_back(self, val: (U0, U1, U2)) -> Self::Output {
        val
    }
}

impl<U0, U1, U2, U3> AppendTupleToTuple<(U0, U1, U2, U3)> for () {
    type Output = (U0, U1, U2, U3);
    fn append_back(self, val: (U0, U1, U2, U3)) -> Self::Output {
        val
    }
}

impl<U0, U1, U2, U3, U4> AppendTupleToTuple<(U0, U1, U2, U3, U4)> for () {
    type Output = (U0, U1, U2, U3, U4);
    fn append_back(self, val: (U0, U1, U2, U3, U4)) -> Self::Output {
        val
    }
}

impl<U0, U1, U2, U3, U4, U5> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5)> for () {
    type Output = (U0, U1, U2, U3, U4, U5);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5)) -> Self::Output {
        val
    }
}

impl<U0, U1, U2, U3, U4, U5, U6> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6)> for () {
    type Output = (U0, U1, U2, U3, U4, U5, U6);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6)) -> Self::Output {
        val
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7)> for () {
    type Output = (U0, U1, U2, U3, U4, U5, U6, U7);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7)) -> Self::Output {
        val
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, U8> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7, U8)>
    for ()
{
    type Output = (U0, U1, U2, U3, U4, U5, U6, U7, U8);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7, U8)) -> Self::Output {
        val
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)> for ()
{
    type Output = (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)) -> Self::Output {
        val
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)> for ()
{
    type Output = (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)) -> Self::Output {
        val
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11)> for ()
{
    type Output = (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11)) -> Self::Output {
        val
    }
}

impl<V0> AppendTupleToTuple<()> for (V0,) {
    type Output = (V0,);
    fn append_back(self, _val: ()) -> Self::Output {
        self
    }
}
impl<U0, V0> AppendTupleToTuple<(U0,)> for (V0,) {
    type Output = (V0, U0);
    fn append_back(self, val: (U0,)) -> Self::Output {
        (self.0, val.0)
    }
}

impl<U0, U1, V0> AppendTupleToTuple<(U0, U1)> for (V0,) {
    type Output = (V0, U0, U1);
    fn append_back(self, val: (U0, U1)) -> Self::Output {
        (self.0, val.0, val.1)
    }
}
impl<U0, U1, U2, V0> AppendTupleToTuple<(U0, U1, U2)> for (V0,) {
    type Output = (V0, U0, U1, U2);
    fn append_back(self, val: (U0, U1, U2)) -> Self::Output {
        (self.0, val.0, val.1, val.2)
    }
}
impl<U0, U1, U2, U3, V0> AppendTupleToTuple<(U0, U1, U2, U3)> for (V0,) {
    type Output = (V0, U0, U1, U2, U3);
    fn append_back(self, val: (U0, U1, U2, U3)) -> Self::Output {
        (self.0, val.0, val.1, val.2, val.3)
    }
}

impl<U0, U1, U2, U3, U4, V0> AppendTupleToTuple<(U0, U1, U2, U3, U4)> for (V0,) {
    type Output = (V0, U0, U1, U2, U3, U4);
    fn append_back(self, val: (U0, U1, U2, U3, U4)) -> Self::Output {
        (self.0, val.0, val.1, val.2, val.3, val.4)
    }
}

impl<U0, U1, U2, U3, U4, U5, V0> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5)> for (V0,) {
    type Output = (V0, U0, U1, U2, U3, U4, U5);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5)) -> Self::Output {
        (self.0, val.0, val.1, val.2, val.3, val.4, val.5)
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, V0> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6)> for (V0,) {
    type Output = (V0, U0, U1, U2, U3, U4, U5, U6);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6)) -> Self::Output {
        (self.0, val.0, val.1, val.2, val.3, val.4, val.5, val.6)
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, V0> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7)>
    for (V0,)
{
    type Output = (V0, U0, U1, U2, U3, U4, U5, U6, U7);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7)) -> Self::Output {
        (
            self.0, val.0, val.1, val.2, val.3, val.4, val.5, val.6, val.7,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, V0>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7, U8)> for (V0,)
{
    type Output = (V0, U0, U1, U2, U3, U4, U5, U6, U7, U8);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7, U8)) -> Self::Output {
        (
            self.0, val.0, val.1, val.2, val.3, val.4, val.5, val.6, val.7, val.8,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, V0>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)> for (V0,)
{
    type Output = (V0, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)) -> Self::Output {
        (
            self.0, val.0, val.1, val.2, val.3, val.4, val.5, val.6, val.7, val.8, val.9,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, V0>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)> for (V0,)
{
    type Output = (V0, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)) -> Self::Output {
        (
            self.0, val.0, val.1, val.2, val.3, val.4, val.5, val.6, val.7, val.8, val.9, val.10,
        )
    }
}

impl<V0, V1> AppendTupleToTuple<()> for (V0, V1) {
    type Output = (V0, V1);
    fn append_back(self, _val: ()) -> Self::Output {
        self
    }
}
impl<U0, V0, V1> AppendTupleToTuple<(U0,)> for (V0, V1) {
    type Output = (V0, V1, U0);
    fn append_back(self, val: (U0,)) -> Self::Output {
        (self.0, self.1, val.0)
    }
}

impl<U0, U1, V0, V1> AppendTupleToTuple<(U0, U1)> for (V0, V1) {
    type Output = (V0, V1, U0, U1);
    fn append_back(self, val: (U0, U1)) -> Self::Output {
        (self.0, self.1, val.0, val.1)
    }
}
impl<U0, U1, U2, V0, V1> AppendTupleToTuple<(U0, U1, U2)> for (V0, V1) {
    type Output = (V0, V1, U0, U1, U2);
    fn append_back(self, val: (U0, U1, U2)) -> Self::Output {
        (self.0, self.1, val.0, val.1, val.2)
    }
}
impl<U0, U1, U2, U3, V0, V1> AppendTupleToTuple<(U0, U1, U2, U3)> for (V0, V1) {
    type Output = (V0, V1, U0, U1, U2, U3);
    fn append_back(self, val: (U0, U1, U2, U3)) -> Self::Output {
        (self.0, self.1, val.0, val.1, val.2, val.3)
    }
}

impl<U0, U1, U2, U3, U4, V0, V1> AppendTupleToTuple<(U0, U1, U2, U3, U4)> for (V0, V1) {
    type Output = (V0, V1, U0, U1, U2, U3, U4);
    fn append_back(self, val: (U0, U1, U2, U3, U4)) -> Self::Output {
        (self.0, self.1, val.0, val.1, val.2, val.3, val.4)
    }
}

impl<U0, U1, U2, U3, U4, U5, V0, V1> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5)> for (V0, V1) {
    type Output = (V0, V1, U0, U1, U2, U3, U4, U5);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5)) -> Self::Output {
        (self.0, self.1, val.0, val.1, val.2, val.3, val.4, val.5)
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, V0, V1> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6)>
    for (V0, V1)
{
    type Output = (V0, V1, U0, U1, U2, U3, U4, U5, U6);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6)) -> Self::Output {
        (
            self.0, self.1, val.0, val.1, val.2, val.3, val.4, val.5, val.6,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, V0, V1> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7)>
    for (V0, V1)
{
    type Output = (V0, V1, U0, U1, U2, U3, U4, U5, U6, U7);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7)) -> Self::Output {
        (
            self.0, self.1, val.0, val.1, val.2, val.3, val.4, val.5, val.6, val.7,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, V0, V1>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7, U8)> for (V0, V1)
{
    type Output = (V0, V1, U0, U1, U2, U3, U4, U5, U6, U7, U8);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7, U8)) -> Self::Output {
        (
            self.0, self.1, val.0, val.1, val.2, val.3, val.4, val.5, val.6, val.7, val.8,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, V0, V1>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)> for (V0, V1)
{
    type Output = (V0, V1, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)) -> Self::Output {
        (
            self.0, self.1, val.0, val.1, val.2, val.3, val.4, val.5, val.6, val.7, val.8, val.9,
        )
    }
}

impl<V0, V1, V2> AppendTupleToTuple<()> for (V0, V1, V2) {
    type Output = (V0, V1, V2);
    fn append_back(self, _val: ()) -> Self::Output {
        self
    }
}
impl<U0, V0, V1, V2> AppendTupleToTuple<(U0,)> for (V0, V1, V2) {
    type Output = (V0, V1, V2, U0);
    fn append_back(self, val: (U0,)) -> Self::Output {
        (self.0, self.1, self.2, val.0)
    }
}

impl<U0, U1, V0, V1, V2> AppendTupleToTuple<(U0, U1)> for (V0, V1, V2) {
    type Output = (V0, V1, V2, U0, U1);
    fn append_back(self, val: (U0, U1)) -> Self::Output {
        (self.0, self.1, self.2, val.0, val.1)
    }
}

impl<U0, U1, U2, V0, V1, V2> AppendTupleToTuple<(U0, U1, U2)> for (V0, V1, V2) {
    type Output = (V0, V1, V2, U0, U1, U2);
    fn append_back(self, val: (U0, U1, U2)) -> Self::Output {
        (self.0, self.1, self.2, val.0, val.1, val.2)
    }
}

impl<U0, U1, U2, U3, V0, V1, V2> AppendTupleToTuple<(U0, U1, U2, U3)> for (V0, V1, V2) {
    type Output = (V0, V1, V2, U0, U1, U2, U3);
    fn append_back(self, val: (U0, U1, U2, U3)) -> Self::Output {
        (self.0, self.1, self.2, val.0, val.1, val.2, val.3)
    }
}

impl<U0, U1, U2, U3, U4, V0, V1, V2> AppendTupleToTuple<(U0, U1, U2, U3, U4)> for (V0, V1, V2) {
    type Output = (V0, V1, V2, U0, U1, U2, U3, U4);
    fn append_back(self, val: (U0, U1, U2, U3, U4)) -> Self::Output {
        (self.0, self.1, self.2, val.0, val.1, val.2, val.3, val.4)
    }
}

impl<U0, U1, U2, U3, U4, U5, V0, V1, V2> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5)>
    for (V0, V1, V2)
{
    type Output = (V0, V1, V2, U0, U1, U2, U3, U4, U5);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5)) -> Self::Output {
        (
            self.0, self.1, self.2, val.0, val.1, val.2, val.3, val.4, val.5,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, V0, V1, V2> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6)>
    for (V0, V1, V2)
{
    type Output = (V0, V1, V2, U0, U1, U2, U3, U4, U5, U6);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6)) -> Self::Output {
        (
            self.0, self.1, self.2, val.0, val.1, val.2, val.3, val.4, val.5, val.6,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, V0, V1, V2>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7)> for (V0, V1, V2)
{
    type Output = (V0, V1, V2, U0, U1, U2, U3, U4, U5, U6, U7);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7)) -> Self::Output {
        (
            self.0, self.1, self.2, val.0, val.1, val.2, val.3, val.4, val.5, val.6, val.7,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, V0, V1, V2>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7, U8)> for (V0, V1, V2)
{
    type Output = (V0, V1, V2, U0, U1, U2, U3, U4, U5, U6, U7, U8);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7, U8)) -> Self::Output {
        (
            self.0, self.1, self.2, val.0, val.1, val.2, val.3, val.4, val.5, val.6, val.7, val.8,
        )
    }
}

impl<V0, V1, V2, V3> AppendTupleToTuple<()> for (V0, V1, V2, V3) {
    type Output = (V0, V1, V2, V3);
    fn append_back(self, _val: ()) -> Self::Output {
        self
    }
}
impl<U0, V0, V1, V2, V3> AppendTupleToTuple<(U0,)> for (V0, V1, V2, V3) {
    type Output = (V0, V1, V2, V3, U0);
    fn append_back(self, val: (U0,)) -> Self::Output {
        (self.0, self.1, self.2, self.3, val.0)
    }
}

impl<U0, U1, V0, V1, V2, V3> AppendTupleToTuple<(U0, U1)> for (V0, V1, V2, V3) {
    type Output = (V0, V1, V2, V3, U0, U1);
    fn append_back(self, val: (U0, U1)) -> Self::Output {
        (self.0, self.1, self.2, self.3, val.0, val.1)
    }
}

impl<U0, U1, U2, V0, V1, V2, V3> AppendTupleToTuple<(U0, U1, U2)> for (V0, V1, V2, V3) {
    type Output = (V0, V1, V2, V3, U0, U1, U2);
    fn append_back(self, val: (U0, U1, U2)) -> Self::Output {
        (self.0, self.1, self.2, self.3, val.0, val.1, val.2)
    }
}

impl<U0, U1, U2, U3, V0, V1, V2, V3> AppendTupleToTuple<(U0, U1, U2, U3)> for (V0, V1, V2, V3) {
    type Output = (V0, V1, V2, V3, U0, U1, U2, U3);
    fn append_back(self, val: (U0, U1, U2, U3)) -> Self::Output {
        (self.0, self.1, self.2, self.3, val.0, val.1, val.2, val.3)
    }
}

impl<U0, U1, U2, U3, U4, V0, V1, V2, V3> AppendTupleToTuple<(U0, U1, U2, U3, U4)>
    for (V0, V1, V2, V3)
{
    type Output = (V0, V1, V2, V3, U0, U1, U2, U3, U4);
    fn append_back(self, val: (U0, U1, U2, U3, U4)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, val.0, val.1, val.2, val.3, val.4,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, V0, V1, V2, V3> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5)>
    for (V0, V1, V2, V3)
{
    type Output = (V0, V1, V2, V3, U0, U1, U2, U3, U4, U5);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, val.0, val.1, val.2, val.3, val.4, val.5,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, V0, V1, V2, V3> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6)>
    for (V0, V1, V2, V3)
{
    type Output = (V0, V1, V2, V3, U0, U1, U2, U3, U4, U5, U6);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, val.0, val.1, val.2, val.3, val.4, val.5, val.6,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, U7, V0, V1, V2, V3>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6, U7)> for (V0, V1, V2, V3)
{
    type Output = (V0, V1, V2, V3, U0, U1, U2, U3, U4, U5, U6, U7);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6, U7)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, val.0, val.1, val.2, val.3, val.4, val.5, val.6, val.7,
        )
    }
}

impl<V0, V1, V2, V3, V4> AppendTupleToTuple<()> for (V0, V1, V2, V3, V4) {
    type Output = (V0, V1, V2, V3, V4);
    fn append_back(self, _val: ()) -> Self::Output {
        self
    }
}
impl<U0, V0, V1, V2, V3, V4> AppendTupleToTuple<(U0,)> for (V0, V1, V2, V3, V4) {
    type Output = (V0, V1, V2, V3, V4, U0);
    fn append_back(self, val: (U0,)) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, val.0)
    }
}

impl<U0, U1, V0, V1, V2, V3, V4> AppendTupleToTuple<(U0, U1)> for (V0, V1, V2, V3, V4) {
    type Output = (V0, V1, V2, V3, V4, U0, U1);
    fn append_back(self, val: (U0, U1)) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, val.0, val.1)
    }
}

impl<U0, U1, U2, V0, V1, V2, V3, V4> AppendTupleToTuple<(U0, U1, U2)> for (V0, V1, V2, V3, V4) {
    type Output = (V0, V1, V2, V3, V4, U0, U1, U2);
    fn append_back(self, val: (U0, U1, U2)) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, val.0, val.1, val.2)
    }
}

impl<U0, U1, U2, U3, V0, V1, V2, V3, V4> AppendTupleToTuple<(U0, U1, U2, U3)>
    for (V0, V1, V2, V3, V4)
{
    type Output = (V0, V1, V2, V3, V4, U0, U1, U2, U3);
    fn append_back(self, val: (U0, U1, U2, U3)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, val.0, val.1, val.2, val.3,
        )
    }
}

impl<U0, U1, U2, U3, U4, V0, V1, V2, V3, V4> AppendTupleToTuple<(U0, U1, U2, U3, U4)>
    for (V0, V1, V2, V3, V4)
{
    type Output = (V0, V1, V2, V3, V4, U0, U1, U2, U3, U4);
    fn append_back(self, val: (U0, U1, U2, U3, U4)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, val.0, val.1, val.2, val.3, val.4,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, V0, V1, V2, V3, V4> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5)>
    for (V0, V1, V2, V3, V4)
{
    type Output = (V0, V1, V2, V3, V4, U0, U1, U2, U3, U4, U5);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, val.0, val.1, val.2, val.3, val.4, val.5,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, U6, V0, V1, V2, V3, V4>
    AppendTupleToTuple<(U0, U1, U2, U3, U4, U5, U6)> for (V0, V1, V2, V3, V4)
{
    type Output = (V0, V1, V2, V3, V4, U0, U1, U2, U3, U4, U5, U6);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5, U6)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, val.0, val.1, val.2, val.3, val.4, val.5, val.6,
        )
    }
}

impl<V0, V1, V2, V3, V4, V5> AppendTupleToTuple<()> for (V0, V1, V2, V3, V4, V5) {
    type Output = (V0, V1, V2, V3, V4, V5);
    fn append_back(self, _val: ()) -> Self::Output {
        self
    }
}
impl<U0, V0, V1, V2, V3, V4, V5> AppendTupleToTuple<(U0,)> for (V0, V1, V2, V3, V4, V5) {
    type Output = (V0, V1, V2, V3, V4, V5, U0);
    fn append_back(self, val: (U0,)) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, val.0)
    }
}

impl<U0, U1, V0, V1, V2, V3, V4, V5> AppendTupleToTuple<(U0, U1)> for (V0, V1, V2, V3, V4, V5) {
    type Output = (V0, V1, V2, V3, V4, V5, U0, U1);
    fn append_back(self, val: (U0, U1)) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, val.0, val.1)
    }
}

impl<U0, U1, U2, V0, V1, V2, V3, V4, V5> AppendTupleToTuple<(U0, U1, U2)>
    for (V0, V1, V2, V3, V4, V5)
{
    type Output = (V0, V1, V2, V3, V4, V5, U0, U1, U2);
    fn append_back(self, val: (U0, U1, U2)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, val.0, val.1, val.2,
        )
    }
}

impl<U0, U1, U2, U3, V0, V1, V2, V3, V4, V5> AppendTupleToTuple<(U0, U1, U2, U3)>
    for (V0, V1, V2, V3, V4, V5)
{
    type Output = (V0, V1, V2, V3, V4, V5, U0, U1, U2, U3);
    fn append_back(self, val: (U0, U1, U2, U3)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, val.0, val.1, val.2, val.3,
        )
    }
}

impl<U0, U1, U2, U3, U4, V0, V1, V2, V3, V4, V5> AppendTupleToTuple<(U0, U1, U2, U3, U4)>
    for (V0, V1, V2, V3, V4, V5)
{
    type Output = (V0, V1, V2, V3, V4, V5, U0, U1, U2, U3, U4);
    fn append_back(self, val: (U0, U1, U2, U3, U4)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, val.0, val.1, val.2, val.3, val.4,
        )
    }
}

impl<U0, U1, U2, U3, U4, U5, V0, V1, V2, V3, V4, V5> AppendTupleToTuple<(U0, U1, U2, U3, U4, U5)>
    for (V0, V1, V2, V3, V4, V5)
{
    type Output = (V0, V1, V2, V3, V4, V5, U0, U1, U2, U3, U4, U5);
    fn append_back(self, val: (U0, U1, U2, U3, U4, U5)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, val.0, val.1, val.2, val.3, val.4,
            val.5,
        )
    }
}

impl<V0, V1, V2, V3, V4, V5, V6> AppendTupleToTuple<()> for (V0, V1, V2, V3, V4, V5, V6) {
    type Output = (V0, V1, V2, V3, V4, V5, V6);
    fn append_back(self, _val: ()) -> Self::Output {
        self
    }
}
impl<U0, V0, V1, V2, V3, V4, V5, V6> AppendTupleToTuple<(U0,)> for (V0, V1, V2, V3, V4, V5, V6) {
    type Output = (V0, V1, V2, V3, V4, V5, V6, U0);
    fn append_back(self, val: (U0,)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, val.0,
        )
    }
}

impl<U0, U1, V0, V1, V2, V3, V4, V5, V6> AppendTupleToTuple<(U0, U1)>
    for (V0, V1, V2, V3, V4, V5, V6)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, U0, U1);
    fn append_back(self, val: (U0, U1)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, val.0, val.1,
        )
    }
}

impl<U0, U1, U2, V0, V1, V2, V3, V4, V5, V6> AppendTupleToTuple<(U0, U1, U2)>
    for (V0, V1, V2, V3, V4, V5, V6)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, U0, U1, U2);
    fn append_back(self, val: (U0, U1, U2)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, val.0, val.1, val.2,
        )
    }
}

impl<U0, U1, U2, U3, V0, V1, V2, V3, V4, V5, V6> AppendTupleToTuple<(U0, U1, U2, U3)>
    for (V0, V1, V2, V3, V4, V5, V6)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, U0, U1, U2, U3);
    fn append_back(self, val: (U0, U1, U2, U3)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, val.0, val.1, val.2, val.3,
        )
    }
}

impl<U0, U1, U2, U3, U4, V0, V1, V2, V3, V4, V5, V6> AppendTupleToTuple<(U0, U1, U2, U3, U4)>
    for (V0, V1, V2, V3, V4, V5, V6)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, U0, U1, U2, U3, U4);
    fn append_back(self, val: (U0, U1, U2, U3, U4)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, val.0, val.1, val.2, val.3,
            val.4,
        )
    }
}

impl<V0, V1, V2, V3, V4, V5, V6, V7> AppendTupleToTuple<()> for (V0, V1, V2, V3, V4, V5, V6, V7) {
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7);
    fn append_back(self, _val: ()) -> Self::Output {
        self
    }
}
impl<U0, V0, V1, V2, V3, V4, V5, V6, V7> AppendTupleToTuple<(U0,)>
    for (V0, V1, V2, V3, V4, V5, V6, V7)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, U0);
    fn append_back(self, val: (U0,)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, val.0,
        )
    }
}

impl<U0, U1, V0, V1, V2, V3, V4, V5, V6, V7> AppendTupleToTuple<(U0, U1)>
    for (V0, V1, V2, V3, V4, V5, V6, V7)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, U0, U1);
    fn append_back(self, val: (U0, U1)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, val.0, val.1,
        )
    }
}

impl<U0, U1, U2, V0, V1, V2, V3, V4, V5, V6, V7> AppendTupleToTuple<(U0, U1, U2)>
    for (V0, V1, V2, V3, V4, V5, V6, V7)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, U0, U1, U2);
    fn append_back(self, val: (U0, U1, U2)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, val.0, val.1, val.2,
        )
    }
}

impl<U0, U1, U2, U3, V0, V1, V2, V3, V4, V5, V6, V7> AppendTupleToTuple<(U0, U1, U2, U3)>
    for (V0, V1, V2, V3, V4, V5, V6, V7)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, U0, U1, U2, U3);
    fn append_back(self, val: (U0, U1, U2, U3)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, val.0, val.1, val.2,
            val.3,
        )
    }
}

impl<V0, V1, V2, V3, V4, V5, V6, V7, V8> AppendTupleToTuple<()>
    for (V0, V1, V2, V3, V4, V5, V6, V7, V8)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, V8);
    fn append_back(self, _val: ()) -> Self::Output {
        self
    }
}
impl<U0, V0, V1, V2, V3, V4, V5, V6, V7, V8> AppendTupleToTuple<(U0,)>
    for (V0, V1, V2, V3, V4, V5, V6, V7, V8)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, V8, U0);
    fn append_back(self, val: (U0,)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, val.0,
        )
    }
}

impl<U0, U1, V0, V1, V2, V3, V4, V5, V6, V7, V8> AppendTupleToTuple<(U0, U1)>
    for (V0, V1, V2, V3, V4, V5, V6, V7, V8)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, V8, U0, U1);
    fn append_back(self, val: (U0, U1)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, val.0, val.1,
        )
    }
}

impl<U0, U1, U2, V0, V1, V2, V3, V4, V5, V6, V7, V8> AppendTupleToTuple<(U0, U1, U2)>
    for (V0, V1, V2, V3, V4, V5, V6, V7, V8)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, V8, U0, U1, U2);
    fn append_back(self, val: (U0, U1, U2)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, val.0, val.1,
            val.2,
        )
    }
}

impl<V0, V1, V2, V3, V4, V5, V6, V7, V8, V9> AppendTupleToTuple<()>
    for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9);
    fn append_back(self, _val: ()) -> Self::Output {
        self
    }
}
impl<U0, V0, V1, V2, V3, V4, V5, V6, V7, V8, V9> AppendTupleToTuple<(U0,)>
    for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, U0);
    fn append_back(self, val: (U0,)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, val.0,
        )
    }
}

impl<U0, U1, V0, V1, V2, V3, V4, V5, V6, V7, V8, V9> AppendTupleToTuple<(U0, U1)>
    for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, U0, U1);
    fn append_back(self, val: (U0, U1)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, val.0,
            val.1,
        )
    }
}

impl<V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10> AppendTupleToTuple<()>
    for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10);
    fn append_back(self, _val: ()) -> Self::Output {
        self
    }
}
impl<U0, V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10> AppendTupleToTuple<(U0,)>
    for (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10)
{
    type Output = (V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, V10, U0);
    fn append_back(self, val: (U0,)) -> Self::Output {
        (
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9,
            self.10, val.0,
        )
    }
}
