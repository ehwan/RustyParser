use std::iter::Iterator;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

/// This Parser will compare the input string starts with the given &str.
/// for borrowing-safety, the lifetime of str must be 'static.
/// for non-static string, use StringEqualParser
#[derive(Debug, Clone, Copy)]
pub struct StrEqualParser<'a> {
    string: &'a str,
}
impl<'a> StrEqualParser<'a> {
    pub fn new(string: &'a str) -> Self {
        StrEqualParser { string }
    }
}
impl<'a, It> Parser<It> for StrEqualParser<'a>
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
impl<'a> IntoParser for StrEqualParser<'a> {
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}
impl IntoParser for &'static str {
    type Into = StrEqualParser<'static>;
    fn into_parser(self) -> Self::Into {
        StrEqualParser::new(self)
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
        StrEqualParser::new(&self.string).parse(it)
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

#[derive(Debug, Clone, Copy)]
pub struct StrEqualByParser<'a, Predicate, ItemType>
where
    Predicate: Fn(ItemType, char) -> bool,
{
    string: &'a str,
    predicate: Predicate,
    _phantom: std::marker::PhantomData<ItemType>,
}
impl<'a, Predicate, ItemType> StrEqualByParser<'a, Predicate, ItemType>
where
    Predicate: Fn(ItemType, char) -> bool,
{
    pub fn new(string: &'a str, predicate: Predicate) -> Self {
        Self {
            string,
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<'a, Predicate, ItemType, It> Parser<It> for StrEqualByParser<'a, Predicate, ItemType>
where
    Predicate: Fn(ItemType, char) -> bool,
    It: InputIteratorTrait + Iterator<Item = ItemType>,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let mut it = it;
        // use take?
        for ch in self.string.chars() {
            match it.next() {
                Some(ch2) => {
                    if (self.predicate)(ch2, ch) {
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
impl<'a, Predicate, ItemType> IntoParser for StrEqualByParser<'a, Predicate, ItemType>
where
    Predicate: Fn(ItemType, char) -> bool,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[derive(Debug, Clone)]
pub struct StringEqualByParser<Predicate, ItemType>
where
    Predicate: Fn(ItemType, char) -> bool,
{
    string: String,
    predicate: Predicate,
    _phantom: std::marker::PhantomData<ItemType>,
}

impl<Predicate, ItemType> StringEqualByParser<Predicate, ItemType>
where
    Predicate: Fn(ItemType, char) -> bool,
{
    pub fn new(string: String, predicate: Predicate) -> Self {
        Self {
            string,
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<Predicate, ItemType, It> Parser<It> for StringEqualByParser<Predicate, ItemType>
where
    It: InputIteratorTrait + Iterator<Item = ItemType>,
    Predicate: Fn(ItemType, char) -> bool,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let mut it = it;
        // use take?
        for ch in self.string.chars() {
            match it.next() {
                Some(ch2) => {
                    if (self.predicate)(ch2, ch) {
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

impl<Predicate, ItemType> IntoParser for StringEqualByParser<Predicate, ItemType>
where
    Predicate: Fn(ItemType, char) -> bool,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

/// This Parser will compare the input string starts with the given string.
/// 'string' must be a iterator of slice &[U] or &str.chars()
#[derive(Debug, Clone, Copy)]
pub struct SliceEqualParser<'a, T: 'a> {
    slice: &'a [T],
}

impl<'a, T: 'a> SliceEqualParser<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        SliceEqualParser { slice }
    }
}

impl<'a, T: 'a, It> Parser<It> for SliceEqualParser<'a, T>
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

impl<'a, T: 'a> IntoParser for SliceEqualParser<'a, T> {
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

impl<T: 'static> IntoParser for &'static [T] {
    type Into = SliceEqualParser<'static, T>;
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
        SliceEqualParser::new(&self.vec).parse(it)
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

#[derive(Debug, Clone, Copy)]
pub struct SliceEqualByParser<'a, T: 'a, Predicate, ItemType>
where
    Predicate: Fn(ItemType, &T) -> bool,
{
    slice: &'a [T],
    predicate: Predicate,
    _phantom: std::marker::PhantomData<ItemType>,
}

impl<'a, T: 'a, Predicate, ItemType> SliceEqualByParser<'a, T, Predicate, ItemType>
where
    Predicate: Fn(ItemType, &T) -> bool,
{
    pub fn new(slice: &'a [T], predicate: Predicate) -> Self {
        Self {
            slice,
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, T: 'a, Predicate, ItemType, It> Parser<It>
    for SliceEqualByParser<'a, T, Predicate, ItemType>
where
    It: InputIteratorTrait + Iterator<Item = ItemType>,
    Predicate: Fn(ItemType, &T) -> bool,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let mut it = it;
        // use take?
        for ch in self.slice.iter() {
            match it.next() {
                Some(ch2) => {
                    if (self.predicate)(ch2, ch) {
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

impl<'a, T: 'a, Predicate, ItemType> IntoParser for SliceEqualByParser<'a, T, Predicate, ItemType>
where
    Predicate: Fn(ItemType, &T) -> bool,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[derive(Debug, Clone)]
pub struct VecEqualByParser<T, Predicate, ItemType>
where
    Predicate: Fn(ItemType, &T) -> bool,
{
    vec: Vec<T>,
    predicate: Predicate,
    _phantom: std::marker::PhantomData<ItemType>,
}

impl<T, Predicate, ItemType> VecEqualByParser<T, Predicate, ItemType>
where
    Predicate: Fn(ItemType, &T) -> bool,
{
    pub fn new(vec: Vec<T>, predicate: Predicate) -> Self {
        VecEqualByParser {
            vec,
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, Predicate, ItemType, It> Parser<It> for VecEqualByParser<T, Predicate, ItemType>
where
    It: InputIteratorTrait + Iterator<Item = ItemType>,
    Predicate: Fn(ItemType, &T) -> bool,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let mut it = it;
        // use take?
        for ch in self.vec.iter() {
            match it.next() {
                Some(ch2) => {
                    if (self.predicate)(ch2, ch) {
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

impl<T, Predicate, ItemType> IntoParser for VecEqualByParser<T, Predicate, ItemType>
where
    Predicate: Fn(ItemType, &T) -> bool,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn success1() {
        let parser = StrEqualParser::new("hello");

        let res = parser.parse("hello_world!!".chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "_world!!");
    }

    #[test]
    fn success2() {
        let parser = StrEqualByParser::new("hello", |a: char, b: char| a.to_ascii_lowercase() == b);

        let res = parser.parse("HeLlo_world!!".chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "_world!!");
    }

    #[test]
    fn fail1() {
        let parser = StrEqualParser::new("hello");

        let res = parser.parse("hell_world!!".chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "hell_world!!");
    }
}
