use std::iter::Iterator;

use super::util::valtup::AppendValueToTuple;
use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultTuple;
use crate::core::traits::ResultValue;

#[derive(Debug, Clone)]
pub struct SeqTupValParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
    ParserB: ResultValue<It> + Parser<It>,
    <ParserA as Parser<It>>::Output: AppendValueToTuple<<ParserB as Parser<It>>::Output>,
{
    pub parser_a: ParserA,
    pub parser_b: ParserB,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserA, ParserB, It> SeqTupValParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
    ParserB: ResultValue<It> + Parser<It>,
    <ParserA as Parser<It>>::Output: AppendValueToTuple<<ParserB as Parser<It>>::Output>,
{
    pub fn new(parser_a: ParserA, parser_b: ParserB) -> Self {
        Self {
            parser_a: parser_a,
            parser_b: parser_b,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserA, ParserB, It> ResultTuple<It> for SeqTupValParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
    ParserB: ResultValue<It> + Parser<It>,
    <ParserA as Parser<It>>::Output: AppendValueToTuple<<ParserB as Parser<It>>::Output>,
{
}

impl<ParserA, ParserB, It> Parser<It> for SeqTupValParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
    ParserB: ResultValue<It> + Parser<It>,
    <ParserA as Parser<It>>::Output: AppendValueToTuple<<ParserB as Parser<It>>::Output>,
{
    type Output = <<ParserA as Parser<It>>::Output as AppendValueToTuple<
        <ParserB as Parser<It>>::Output,
    >>::BackOutput;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let res_a = self.parser_a.parse(it);
        if let Some(val_a) = res_a.output {
            let res_b = self.parser_b.parse(res_a.it);
            if let Some(val_b) = res_b.output {
                ParseResult {
                    output: Some(val_a.append_back(val_b)),
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
    use super::super::seq_valval::SeqValValParser;
    use super::*;
    use crate::core::singlerange::SingleRangeParser;
    use crate::core::traits::Parser;

    #[test]
    fn success_test() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let alpha_parser = SingleRangeParser::new('a'..='z');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), alpha_parser);

        let tupval_parser = SeqTupValParser::new(seq_parser, digit_parser);

        let str = "1a2abcde";
        let res = tupval_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1', 'a', '2')));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcde");
    }

    #[test]
    fn fail_test1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let alpha_parser = SingleRangeParser::new('a'..='z');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), alpha_parser);

        let tupval_parser = SeqTupValParser::new(seq_parser, digit_parser);

        let str = "xa2abcde";
        let res = tupval_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "xa2abcde");
    }

    #[test]
    fn fail_test2() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let alpha_parser = SingleRangeParser::new('a'..='z');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), alpha_parser);

        let tupval_parser = SeqTupValParser::new(seq_parser, digit_parser);

        let str = "132abcde";
        let res = tupval_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "132abcde");
    }
    #[test]
    fn fail_test3() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let alpha_parser = SingleRangeParser::new('a'..='z');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), alpha_parser);

        let tupval_parser = SeqTupValParser::new(seq_parser, digit_parser);

        let str = "1axabcde";
        let res = tupval_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "1axabcde");
    }
}
