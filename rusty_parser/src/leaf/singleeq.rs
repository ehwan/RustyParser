use std::iter::Iterator;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct SingleEqualParser<CharacterType> {
    pub character: CharacterType,
}

impl<CharacterType> SingleEqualParser<CharacterType> {
    pub fn new(character: CharacterType) -> Self {
        SingleEqualParser { character }
    }
}

impl<CharacterType, It> Parser<It> for SingleEqualParser<CharacterType>
where
    It: InputIteratorTrait,
    <It as Iterator>::Item: PartialEq<CharacterType>,
{
    type Output = (<It as Iterator>::Item,);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = it.next() {
            if val == self.character {
                ParseResult {
                    output: Some((val,)),
                    it,
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
            if val == self.character {
                ParseResult {
                    output: Some(()),
                    it,
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

impl<CharType> IntoParser for SingleEqualParser<CharType> {
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

impl IntoParser for char {
    type Into = SingleEqualParser<char>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for i8 {
    type Into = SingleEqualParser<i8>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for i16 {
    type Into = SingleEqualParser<i16>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for i32 {
    type Into = SingleEqualParser<i32>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for i64 {
    type Into = SingleEqualParser<i64>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for i128 {
    type Into = SingleEqualParser<i128>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for isize {
    type Into = SingleEqualParser<isize>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for u8 {
    type Into = SingleEqualParser<u8>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for u16 {
    type Into = SingleEqualParser<u16>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for u32 {
    type Into = SingleEqualParser<u32>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for u64 {
    type Into = SingleEqualParser<u64>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for u128 {
    type Into = SingleEqualParser<u128>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}
impl IntoParser for usize {
    type Into = SingleEqualParser<usize>;
    fn into_parser(self) -> Self::Into {
        SingleEqualParser::new(self)
    }
}

#[derive(Debug)]
pub struct SingleEqualByParser<CharacterType, Predicate, ItemType>
where
    Predicate: Fn(ItemType, &CharacterType) -> bool,
{
    character: CharacterType,
    predicate: Predicate,
    _phantom: std::marker::PhantomData<ItemType>,
}

impl<CharType, Predicate, ItemType> SingleEqualByParser<CharType, Predicate, ItemType>
where
    Predicate: Fn(ItemType, &CharType) -> bool,
{
    pub fn new(character: CharType, predicate: Predicate) -> Self {
        Self {
            character,
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<CharType, Predicate, ItemType> Clone for SingleEqualByParser<CharType, Predicate, ItemType>
where
    Predicate: Fn(ItemType, &CharType) -> bool + Clone,
    CharType: Clone,
{
    fn clone(&self) -> Self {
        Self {
            character: self.character.clone(),
            predicate: self.predicate.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<CharType, Predicate, ItemType> Copy for SingleEqualByParser<CharType, Predicate, ItemType>
where
    Predicate: Fn(ItemType, &CharType) -> bool + Copy,
    CharType: Copy,
    std::marker::PhantomData<ItemType>: Copy,
{
}

impl<CharType, Predicate, ItemType, It> Parser<It>
    for SingleEqualByParser<CharType, Predicate, ItemType>
where
    It: InputIteratorTrait + Iterator<Item = ItemType>,
    Predicate: Fn(ItemType, &CharType) -> bool,
    ItemType: Clone,
{
    type Output = (<It as Iterator>::Item,);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = it.next() {
            if (self.predicate)(val.clone(), &self.character) {
                ParseResult {
                    output: Some((val,)),
                    it,
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
            if (self.predicate)(val, &self.character) {
                ParseResult {
                    output: Some(()),
                    it,
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

impl<CharType, Predicate, ItemType> IntoParser
    for SingleEqualByParser<CharType, Predicate, ItemType>
where
    Predicate: Fn(ItemType, &CharType) -> bool,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::string::String;

    use super::*;

    #[test]
    fn success1() {
        let a_parser = SingleEqualParser::new('a');
        // success
        let start_with_a_string = String::from("abcde");
        let res = a_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, Some(('a',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn success2() {
        let b_parser = SingleEqualParser::new('b');
        // success
        let start_with_b_string = String::from("bacde");
        let res = b_parser.parse(start_with_b_string.chars());
        assert_eq!(res.output, Some(('b',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "acde");
    }
    #[test]
    fn fail1() {
        let a_parser = SingleEqualParser::new('a');
        // this case is fail
        let start_with_b_string = String::from("bacde");
        let res = a_parser.parse(start_with_b_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bacde");
    }
    #[test]
    fn fail2() {
        let b_parser = SingleEqualParser::new('b');
        // this case is fail
        let start_with_a_string = String::from("abcde");
        let res = b_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }

    #[test]
    fn null() {
        let x_parser = SingleEqualParser::new('x');
        let empty_string = String::from("");
        let res = x_parser.parse(empty_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "");
    }

    #[test]
    fn match_success1() {
        let a_parser = SingleEqualParser::new('a');
        // success
        let start_with_a_string = String::from("abcde");
        let res = a_parser.match_pattern(start_with_a_string.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn match_success2() {
        let b_parser = SingleEqualParser::new('b');
        // success
        let start_with_b_string = String::from("bacde");
        let res = b_parser.match_pattern(start_with_b_string.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "acde");
    }
    #[test]
    fn match_fail1() {
        let a_parser = SingleEqualParser::new('a');
        // this case is fail
        let start_with_b_string = String::from("bacde");
        let res = a_parser.match_pattern(start_with_b_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bacde");
    }
    #[test]
    fn match_fail2() {
        let b_parser = SingleEqualParser::new('b');
        // this case is fail
        let start_with_a_string = String::from("abcde");
        let res = b_parser.match_pattern(start_with_a_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }

    #[test]
    fn match_null() {
        let x_parser = SingleEqualParser::new('x');
        let empty_string = String::from("");
        let res = x_parser.match_pattern(empty_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "");
    }

    #[test]
    fn by_success1() {
        let a_parser = SingleEqualByParser::new('a', |a: char, b: &char| a == *b);
        let start_with_a_string = String::from("abcde");
        let res = a_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, Some(('a',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn by_success2() {
        let b_parser = SingleEqualByParser::new('b', |a: char, b: &char| a == *b);
        let start_with_a_string = String::from("bacde");
        let res = b_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, Some(('b',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "acde");
    }
    #[test]
    fn by_fail1() {
        let a_parser = SingleEqualByParser::new('a', |a: char, b: &char| a == *b);
        let start_with_a_string = String::from("bacde");
        let res = a_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bacde");
    }

    #[test]
    fn by_null() {
        let a_parser = SingleEqualByParser::new('a', |a: char, b: &char| a == *b);
        let res = a_parser.parse("".chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "");
    }
}
