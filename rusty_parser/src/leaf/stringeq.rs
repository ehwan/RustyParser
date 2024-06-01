use std::iter::Iterator;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

/// This Parser will compare the input string starts with the given &str.
/// for borrowing-safety, the lifetime of str must be 'static.
/// for non-static string, use StringEqualParser
#[derive(Debug, Clone, Copy)]
pub struct StaticStrEqualParser {
    string: &'static str,
}
impl StaticStrEqualParser {
    pub fn new(string: &'static str) -> Self {
        StaticStrEqualParser { string }
    }
}
impl<It> Parser<It> for StaticStrEqualParser
where
    It: InputIteratorTrait,
    <It as Iterator>::Item: PartialEq<char>,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let mut it = it;
        // use take?
        for ch in self.string.chars() {
            match it.next() {
                Some(ch2) => {
                    if ch2 == ch {
                        continue;
                    } else {
                        return ParseResult {
                            output: None,
                            it: i0,
                        };
                    }
                }
                None => {
                    return ParseResult {
                        output: None,
                        it: i0,
                    }
                }
            }
        }
        ParseResult {
            output: Some(()),
            it,
        }
    }
}
impl IntoParser for StaticStrEqualParser {
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}
impl IntoParser for &'static str {
    type Into = StaticStrEqualParser;
    fn into_parser(self) -> Self::Into {
        StaticStrEqualParser::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct StringEqualParser {
    string: String,
}

impl StringEqualParser {
    pub fn new(string: String) -> Self {
        StringEqualParser { string }
    }
}

impl<It> Parser<It> for StringEqualParser
where
    It: InputIteratorTrait,
    <It as Iterator>::Item: PartialEq<char>,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let mut it = it;
        // use take?
        for ch in self.string.chars() {
            match it.next() {
                Some(ch2) => {
                    if ch2 == ch {
                        continue;
                    } else {
                        return ParseResult {
                            output: None,
                            it: i0,
                        };
                    }
                }
                None => {
                    return ParseResult {
                        output: None,
                        it: i0,
                    }
                }
            }
        }
        ParseResult {
            output: Some(()),
            it,
        }
    }
}

impl IntoParser for StringEqualParser {
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

impl IntoParser for String {
    type Into = StringEqualParser;
    fn into_parser(self) -> Self::Into {
        StringEqualParser::new(self)
    }
}

/// This Parser will compare the input string starts with the given string.
/// 'string' must be a iterator of slice &[U] or &str.chars()
#[derive(Debug, Clone, Copy)]
pub struct SliceEqualParser<T: 'static> {
    slice: &'static [T],
}

impl<T: 'static> SliceEqualParser<T> {
    pub fn new(slice: &'static [T]) -> Self {
        SliceEqualParser { slice }
    }
}

impl<T: 'static, It> Parser<It> for SliceEqualParser<T>
where
    It: InputIteratorTrait,
    <It as Iterator>::Item: PartialEq<T>,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let mut it = it;
        // use take?
        for ch in self.slice.iter() {
            match it.next() {
                Some(ch2) => {
                    if ch2 == *ch {
                        continue;
                    } else {
                        return ParseResult {
                            output: None,
                            it: i0,
                        };
                    }
                }
                None => {
                    return ParseResult {
                        output: None,
                        it: i0,
                    }
                }
            }
        }
        ParseResult {
            output: Some(()),
            it,
        }
    }
}

impl<T: 'static> IntoParser for SliceEqualParser<T> {
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

impl<T: 'static> IntoParser for &'static [T] {
    type Into = SliceEqualParser<T>;
    fn into_parser(self) -> Self::Into {
        SliceEqualParser::new(self)
    }
}

#[derive(Debug, Clone)]
pub struct VecEqualParser<T> {
    vec: Vec<T>,
}

impl<T> VecEqualParser<T> {
    pub fn new(vec: Vec<T>) -> Self {
        VecEqualParser { vec }
    }
}

impl<T, It> Parser<It> for VecEqualParser<T>
where
    It: InputIteratorTrait,
    <It as Iterator>::Item: PartialEq<T>,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let mut it = it;
        // use take?
        for ch in self.vec.iter() {
            match it.next() {
                Some(ch2) => {
                    if ch2 == *ch {
                        continue;
                    } else {
                        return ParseResult {
                            output: None,
                            it: i0,
                        };
                    }
                }
                None => {
                    return ParseResult {
                        output: None,
                        it: i0,
                    }
                }
            }
        }
        ParseResult {
            output: Some(()),
            it,
        }
    }
}

impl<T> IntoParser for VecEqualParser<T> {
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

impl<T> IntoParser for Vec<T> {
    type Into = VecEqualParser<T>;
    fn into_parser(self) -> Self::Into {
        VecEqualParser::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn success1() {
        let parser = StaticStrEqualParser::new("hello");

        let res = parser.parse("hello_world!!".chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "_world!!");
    }

    #[test]
    fn fail1() {
        let parser = StaticStrEqualParser::new("hello");

        let res = parser.parse("hell_world!!".chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "hell_world!!");
    }
}
