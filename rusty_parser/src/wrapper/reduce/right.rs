use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

use crate::wrapper::tupleutils::concat::AppendTupleToTuple;
use crate::wrapper::tupleutils::singlevalue::SingleValueAutoTuple;
use crate::wrapper::tupleutils::unpack::TupleUnpack;

#[derive(Debug, Clone, Copy)]
pub struct ReduceRightParser<LhsParser, RhsParser, Reducer> {
    lhs: LhsParser,
    rhs: RhsParser,
    reducer: Reducer,
}

impl<LhsParser, RhsParser, Reducer> ReduceRightParser<LhsParser, RhsParser, Reducer> {
    pub fn new(lhs: LhsParser, rhs: RhsParser, reducer: Reducer) -> Self {
        Self { lhs, rhs, reducer }
    }
}

impl<LhsParser, RhsParser, Reducer, It, LhsOutput, RhsOutput, TupleMerged, ReducerOutput> Parser<It>
    for ReduceRightParser<LhsParser, RhsParser, Reducer>
where
    It: InputIteratorTrait,
    LhsParser: Parser<It, Output = LhsOutput>,
    RhsParser: Parser<It, Output = RhsOutput>,
    LhsOutput: Tuple + AppendTupleToTuple<RhsOutput, Output = TupleMerged>,
    RhsOutput: Tuple,
    Reducer: TupleUnpack<TupleMerged, Output = ReducerOutput>,
    ReducerOutput: SingleValueAutoTuple<RhsOutput, Output = RhsOutput>,
{
    type Output = RhsOutput;

    // lhs lhs lhs ... lhs rhs
    // ( lhs ( lhs ( lhs ( lhs rhs ) ) ) )
    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();

        // checkpoints after success of lhs
        let mut lhs_its: Vec<(LhsOutput, It)> = Vec::new();
        let mut it = it;
        loop {
            let res_lhs = self.lhs.parse(it);
            if let Some(res) = res_lhs.output {
                lhs_its.push((res, res_lhs.it.clone()));
                it = res_lhs.it;
            } else {
                break;
            }
        }

        // now unloop lhs_its backward and check if rhs matches.
        while let Some((lhs_output, it)) = lhs_its.pop() {
            let res_rhs = self.rhs.parse(it);
            if let Some(mut rhs_output) = res_rhs.output {
                // rhs matches; reduce and return

                // lhs + rhs tuple merged
                let output_merged = lhs_output.append_back(rhs_output);
                rhs_output = self.reducer.map(output_merged).wrap();

                while let Some((lhs_output, _)) = lhs_its.pop() {
                    let output_merged = lhs_output.append_back(rhs_output);
                    rhs_output = self.reducer.map(output_merged).wrap();
                }
                return ParseResult {
                    output: Some(rhs_output),
                    it: res_rhs.it,
                };
            } else {
                continue;
            }
        }

        // rhs matches failed for all lhs_its
        // try i0
        self.rhs.parse(i0)
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let i0 = it.clone();

        // checkpoints after success of lhs
        let mut lhs_its: Vec<It> = Vec::new();
        let mut it = it;
        loop {
            let res_lhs = self.lhs.match_pattern(it);
            if res_lhs.output.is_some() {
                lhs_its.push(res_lhs.it.clone());
                it = res_lhs.it;
            } else {
                break;
            }
        }

        // now unloop lhs_its backward and check if rhs matches.
        while let Some(it) = lhs_its.pop() {
            let res_rhs = self.rhs.match_pattern(it);
            if res_rhs.output.is_some() {
                // rhs matches; return
                return ParseResult {
                    output: Some(()),
                    it: res_rhs.it,
                };
            } else {
                continue;
            }
        }

        // rhs matches failed for all lhs_its
        // try i0
        self.rhs.match_pattern(i0)
    }
}

impl<LhsParser, RhsParser, Reducer> IntoParser
    for ReduceRightParser<LhsParser, RhsParser, Reducer>
{
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
        let alphabet_parser =
            SingleRangeParser::from('a'..='z').map(|val: char| -> i32 { val as i32 - 'a' as i32 });
        let reduced_right =
            ReduceRightParser::new(digit_parser, alphabet_parser, |lhs: i32, rhs: i32| -> i32 {
                rhs * 10 + lhs
            });

        let res = reduced_right.parse("123456dcba".chars());
        assert_eq!(res.output, Some((3654321,)));
        assert_eq!(res.it.collect::<String>(), "cba");
    }

    #[test]
    fn success2() {
        let digit_parser =
            SingleRangeParser::from('0'..='9').map(|val: char| -> i32 { val as i32 - '0' as i32 });
        let alphabet_parser =
            SingleRangeParser::from('a'..='z').map(|val: char| -> i32 { val as i32 - 'a' as i32 });
        let reduced_right = ReduceRightParser::new(
            digit_parser,
            alphabet_parser,
            |lhs: i32, rhs: i32| -> (i32,) { (rhs * 10 + lhs,) },
        );

        let res = reduced_right.parse("123456dcba".chars());
        assert_eq!(res.output, Some((3654321,)));
        assert_eq!(res.it.collect::<String>(), "cba");
    }
    #[test]
    fn fail1() {
        let digit_parser =
            SingleRangeParser::from('0'..='9').map(|val: char| -> i32 { val as i32 - '0' as i32 });
        let alphabet_parser =
            SingleRangeParser::from('a'..='z').map(|val: char| -> i32 { val as i32 - 'a' as i32 });
        let reduced_right = ReduceRightParser::new(
            digit_parser,
            alphabet_parser,
            |lhs: i32, rhs: i32| -> (i32,) { (rhs * 10 + lhs,) },
        );

        let res = reduced_right.parse("123456".chars());
        assert_eq!(res.output, None);
        assert_eq!(res.it.collect::<String>(), "123456");
    }
}
