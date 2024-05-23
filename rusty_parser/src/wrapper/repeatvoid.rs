use std::iter::Iterator;
use std::ops::RangeBounds;

use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultVoid;

use rusty_parser_derive::ResultVoid;

#[derive(Debug, Clone, ResultVoid)]
pub struct RepeatVoidParser<ParserType, RangeType, Idx, It>
where
    It: Iterator + Clone,
    RangeType: RangeBounds<Idx>,
    ParserType: Parser<It> + ResultVoid,
    Idx: PartialOrd + PartialEq + PartialOrd<i32> + PartialEq<i32>,
    i32: PartialOrd + PartialEq + PartialOrd<Idx> + PartialEq<Idx>,
{
    parser: ParserType,
    range: RangeType,
    _phantom: std::marker::PhantomData<It>,
    _phantom2: std::marker::PhantomData<Idx>,
}

impl<ParserType, RangeType, Idx, It> RepeatVoidParser<ParserType, RangeType, Idx, It>
where
    It: Iterator + Clone,
    RangeType: RangeBounds<Idx>,
    ParserType: Parser<It> + ResultVoid,
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

impl<ParserType, RangeType, Idx, It> Parser<It> for RepeatVoidParser<ParserType, RangeType, Idx, It>
where
    It: Iterator + Clone,
    RangeType: RangeBounds<Idx>,
    ParserType: Parser<It> + ResultVoid,
    Idx: PartialOrd + PartialEq + PartialOrd<i32> + PartialEq<i32>,
    i32: PartialOrd + PartialEq + PartialOrd<Idx> + PartialEq<Idx>,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
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
            let res = self.parser.parse(it);
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
    fn match_pattern(&self, it: It) -> ParseResult<Self::Output, It> {
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
    use crate::core::stringeq::StringEqualParser;

    #[test]
    fn success_test1() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let repeat_parser = RepeatVoidParser::new(hello_parser, 1..=3);

        let str = "hellohellohellohello";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(rest, "hello");
    }
    #[test]
    fn success_test2() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let repeat_parser = RepeatVoidParser::new(hello_parser, ..=4);

        let str = "hellohellohellohello";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(rest, "");
    }
    #[test]
    fn success_test3() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let repeat_parser = RepeatVoidParser::new(hello_parser, 4..);

        let str = "hellohellohellohello";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(rest, "");
    }
    #[test]
    fn fail_test1() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let repeat_parser = RepeatVoidParser::new(hello_parser, 5..=10);

        let str = "hellohellohellohello";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "hellohellohellohello");
    }
    #[test]
    fn fail_test2() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let repeat_parser = RepeatVoidParser::new(hello_parser, 5..=10);

        let str = "hellohellhellohello";
        let res = repeat_parser.parse(str.chars());

        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "hellohellhellohello");
    }
}
