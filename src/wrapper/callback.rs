use std::iter::Iterator;

use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultValue;

// Callback function's return value would be new value of the parser
// Note that ResultType will be fixed to ResultValue, even function returns Tuple
// Note that Callback takes Parser's output as input;
// for ResultVoid Parser, must take () as input to Callback

#[derive(Debug, Clone)]
pub struct CallbackParser<ParserType, CallbackType, CallbackOutput, It>
where
    It: Iterator + Clone,
    ParserType: Parser<It>,
    CallbackType: Fn(<ParserType as Parser<It>>::Output) -> Option<CallbackOutput>,
{
    parser: ParserType,
    callback: CallbackType,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserType, CallbackType, CallbackOutput, It>
    CallbackParser<ParserType, CallbackType, CallbackOutput, It>
where
    It: Iterator + Clone,
    ParserType: Parser<It>,
    CallbackType: Fn(<ParserType as Parser<It>>::Output) -> Option<CallbackOutput>,
{
    pub fn new(parser: ParserType, callback: CallbackType) -> Self {
        Self {
            parser: parser,
            callback: callback,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserType, CallbackType, CallbackOutput, It> ResultValue<It>
    for CallbackParser<ParserType, CallbackType, CallbackOutput, It>
where
    It: Iterator + Clone,
    ParserType: Parser<It>,
    CallbackType: Fn(<ParserType as Parser<It>>::Output) -> Option<CallbackOutput>,
{
}

impl<ParserType, CallbackType, CallbackOutput, It> Parser<It>
    for CallbackParser<ParserType, CallbackType, CallbackOutput, It>
where
    It: Iterator + Clone,
    ParserType: Parser<It>,
    CallbackType: Fn(<ParserType as Parser<It>>::Output) -> Option<CallbackOutput>,
{
    type Output = CallbackOutput;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let res = self.parser.parse(it);
        if let Some(val) = res.output {
            if let Some(newval) = (self.callback)(val) {
                ParseResult {
                    output: Some(newval),
                    // output: Some(newval),
                    it: res.it,
                }
            } else {
                ParseResult {
                    output: None,
                    it: i0,
                }
            }
        } else {
            ParseResult {
                output: None,
                it: i0,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::singlerange::SingleRangeParser;

    #[test]
    fn success_test() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let callback_parser =
            CallbackParser::new(digit_parser, |val| -> Option<i32> { Some(val as i32) });

        let str = "123hello";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, Some('1' as i32));
        let rest: String = res.it.collect();
        assert_eq!(rest, "23hello");
    }
    #[test]
    fn success_fail1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let callback_parser =
            CallbackParser::new(digit_parser, |val| -> Option<i32> { Some(val as i32) });

        let str = "a23hello";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a23hello");
    }
    #[test]
    fn success_none1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let callback_parser = CallbackParser::new(digit_parser, |_val| -> Option<i32> { None });

        let str = "123hello";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "123hello");
    }
    #[test]
    fn success_none2() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let callback_parser = CallbackParser::new(digit_parser, |_val| -> Option<i32> { None });

        let str = "a23hello";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a23hello");
    }
}
