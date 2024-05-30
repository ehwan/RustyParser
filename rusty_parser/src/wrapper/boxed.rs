use std::ops::Deref;
use std::ops::DerefMut;

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
    pub fn new<ParserType>(parser: ParserType) -> Self
    where
        ParserType: for<'a> Parser<std::str::Chars<'a>, Output = Output> + 'static,
    {
        Self {
            parser: std::boxed::Box::new(parser),
        }
    }
    pub fn assign<ParserType>(&mut self, parser: ParserType)
    where
        ParserType: for<'a> Parser<std::str::Chars<'a>, Output = Output> + 'static,
    {
        self.parser = std::boxed::Box::new(parser);
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

pub struct DynBoxSlice<Output, T>
where
    Output: Tuple,
{
    parser: std::boxed::Box<dyn for<'a> Parser<std::slice::Iter<'a, T>, Output = Output>>,
}

impl<Output, T> DynBoxSlice<Output, T>
where
    Output: Tuple,
{
    pub fn new<ParserType>(parser: ParserType) -> Self
    where
        ParserType: for<'a> Parser<std::slice::Iter<'a, T>, Output = Output> + 'static,
    {
        Self {
            parser: std::boxed::Box::new(parser),
        }
    }
    pub fn assign<ParserType>(&mut self, parser: ParserType)
    where
        ParserType: for<'a> Parser<std::slice::Iter<'a, T>, Output = Output> + 'static,
    {
        self.parser = std::boxed::Box::new(parser);
    }
}

impl<'a, Output, T> Parser<std::slice::Iter<'a, T>> for DynBoxSlice<Output, T>
where
    Output: Tuple,
{
    type Output = Output;

    fn parse(
        &self,
        it: std::slice::Iter<'a, T>,
    ) -> ParseResult<Self::Output, std::slice::Iter<'a, T>> {
        self.parser.parse(it)
    }

    fn match_pattern(
        &self,
        it: std::slice::Iter<'a, T>,
    ) -> ParseResult<(), std::slice::Iter<'a, T>> {
        self.parser.match_pattern(it)
    }
}

impl<Output, T> Deref for DynBoxSlice<Output, T>
where
    Output: Tuple,
{
    type Target = std::boxed::Box<dyn for<'a> Parser<std::slice::Iter<'a, T>, Output = Output>>;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}
impl<Output, T> DerefMut for DynBoxSlice<Output, T>
where
    Output: Tuple,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}

pub fn box_chars<ParserType, Output>(parser: ParserType) -> DynBoxChars<Output>
where
    Output: Tuple,
    ParserType: for<'a> Parser<std::str::Chars<'a>, Output = Output> + 'static,
{
    DynBoxChars::new(parser)
}
pub fn box_slice<ParserType, Output, T>(parser: ParserType) -> DynBoxSlice<Output, T>
where
    Output: Tuple,
    ParserType: for<'a> Parser<std::slice::Iter<'a, T>, Output = Output> + 'static,
{
    DynBoxSlice::new(parser)
}

// pub fn box_chars<ParserType>(parser: ParserType) -> DynBoxChars<ParserType> {
//     BoxedParser::new(Box::new(parser))
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singleeq::SingleEqualParser;
    use crate::leaf::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
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
