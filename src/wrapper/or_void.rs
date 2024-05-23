use std::iter::Iterator;

use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultVoid;

#[derive(Debug, Clone)]
pub struct OrVoid<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: Parser<It, Output = ()> + ResultVoid<It>,
    ParserB: Parser<It, Output = ()> + ResultVoid<It>,
{
    parser_a: ParserA,
    parser_b: ParserB,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserA, ParserB, It> OrVoid<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: Parser<It, Output = ()> + ResultVoid<It>,
    ParserB: Parser<It, Output = ()> + ResultVoid<It>,
{
    pub fn new(parser_a: ParserA, parser_b: ParserB) -> Self {
        Self {
            parser_a: parser_a,
            parser_b: parser_b,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserA, ParserB, It> ResultVoid<It> for OrVoid<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: Parser<It, Output = ()> + ResultVoid<It>,
    ParserB: Parser<It, Output = ()> + ResultVoid<It>,
{
}

impl<ParserA, ParserB, It> Parser<It> for OrVoid<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: Parser<It, Output = ()> + ResultVoid<It>,
    ParserB: Parser<It, Output = ()> + ResultVoid<It>,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let res = self.parser_a.parse(it);
        if let Some(_) = res.output {
            return ParseResult {
                output: Some(()),
                it: res.it,
            };
        }
        let res = self.parser_b.parse(res.it);
        if let Some(_) = res.output {
            return ParseResult {
                output: Some(()),
                it: res.it,
            };
        }

        ParseResult {
            output: None,
            it: res.it,
        }
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let res = self.parser_a.match_pattern(it);
        if let Some(_) = res.output {
            return ParseResult {
                output: Some(()),
                it: res.it,
            };
        }
        let res = self.parser_b.match_pattern(res.it);
        if let Some(_) = res.output {
            return ParseResult {
                output: Some(()),
                it: res.it,
            };
        }

        ParseResult {
            output: None,
            it: res.it,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::stringeq::StringEqualParser;
    #[test]
    fn success_test() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let world_parser = StringEqualParser::new("world".chars());
        let or_parser = OrVoid::new(hello_parser, world_parser);

        let str = "helloworldabcd";

        let res = or_parser.parse(str.chars());
        assert_eq!(res.output, Some(()));
        let res = or_parser.parse(res.it);
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }

    #[test]
    fn fail_test1() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let world_parser = StringEqualParser::new("world".chars());
        let or_parser = OrVoid::new(hello_parser, world_parser);

        let str = "hellaworldabcd";

        let res = or_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "hellaworldabcd");
    }

    #[test]
    fn fail_test2() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let world_parser = StringEqualParser::new("world".chars());
        let or_parser = OrVoid::new(hello_parser, world_parser);

        let str = "helloworlxabcd";

        let res = or_parser.parse(str.chars());
        assert_eq!(res.output, Some(()));
        let res = or_parser.parse(res.it);
        let rest: String = res.it.collect();
        assert_eq!(rest, "worlxabcd");
    }
}
