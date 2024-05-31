use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

// reference wrapper for Parser
#[derive(Debug, Clone, Copy)]
pub struct ReferenceParser<'a, ParserType>
where
    ParserType: 'a,
{
    parser: &'a ParserType,
}

impl<'a, ParserType> ReferenceParser<'a, ParserType>
where
    ParserType: 'a,
{
    pub fn new(parser: &'a ParserType) -> Self {
        Self { parser: parser }
    }
}

impl<'a, ParserType, It> Parser<It> for ReferenceParser<'a, ParserType>
where
    ParserType: 'a + Parser<It>,
    It: InputIteratorTrait,
{
    type Output = <ParserType as Parser<It>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        self.parser.parse(it)
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        self.parser.match_pattern(it)
    }
}

impl<'a, ParserType> IntoParser for ReferenceParser<'a, ParserType>
where
    ParserType: 'a,
{
    type Into = ReferenceParser<'a, ParserType>;
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
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_parser = ReferenceParser::new(&digit_parser);

        let str = "123456abcd";
        let res = digit_parser.parse(str.chars());

        assert_eq!(res.output, Some(('1',)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "23456abcd");
    }
    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_parser = ReferenceParser::new(&digit_parser);

        let str = "a23456abcd";
        let res = digit_parser.parse(str.chars());

        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a23456abcd");
    }
}
