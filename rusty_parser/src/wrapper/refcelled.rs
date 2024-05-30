use std::cell::RefCell;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

impl<ParserType, It> Parser<It> for RefCell<ParserType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    type Output = <ParserType as Parser<It>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        self.borrow().parse(it)
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        self.borrow().match_pattern(it)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let boxed = Box::new(digit_parser);
        let refed = RefCell::new(boxed);

        let str = "123456abcd";

        let res = refed.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "23456abcd");

        *refed.borrow_mut() = Box::new(SingleRangeParser::new('a'..='z'));
        let res = refed.parse(res.it);
        assert_eq!(res.output, None);
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "23456abcd");
    }
}
