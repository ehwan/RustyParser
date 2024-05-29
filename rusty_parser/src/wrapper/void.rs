use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct VoidParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    parser: ParserType,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserType, It> VoidParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    pub fn new(parser: ParserType) -> Self {
        Self {
            parser: parser,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserType, It> Parser<It> for VoidParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let res = self.parser.match_pattern(it);
        if let Some(_) = res.output {
            ParseResult {
                output: Some(()),
                it: res.it,
            }
        } else {
            ParseResult {
                output: None,
                it: res.it,
            }
        }
    }
}

pub fn void_<ParserType, It>(parser: ParserType) -> VoidParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    VoidParser::new(parser)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_parser = VoidParser::new(digit_parser);

        let str = "123456abcd";
        let res = digit_parser.parse(str.chars());

        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(rest, "23456abcd");
    }
    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_parser = VoidParser::new(digit_parser);

        let str = "a23456abcd";
        let res = digit_parser.parse(str.chars());

        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a23456abcd");
    }
}
