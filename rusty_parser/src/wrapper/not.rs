use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct NotParser<ParserA, ParserB> {
    parser_a: ParserA,
    parser_b: ParserB,
}

impl<ParserA, ParserB> NotParser<ParserA, ParserB> {
    pub fn new(parser_a: ParserA, parser_b: ParserB) -> Self {
        Self { parser_a, parser_b }
    }
}

impl<ParserA, ParserB, It> Parser<It> for NotParser<ParserA, ParserB>
where
    It: InputIteratorTrait,
    ParserA: Parser<It>,
    ParserB: Parser<It>,
{
    type Output = <ParserA as Parser<It>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        if self.parser_b.match_pattern(it).output.is_some() {
            ParseResult {
                output: None,
                it: i0,
            }
        } else {
            self.parser_a.parse(i0)
        }
    }

    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let i0 = it.clone();
        if self.parser_b.match_pattern(it).output.is_some() {
            ParseResult {
                output: None,
                it: i0,
            }
        } else {
            self.parser_a.match_pattern(i0)
        }
    }
}

impl<ParserA, ParserB> IntoParser for NotParser<ParserA, ParserB> {
    type Into = NotParser<ParserA, ParserB>;

    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::{singleeq::SingleEqualParser, singlerange::SingleRangeParser};
    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = NotParser::new(digit_parser, SingleEqualParser::new('4'));

        let res = digit_parser.parse("3a".chars());
        assert_eq!(res.output, Some(('3',)));
        assert_eq!(res.it.collect::<String>(), "a");
    }
    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = NotParser::new(digit_parser, SingleEqualParser::new('4'));

        let res = digit_parser.parse("4a".chars());
        assert_eq!(res.output, None);
        assert_eq!(res.it.collect::<String>(), "4a");
    }
}
