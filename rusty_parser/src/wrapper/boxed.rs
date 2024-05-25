use std::boxed::Box;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

// Box< dyn Parser > wrapper for Parser
// this can take any Parser type with fixed Output
#[derive()]
pub struct BoxedParser<'a, Output, It>
where
    It: InputIteratorTrait,
    Output: Tuple,
{
    parser: Box<dyn Parser<It, Output = Output> + 'a>,
    _phantom: std::marker::PhantomData<It>,
}

impl<'a, Output, It> BoxedParser<'a, Output, It>
where
    It: InputIteratorTrait,
    Output: Tuple,
{
    pub fn new<ParserType>(parser: ParserType) -> Self
    where
        ParserType: Parser<It, Output = Output> + 'a,
    {
        Self {
            parser: Box::new(parser),
            _phantom: std::marker::PhantomData,
        }
    }

    // assign new parser
    pub fn assign<ParserType>(&mut self, parser: ParserType)
    where
        ParserType: Parser<It, Output = Output> + 'a,
    {
        self.parser = Box::new(parser);
    }
}

impl<'a, Output, It> Parser<It> for BoxedParser<'a, Output, It>
where
    It: InputIteratorTrait,
    Output: Tuple,
{
    type Output = Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        self.parser.as_ref().parse(it)
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        self.parser.as_ref().match_pattern(it)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::singleeq::SingleEqualParser;
    use crate::core::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let a_parser = SingleEqualParser::new('a');

        let str = "1a2b3c4d5e6f7g8h9i0j";

        let mut boxed_digit_parser = BoxedParser::new(digit_parser);
        let res = boxed_digit_parser.parse(str.chars());
        let rest: String = res.it.clone().collect();
        assert_eq!(res.output, Some(('1',)));
        assert_eq!(rest, "a2b3c4d5e6f7g8h9i0j");

        // set another parser to same variable
        boxed_digit_parser = BoxedParser::new(a_parser);
        let res = boxed_digit_parser.parse(res.it);
        let rest: String = res.it.collect();
        assert_eq!(res.output, Some(('a',)));
        assert_eq!(rest, "2b3c4d5e6f7g8h9i0j");
    }
}
