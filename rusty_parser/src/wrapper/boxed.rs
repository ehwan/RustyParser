use std::boxed::Box;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

pub struct BoxedParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It> + ?Sized,
{
    parser: std::boxed::Box<ParserType>,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserType, It> BoxedParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It> + ?Sized,
{
    pub fn new(parser: Box<ParserType>) -> Self {
        Self {
            parser: parser,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<ParserType, It> Parser<It> for BoxedParser<ParserType, It>
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

impl<ParserType, It> Deref for BoxedParser<ParserType, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    type Target = ParserType;

    fn deref(&self) -> &Self::Target {
        &self.parser
    }
}
impl<ParserType, It> DerefMut for BoxedParser<ParserType, It>
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
    use crate::leaf::singleeq::SingleEqualParser;
    use crate::leaf::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let a_parser = SingleEqualParser::new('a');

        let str = "1a2b3c4d5e6f7g8h9i0j";
        let mut boxed: BoxedParser<
            dyn Parser<std::str::Chars<'_>, Output = (char,)>,
            std::str::Chars<'_>,
        > = BoxedParser::new(Box::new(digit_parser));
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
