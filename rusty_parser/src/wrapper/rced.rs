use std::rc::Rc;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

impl<ParserType, It> Parser<It> for Rc<ParserType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    type Output = <ParserType as Parser<It>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        self.as_ref().parse(it)
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        self.as_ref().match_pattern(it)
    }
}

impl<ParserType> IntoParser for Rc<ParserType> {
    type Into = Self;

    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;

    use super::*;
    use crate::{
        leaf::{singleeq::SingleEqualParser, singlerange::SingleRangeParser},
        wrapper::boxed::chars::DynBoxChars,
    };

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_boxed: DynBoxChars<(char,)> = DynBoxChars::new(digit_parser);
        let digit_refcelled = RefCell::new(digit_boxed);

        let a_parser = SingleEqualParser::new('a');

        // let 2 parsers point to the same parser
        let rc1 = Rc::new(digit_refcelled);
        let rc2 = Rc::clone(&rc1);

        let str = "123456abcd";

        let res = rc1.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "23456abcd");

        // now change rcde1 to a_parser
        rc1.borrow_mut().assign(a_parser);

        // since rced1 and rced2 point to the same parser, rced2 should also be a_parser
        let res = rc2.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "123456abcd");
    }
}
