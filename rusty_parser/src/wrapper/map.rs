use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;
use crate::wrapper::tupleunpack::TupleUnpack;

// Callback takes Parser's output as input;
// Callback function's return value would be new value of the parser

#[derive(Debug, Clone, Copy)]
pub struct MapParser<ParserType, ClosureType> {
    parser: ParserType,
    callback: ClosureType,
}

impl<ParserType, ClosureType> MapParser<ParserType, ClosureType> {
    pub fn new(parser: ParserType, callback: ClosureType) -> Self {
        Self { parser, callback }
    }
}

impl<ParserType, ClosureType, It> Parser<It> for MapParser<ParserType, ClosureType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
    ClosureType: TupleUnpack<<ParserType as Parser<It>>::Output>,
    <ParserType as Parser<It>>::Output: Tuple,
    ClosureType::Output: Tuple,
{
    type Output = ClosureType::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let res = self.parser.parse(it);
        if let Some(val) = res.output {
            let callback_res = (self.callback).map(val);
            ParseResult {
                output: Some(callback_res),
                it: res.it,
            }
        } else {
            ParseResult {
                output: None,
                it: res.it,
            }
        }
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        self.parser.match_pattern(it)
    }
}

impl<ParserType, ClosureType> IntoParser for MapParser<ParserType, ClosureType> {
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let callback_parser = MapParser::new(digit_parser, |val: char| -> i32 { val as i32 });

        let str = "123hello";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1' as i32,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "23hello");
    }
    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let callback_parser = MapParser::new(digit_parser, |val: char| -> i32 { val as i32 });

        let str = "a23hello";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a23hello");
    }
}
