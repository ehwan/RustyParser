use std::ops::Deref;
use std::ops::DerefMut;

use crate::core::into_parser::IntoParser;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

use crate::leaf::panic::Panic;

pub struct DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone,
{
    parser: std::boxed::Box<
        dyn for<'a> Parser<std::iter::Cloned<std::slice::Iter<'a, T>>, Output = Output>,
    >,
}

impl<Output, T> DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone,
{
    pub fn new<ParserType: IntoParser>(parser: ParserType) -> Self
    where
        ParserType::Into:
            for<'a> Parser<std::iter::Cloned<std::slice::Iter<'a, T>>, Output = Output> + 'static,
    {
        Self {
            parser: std::boxed::Box::new(parser.into_parser()),
        }
    }
    pub fn assign<ParserType: IntoParser>(&mut self, parser: ParserType)
    where
        ParserType::Into:
            for<'a> Parser<std::iter::Cloned<std::slice::Iter<'a, T>>, Output = Output> + 'static,
    {
        self.parser = std::boxed::Box::new(parser.into_parser());
    }
}
/// default to dummy parser that always panic
impl<Output: Tuple + 'static, T: Clone + 'static> Default for DynBoxSlice<Output, T> {
    fn default() -> Self {
        Self::new(Panic::new())
    }
}

impl<'a, Output, T> Parser<std::iter::Cloned<std::slice::Iter<'a, T>>> for DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone,
{
    type Output = Output;

    fn parse(
        &self,
        it: std::iter::Cloned<std::slice::Iter<'a, T>>,
    ) -> ParseResult<Self::Output, std::iter::Cloned<std::slice::Iter<'a, T>>> {
        self.parser.parse(it)
    }

    fn match_pattern(
        &self,
        it: std::iter::Cloned<std::slice::Iter<'a, T>>,
    ) -> ParseResult<(), std::iter::Cloned<std::slice::Iter<'a, T>>> {
        self.parser.match_pattern(it)
    }
}

impl<Output, T> Deref for DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone,
{
    type Target = std::boxed::Box<
        dyn for<'a> Parser<std::iter::Cloned<std::slice::Iter<'a, T>>, Output = Output>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}
impl<Output, T> DerefMut for DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}
impl<Output, T> IntoParser for DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }

    // TODO no boxed here
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[should_panic]
    fn panic_test2() {
        let boxed: DynBoxSlice<(i32,), i32> = Default::default();
        boxed.parse((&[1, 2, 3]).iter().cloned());
        boxed.match_pattern((&[1, 2, 3]).iter().cloned());
    }
}
