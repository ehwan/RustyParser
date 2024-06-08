use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

use crate::wrapper::tupleutils::concat::AppendTupleToTuple;
use crate::wrapper::tupleutils::unpack::TupleUnpack;

#[derive(Debug, Clone, Copy)]
pub struct ReduceInitParser<LhsParser, Init, Reducer>
where
    Init: Clone,
{
    lhs: LhsParser,
    init: Init,
    reducer: Reducer,
}

impl<LhsParser, Init, Reducer> ReduceInitParser<LhsParser, Init, Reducer>
where
    Init: Clone,
{
    pub fn new(lhs: LhsParser, init: Init, reducer: Reducer) -> Self {
        Self { lhs, init, reducer }
    }
}

impl<LhsParser, Init, Reducer, It, LhsOutput, TupleMerged> Parser<It>
    for ReduceInitParser<LhsParser, Init, Reducer>
where
    Init: Clone,
    It: InputIteratorTrait,
    LhsParser: Parser<It, Output = LhsOutput>,
    LhsOutput: Tuple,
    (Init,): AppendTupleToTuple<LhsOutput, Output = TupleMerged>,
    Reducer: TupleUnpack<TupleMerged, Output = Init>,
{
    type Output = (Init,);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut init = self.init.clone();
        let mut it = it;
        loop {
            let res = self.lhs.parse(it);
            if res.output.is_none() {
                return ParseResult {
                    output: Some((init,)),
                    it: res.it,
                };
            }
            let merge_init_lhs = (init,).append_back(res.output.unwrap());
            init = self.reducer.map(merge_init_lhs);
            it = res.it;
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

impl<LhsParser, Init, Reducer> IntoParser for ReduceInitParser<LhsParser, Init, Reducer>
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
        let reduced = ReduceInitParser::new(digit_parser, 0, |acc, rhs| acc * 10 + rhs);

        let res = reduced.parse("123456abcd".chars());

        assert_eq!(res.output, Some((123456,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn success2() {
        let digit_parser =
            SingleRangeParser::from('0'..='9').map(|val: char| -> i32 { val as i32 - '0' as i32 });
        let reduced = ReduceInitParser::new(digit_parser, 0, |acc, rhs| acc * 10 + rhs);

        let res = reduced.parse("1abcd".chars());

        assert_eq!(res.output, Some((1,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn success3() {
        let digit_parser =
            SingleRangeParser::from('0'..='9').map(|val: char| -> i32 { val as i32 - '0' as i32 });
        let reduced = ReduceInitParser::new(digit_parser, 0, |acc, rhs| acc * 10 + rhs);

        let res = reduced.parse("abcd".chars());

        assert_eq!(res.output, Some((0,)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
}
