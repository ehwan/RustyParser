use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;
use crate::wrapper::tupleutils::option::OptionOutputSpecialize;
use crate::wrapper::tupleutils::singlevalue::SingleValueAutoTuple;

#[derive(Debug, Clone, Copy)]
pub struct OptionalParser<ParserType> {
    parser: ParserType,
}

impl<ParserType> OptionalParser<ParserType> {
    pub fn new(parser: ParserType) -> Self {
        Self { parser }
    }
}

impl<ParserType, It> Parser<It> for OptionalParser<ParserType>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
    <ParserType as Parser<It>>::Output: OptionOutputSpecialize,
{
    type Output = (<<ParserType as Parser<It>>::Output as OptionOutputSpecialize>::Output,);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let res = self.parser.parse(it);
        if let Some(val) = res.output {
            ParseResult {
                output: Some((val.make_some(),)),
                it: res.it,
            }
        } else {
            ParseResult {
                output: Some((
                    <<ParserType as Parser<It>>::Output as OptionOutputSpecialize>::make_none(),
                )),
                it: res.it,
            }
        }
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let res = self.parser.match_pattern(it);
        ParseResult {
            output: Some(()),
            it: res.it,
        }
    }
}

impl<ParserType> IntoParser for OptionalParser<ParserType> {
    type Into = OptionalParser<ParserType>;

    fn into_parser(self) -> Self::Into {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OptionalOrParser<ParserType, Output>
where
    Output: Clone,
{
    parser: ParserType,
    output: Output,
}

impl<ParserType, Output> OptionalOrParser<ParserType, Output>
where
    Output: Clone,
{
    pub fn new(parser: ParserType, output: Output) -> Self {
        Self { parser, output }
    }
}

impl<ParserType, Output, ParserOutput, It> Parser<It> for OptionalOrParser<ParserType, Output>
where
    It: InputIteratorTrait,
    ParserType: Parser<It, Output = ParserOutput>,
    Output: Clone + SingleValueAutoTuple<ParserOutput, Output = ParserOutput>,
    <Output as SingleValueAutoTuple<ParserOutput>>::Output: Tuple,
{
    type Output = ParserOutput;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let res = self.parser.parse(it);
        if let Some(val) = res.output {
            ParseResult {
                output: Some(val),
                it: res.it,
            }
        } else {
            ParseResult {
                output: Some(self.output.clone().wrap()),
                it: res.it,
            }
        }
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let res = self.parser.match_pattern(it);
        ParseResult {
            output: Some(()),
            it: res.it,
        }
    }
}

impl<ParserType, Output> IntoParser for OptionalOrParser<ParserType, Output>
where
    Output: Clone,
{
    type Into = OptionalOrParser<ParserType, Output>;

    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singlerange::SingleRangeParser;

    #[test]
    fn success() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = OptionalParser::new(digit_parser);

        let str = "1a2bhello";
        let res = digit_parser.parse(str.chars());
        assert_eq!(res.output, Some((Some('1'),)));
        let res = digit_parser.parse(res.it);
        assert_eq!(res.output, Some((None,)));
        let res = digit_parser.parse(res.it);
        assert_eq!(res.output, Some((None,)));

        let rest: String = res.it.collect();
        assert_eq!(rest, "a2bhello");
    }

    #[test]
    fn success2() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = OptionalOrParser::new(digit_parser, ('x',));

        let str = "1a2bhello";
        let res = digit_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let res = digit_parser.parse(res.it);
        assert_eq!(res.output, Some(('x',)));

        let rest: String = res.it.collect();
        assert_eq!(rest, "a2bhello");
    }
    #[test]
    fn success3() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = OptionalOrParser::new(digit_parser, 'x');

        let str = "1a2bhello";
        let res = digit_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let res = digit_parser.parse(res.it);
        assert_eq!(res.output, Some(('x',)));

        let rest: String = res.it.collect();
        assert_eq!(rest, "a2bhello");
    }
}
