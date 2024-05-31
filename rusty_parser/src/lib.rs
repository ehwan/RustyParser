//! # Rusty Parser
//!
//! A Generic compile-time Parser generator and Pattern Matching Library written in Rust
//!
//! Creating custom parsers for various data formats.
//!
//! # Example
//! ```rust
//! use rusty_parser as rp;
//! use rp::IntoParser;
//
//! // define pattern
//! // digit: [0-9]
//! // this will match one digit, and returns (char,), the character it parsed
//! let digit_parser = rp::range('0'..='9');
//
//! // define pattern
//! // num: digit+
//! // this will match one or more digits, and returns (Vec<char>,), the character it parsed
//! let num_parser = digit_parser.repeat(0..);
//
//! // map the output
//! // ( Vec<char>, )  -->  (i32, )
//! let num_parser = num_parser.map(|(digits,): (Vec<char>,)| -> (i32,) {
//!     let mut num = 0;
//!     for ch in digits {
//!         num = num * 10 + (ch as i32 - '0' as i32);
//!     }
//!     (num,)
//! });
//
//! // parse input iterator with given pattern, and return the result
//! let res = rp::parse(&num_parser, "123456hello_world".chars());
//
//! // res contains the result of parsing
//! assert_eq!(res.output.unwrap(), (123456,));
//
//! // res.it: iterator after parsing
//! // here, '123456' is parsed, so the rest is "hello_world"
//! assert_eq!(res.it.collect::<String>(), "hello_world");
//! ```

pub(crate) mod core;
pub(crate) mod leaf;
pub(crate) mod wrapper;

use std::ops::RangeBounds;

/// Trait for converting possible types to Parser.
///
/// This trait contains useful member functions for parser generation.
pub use crate::core::into_parser::IntoParser;

/// convert the given type to Parser ( if it impl IntoParser )
pub fn into_parser<ParserType: IntoParser>(parser: ParserType) -> ParserType::Into {
    parser.into_parser()
}

/// Parser trait.
///
/// for parse(), match_pattern() functions
pub use core::parser::Parser;

/// struct that holds the result of parsing.
pub use core::result::ParseResult;

/// A trait alias that Input Iterator must hold.
pub use crate::core::iterator_bound::InputIteratorTrait;

/// Parse the input with the given parser.
pub fn parse<ParserType, It>(
    parser: &ParserType,
    it: It,
) -> ParseResult<<ParserType as Parser<It>>::Output, It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    parser.parse(it)
}

/// Match the input with the given parser.
///
/// This does not construct the output, just check the input is matched or not.
pub fn match_pattern<ParserType, It>(parser: &ParserType, it: It) -> ParseResult<(), It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    parser.match_pattern(it)
}

/// Check one character is equal to the given character.
///
/// `Output`: `(Iterator::Item,)`
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let a_parser = rp::one('a');
/// let a_parser = 'a'.into_parser();
/// ```
pub fn one<CharType>(ch: CharType) -> leaf::singleeq::SingleEqualParser<CharType> {
    leaf::singleeq::SingleEqualParser::new(ch)
}

/// Check one character is in the given range.
///
/// `Output`: `(Iterator::Item,)`
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let digit_parser = rp::range( '0'..='9' );
/// let digit_parser = ('0'..='9').into_parser();
/// ```
pub fn range<RangeType, Idx>(
    range_: RangeType,
) -> leaf::singlerange::SingleRangeParser<RangeType, Idx>
where
    Idx: PartialOrd + PartialEq,
    RangeType: RangeBounds<Idx>,
{
    leaf::singlerange::SingleRangeParser::new(range_)
}

/// This Parser will compare the input string starts with the given string.
///
/// `Output`: `()`
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let hello_parser = rp::chars("hello");
/// let hello_parser = "hello".into_parser();
/// ```
pub fn chars<'a>(str: &'a str) -> leaf::sliceeq::SliceEqualParser<std::str::Chars<'a>> {
    leaf::sliceeq::SliceEqualParser::new(str.chars())
}

/// This Parser will compare the input string starts with the given slice
///
/// `Output`: `()`
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let hello_parser = rp::slice(&[104, 101, 108, 108, 111]);
/// let hello_parser = (&[104, 101, 108, 108, 111]).into_parser();
/// ```
pub fn slice<'a, T>(
    str: &'a [T],
) -> leaf::sliceeq::SliceEqualParser<std::iter::Copied<std::slice::Iter<'a, T>>>
where
    T: Clone + Copy,
{
    leaf::sliceeq::SliceEqualParser::new(str.iter().copied())
}

/// This Parser will always success and return the clone of given output.
///
/// `Output`: Output you provided
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let parser = rp::constant( (1, 2, 3) );
/// ```
pub fn constant<Output>(output: Output) -> leaf::constant::ConstantParser<Output>
where
    Output: core::tuple::Tuple + Clone,
{
    leaf::constant::ConstantParser::new(output)
}

/// Parser that success if reached end of input
///
/// `Output`: `()`
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let end_parser = rp::end();
/// ```
pub fn end() -> leaf::end::EndParser {
    leaf::end::EndParser::new()
}

/// This Parser will always fail.
///
/// `Output`: `()`
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let parser = rp::fail();
/// ```
pub fn fail() -> leaf::fail::Fail {
    leaf::fail::Fail::new()
}

/// Dictionary using trie, implementation uses BTreeMap; O(log(N)) search.
///
/// `Output`: Output you inserted
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let mut parser = rp::DictBTree::new();
///
/// parser.insert("hello".chars(), (1,));
/// parser.insert("hello_world".chars(), (2,));
/// parser.insert("world".chars(), (3,));
///
/// // this will match as long as possible
/// let res = rp::parse(&parser, "hello_world_abcdefg".chars());
/// assert_eq!(res.output.unwrap(), (2,));
/// // 'hello_world' is parsed, so the rest is "_abcdefg"
/// assert_eq!(res.it.collect::<String>(), "_abcdefg");
///
/// // match 'hello' only
/// let res = rp::parse(&parser, "hello_wo".chars());
/// assert_eq!(res.output.unwrap(), (1,));
/// ```
pub use leaf::dict_btree::DictBTreeParser as DictBTree;

/// Dictionary using trie, implementation uses HashMap; O(1) search.
///
/// `Output`: Output you inserted
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let mut parser = rp::DictHashMap::new();
///
/// parser.insert("hello".chars(), (1,));
/// parser.insert("hello_world".chars(), (2,));
/// parser.insert("world".chars(), (3,));
///
/// // this will match as long as possible
/// let res = rp::parse(&parser, "hello_world_abcdefg".chars());
/// assert_eq!(res.output.unwrap(), (2,));
/// // 'hello_world' is parsed, so the rest is "_abcdefg"
/// assert_eq!(res.it.collect::<String>(), "_abcdefg");
///
/// // match 'hello' only
/// let res = rp::parse(&parser, "hello_wo".chars());
/// assert_eq!(res.output.unwrap(), (1,));
/// ```
pub use leaf::dict_hashmap::DictHashMapParser as DictHashMap;

/// Rc\<Parser\> wrapper.
pub use wrapper::rced::RcedParser as Rc;

/// RefCell\<Parser\> wrapper.
pub use wrapper::refcelled::RefCelledParser as RefCell;

/// A Box\<dyn Parser\> wrapper for iterators of std::str::Chars.
/// This can take any parser with Output of `Output`.
pub use wrapper::boxed::DynBoxChars;

/// A Box\<dyn Parser\> wrapper for iterators of std::slice::Iter.
/// This can take any parser with Output of `Output`.
pub use wrapper::boxed::DynBoxSlice;

// ================== useful macros below ==================

/// A binary or combinator
pub use wrapper::or::or;

/// A binary seq combinator
pub use wrapper::seq::seq;

/// A macro for creating a sequence of parsers.
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// // 'a', and then 'b'
/// let ab_parser = rp::seq!('a', 'b', 'c'); // IntoParser for char
///
#[macro_export]
macro_rules! seq {
    // Base case: just return
    ($single:expr) => {
        $single
    };

    // N arguments
    ($first:expr, $($rest:expr),+) => {
        $crate::seq($first, $crate::seq!($($rest),+))
    };
}

/// A macro for creating or combination of parsers.
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// // 'a' or 'b' or 'c'
/// let ab_parser = rp::or!('a', 'b', 'c'); // IntoParser for char
/// ```
#[macro_export]
macro_rules! or {
    // Base case: just return
    ($single:expr) => {
        $single
    };

    // N arguments
    ($first:expr, $($rest:expr),+) => {
        $crate::or( $first, $crate::or!($($rest),+) )
    };
}
