use std::iter::Iterator;

use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

use rusty_parser_derive::ParserHelper;

// Callback takes Parser's output as input;
// Callback function's return value would be new value of the parser

#[derive(Debug, Clone, ParserHelper)]
pub struct MapParser<ParserType, MapperType, MapOutput, It>
where
    It: Iterator + Clone,
    ParserType: Parser<It>,
    MapperType: Fn(<ParserType as Parser<It>>::Output) -> MapOutput,
    MapOutput: Tuple,
{
    parser: ParserType,
    callback: MapperType,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserType, MapperType, MapOutput, It> MapParser<ParserType, MapperType, MapOutput, It>
where
    It: Iterator + Clone,
    ParserType: Parser<It>,
    MapperType: Fn(<ParserType as Parser<It>>::Output) -> MapOutput,
    MapOutput: Tuple,
{
    pub fn new(parser: ParserType, callback: MapperType) -> Self {
        Self {
            parser: parser,
            callback: callback,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserType, MapperType, MapOutput, It> Parser<It>
    for MapParser<ParserType, MapperType, MapOutput, It>
where
    It: Iterator + Clone,
    ParserType: Parser<It>,
    MapperType: Fn(<ParserType as Parser<It>>::Output) -> MapOutput,
    MapOutput: Tuple,
{
    type Output = MapOutput;

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let callback_parser = MapParser::new(digit_parser, |val| -> (i32,) { (val.0 as i32,) });

        let str = "123hello";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1' as i32,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "23hello");
    }
    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let callback_parser = MapParser::new(digit_parser, |val| -> (i32,) { (val.0 as i32,) });

        let str = "a23hello";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a23hello");
    }
}
