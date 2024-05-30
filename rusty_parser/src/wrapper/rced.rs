use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

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
    pub fn clone(&self) -> Self {
        Self {
            parser: Rc::clone(&self.parser),
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
        self.parser.parse(it)
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        self.parser.match_pattern(it)
    }
}

impl<ParserType, It> Deref for RcedParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    type Target = Rc<ParserType>;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}
impl<ParserType, It> DerefMut for RcedParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}

pub fn rc<ParserType, It>(parser: ParserType) -> RcedParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    RcedParser::new(parser)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        leaf::{singleeq::SingleEqualParser, singlerange::SingleRangeParser},
        wrapper::{boxed::BoxedParser, refcelled::RefCelledParser},
    };

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_boxed: BoxedParser<dyn Parser<std::str::Chars<'_>, Output = (char,)>> =
            BoxedParser::new(Box::new(digit_parser));
        let digit_refcelled = RefCelledParser::new(digit_boxed);

        let a_parser = SingleEqualParser::new('a');

        // let 2 parsers point to the same parser
        let rced1 = RcedParser::new(digit_refcelled);
        let rced2 = RcedParser::clone(&rced1);

        let str = "123456abcd";

        let res = rced1.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "23456abcd");

        // now change rcde1 to a_parser
        rced1.borrow_mut().assign(Box::new(a_parser));

        // since rced1 and rced2 point to the same parser, rced2 should also be a_parser
        let res = rced2.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "123456abcd");
    }
}
