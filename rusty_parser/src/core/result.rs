use super::iterator_bound::InputIteratorTrait;
use super::tuple::Tuple;

#[derive(Debug)]
pub struct ParseResult<Output, It>
where
    Output: Tuple,
    It: InputIteratorTrait,
{
    /// Output of parsing, extracted data.
    ///
    /// 'None' means parsing failed
    pub output: Option<Output>,

    /// iterator after parsing.
    ///
    /// if parsing failed, this will be the same as the input iterator.
    pub it: It,
}
