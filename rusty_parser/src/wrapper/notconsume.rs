use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct NotConsumeParser<ParserType> {
    parser: ParserType,
}

impl<ParserType> NotConsumeParser<ParserType> {
    pub fn new(parser: ParserType) -> Self {
        Self { parser }
    }
}

impl<ParserType, It> Parser<It> for NotConsumeParser<ParserType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    type Output = <ParserType as Parser<It>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        if let Some(val) = self.parser.parse(it).output {
            ParseResult {
                output: Some(val),
                it: i0,
            }
        } else {
            ParseResult {
                output: None,
                it: i0,
            }
        }
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let i0 = it.clone();
        if self.parser.match_pattern(it).output.is_some() {
            ParseResult {
                output: Some(()),
                it: i0,
            }
        } else {
            ParseResult {
                output: None,
                it: i0,
            }
        }
    }
}

impl<ParserType> IntoParser for NotConsumeParser<ParserType> {
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
    fn success1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = NotConsumeParser::new(digit_parser);

        let str = "123456abcd";

        let res = digit_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "123456abcd");
    }

    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = NotConsumeParser::new(digit_parser);

        let str = "a123456abcd";

        let res = digit_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "a123456abcd");
    }
}
