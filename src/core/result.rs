use std::iter::Iterator;

#[derive(Debug)]
pub struct ParseResult<Output, It>
where
    It: Iterator + Clone,
{
    pub output: Option<Output>,
    pub it: It,
}
