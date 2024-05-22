use std::iter::Iterator;

use super::util::tuptup::AppendTupleToTuple;
use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultTuple;

#[derive(Debug, Clone)]
pub struct SeqTupTupParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
    ParserB: ResultTuple<It> + Parser<It>,
    <ParserA as Parser<It>>::Output: AppendTupleToTuple<<ParserB as Parser<It>>::Output>,
{
    pub parser_a: ParserA,
    pub parser_b: ParserB,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserA, ParserB, It> SeqTupTupParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
    ParserB: ResultTuple<It> + Parser<It>,
    <ParserA as Parser<It>>::Output: AppendTupleToTuple<<ParserB as Parser<It>>::Output>,
{
    pub fn new(parser_a: ParserA, parser_b: ParserB) -> Self {
        Self {
            parser_a: parser_a,
            parser_b: parser_b,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserA, ParserB, It> ResultTuple<It> for SeqTupTupParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
    ParserB: ResultTuple<It> + Parser<It>,
    <ParserA as Parser<It>>::Output: AppendTupleToTuple<<ParserB as Parser<It>>::Output>,
{
}

impl<ParserA, ParserB, It> Parser<It> for SeqTupTupParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
    ParserB: ResultTuple<It> + Parser<It>,
    <ParserA as Parser<It>>::Output: AppendTupleToTuple<<ParserB as Parser<It>>::Output>,
{
    type Output = <<ParserA as Parser<It>>::Output as AppendTupleToTuple<
        <ParserB as Parser<It>>::Output,
    >>::Output;

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
        let seq_parser = SeqValValParser::new(digit_parser.clone(), digit_parser);

        let seqseq_parser = SeqTupTupParser::new(seq_parser.clone(), seq_parser);

        let str = "1234abcd";
        let res = seqseq_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1', '2', '3', '4')));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }

    #[test]
    fn fail_test1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), digit_parser);

        let seqseq_parser = SeqTupTupParser::new(seq_parser.clone(), seq_parser);

        let str = "a234abcd";
        let res = seqseq_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a234abcd");
    }

    #[test]
    fn fail_test2() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), digit_parser);

        let seqseq_parser = SeqTupTupParser::new(seq_parser.clone(), seq_parser);

        let str = "1a34abcd";
        let res = seqseq_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "1a34abcd");
    }
    #[test]
    fn fail_test3() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), digit_parser);

        let seqseq_parser = SeqTupTupParser::new(seq_parser.clone(), seq_parser);

        let str = "12a4abcd";
        let res = seqseq_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "12a4abcd");
    }
    #[test]
    fn fail_test4() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let seq_parser = SeqValValParser::new(digit_parser.clone(), digit_parser);

        let seqseq_parser = SeqTupTupParser::new(seq_parser.clone(), seq_parser);

        let str = "123aabcd";
        let res = seqseq_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "123aabcd");
    }
}
