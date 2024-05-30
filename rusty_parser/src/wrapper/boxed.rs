use std::boxed::Box;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

impl<ParserType, It> Parser<It> for Box<ParserType>
where
    ParserType: Parser<It>,
    It: InputIteratorTrait,
{
    type Output = ParserType::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        self.as_ref().parse(it)
    }

    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        self.as_ref().match_pattern(it)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singleeq::SingleEqualParser;
    use crate::leaf::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let a_parser = SingleEqualParser::new('a');

        let str = "1a2b3c4d5e6f7g8h9i0j";
        let mut boxed: Box<dyn Parser<_, Output = (char,)>> = Box::new(digit_parser);
        let res = boxed.parse(str.chars());
        let rest: String = res.it.clone().collect();
        assert_eq!(res.output, Some(('1',)));
        assert_eq!(rest, "a2b3c4d5e6f7g8h9i0j");

        // set another parser to same variable
        boxed = Box::new(a_parser);
        let res = boxed.parse(res.it);
        let rest: String = res.it.collect();
        assert_eq!(res.output, Some(('a',)));
        assert_eq!(rest, "2b3c4d5e6f7g8h9i0j");
    }
}
