use crate::core::into_parser::IntoParser;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct StrParser<ParserType> {
    parser: ParserType,
}

impl<ParserType> StrParser<ParserType> {
    pub fn new(parser: ParserType) -> Self {
        Self { parser: parser }
    }
}

impl<'a, ParserType> Parser<std::str::Chars<'a>> for StrParser<ParserType>
where
    ParserType: Parser<std::str::Chars<'a>>,
{
    type Output = (&'a str,);

    fn parse(&self, it: std::str::Chars<'a>) -> ParseResult<Self::Output, std::str::Chars<'a>> {
        let i0 = it.clone();
        let res = self.parser.match_pattern(it);
        if let Some(_) = res.output {
            // this is length of bytes
            let len = i0.as_str().len() - res.it.as_str().len();
            ParseResult {
                // and this is byte slice casted to str
                output: Some((&i0.as_str()[..len],)),
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

impl<ParserType> IntoParser for StrParser<ParserType> {
    type Into = StrParser<ParserType>;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SliceParser<ParserType> {
    parser: ParserType,
}

impl<ParserType> SliceParser<ParserType> {
    pub fn new(parser: ParserType) -> Self {
        Self { parser: parser }
    }
}
impl<'a, T, ParserType> Parser<std::slice::Iter<'a, T>> for SliceParser<ParserType>
where
    ParserType: Parser<std::slice::Iter<'a, T>>,
{
    type Output = (&'a [T],);

    fn parse(
        &self,
        it: std::slice::Iter<'a, T>,
    ) -> ParseResult<Self::Output, std::slice::Iter<'a, T>> {
        let i0 = it.clone();
        let res = self.parser.match_pattern(it);
        if let Some(_) = res.output {
            let len = i0.len() - res.it.len();
            ParseResult {
                // and this is byte slice casted to str
                output: Some((&i0.as_slice()[..len],)),
                it: res.it,
            }
        } else {
            ParseResult {
                output: None,
                it: res.it,
            }
        }
    }
    fn match_pattern(
        &self,
        it: std::slice::Iter<'a, T>,
    ) -> ParseResult<(), std::slice::Iter<'a, T>> {
        self.parser.match_pattern(it)
    }
}

impl<ParserType> IntoParser for SliceParser<ParserType> {
    type Into = SliceParser<ParserType>;
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
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_parser = SeqParser::new(digit_parser.clone(), digit_parser);
        let digit_parser = StrParser::new(digit_parser);

        let str = "123456abcd";
        let res = digit_parser.parse(str.chars());
        assert_eq!(res.output.unwrap(), ("12",));
        assert_eq!(res.it.collect::<String>(), "3456abcd");
    }
    #[test]
    fn fail() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_parser = SeqParser::new(digit_parser.clone(), digit_parser);
        let digit_parser = StrParser::new(digit_parser);

        let str = "ab3456abcd";
        let res = digit_parser.parse(str.chars());
        assert_eq!(res.output, None);
        assert_eq!(res.it.collect::<String>(), "ab3456abcd");
    }
}