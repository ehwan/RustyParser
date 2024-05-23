use std::iter::Iterator;

use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultValue;
use crate::core::traits::ResultVoid;

// Callback function's return value would be new value of the parser
// Note that ResultType will be fixed to ResultValue, even function returns Tuple or Void
// Callback takes Parser's output as input;

#[derive(Debug, Clone)]
pub struct CallbackVoidParser<ParserType, CallbackType, CallbackOutput, It>
where
    It: Iterator + Clone,
    ParserType: Parser<It> + ResultVoid<It>,
    CallbackType: Fn() -> Option<CallbackOutput>,
{
    parser: ParserType,
    callback: CallbackType,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserType, CallbackType, CallbackOutput, It>
    CallbackVoidParser<ParserType, CallbackType, CallbackOutput, It>
where
    It: Iterator + Clone,
    ParserType: Parser<It> + ResultVoid<It>,
    CallbackType: Fn() -> Option<CallbackOutput>,
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
    for CallbackVoidParser<ParserType, CallbackType, CallbackOutput, It>
where
    It: Iterator + Clone,
    ParserType: Parser<It> + ResultVoid<It>,
    CallbackType: Fn() -> Option<CallbackOutput>,
{
}

impl<ParserType, CallbackType, CallbackOutput, It> Parser<It>
    for CallbackVoidParser<ParserType, CallbackType, CallbackOutput, It>
where
    It: Iterator + Clone,
    ParserType: Parser<It> + ResultVoid<It>,
    CallbackType: Fn() -> Option<CallbackOutput>,
{
    type Output = CallbackOutput;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let res = self.parser.parse(it);
        if let Some(_) = res.output {
            if let Some(newval) = (self.callback)() {
                ParseResult {
                    output: Some(newval),
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
    use crate::core::stringeq::StringEqualParser;

    #[test]
    fn success_test() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let callback_parser = CallbackVoidParser::new(hello_parser, || -> Option<i32> { Some(0) });

        let str = "hello123";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, Some(0));
        let rest: String = res.it.collect();
        assert_eq!(rest, "123");
    }
    #[test]
    fn fail_test1() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let callback_parser = CallbackVoidParser::new(hello_parser, || -> Option<i32> { Some(0) });

        let str = "hella123";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "hella123");
    }
    #[test]
    fn fail_test2() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let callback_parser = CallbackVoidParser::new(hello_parser, || -> Option<i32> { None });

        let str = "hello123";

        let res = callback_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "hello123");
    }
}
