use super::vecmerge::VectorOutputSpecialize;
use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::range_copyable::{RangeBound, ToCopyable};
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

pub type RepeatCountType = i32;

#[derive(Debug, Clone, Copy)]
pub struct RepeatParser<ParserType, RangeType>
where
    RangeType: RangeBound<RepeatCountType>,
{
    parser: ParserType,
    range: RangeType,
}

impl<ParserType, RangeType> RepeatParser<ParserType, RangeType>
where
    RangeType: RangeBound<RepeatCountType>,
{
    pub fn new(parser: ParserType, range: RangeType) -> Self {
        Self {
            parser: parser,
            range: range,
        }
    }
    pub fn from<RangeType_>(parser: ParserType, range: RangeType_) -> Self
    where
        RangeType_: ToCopyable<Into = RangeType>,
    {
        Self {
            parser: parser,
            range: range.into(),
        }
    }
}

impl<ParserType, RangeType, It> Parser<It> for RepeatParser<ParserType, RangeType>
where
    RangeType: RangeBound<RepeatCountType>,
    It: InputIteratorTrait,
    ParserType: Parser<It>,
    <ParserType as Parser<It>>::Output: VectorOutputSpecialize,
    <<ParserType as Parser<It>>::Output as VectorOutputSpecialize>::Output: Tuple,
{
    type Output = <<ParserType as Parser<It>>::Output as VectorOutputSpecialize>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        // is this how 'template specialization' done in Rust?
        let mut output =
            <<ParserType as Parser<It>>::Output as VectorOutputSpecialize>::new_output();
        let mut it = it;
        let mut count: RepeatCountType = 0;
        loop {
            // check reached max count
            if self.range.contains(&count) && self.range.contains(&(count + 1)) == false {
                return ParseResult {
                    output: Some(output),
                    it: it,
                };
            }
            let res = self.parser.parse(it);
            if let Some(val) = res.output {
                count += 1;
                val.push_this_to_output(&mut output);
                it = res.it;
            } else {
                if self.range.contains(&count) {
                    return ParseResult {
                        output: Some(output),
                        it: res.it,
                    };
                } else {
                    return ParseResult {
                        output: None,
                        it: i0,
                    };
                }
            }
        }
    }
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let i0 = it.clone();
        let mut it = it;
        let mut count: RepeatCountType = 0;
        loop {
            // check reached max count
            if self.range.contains(&count) && self.range.contains(&(count + 1)) == false {
                return ParseResult {
                    output: Some(()),
                    it: it,
                };
            }
            let res = self.parser.match_pattern(it);
            if let Some(_) = res.output {
                count += 1;
                it = res.it;
            } else {
                if self.range.contains(&count) {
                    return ParseResult {
                        output: Some(()),
                        it: res.it,
                    };
                } else {
                    return ParseResult {
                        output: None,
                        it: i0,
                    };
                }
            }
        }
    }
}

impl<ParserType, RangeType> IntoParser for RepeatParser<ParserType, RangeType>
where
    RangeType: RangeBound<RepeatCountType>,
{
    type Into = RepeatParser<ParserType, RangeType>;
    fn into_parser(self) -> Self::Into {
        self
    }
}

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
        let digit_parser = SeqParser::new(digit_parser.clone(), digit_parser);
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
