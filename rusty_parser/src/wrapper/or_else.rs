use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;
use crate::wrapper::tupleutils::singlevalue::SingleValueAutoTuple;

#[derive(Debug, Clone, Copy)]
pub struct OptionalOrElseParser<Parser, F> {
    parser: Parser,
    closure: F,
}

impl<Parser, F> OptionalOrElseParser<Parser, F> {
    pub fn new(parser: Parser, closure: F) -> Self {
        Self { parser, closure }
    }
}

impl<ParserA, F, It, FnOutput> Parser<It> for OptionalOrElseParser<ParserA, F>
where
    It: InputIteratorTrait,
    ParserA: Parser<It>,
    F: Fn() -> FnOutput,
    FnOutput: SingleValueAutoTuple<ParserA::Output, Output = ParserA::Output>,
    FnOutput::Output: Tuple,
{
    type Output = <ParserA as Parser<It>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let res = self.parser.parse(it);
        if let Some(val) = res.output {
            return ParseResult {
                output: Some(val),
                it: res.it,
            };
        }
        ParseResult {
            output: Some((self.closure)().wrap()),
            it: res.it,
        }
    }

    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let res = self.parser.match_pattern(it);
        if res.output.is_some() {
            ParseResult {
                output: Some(()),
                it: res.it,
            }
        } else {
            ParseResult {
                output: Some(()),
                it: res.it,
            }
        }
    }
}

impl<Parser, F> IntoParser for OptionalOrElseParser<Parser, F> {
    type Into = OptionalOrElseParser<Parser, F>;

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
        let digit_or_parser = OptionalOrElseParser::new(digit_parser, || 'a');

        let str = "1a2b3c";

        let res = digit_or_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        assert_eq!(res.it.as_str(), "a2b3c");
    }

    #[test]
    fn success2() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_or_parser = OptionalOrElseParser::new(digit_parser, || 'a');

        let str = "ba2b3c";

        let res = digit_or_parser.parse(str.chars());
        assert_eq!(res.output, Some(('a',)));
        assert_eq!(res.it.as_str(), "ba2b3c");
    }
}
