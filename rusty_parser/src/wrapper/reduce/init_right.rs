use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

use crate::wrapper::tupleutils::concat::AppendTupleToTuple;
use crate::wrapper::tupleutils::unpack::TupleUnpack;

use std::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub struct ReduceRightInitParser<LhsParser, Init, Reducer>
where
    Init: Clone,
{
    lhs: LhsParser,
    init: Init,
    reducer: Reducer,
}

impl<LhsParser, Init, Reducer> ReduceRightInitParser<LhsParser, Init, Reducer>
where
    Init: Clone,
{
    pub fn new(lhs: LhsParser, init: Init, reducer: Reducer) -> Self {
        Self { lhs, init, reducer }
    }
}

impl<LhsParser, Init, Reducer, It, LhsOutput, TupleMerged> Parser<It>
    for ReduceRightInitParser<LhsParser, Init, Reducer>
where
    Init: Clone,
    It: InputIteratorTrait,
    LhsParser: Parser<It, Output = LhsOutput>,
    LhsOutput: Tuple + AppendTupleToTuple<(Init,), Output = TupleMerged>,
    Reducer: TupleUnpack<TupleMerged, Output = Init>,
{
    type Output = (Init,);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut lhs_outputs: Vec<LhsOutput> = Vec::new();
        let mut it = it;
        loop {
            let res = self.lhs.parse(it);
            it = res.it;
            if let Some(output) = res.output {
                lhs_outputs.push(output);
            } else {
                break;
            }
        }

        let mut init = self.init.clone();
        for lhs_output in lhs_outputs.into_iter().rev() {
            // tuple concat (lhs_output, init)
            let merge_lhs_init = lhs_output.append_back((init,));
            init = self.reducer.map(merge_lhs_init);
        }
        ParseResult {
            output: Some((init,)),
            it,
        }
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let mut it = it;
        loop {
            let res = self.lhs.match_pattern(it);
            if res.output.is_none() {
                return ParseResult {
                    output: Some(()),
                    it: res.it,
                };
            }
            it = res.it;
        }
    }
}

impl<LhsParser, Init, Reducer> IntoParser for ReduceRightInitParser<LhsParser, Init, Reducer>
where
    Init: Clone,
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
        let reduced = ReduceRightInitParser::new(digit_parser, 0, |lhs, acc| acc * 10 + lhs);

        let res = reduced.parse("123456abcd".chars());

        assert_eq!(res.output, Some((654321,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn success2() {
        let digit_parser =
            SingleRangeParser::from('0'..='9').map(|val: char| -> i32 { val as i32 - '0' as i32 });
        let reduced = ReduceRightInitParser::new(digit_parser, 0, |lhs, acc| acc * 10 + lhs);

        let res = reduced.parse("1abcd".chars());

        assert_eq!(res.output, Some((1,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn success3() {
        let digit_parser =
            SingleRangeParser::from('0'..='9').map(|val: char| -> i32 { val as i32 - '0' as i32 });
        let reduced = ReduceRightInitParser::new(digit_parser, 0, |lhs, acc| acc * 10 + lhs);

        let res = reduced.parse("abcd".chars());

        assert_eq!(res.output, Some((0,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
}
