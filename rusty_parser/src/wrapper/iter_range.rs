// Parer that captures [begin, end) range of input

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct IterParser<ParserType> {
    parser: ParserType,
}

impl<ParserType> IterParser<ParserType> {
    pub fn new(parser: ParserType) -> Self {
        Self { parser: parser }
    }
}

impl<ParserType, It> Parser<It> for IterParser<ParserType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    type Output = (It, It);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let res = self.parser.match_pattern(it);
        if let Some(_) = res.output {
            ParseResult {
                output: Some((i0, res.it.clone())),
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

pub fn iter<ParserType, It>(parser: ParserType) -> IterParser<ParserType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    IterParser::new(parser)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{leaf::singlerange::SingleRangeParser, wrapper::seq::SeqParser};

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_parser = SeqParser::new(digit_parser.clone(), digit_parser);
        let digit_parser = IterParser::new(digit_parser);

        let str = "123456abcd";
        let res = digit_parser.parse(str.chars());
        let (begin, end) = res.output.unwrap();
        assert_eq!(begin.collect::<String>(), "123456abcd");
        assert_eq!(end.collect::<String>(), "3456abcd");
    }
}
