use std::ops::Deref;
use std::ops::DerefMut;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

pub struct RcedParser<ParserType> {
    parser: std::rc::Rc<ParserType>,
}

impl<ParserType> RcedParser<ParserType> {
    pub fn new(parser: ParserType) -> Self {
        Self {
            parser: std::rc::Rc::new(parser),
        }
    }
    pub fn clone(&self) -> Self {
        Self {
            parser: std::rc::Rc::clone(&self.parser),
        }
    }
}

impl<ParserType, It> Parser<It> for RcedParser<ParserType>
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

impl<ParserType> Deref for RcedParser<ParserType> {
    type Target = std::rc::Rc<ParserType>;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}
impl<ParserType> DerefMut for RcedParser<ParserType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}

pub fn rc<ParserType: IntoParser>(parser: ParserType) -> RcedParser<ParserType::Into> {
    RcedParser::new(parser.into_parser())
}

impl<ParserType> IntoParser for RcedParser<ParserType> {
    type Into = Self;

    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        leaf::{singleeq::SingleEqualParser, singlerange::SingleRangeParser},
        wrapper::boxed::DynBoxChars,
        wrapper::refcelled::RefCelledParser,
    };

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_boxed: DynBoxChars<(char,)> = DynBoxChars::new(digit_parser);
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
        rced1.borrow_mut().assign(a_parser);

        // since rced1 and rced2 point to the same parser, rced2 should also be a_parser
        let res = rced2.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "123456abcd");
    }
}
