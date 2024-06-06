use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct InspectParser<ParserType, Closure> {
    parser: ParserType,
    closure: Closure,
}

impl<ParserType, Closure> InspectParser<ParserType, Closure> {
    pub fn new(parser: ParserType, closure: Closure) -> Self {
        Self { parser, closure }
    }
}

impl<ParserType, Closure, It> Parser<It> for InspectParser<ParserType, Closure>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
    Closure: Fn(),
{
    type Output = <ParserType as Parser<It>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        (self.closure)();
        self.parser.parse(it)
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        (self.closure)();
        self.parser.match_pattern(it)
    }
}

impl<ParserType, ClosureType> IntoParser for InspectParser<ParserType, ClosureType> {
    type Into = Self;

    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::leaf::singlerange::SingleRangeParser;

    use super::*;

    #[test]
    fn success() {
        let parser = SingleRangeParser::from('0'..='9');
        let parser = InspectParser::new(parser, || {
            println!("Hello, world!");
        });
        let result = parser.parse("01234".chars());
        assert_eq!(result.output, Some(('0',)));
    }
}
