use std::iter::Iterator;

use super::util::valtup::AppendValueToTuple;
use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultTuple;
use crate::core::traits::ResultValue;

#[derive(Debug, Clone)]
pub struct SeqValTupParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultValue<It> + Parser<It>,
    ParserB: ResultTuple<It> + Parser<It>,
    <ParserB as Parser<It>>::Output: AppendValueToTuple<<ParserA as Parser<It>>::Output>,
{
    pub parser_a: ParserA,
    pub parser_b: ParserB,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserA, ParserB, It> SeqValTupParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultValue<It> + Parser<It>,
    ParserB: ResultTuple<It> + Parser<It>,
    <ParserB as Parser<It>>::Output: AppendValueToTuple<<ParserA as Parser<It>>::Output>,
{
    pub fn new(parser_a: ParserA, parser_b: ParserB) -> SeqValTupParser<ParserA, ParserB, It> {
        Self {
            parser_a: parser_a,
            parser_b: parser_b,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserA, ParserB, It> ResultTuple<It> for SeqValTupParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultValue<It> + Parser<It>,
    ParserB: ResultTuple<It> + Parser<It>,
    <ParserB as Parser<It>>::Output: AppendValueToTuple<<ParserA as Parser<It>>::Output>,
{
}

impl<ParserA, ParserB, It> Parser<It> for SeqValTupParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultValue<It> + Parser<It>,
    ParserB: ResultTuple<It> + Parser<It>,
    <ParserB as Parser<It>>::Output: AppendValueToTuple<<ParserA as Parser<It>>::Output>,
{
    type Output = <<ParserB as Parser<It>>::Output as AppendValueToTuple<
        <ParserA as Parser<It>>::Output,
    >>::FrontOutput;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let res_a = self.parser_a.parse(it);
        if let Some(output_a) = res_a.output {
            let res_b = self.parser_b.parse(res_a.it);
            if let Some(output_b) = res_b.output {
                ParseResult {
                    output: Some(output_b.append_front(output_a)),
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
    use super::super::valval::SeqValValParser;
    use super::*;
    use crate::core::singlerange::SingleRangeParser;
    use crate::core::traits::Parser;

    #[test]
    fn success_test() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let alpha_parser = SingleRangeParser::new('a'..='z');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), alpha_parser);

        let valtup_parser = SeqValTupParser::new(digit_parser, seq_parser);

        let str = "12abcde";
        let res = valtup_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1', '2', 'a')));
        let rest: String = res.it.collect();
        assert_eq!(rest, "bcde");
    }

    #[test]
    fn fail_test1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let alpha_parser = SingleRangeParser::new('a'..='z');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), alpha_parser);

        let valtup_parser = SeqValTupParser::new(digit_parser, seq_parser);

        let str = "a2abcde";
        let res = valtup_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a2abcde");
    }

    #[test]
    fn fail_test2() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let alpha_parser = SingleRangeParser::new('a'..='z');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), alpha_parser);

        let valtup_parser = SeqValTupParser::new(digit_parser, seq_parser);

        let str = "1aabcde";
        let res = valtup_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "1aabcde");
    }
    #[test]
    fn fail_test3() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let alpha_parser = SingleRangeParser::new('a'..='z');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), alpha_parser);

        let valtup_parser = SeqValTupParser::new(digit_parser, seq_parser);

        let str = "123bcde";
        let res = valtup_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "123bcde");
    }
}
