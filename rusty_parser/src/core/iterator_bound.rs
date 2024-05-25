use std::iter::Iterator;

// this is trait bound alias for input Iterator
pub trait InputIteratorTrait: Iterator + Clone {}

impl<T> InputIteratorTrait for T where T: Iterator + Clone {}
