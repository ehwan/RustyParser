use std::ops::RangeBounds;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct SingleRangeParser<RangeType, Idx>
where
    Idx: PartialOrd + PartialEq,
    RangeType: RangeBounds<Idx>,
{
    pub range: RangeType,
    _phantom: std::marker::PhantomData<Idx>,
}

impl<RangeType, Idx> SingleRangeParser<RangeType, Idx>
where
    Idx: PartialOrd + PartialEq,
    RangeType: RangeBounds<Idx>,
{
    pub fn new(range: RangeType) -> Self {
        SingleRangeParser {
            range: range,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<RangeType, It, Idx> Parser<It> for SingleRangeParser<RangeType, Idx>
where
    It: InputIteratorTrait,
    Idx: PartialOrd
        + PartialEq
        + PartialOrd<<It as Iterator>::Item>
        + PartialEq<<It as Iterator>::Item>,
    <It as Iterator>::Item: PartialOrd<Idx> + PartialEq<Idx>,
    RangeType: RangeBounds<Idx>,
{
    type Output = (<It as Iterator>::Item,);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = it.next() {
            if self.range.contains(&val) {
                ParseResult {
                    output: Some((val,)),
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

impl<RangeType, Idx> IntoParser for SingleRangeParser<RangeType, Idx>
where
    Idx: PartialOrd + PartialEq,
    RangeType: RangeBounds<Idx>,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}
impl<Idx> IntoParser for std::ops::Range<Idx>
where
    Idx: PartialOrd + PartialEq,
{
    type Into = SingleRangeParser<std::ops::Range<Idx>, Idx>;
    fn into_parser(self) -> Self::Into {
        SingleRangeParser::new(self)
    }
}
impl<Idx> IntoParser for std::ops::RangeFrom<Idx>
where
    Idx: PartialOrd + PartialEq,
{
    type Into = SingleRangeParser<std::ops::RangeFrom<Idx>, Idx>;
    fn into_parser(self) -> Self::Into {
        SingleRangeParser::new(self)
    }
}
impl<Idx> IntoParser for std::ops::RangeTo<Idx>
where
    Idx: PartialOrd + PartialEq,
{
    type Into = SingleRangeParser<std::ops::RangeTo<Idx>, Idx>;
    fn into_parser(self) -> Self::Into {
        SingleRangeParser::new(self)
    }
}
impl<Idx> IntoParser for std::ops::RangeInclusive<Idx>
where
    Idx: PartialOrd + PartialEq,
{
    type Into = SingleRangeParser<std::ops::RangeInclusive<Idx>, Idx>;
    fn into_parser(self) -> Self::Into {
        SingleRangeParser::new(self)
    }
}
impl<Idx> IntoParser for std::ops::RangeToInclusive<Idx>
where
    Idx: PartialOrd + PartialEq,
{
    type Into = SingleRangeParser<std::ops::RangeToInclusive<Idx>, Idx>;
    fn into_parser(self) -> Self::Into {
        SingleRangeParser::new(self)
    }
}

#[cfg(test)]
mod tests {
    use std::string::String;

    use super::*;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let start_with_digit_string = String::from("0abcd");
        let res = digit_parser.parse(start_with_digit_string.chars());
        assert_eq!(res.output, Some(('0',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcd");
    }
    #[test]
    fn success2() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let start_with_digit_string = String::from("4abcd");
        let res = digit_parser.parse(start_with_digit_string.chars());
        assert_eq!(res.output, Some(('4',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcd");
    }
    #[test]
    fn success3() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let start_with_digit_string = String::from("9abcd");
        let res = digit_parser.parse(start_with_digit_string.chars());
        assert_eq!(res.output, Some(('9',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcd");
    }
    #[test]
    fn fail1() {
        let digit_parser = SingleRangeParser::new('0'..'9');
        let start_with_alpha_string = String::from("9abcd");
        let res = digit_parser.parse(start_with_alpha_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "9abcd");
    }
    #[test]
    fn success4() {
        // alpha parser tests a character is in range of 'a'..'z' ( z is not included! )
        let alpha_parser = SingleRangeParser::new('a'..'z');
        let start_with_alpha_string = String::from("ybcde");
        let res = alpha_parser.parse(start_with_alpha_string.chars());
        assert_eq!(res.output, Some(('y',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }

    #[test]
    fn match_success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let start_with_digit_string = String::from("0abcd");
        let res = digit_parser.match_pattern(start_with_digit_string.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcd");
    }
    #[test]
    fn match_success2() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let start_with_digit_string = String::from("4abcd");
        let res = digit_parser.match_pattern(start_with_digit_string.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcd");
    }
    #[test]
    fn match_success3() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let start_with_digit_string = String::from("9abcd");
        let res = digit_parser.match_pattern(start_with_digit_string.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcd");
    }
    #[test]
    fn match_fail1() {
        let digit_parser = SingleRangeParser::new('0'..'9');
        let start_with_alpha_string = String::from("9abcd");
        let res = digit_parser.match_pattern(start_with_alpha_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "9abcd");
    }
    #[test]
    fn match_success4() {
        // alpha parser tests a character is in range of 'a'..'z' ( z is not included! )
        let alpha_parser = SingleRangeParser::new('a'..'z');
        let start_with_alpha_string = String::from("ybcde");
        let res = alpha_parser.match_pattern(start_with_alpha_string.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
}
