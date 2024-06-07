use std::ops::Deref;
use std::ops::DerefMut;

use crate::core::into_parser::IntoParser;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

use crate::leaf::panic::Panic;

pub struct DynBoxSliceCopied<Output, T>
where
    Output: Tuple,
    T: Copy,
{
    parser: std::boxed::Box<
        dyn for<'a> Parser<std::iter::Copied<std::slice::Iter<'a, T>>, Output = Output>,
    >,
}

impl<Output, T> DynBoxSliceCopied<Output, T>
where
    Output: Tuple,
    T: Copy,
{
    pub fn new<ParserType: IntoParser>(parser: ParserType) -> Self
    where
        ParserType::Into:
            for<'a> Parser<std::iter::Copied<std::slice::Iter<'a, T>>, Output = Output> + 'static,
    {
        Self {
            parser: std::boxed::Box::new(parser.into_parser()),
        }
    }
    pub fn assign<ParserType: IntoParser>(&mut self, parser: ParserType)
    where
        ParserType::Into:
            for<'a> Parser<std::iter::Copied<std::slice::Iter<'a, T>>, Output = Output> + 'static,
    {
        self.parser = std::boxed::Box::new(parser.into_parser());
    }
}
/// default to dummy parser that always panic
impl<Output: Tuple + 'static, T: Copy + 'static> Default for DynBoxSliceCopied<Output, T> {
    fn default() -> Self {
        Self::new(Panic::new())
    }
}

impl<'a, Output, T> Parser<std::iter::Copied<std::slice::Iter<'a, T>>>
    for DynBoxSliceCopied<Output, T>
where
    Output: Tuple,
    T: Copy,
{
    type Output = Output;

    fn parse(
        &self,
        it: std::iter::Copied<std::slice::Iter<'a, T>>,
    ) -> ParseResult<Self::Output, std::iter::Copied<std::slice::Iter<'a, T>>> {
        self.parser.parse(it)
    }

    fn match_pattern(
        &self,
        it: std::iter::Copied<std::slice::Iter<'a, T>>,
    ) -> ParseResult<(), std::iter::Copied<std::slice::Iter<'a, T>>> {
        self.parser.match_pattern(it)
    }
}

impl<Output, T> Deref for DynBoxSliceCopied<Output, T>
where
    Output: Tuple,
    T: Copy,
{
    type Target = std::boxed::Box<
        dyn for<'a> Parser<std::iter::Copied<std::slice::Iter<'a, T>>, Output = Output>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}
impl<Output, T> DerefMut for DynBoxSliceCopied<Output, T>
where
    Output: Tuple,
    T: Copy,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}
impl<Output, T> IntoParser for DynBoxSliceCopied<Output, T>
where
    Output: Tuple,
    T: Copy,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }

    // TODO no boxed here
}
