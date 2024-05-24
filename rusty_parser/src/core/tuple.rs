// marker trait for tuple types
pub trait Tuple {}

impl Tuple for () {}
impl<U0> Tuple for (U0,) {}
impl<U0, U1> Tuple for (U0, U1) {}
impl<U0, U1, U2> Tuple for (U0, U1, U2) {}
impl<U0, U1, U2, U3> Tuple for (U0, U1, U2, U3) {}
impl<U0, U1, U2, U3, U4> Tuple for (U0, U1, U2, U3, U4) {}
impl<U0, U1, U2, U3, U4, U5> Tuple for (U0, U1, U2, U3, U4, U5) {}
impl<U0, U1, U2, U3, U4, U5, U6> Tuple for (U0, U1, U2, U3, U4, U5, U6) {}
impl<U0, U1, U2, U3, U4, U5, U6, U7> Tuple for (U0, U1, U2, U3, U4, U5, U6, U7) {}
impl<U0, U1, U2, U3, U4, U5, U6, U7, U8> Tuple for (U0, U1, U2, U3, U4, U5, U6, U7, U8) {}
impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9> Tuple for (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9) {}
impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10> Tuple
    for (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)
{
}
impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11> Tuple
    for (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11)
{
}
// impl<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12> Tuple for (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12) {}
