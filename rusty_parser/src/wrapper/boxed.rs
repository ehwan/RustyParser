use std::ops::Deref;
use std::ops::DerefMut;

use crate::core::into_parser::IntoParser;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

pub struct DynBoxChars<Output>
where
    Output: Tuple,
{
    parser: std::boxed::Box<dyn for<'a> Parser<std::str::Chars<'a>, Output = Output>>,
}

impl<Output> DynBoxChars<Output>
where
    Output: Tuple,
{
    pub fn new<ParserType: IntoParser>(parser: ParserType) -> Self
    where
        ParserType::Into: for<'a> Parser<std::str::Chars<'a>, Output = Output> + 'static,
    {
        Self {
            parser: std::boxed::Box::new(parser.into_parser()),
        }
    }
    pub fn assign<ParserType: IntoParser>(&mut self, parser: ParserType)
    where
        ParserType::Into: for<'a> Parser<std::str::Chars<'a>, Output = Output> + 'static,
    {
        self.parser = std::boxed::Box::new(parser.into_parser());
    }
}

impl<'a, Output> Parser<std::str::Chars<'a>> for DynBoxChars<Output>
where
    Output: Tuple,
{
    type Output = Output;

    fn parse(&self, it: std::str::Chars<'a>) -> ParseResult<Self::Output, std::str::Chars<'a>> {
        self.parser.parse(it)
    }

    fn match_pattern(&self, it: std::str::Chars<'a>) -> ParseResult<(), std::str::Chars<'a>> {
        self.parser.match_pattern(it)
    }
}

impl<Output> Deref for DynBoxChars<Output>
where
    Output: Tuple,
{
    type Target = std::boxed::Box<dyn for<'a> Parser<std::str::Chars<'a>, Output = Output>>;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}
impl<Output> DerefMut for DynBoxChars<Output>
where
    Output: Tuple,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}
impl<Output> IntoParser for DynBoxChars<Output>
where
    Output: Tuple,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

pub struct DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone + Copy,
{
    parser: std::boxed::Box<
        dyn for<'a> Parser<std::iter::Copied<std::slice::Iter<'a, T>>, Output = Output>,
    >,
}

impl<Output, T> DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone + Copy,
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

impl<'a, Output, T> Parser<std::iter::Copied<std::slice::Iter<'a, T>>> for DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone + Copy,
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

impl<Output, T> Deref for DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone + Copy,
{
    type Target = std::boxed::Box<
        dyn for<'a> Parser<std::iter::Copied<std::slice::Iter<'a, T>>, Output = Output>,
    >;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}
impl<Output, T> DerefMut for DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone + Copy,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}
impl<Output, T> IntoParser for DynBoxSlice<Output, T>
where
    Output: Tuple,
    T: Clone + Copy,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singleeq::SingleEqualParser;
    use crate::leaf::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let a_parser = SingleEqualParser::new('a');

        let str = "1a2b3c4d5e6f7g8h9i0j";
        let mut boxed: DynBoxChars<(char,)> = DynBoxChars::new(digit_parser);
        let res = boxed.parse(str.chars());
        let rest: String = res.it.clone().collect();
        assert_eq!(res.output, Some(('1',)));
        assert_eq!(rest, "a2b3c4d5e6f7g8h9i0j");

        // set another parser to same variable
        boxed.assign(a_parser);
        let res = boxed.parse(res.it);
        let rest: String = res.it.collect();
        assert_eq!(res.output, Some(('a',)));
        assert_eq!(rest, "2b3c4d5e6f7g8h9i0j");
    }
}
