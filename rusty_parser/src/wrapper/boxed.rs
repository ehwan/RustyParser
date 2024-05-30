use std::boxed::Box;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

pub struct BoxedParser<ParserType>
where
    ParserType: ?Sized,
{
    parser: std::boxed::Box<ParserType>,
}

impl<ParserType> BoxedParser<ParserType>
where
    ParserType: ?Sized,
{
    pub fn new(parser: Box<ParserType>) -> Self {
        Self { parser: parser }
    }

    pub fn assign(&mut self, parser: Box<ParserType>) {
        self.parser = parser;
    }
}
impl<ParserType, It> Parser<It> for BoxedParser<ParserType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It> + ?Sized,
{
    type Output = <ParserType as Parser<It>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        self.parser.parse(it)
    }

    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        self.parser.match_pattern(it)
    }
}

impl<ParserType> Deref for BoxedParser<ParserType> {
    type Target = Box<ParserType>;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}
impl<ParserType> DerefMut for BoxedParser<ParserType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parser
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
        let mut boxed: BoxedParser<dyn Parser<std::str::Chars<'_>, Output = (char,)>> =
            BoxedParser::new(Box::new(digit_parser));
        let res = boxed.parse(str.chars());
        let rest: String = res.it.clone().collect();
        assert_eq!(res.output, Some(('1',)));
        assert_eq!(rest, "a2b3c4d5e6f7g8h9i0j");

        // set another parser to same variable
        boxed.parser = Box::new(a_parser);
        let res = boxed.parse(res.it);
        let rest: String = res.it.collect();
        assert_eq!(res.output, Some(('a',)));
        assert_eq!(rest, "2b3c4d5e6f7g8h9i0j");
    }
}
