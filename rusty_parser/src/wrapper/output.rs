use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct OutputParser<ParserType, OutputType>
where
    OutputType: Clone,
{
    parser: ParserType,
    output: OutputType,
}

impl<ParserType, OutputType> OutputParser<ParserType, OutputType>
where
    OutputType: Clone,
{
    pub fn new(parser: ParserType, output: OutputType) -> Self {
        Self { parser, output }
    }
}

impl<ParserType, OutputType, It> Parser<It> for OutputParser<ParserType, OutputType>
where
    OutputType: Clone,
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    type Output = (OutputType,);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let res = self.parser.match_pattern(it);
        if res.output.is_some() {
            ParseResult {
                output: Some((self.output.clone(),)),
                it: res.it,
            }
        } else {
            ParseResult {
                output: None,
                it: res.it,
            }
        }
    }
}

impl<ParserType, OutputType> IntoParser for OutputParser<ParserType, OutputType>
where
    OutputType: Clone,
{
    type Into = OutputParser<ParserType, OutputType>;
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
        let digit_parser = OutputParser::new(digit_parser, 1);

        let str = "123456abcd";
        let res = digit_parser.parse(str.chars());

        assert_eq!(res.output, Some((1,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "23456abcd");
    }
    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = OutputParser::new(digit_parser, 2);

        let str = "a23456abcd";
        let res = digit_parser.parse(str.chars());

        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a23456abcd");
    }
}
