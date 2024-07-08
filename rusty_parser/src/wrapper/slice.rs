use crate::core::into_parser::IntoParser;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;
use crate::InputIteratorTrait;

use super::tupleutils::concat::AppendTupleToTuple;

#[derive(Debug, Clone, Copy)]
pub struct StringParser<ParserType> {
    parser: ParserType,
}

impl<ParserType> StringParser<ParserType> {
    pub fn new(parser: ParserType) -> Self {
        Self { parser }
    }
}

impl<'a, ParserType> Parser<std::str::Chars<'a>> for StringParser<ParserType>
where
    ParserType: Parser<std::str::Chars<'a>>,
{
    type Output = (String,);

    fn parse(&self, it: std::str::Chars<'a>) -> ParseResult<Self::Output, std::str::Chars<'a>> {
        let i0 = it.clone();
        let res = self.parser.match_pattern(it);
        if res.output.is_some() {
            // this is length in bytes
            let len = i0.as_str().len() - res.it.as_str().len();
            ParseResult {
                // and this is byte slice casted to str
                output: Some((String::from(&i0.as_str()[..len]),)),
                it: res.it,
            }
        } else {
            ParseResult {
                output: None,
                it: res.it,
            }
        }
    }
    fn match_pattern(&self, it: std::str::Chars<'a>) -> ParseResult<(), std::str::Chars<'a>> {
        self.parser.match_pattern(it)
    }
}

impl<ParserType> IntoParser for StringParser<ParserType> {
    type Into = StringParser<ParserType>;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnumerateParser<ParserType> {
    parser: ParserType,
}

impl<ParserType> EnumerateParser<ParserType> {
    pub fn new(parser: ParserType) -> Self {
        Self { parser }
    }
}

impl<ParserType, It> Parser<It> for EnumerateParser<ParserType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
    (It, It): AppendTupleToTuple<ParserType::Output>,
    <(It, It) as AppendTupleToTuple<ParserType::Output>>::Output: Tuple,
{
    type Output = <(It, It) as AppendTupleToTuple<ParserType::Output>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let res = self.parser.parse(it);
        if let Some(val) = res.output {
            // this is length in bytes
            ParseResult {
                output: Some((i0, res.it.clone()).append_back(val)),
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

impl<ParserType> IntoParser for EnumerateParser<ParserType> {
    type Into = EnumerateParser<ParserType>;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VecParser<ParserType> {
    parser: ParserType,
}

impl<ParserType> VecParser<ParserType> {
    pub fn new(parser: ParserType) -> Self {
        Self { parser }
    }
}
impl<It, ParserType> Parser<It> for VecParser<ParserType>
where
    It: InputIteratorTrait + ExactSizeIterator,
    ParserType: Parser<It>,
{
    type Output = (Vec<<It as Iterator>::Item>,);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let res = self.parser.match_pattern(it);
        if res.output.is_some() {
            let len = i0.len() - res.it.len();
            ParseResult {
                output: Some((i0.take(len).collect(),)),
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

impl<ParserType> IntoParser for VecParser<ParserType> {
    type Into = VecParser<ParserType>;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{leaf::singlerange::SingleRangeParser, wrapper::seq::SeqParser};

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = SeqParser::new(digit_parser, digit_parser);
        let digit_parser = StringParser::new(digit_parser);

        let str = "123456abcd";
        let res = digit_parser.parse(str.chars());
        assert_eq!(res.output.unwrap(), ("12".to_string(),));
        assert_eq!(res.it.collect::<String>(), "3456abcd");
    }
    #[test]
    fn fail() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = SeqParser::new(digit_parser, digit_parser);
        let digit_parser = StringParser::new(digit_parser);

        let str = "ab3456abcd";
        let res = digit_parser.parse(str.chars());
        assert_eq!(res.output, None);
        assert_eq!(res.it.collect::<String>(), "ab3456abcd");
    }
}
