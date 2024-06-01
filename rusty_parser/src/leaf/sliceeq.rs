use std::iter::IntoIterator;
use std::iter::Iterator;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

/// This Parser will compare the input string starts with the given string.
/// 'string' must be a iterator of slice &[U] or &str.chars()
#[derive(Debug, Clone, Copy)]
pub struct SliceEqualParser<SliceIter>
where
    SliceIter: IntoIterator + Clone,
{
    string: SliceIter,
}

impl<SliceIter> SliceEqualParser<SliceIter>
where
    SliceIter: IntoIterator + Clone,
{
    pub fn new(string: SliceIter) -> Self {
        SliceEqualParser { string }
    }
}

impl<StringContainer, It> Parser<It> for SliceEqualParser<StringContainer>
where
    StringContainer: IntoIterator + Clone,
    It: InputIteratorTrait,
    <It as Iterator>::Item:
        PartialEq<<<StringContainer as IntoIterator>::IntoIter as Iterator>::Item>,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let mut it = it;
        // use take?
        for ch in self.string.clone() {
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

impl<SliceIter> IntoParser for SliceEqualParser<SliceIter>
where
    SliceIter: IntoIterator + Clone,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

impl<'a> IntoParser for &'a str {
    type Into = SliceEqualParser<std::str::Chars<'a>>;
    fn into_parser(self) -> Self::Into {
        SliceEqualParser::new(self.chars())
    }
}
impl<'a, T> IntoParser for &'a [T]
where
    T: Clone + Copy,
{
    type Into = SliceEqualParser<std::slice::Iter<'a, T>>;
    fn into_parser(self) -> Self::Into {
        SliceEqualParser::new(self.iter())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn success1() {
        let pattern = "hello";
        let parser = SliceEqualParser::new(pattern.chars());

        let str: String = "hello_world!!".to_string();
        let res = parser.parse(str.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "_world!!");
    }

    #[test]
    fn fail1() {
        let pattern = "hello";
        let parser = SliceEqualParser::new(pattern.chars());

        let str: String = "hell_world!!".to_string();
        let res = parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "hell_world!!");
    }
}
