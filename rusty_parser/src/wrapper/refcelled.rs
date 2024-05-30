use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

pub struct RefCelledParser<ParserType> {
    parser: RefCell<ParserType>,
}

impl<ParserType> RefCelledParser<ParserType> {
    pub fn new(parser: ParserType) -> Self {
        Self {
            parser: RefCell::new(parser),
        }
    }
}

impl<ParserType, It> Parser<It> for RefCelledParser<ParserType>
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

impl<ParserType> Deref for RefCelledParser<ParserType> {
    type Target = RefCell<ParserType>;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}
impl<ParserType> DerefMut for RefCelledParser<ParserType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
    }
}

pub fn refcell<ParserType, It>(parser: ParserType) -> RefCelledParser<ParserType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    RefCelledParser::new(parser)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singlerange::SingleRangeParser;
    use crate::wrapper::boxed::BoxedParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let boxed: BoxedParser<dyn Parser<std::str::Chars<'_>, Output = (char,)>> =
            BoxedParser::new(Box::new(digit_parser));
        let refed = RefCelledParser::new(boxed);

        let str = "123456abcd";

        let res = refed.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "23456abcd");

        refed
            .borrow_mut()
            .assign(Box::new(SingleRangeParser::new('a'..='z')));
        let res = refed.parse(res.it);
        assert_eq!(res.output, None);
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "23456abcd");
    }
}
