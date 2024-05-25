use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

// reference wrapper for Parser
#[derive(Debug, Clone)]
pub struct ReferenceParser<'a, ParserType, It>
where
    ParserType: 'a + Parser<It>,
    It: InputIteratorTrait,
{
    parser: &'a ParserType,
    _phantom: std::marker::PhantomData<It>,
}

impl<'a, ParserType, It> ReferenceParser<'a, ParserType, It>
where
    ParserType: 'a + Parser<It>,
    It: InputIteratorTrait,
{
    pub fn new(parser: &'a ParserType) -> Self {
        Self {
            parser: parser,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, ParserType, It> Parser<It> for ReferenceParser<'a, ParserType, It>
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::singlerange::SingleRangeParser;

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
