use std::iter::Iterator;
use std::ops::RangeBounds;

use super::result::ParseResult;
use super::traits::Parser;
use super::traits::ResultValue;

#[derive(Debug, Clone)]
pub struct SingleRangeParser<RangeType, Idx, It>
where
    It: Iterator + Clone,
    Idx: PartialOrd
        + PartialEq
        + PartialOrd<<It as Iterator>::Item>
        + PartialEq<<It as Iterator>::Item>,
    <It as Iterator>::Item: PartialOrd<Idx> + PartialEq<Idx>,
    RangeType: RangeBounds<Idx>,
{
    pub range: RangeType,
    _phantom: std::marker::PhantomData<Idx>,
    _phantom2: std::marker::PhantomData<It>,
}

impl<RangeType, It, Idx> SingleRangeParser<RangeType, Idx, It>
where
    It: Iterator + Clone,
    Idx: PartialOrd
        + PartialEq
        + PartialOrd<<It as Iterator>::Item>
        + PartialEq<<It as Iterator>::Item>,
    <It as Iterator>::Item: PartialOrd<Idx> + PartialEq<Idx>,
    RangeType: RangeBounds<Idx>,
{
    pub fn new(range: RangeType) -> SingleRangeParser<RangeType, Idx, It> {
        SingleRangeParser {
            range: range,
            _phantom: std::marker::PhantomData,
            _phantom2: std::marker::PhantomData,
        }
    }
}
impl<RangeType, It, Idx> ResultValue<It> for SingleRangeParser<RangeType, Idx, It>
where
    It: Iterator + Clone,
    Idx: PartialOrd
        + PartialEq
        + PartialOrd<<It as Iterator>::Item>
        + PartialEq<<It as Iterator>::Item>,
    <It as Iterator>::Item: PartialOrd<Idx> + PartialEq<Idx>,
    RangeType: RangeBounds<Idx>,
{
}

impl<RangeType, It, Idx> Parser<It> for SingleRangeParser<RangeType, Idx, It>
where
    It: Iterator + Clone,
    Idx: PartialOrd
        + PartialEq
        + PartialOrd<<It as Iterator>::Item>
        + PartialEq<<It as Iterator>::Item>,
    <It as Iterator>::Item: PartialOrd<Idx> + PartialEq<Idx>,
    RangeType: RangeBounds<Idx>,
{
    type Output = <It as Iterator>::Item;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = it.next() {
            if self.range.contains(&val) {
                ParseResult {
                    output: Some(val),
                    it: it,
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
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = it.next() {
            if self.range.contains(&val) {
                ParseResult {
                    output: Some(()),
                    it: it,
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
mod tests {
    use std::string::String;

    use super::super::traits::Parser;
    use super::SingleRangeParser;

    #[test]
    fn parse_digit_test_start() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let start_with_digit_string = String::from("0abcd");
        let res = digit_parser.parse(start_with_digit_string.chars());
        assert_eq!(res.output, Some('0'));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcd");
    }
    #[test]
    fn parse_digit_test_mid() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let start_with_digit_string = String::from("4abcd");
        let res = digit_parser.parse(start_with_digit_string.chars());
        assert_eq!(res.output, Some('4'));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcd");
    }
    #[test]
    fn parse_digit_test_end() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let start_with_digit_string = String::from("9abcd");
        let res = digit_parser.parse(start_with_digit_string.chars());
        assert_eq!(res.output, Some('9'));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcd");
    }
    #[test]
    fn parse_digit_test_fail() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let start_with_alpha_string = String::from("abcde");
        let res = digit_parser.parse(start_with_alpha_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }
    #[test]
    fn parse_alpha_test_start() {
        // alpha parser tests a character is in range of 'a'..'z' ( z is not included! )
        let alpha_parser = SingleRangeParser::new('a'..'z');
        let start_with_alpha_string = String::from("abcde");
        let res = alpha_parser.parse(start_with_alpha_string.chars());
        assert_eq!(res.output, Some('a'));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn parse_alpha_test_mid() {
        // alpha parser tests a character is in range of 'a'..'z' ( z is not included! )
        let alpha_parser = SingleRangeParser::new('a'..'z');
        let start_with_alpha_string = String::from("gbcde");
        let res = alpha_parser.parse(start_with_alpha_string.chars());
        assert_eq!(res.output, Some('g'));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn parse_alpha_test_end() {
        // alpha parser tests a character is in range of 'a'..'z' ( z is not included! )
        let alpha_parser = SingleRangeParser::new('a'..'z');
        let start_with_alpha_string = String::from("zbcde");
        let res = alpha_parser.parse(start_with_alpha_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "zbcde");
    }
    #[test]
    fn parse_alpha_test_end_inclusive() {
        // alpha parser tests a character is in range of 'a'..'z' ( z is not included! )
        let alpha_parser = SingleRangeParser::new('a'..'z');
        let start_with_alpha_string = String::from("ybcde");
        let res = alpha_parser.parse(start_with_alpha_string.chars());
        assert_eq!(res.output, Some('y'));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
}
