use std::iter::Iterator;

use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultTuple;
use crate::core::traits::ResultValue;

#[derive(Debug, Clone)]
pub struct SeqValValParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultValue<It> + Parser<It>,
    ParserB: ResultValue<It> + Parser<It>,
{
    pub parser_a: ParserA,
    pub parser_b: ParserB,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserA, ParserB, It> SeqValValParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultValue<It> + Parser<It>,
    ParserB: ResultValue<It> + Parser<It>,
{
    pub fn new(parser_a: ParserA, parser_b: ParserB) -> SeqValValParser<ParserA, ParserB, It> {
        SeqValValParser {
            parser_a: parser_a,
            parser_b: parser_b,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserA, ParserB, It> ResultTuple<It> for SeqValValParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultValue<It> + Parser<It>,
    ParserB: ResultValue<It> + Parser<It>,
{
}

impl<ParserA, ParserB, It> Parser<It> for SeqValValParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultValue<It> + Parser<It>,
    ParserB: ResultValue<It> + Parser<It>,
{
    type Output = (
        <ParserA as Parser<It>>::Output,
        <ParserB as Parser<It>>::Output,
    );

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let res_a = self.parser_a.parse(it);
        if let Some(output_a) = res_a.output {
            let res_b = self.parser_b.parse(res_a.it);
            if let Some(output_b) = res_b.output {
                ParseResult {
                    output: Some((output_a, output_b)),
                    it: res_b.it,
                }
            } else {
                ParseResult {
                    output: None,
                    it: i0,
                }
            }
        } else {
            ParseResult {
                output: None,
                it: i0,
            }
        }
    }

    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let i0 = it.clone();
        let res_a = self.parser_a.match_pattern(it);
        if let Some(_) = res_a.output {
            let res_b = self.parser_b.match_pattern(res_a.it);
            if let Some(_) = res_b.output {
                ParseResult {
                    output: Some(()),
                    it: res_b.it,
                }
            } else {
                ParseResult {
                    output: None,
                    it: i0,
                }
            }
        } else {
            ParseResult {
                output: None,
                it: i0,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::singlerange::SingleRangeParser;
    use crate::core::traits::Parser;

    #[test]
    fn seq_parser_success_test() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let alpha_parser = SingleRangeParser::new('a'..='z');
        let seq_parser = SeqValValParser::new(digit_parser, alpha_parser);

        let str = "1ahello_world";
        let res = seq_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1', 'a')));
        let rest: String = res.it.collect();
        assert_eq!(rest, "hello_world");
    }

    #[test]
    fn seq_parser_fail_test() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let alpha_parser = SingleRangeParser::new('a'..='z');
        let seq_parser = SeqValValParser::new(digit_parser, alpha_parser);

        let str = "a1hello_world";
        let res = seq_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a1hello_world");
    }
}
