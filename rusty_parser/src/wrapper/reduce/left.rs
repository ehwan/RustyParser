use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

use crate::wrapper::tupleutils::concat::AppendTupleToTuple;
use crate::wrapper::tupleutils::singlevalue::SingleValueAutoTuple;
use crate::wrapper::tupleutils::unpack::TupleUnpack;

#[derive(Debug, Clone, Copy)]
pub struct ReduceLeftParser<LhsParser, RhsParser, Reducer> {
    lhs: LhsParser,
    rhs: RhsParser,
    reducer: Reducer,
}

impl<LhsParser, RhsParser, Reducer> ReduceLeftParser<LhsParser, RhsParser, Reducer> {
    pub fn new(lhs: LhsParser, rhs: RhsParser, reducer: Reducer) -> Self {
        Self { lhs, rhs, reducer }
    }
}

impl<LhsParser, RhsParser, Reducer, It, LhsOutput, RhsOutput, TupleMerged, ReducerOutput> Parser<It>
    for ReduceLeftParser<LhsParser, RhsParser, Reducer>
where
    It: InputIteratorTrait,
    LhsParser: Parser<It, Output = LhsOutput>,
    RhsParser: Parser<It, Output = RhsOutput>,
    LhsOutput: Tuple + AppendTupleToTuple<RhsOutput, Output = TupleMerged>,
    RhsOutput: Tuple,
    Reducer: TupleUnpack<TupleMerged, Output = ReducerOutput>,
    ReducerOutput: SingleValueAutoTuple<LhsOutput, Output = LhsOutput>,
{
    type Output = LhsOutput;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut lhs_res = self.lhs.parse(it);
        if lhs_res.output.is_none() {
            return ParseResult {
                output: None,
                it: lhs_res.it,
            };
        }
        loop {
            let rhs_res = self.rhs.parse(lhs_res.it);
            if rhs_res.output.is_none() {
                return ParseResult {
                    output: lhs_res.output,
                    it: rhs_res.it,
                };
            }

            // lhs + rhs tuple merged
            let output_merged = lhs_res.output.unwrap().append_back(rhs_res.output.unwrap());
            let reduced = self.reducer.map(output_merged);

            lhs_res.output = Some(reduced.wrap());
            lhs_res.it = rhs_res.it;
        }
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let mut lhs_res = self.lhs.match_pattern(it);
        if lhs_res.output.is_none() {
            return ParseResult {
                output: None,
                it: lhs_res.it,
            };
        }
        loop {
            let rhs_res = self.rhs.match_pattern(lhs_res.it);
            if rhs_res.output.is_none() {
                return ParseResult {
                    output: Some(()),
                    it: rhs_res.it,
                };
            }
            lhs_res.it = rhs_res.it;
        }
    }
}

impl<LhsParser, RhsParser, Reducer> IntoParser for ReduceLeftParser<LhsParser, RhsParser, Reducer> {
    type Into = Self;

    fn into_parser(self) -> Self {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::leaf::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser =
            SingleRangeParser::from('0'..='9').map(|val: char| -> i32 { val as i32 - '0' as i32 });
        let reduced_left =
            ReduceLeftParser::new(digit_parser, digit_parser, |lhs, rhs| lhs * 10 + rhs);

        let res = reduced_left.parse("123456abcd".chars());

        assert_eq!(res.output, Some((123456,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn success2() {
        let digit_parser =
            SingleRangeParser::from('0'..='9').map(|val: char| -> i32 { val as i32 - '0' as i32 });
        let reduced_left =
            ReduceLeftParser::new(digit_parser, digit_parser, |lhs, rhs| lhs * 10 + rhs);

        let res = reduced_left.parse("1abcd".chars());

        assert_eq!(res.output, Some((1,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn success3() {
        let digit_parser =
            SingleRangeParser::from('0'..='9').map(|val: char| -> i32 { val as i32 - '0' as i32 });
        let reduced_left =
            ReduceLeftParser::new(digit_parser, digit_parser, |lhs, rhs| -> (i32,) {
                (lhs * 10 + rhs,)
            });

        let res = reduced_left.parse("123456abcd".chars());

        assert_eq!(res.output, Some((123456,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn fail1() {
        let digit_parser =
            SingleRangeParser::from('0'..='9').map(|val: char| -> i32 { val as i32 - '0' as i32 });
        let reduced_left =
            ReduceLeftParser::new(digit_parser, digit_parser, |lhs, rhs| lhs * 10 + rhs);

        let res = reduced_left.parse("abcd".chars());

        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
}
