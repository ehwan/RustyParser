use std::iter::Iterator;
use std::ops::RangeBounds;
use std::vec::Vec;

use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultValue;

#[derive(Debug, Clone)]
pub struct RepeatValueParser<ParserType, RangeType, Idx, It>
where
    It: Iterator + Clone,
    RangeType: RangeBounds<Idx>,
    ParserType: Parser<It> + ResultValue<It>,
    Idx: PartialOrd + PartialEq + PartialOrd<i32> + PartialEq<i32>,
    i32: PartialOrd + PartialEq + PartialOrd<Idx> + PartialEq<Idx>,
{
    parser: ParserType,
    range: RangeType,
    _phantom: std::marker::PhantomData<It>,
    _phantom2: std::marker::PhantomData<Idx>,
}

impl<ParserType, RangeType, Idx, It> RepeatValueParser<ParserType, RangeType, Idx, It>
where
    It: Iterator + Clone,
    RangeType: RangeBounds<Idx>,
    ParserType: Parser<It> + ResultValue<It>,
    Idx: PartialOrd + PartialEq + PartialOrd<i32> + PartialEq<i32>,
    i32: PartialOrd + PartialEq + PartialOrd<Idx> + PartialEq<Idx>,
{
    pub fn new(parser: ParserType, range: RangeType) -> Self {
        Self {
            parser: parser,
            range: range,
            _phantom: std::marker::PhantomData,
            _phantom2: std::marker::PhantomData,
        }
    }
}

impl<ParserType, RangeType, Idx, It> ResultValue<It>
    for RepeatValueParser<ParserType, RangeType, Idx, It>
where
    It: Iterator + Clone,
    RangeType: RangeBounds<Idx>,
    ParserType: Parser<It> + ResultValue<It>,
    Idx: PartialOrd + PartialEq + PartialOrd<i32> + PartialEq<i32>,
    i32: PartialOrd + PartialEq + PartialOrd<Idx> + PartialEq<Idx>,
{
}

impl<ParserType, RangeType, Idx, It> Parser<It>
    for RepeatValueParser<ParserType, RangeType, Idx, It>
where
    It: Iterator + Clone,
    RangeType: RangeBounds<Idx>,
    ParserType: Parser<It> + ResultValue<It>,
    Idx: PartialOrd + PartialEq + PartialOrd<i32> + PartialEq<i32>,
    i32: PartialOrd + PartialEq + PartialOrd<Idx> + PartialEq<Idx>,
{
    type Output = Vec<<ParserType as Parser<It>>::Output>;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let mut vec: Self::Output = Vec::new();
        let mut it = it;
        let mut count = 0;
        let mut next_count = 1;
        loop {
            // check reached max count
            if self.range.contains(&count) && self.range.contains(&next_count) == false {
                return ParseResult {
                    output: Some(vec),
                    it: it,
                };
            }
            let res = self.parser.parse(it);
            if let Some(val) = res.output {
                count = next_count;
                next_count = count + 1;
                vec.push(val);
                it = res.it;
            } else {
                if self.range.contains(&count) {
                    return ParseResult {
                        output: Some(vec),
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
        let mut count = 0;
        let mut next_count = 1;
        loop {
            // check reached max count
            if self.range.contains(&count) && self.range.contains(&next_count) == false {
                return ParseResult {
                    output: Some(()),
                    it: it,
                };
            }
            let res = self.parser.match_pattern(it);
            if let Some(_) = res.output {
                count = next_count;
                next_count = count + 1;
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::singlerange::SingleRangeParser;

    #[test]
    fn success_test1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let repeat_parser = RepeatValueParser::new(digit_parser, 1..=3);

        let str = "123456abcd";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, Some(vec!['1', '2', '3']));
        let rest: String = res.it.collect();
        assert_eq!(rest, "456abcd");
    }
    #[test]
    fn success_test2() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let repeat_parser = RepeatValueParser::new(digit_parser, 1..=6);

        let str = "123456abcd";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, Some(vec!['1', '2', '3', '4', '5', '6']));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn success_test3() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let repeat_parser = RepeatValueParser::new(digit_parser, 4..);

        let str = "1234abcd";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, Some(vec!['1', '2', '3', '4']));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }
    #[test]
    fn fail_test1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let repeat_parser = RepeatValueParser::new(digit_parser, 5..10);

        let str = "1234abcd";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "1234abcd");
    }
}
