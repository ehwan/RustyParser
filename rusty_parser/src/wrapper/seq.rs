use super::tuplemerge::AppendTupleToTuple;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

#[derive(Debug, Clone, Copy)]
pub struct SeqParser<ParserA, ParserB, It>
where
    It: InputIteratorTrait,
    ParserA: Parser<It>,
    ParserB: Parser<It>,
    <ParserA as Parser<It>>::Output: AppendTupleToTuple<<ParserB as Parser<It>>::Output>,
    <<ParserA as Parser<It>>::Output as AppendTupleToTuple<<ParserB as Parser<It>>::Output>>::Output: Tuple,
{
    pub parser_a: ParserA,
    pub parser_b: ParserB,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserA, ParserB, It> SeqParser<ParserA, ParserB, It>
where
    It: InputIteratorTrait,
    ParserA: Parser<It>,
    ParserB: Parser<It>,
    <ParserA as Parser<It>>::Output: AppendTupleToTuple<<ParserB as Parser<It>>::Output>,
    <<ParserA as Parser<It>>::Output as AppendTupleToTuple<<ParserB as Parser<It>>::Output>>::Output: Tuple,
{
    pub fn new(parser_a: ParserA, parser_b: ParserB) -> Self {
        Self {
            parser_a: parser_a,
            parser_b: parser_b,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserA, ParserB, It> Parser<It> for SeqParser<ParserA, ParserB, It>
where
    It: InputIteratorTrait,
    ParserA: Parser<It>,
    ParserB: Parser<It>,
    <ParserA as Parser<It>>::Output: AppendTupleToTuple<<ParserB as Parser<It>>::Output>,
    <<ParserA as Parser<It>>::Output as AppendTupleToTuple<<ParserB as Parser<It>>::Output>>::Output: Tuple,
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

pub fn seq<ParserA, ParserB, It>(
    parser_a: ParserA,
    parser_b: ParserB,
) -> SeqParser<ParserA, ParserB, It>
where
    It: InputIteratorTrait,
    ParserA: Parser<It>,
    ParserB: Parser<It>,
    <ParserA as Parser<It>>::Output: AppendTupleToTuple<<ParserB as Parser<It>>::Output>,
    <<ParserA as Parser<It>>::Output as AppendTupleToTuple<<ParserB as Parser<It>>::Output>>::Output: Tuple,
{
    SeqParser::new(parser_a, parser_b)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let seq_parser = SeqParser::new(digit_parser.clone(), digit_parser);

        let str = "1234abcd";
        let res = seq_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1', '2')));
        let rest: String = res.it.collect();
        assert_eq!(rest, "34abcd");
    }

    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let seq_parser = SeqParser::new(digit_parser.clone(), digit_parser);

        let str = "1a34abcd";
        let res = seq_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "1a34abcd");
    }

    #[test]
    fn fail2() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let seq_parser = SeqParser::new(digit_parser.clone(), digit_parser);

        let str = "a234abcd";
        let res = seq_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "a234abcd");
    }
}
