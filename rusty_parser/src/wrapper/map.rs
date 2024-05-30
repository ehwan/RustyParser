use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

// Callback takes Parser's output as input;
// Callback function's return value would be new value of the parser

#[derive(Debug, Clone, Copy)]
pub struct MapParser<ParserType, ClosureType> {
    parser: ParserType,
    callback: ClosureType,
}

impl<ParserType, ClosureType> MapParser<ParserType, ClosureType> {
    pub fn new(parser: ParserType, callback: ClosureType) -> Self {
        Self {
            parser: parser,
            callback: callback,
        }
    }
}

impl<ParserType, ClosureType, ClosureOutput, It> Parser<It> for MapParser<ParserType, ClosureType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
    ClosureType: Fn(<ParserType as Parser<It>>::Output) -> ClosureOutput,
    ClosureOutput: Tuple,
{
    type Output = ClosureOutput;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let res = self.parser.parse(it);
        if let Some(val) = res.output {
            let callback_res = (self.callback)(val);
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
        let res = self.parser.match_pattern(it);
        if let Some(_) = res.output {
            ParseResult {
                output: Some(()),
                it: res.it,
            }
        } else {
            ParseResult {
                output: None,
                it: res.it,
            }
        }
    }
}

pub fn map<ParserType, ClosureType, ClosureOutput, It>(
    parser: ParserType,
    callback: ClosureType,
) -> MapParser<ParserType, ClosureType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
    ClosureType: Fn(<ParserType as Parser<It>>::Output) -> ClosureOutput,
    ClosureOutput: Tuple,
{
    MapParser::new(parser, callback)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let callback_parser =
            MapParser::new(digit_parser, |val: (char,)| -> (i32,) { (val.0 as i32,) });

        let str = "123hello";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1' as i32,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "23hello");
    }
    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let callback_parser =
            MapParser::new(digit_parser, |val: (char,)| -> (i32,) { (val.0 as i32,) });

        let str = "a23hello";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a23hello");
    }
}
