use super::tuple::Tuple;
use std::iter::Iterator;

#[derive(Debug)]
pub struct ParseResult<Output: Tuple, It>
where
    It: Iterator + Clone,
{
    pub output: Option<Output>,
    pub it: It,
}
