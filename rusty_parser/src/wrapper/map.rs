use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

// Callback takes Parser's output as input;
// Callback function's return value would be new value of the parser

#[derive(Debug)]
pub struct MapParser<ParserType, ClosureType, ClosureInput, ClosureOutput>
where
    ClosureInput: Tuple,
    ClosureType: Fn(ClosureInput) -> ClosureOutput,
    ClosureOutput: Tuple,
{
    parser: ParserType,
    callback: ClosureType,
    _phantom: std::marker::PhantomData<(ClosureInput, ClosureOutput)>,
}

impl<ParserType, ClosureType, ClosureInput, ClosureOutput> Clone
    for MapParser<ParserType, ClosureType, ClosureInput, ClosureOutput>
where
    ClosureInput: Tuple,
    ClosureType: Fn(ClosureInput) -> ClosureOutput + Clone,
    ClosureOutput: Tuple,
    ParserType: Clone,
{
    fn clone(&self) -> Self {
        Self {
            parser: self.parser.clone(),
            callback: self.callback.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserType, ClosureType, ClosureInput, ClosureOutput> Copy
    for MapParser<ParserType, ClosureType, ClosureInput, ClosureOutput>
where
    ClosureInput: Tuple,
    ClosureType: Fn(ClosureInput) -> ClosureOutput + Copy,
    ClosureOutput: Tuple,
    ParserType: Copy,
    std::marker::PhantomData<(ClosureInput, ClosureOutput)>: Copy,
{
}

impl<ParserType, ClosureType, ClosureInput, ClosureOutput>
    MapParser<ParserType, ClosureType, ClosureInput, ClosureOutput>
where
    ClosureInput: Tuple,
    ClosureType: Fn(ClosureInput) -> ClosureOutput,
    ClosureOutput: Tuple,
{
    pub fn new(parser: ParserType, callback: ClosureType) -> Self {
        Self {
            parser,
            callback,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserType, ClosureType, ClosureInput, ClosureOutput, It> Parser<It>
    for MapParser<ParserType, ClosureType, ClosureInput, ClosureOutput>
where
    It: InputIteratorTrait,
    ClosureInput: Tuple,
    ParserType: Parser<It, Output = ClosureInput>,
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
        self.parser.match_pattern(it)
    }
}

impl<ParserType, ClosureType, ClosureInput, ClosureOutput> IntoParser
    for MapParser<ParserType, ClosureType, ClosureInput, ClosureOutput>
where
    ClosureInput: Tuple,
    ClosureType: Fn(ClosureInput) -> ClosureOutput,
    ClosureOutput: Tuple,
{
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
        let digit_parser = SingleRangeParser::from('0'..='9');
        let callback_parser =
            MapParser::new(digit_parser, |val: (char,)| -> (i32,) { (val.0 as i32,) });

        let str = "a23hello";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a23hello");
    }
}
