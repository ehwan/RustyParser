use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

use crate::wrapper::tuplemerge::AppendTupleToTuple;
use crate::wrapper::tupleunpack::TupleUnpack;

pub type RepeatCountType = usize;

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

impl<LhsParser, RhsParser, Reducer, It, LhsOutput, RhsOutput> Parser<It>
    for ReduceLeftParser<LhsParser, RhsParser, Reducer>
where
    It: InputIteratorTrait,
    LhsParser: Parser<It, Output = LhsOutput>,
    RhsParser: Parser<It, Output = RhsOutput>,
    Reducer: Fn(LhsOutput, RhsOutput) -> LhsOutput,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {}
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {}
}

impl<LhsParser, RhsParser, Reducer> IntoParser for ReduceLeftParser<LhsParser, RhsParser, Reducer> {
    type Into = Self;

    fn into_parser(self) -> Self {
        self
    }
}

/*
#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        leaf::singlerange::SingleRangeParser,
        wrapper::{seq::SeqParser, void::VoidParser},
    };

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let repeat_parser = RepeatParser::from(digit_parser, 1..=3);

        let str = "123456abcd";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, Some((vec!['1', '2', '3',],)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "456abcd");
    }
    #[test]
    fn success2() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let repeat_parser = RepeatParser::from(digit_parser, 1..=6);

        let str = "123456abcd";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, Some((vec!['1', '2', '3', '4', '5', '6',],)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn success3() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let repeat_parser = RepeatParser::from(digit_parser, 4..);

        let str = "1234abcd";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, Some((vec!['1', '2', '3', '4',],)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn success4() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = VoidParser::new(digit_parser);
        let repeat_parser = RepeatParser::from(digit_parser, 4..);

        let str = "1234abcd";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn success5() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let digit_parser = SeqParser::new(digit_parser, digit_parser);
        let repeat_parser = RepeatParser::from(digit_parser, 2..=2);

        let str = "12341234";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, Some((vec![('1', '2'), ('3', '4')],)));
        let rest: String = res.it.collect();
        assert_eq!(rest, "1234");
    }
    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::from('0'..='9');
        let repeat_parser = RepeatParser::from(digit_parser, 5..10);

        let str = "1234abcd";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "1234abcd");
    }
}
*/
