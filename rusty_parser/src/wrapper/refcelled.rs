use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

// RefCell<Parser> wrapper
// this can be combined with BoxedParser, a Box<Parser> wrapper
// for dynamic parser changes
#[derive(Debug, Clone)]
pub struct RefCelledParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    parser: RefCell<ParserType>,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserType, It> RefCelledParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    pub fn new(parser: ParserType) -> Self {
        Self {
            parser: RefCell::new(parser),
            _phantom: std::marker::PhantomData,
        }
    }
    // get &RefCell<ChildParser>
    pub fn refcelled_parser(&self) -> &RefCell<ParserType> {
        &self.parser
    }
}

impl<ParserType, It> Parser<It> for RefCelledParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    type Output = <ParserType as Parser<It>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        self.parser.borrow().parse(it)
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        self.parser.borrow().match_pattern(it)
    }
}

impl<ParserType, It> Deref for RefCelledParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    type Target = RefCell<ParserType>;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}
impl<ParserType, It> DerefMut for RefCelledParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{leaf::singlerange::SingleRangeParser, wrapper::boxed::BoxedParser};

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let boxed = BoxedParser::new(digit_parser);
        let refed = RefCelledParser::new(boxed);

        let str = "123456abcd";

        let res = refed.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "23456abcd");

        *(refed.parser.borrow_mut()) = BoxedParser::new(SingleRangeParser::new('a'..='z'));
        let res = refed.parse(res.it);
        assert_eq!(res.output, None);
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "23456abcd");
    }
}
