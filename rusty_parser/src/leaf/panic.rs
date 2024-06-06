use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

/// A dummy parser that always panic when parsing.
/// This parser is for default value for `Box` parser.
#[derive(Debug)]
pub struct Panic<Output: Tuple> {
    _phantom: std::marker::PhantomData<Output>,
}

impl<Output: Tuple> Panic<Output> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<Output: Tuple> Clone for Panic<Output> {
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<Output: Tuple> Copy for Panic<Output> {}

impl<Output: Tuple> Default for Panic<Output> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Output: Tuple, It> Parser<It> for Panic<Output>
where
    It: InputIteratorTrait,
{
    type Output = Output;

    fn parse(&self, _: It) -> ParseResult<Self::Output, It> {
        panic!("Dummy parser Panic::parse() is called.");
    }

    fn match_pattern(&self, _: It) -> ParseResult<(), It> {
        panic!("Dummy parser Panic::match_pattern() is called.");
    }
}

impl<Output: Tuple> IntoParser for Panic<Output> {
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}
