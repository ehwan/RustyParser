use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct OrParser<ParserA, ParserB> {
    parser_a: ParserA,
    parser_b: ParserB,
}

impl<ParserA, ParserB> OrParser<ParserA, ParserB> {
    pub fn new(parser_a: ParserA, parser_b: ParserB) -> Self {
        Self {
            parser_a: parser_a,
            parser_b: parser_b,
        }
    }
}

impl<ParserA, ParserB, It> Parser<It> for OrParser<ParserA, ParserB>
where
    It: InputIteratorTrait,
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

    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let res = self.parser_a.match_pattern(it);
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
}

pub fn or<ParserA: IntoParser, ParserB: IntoParser>(
    parser_a: ParserA,
    parser_b: ParserB,
) -> OrParser<ParserA::Into, ParserB::Into> {
    OrParser::new(parser_a.into_parser(), parser_b.into_parser())
}

impl<ParserA, ParserB> IntoParser for OrParser<ParserA, ParserB> {
    type Into = OrParser<ParserA, ParserB>;

    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singlerange::SingleRangeParser;
    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let alpha_parser = SingleRangeParser::from('a'..='z');

        let digitalpha_parser = OrParser::new(digit_parser, alpha_parser);
        let str = "1a2b3c";

        let res = digitalpha_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));

        let res = digitalpha_parser.parse(res.it);
        assert_eq!(res.output, Some(('a',)));

        let rest: String = res.it.collect();
        assert_eq!(rest, "2b3c");
    }

    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::from('0'..'9');
        let alpha_parser = SingleRangeParser::from('a'..'z');

        let digitalpha_parser = OrParser::new(digit_parser, alpha_parser);
        let str = "1z2b3c";

        let res = digitalpha_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));

        let res = digitalpha_parser.parse(res.it);
        assert_eq!(res.output, None);

        let rest: String = res.it.collect();
        assert_eq!(rest, "z2b3c");
    }
    #[test]
    fn fail2() {
        let digit_parser = SingleRangeParser::from('0'..'9');
        let alpha_parser = SingleRangeParser::from('a'..'z');

        let digitalpha_parser = OrParser::new(digit_parser, alpha_parser);
        let str = "9a2b3c";

        let res = digitalpha_parser.parse(str.chars());
        assert_eq!(res.output, None);

        let rest: String = res.it.collect();
        assert_eq!(rest, "9a2b3c");
    }
}
