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

    fn new_output() -> Self::Output {
        ()
    }
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
