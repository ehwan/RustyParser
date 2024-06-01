use std::vec::Vec;

// this tarit is used to construct Vec< Output > for Repeat Parser
// if Output is (), new Output will be ()
// if Output is (T0,), new Output will be Vec<T0>
// otherwise, new Output will be Vec<Output>
pub trait VectorOutputSpecialize {
    type Output;

    fn new_output() -> Self::Output;
    fn reserve(output: &mut Self::Output, reserve_size: usize);
    fn push_this_to_output(self, output: &mut Self::Output);
}

// () -> ()
impl VectorOutputSpecialize for () {
    type Output = ();

    fn new_output() -> Self::Output {}
    fn reserve(_output: &mut Self::Output, _reserve_size: usize) {}

    fn push_this_to_output(self, _output: &mut Self::Output) {}
}

// (T0,) -> ( Vec<T0>, )
impl<T0> VectorOutputSpecialize for (T0,) {
    type Output = (Vec<T0>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self.0);
    }
}

// ( T0, T1, ..., Tn ) -> Vec< (T0, T1, ..., Tn) >
// for others
impl<T0, T1> VectorOutputSpecialize for (T0, T1) {
    type Output = (Vec<(T0, T1)>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self);
    }
}

impl<T0, T1, T2> VectorOutputSpecialize for (T0, T1, T2) {
    type Output = (Vec<(T0, T1, T2)>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self);
    }
}

impl<T0, T1, T2, T3> VectorOutputSpecialize for (T0, T1, T2, T3) {
    type Output = (Vec<(T0, T1, T2, T3)>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self);
    }
}

impl<T0, T1, T2, T3, T4> VectorOutputSpecialize for (T0, T1, T2, T3, T4) {
    type Output = (Vec<(T0, T1, T2, T3, T4)>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self);
    }
}

impl<T0, T1, T2, T3, T4, T5> VectorOutputSpecialize for (T0, T1, T2, T3, T4, T5) {
    type Output = (Vec<(T0, T1, T2, T3, T4, T5)>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self);
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> VectorOutputSpecialize for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = (Vec<(T0, T1, T2, T3, T4, T5, T6)>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self);
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> VectorOutputSpecialize for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Output = (Vec<(T0, T1, T2, T3, T4, T5, T6, T7)>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self);
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> VectorOutputSpecialize
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
{
    type Output = (Vec<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self);
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> VectorOutputSpecialize
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
{
    type Output = (Vec<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self);
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> VectorOutputSpecialize
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
    type Output = (Vec<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self);
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> VectorOutputSpecialize
    for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
    type Output = (Vec<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>,);

    fn new_output() -> Self::Output {
        (Vec::new(),)
    }
    fn reserve(output: &mut Self::Output, reserve_size: usize) {
        output.0.reserve(reserve_size);
    }
    fn push_this_to_output(self, output: &mut Self::Output) {
        output.0.push(self);
    }
}
