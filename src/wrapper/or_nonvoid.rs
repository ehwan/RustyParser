use std::iter::Iterator;

use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultValue;

#[derive(Debug, Clone)]
pub struct OrNonVoid<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: Parser<It>,
    ParserB: Parser<It, Output = <ParserA as Parser<It>>::Output>,
{
    parser_a: ParserA,
    parser_b: ParserB,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserA, ParserB, It> OrNonVoid<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: Parser<It>,
    ParserB: Parser<It, Output = <ParserA as Parser<It>>::Output>,
{
    pub fn new(parser_a: ParserA, parser_b: ParserB) -> Self {
        Self {
            parser_a: parser_a,
            parser_b: parser_b,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserA, ParserB, It> ResultValue<It> for OrNonVoid<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: Parser<It>,
    ParserB: Parser<It, Output = <ParserA as Parser<It>>::Output>,
{
}

impl<ParserA, ParserB, It> Parser<It> for OrNonVoid<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: Parser<It>,
    ParserB: Parser<It, Output = <ParserA as Parser<It>>::Output>,
{
    type Output = <ParserA as Parser<It>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let res = self.parser_a.parse(it);
        if let Some(val) = res.output {
            return ParseResult {
                output: Some(val),
                it: res.it,
            };
        }
        let res = self.parser_b.parse(res.it);
        if let Some(val) = res.output {
            return ParseResult {
                output: Some(val),
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
    use crate::core::singlerange::SingleRangeParser;
    #[test]
    fn success_test() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let alpha_parser = SingleRangeParser::new('a'..='z');

        let digitalpha_parser = OrNonVoid::new(digit_parser, alpha_parser);
        let str = "1a2b3c";

        let res = digitalpha_parser.parse(str.chars());
        assert_eq!(res.output, Some('1'));

        let res = digitalpha_parser.parse(res.it);
        assert_eq!(res.output, Some('a'));

        let rest: String = res.it.collect();
        assert_eq!(rest, "2b3c");
    }

    #[test]
    fn fail_test1() {
        let digit_parser = SingleRangeParser::new('0'..'9');
        let alpha_parser = SingleRangeParser::new('a'..'z');

        let digitalpha_parser = OrNonVoid::new(digit_parser, alpha_parser);
        let str = "1z2b3c";

        let res = digitalpha_parser.parse(str.chars());
        assert_eq!(res.output, Some('1'));

        let res = digitalpha_parser.parse(res.it);
        assert_eq!(res.output, None);

        let rest: String = res.it.collect();
        assert_eq!(rest, "z2b3c");
    }
    #[test]
    fn fail_test2() {
        let digit_parser = SingleRangeParser::new('0'..'9');
        let alpha_parser = SingleRangeParser::new('a'..'z');

        let digitalpha_parser = OrNonVoid::new(digit_parser, alpha_parser);
        let str = "9a2b3c";

        let res = digitalpha_parser.parse(str.chars());
        assert_eq!(res.output, None);

        let rest: String = res.it.collect();
        assert_eq!(rest, "9a2b3c");
    }
}
