use std::rc::Rc;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

// Rc<Parser> wrapper
#[derive(Debug, Clone)]
pub struct RcedParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    parser: Rc<ParserType>,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserType, It> RcedParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    pub fn new(parser: ParserType) -> Self {
        Self {
            parser: Rc::new(parser),
            _phantom: std::marker::PhantomData,
        }
    }
    // get &Rc<ChildParser>
    pub fn rced_parser(&self) -> &Rc<ParserType> {
        &self.parser
    }
    pub fn clone(from: &Self) -> Self {
        Self {
            parser: Rc::clone(&from.parser),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserType, It> Parser<It> for RcedParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    type Output = <ParserType as Parser<It>>::Output;

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
    use crate::{
        core::{singleeq::SingleEqualParser, singlerange::SingleRangeParser},
        wrapper::{boxed::BoxedParser, refcelled::RefCelledParser},
    };

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_boxed = BoxedParser::new(digit_parser);
        let digit_refcelled = RefCelledParser::new(digit_boxed);

        let a_parser = SingleEqualParser::new('a');
        let a_boxed = BoxedParser::new(a_parser);

        // let 2 parsers point to the same parser
        let rced1 = RcedParser::new(digit_refcelled);
        let rced2 = RcedParser::clone(&rced1);

        let str = "123456abcd";

        let res = rced1.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "23456abcd");

        // now change rcde1 to a_parser
        *(rced1.rced_parser().refcelled_parser().borrow_mut()) = a_boxed;
        //           ^            ^                  ^
        //           |            |                  |
        //           |            |              Box<Parser>
        //           |     &RefCell<Box<Parser>>
        //      &Rc<RefCell<Box<Parser>>>

        // since rced1 and rced2 point to the same parser, rced2 should also be a_parser
        let res = rced2.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "123456abcd");
    }
}
