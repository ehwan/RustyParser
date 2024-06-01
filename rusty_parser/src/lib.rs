//! # Rusty Parser
//!
//! A Generic compile-time Parser generator and Pattern Matching Library written in Rust
//!
//! RustyParser provides a set of basic parsers, combinators, and parser-generating functions.
//!
//! This library is designed to work with general iterators, but some functionalities are limited to `std::str::Chars` or `std::iter::Cloned<std::slice::Iter>`.
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
//!
//!
//!
//! Those generated parsers are used to parse the input string, and return the extracted data.
//!
//! `parse(...)` takes a Pattern Object and iterator of input string, then returns `ParseResult<Self::Output, It>`.
//!
//!  `match_pattern(...)` is used
//!  when you only want to check if the pattern is matched or not, without extracting data.
//!  For some parsers, like `repeat`, it is expensive to call `parse(...)` to get the output since it invokes `Vec::push` inside.
//!
//! ### Note
//!  - Since the `parse(...)` internally clones the iterator, the iterator must be cheaply clonable.
//!  - `Output` must be `Tuple`, including `()`. If you want to return a single value, use `(Value,)`.

pub(crate) mod core;
pub(crate) mod leaf;
pub(crate) mod wrapper;

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
    range: RangeType,
) -> leaf::singlerange::SingleRangeParser<RangeType::Into, Idx>
where
    Idx: PartialOrd + PartialEq,
    RangeType: crate::core::range_copyable::ToCopyable,
    RangeType::Into: crate::core::range_copyable::RangeBound<Idx>,
{
    leaf::singlerange::SingleRangeParser::from(range)
}

/// This Parser will compare the input string starts with the given string.
///
/// for borrowing-safety, the lifetime of str must be 'static.
/// for non-static string, use `string()` instead.
///
///
/// `Output`: `()`
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let hello_parser = rp::str("hello");
/// let hello_parser = "hello".into_parser();
/// ```
pub fn str(str: &'static str) -> leaf::stringeq::StaticStrEqualParser {
    leaf::stringeq::StaticStrEqualParser::new(str)
}

/// This Parser will compare the input string starts with the given string.
///
/// This will copy all the characters into `String`, so lifetime belongs to the parser itself.
///
///
/// `Output`: `()`
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let hello_parser = rp::string("hello".to_string());
/// let hello_parser = "hello".to_string().into_parser();
/// ```
pub fn string(str: String) -> leaf::stringeq::StringEqualParser {
    leaf::stringeq::StringEqualParser::new(str)
}

/// This Parser will compare the input starts with the given slice
///
/// for borrowing-safety, the lifetime of slice must be 'static.
/// for non-static slice, use `vec()` instead.
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
pub fn slice<T>(slice: &'static [T]) -> leaf::stringeq::SliceEqualParser<T> {
    leaf::stringeq::SliceEqualParser::new(slice)
}

/// This Parser will compare the input starts with the given slice
///
/// This will copy all the characters into `Vec`, so lifetime belongs to the parser itself.
///
///
/// `Output`: `()`
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let hello_parser = rp::vec(vec![104, 101, 108, 108, 111]);
/// let hello_parser = (vec![104, 101, 108, 108, 111]).into_parser();
/// ```
pub fn vec<T>(v: Vec<T>) -> leaf::stringeq::VecEqualParser<T> {
    leaf::stringeq::VecEqualParser::new(v)
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

/// Check single item with the given closure.
///
/// The closure must be: `Fn(Iterator::Item) -> bool`
///
/// `Output`: `(Iterator::Item,)`
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let parser = rp::check( |ch:char| ch.is_alphabetic() );
/// let res = rp::parse( &parser, "hello".chars() );
///
/// let parser = rp::check( |ch:&i32| ch == &1 );
/// let res = rp::parse( &parser, (&[1,2,3]).iter() );
/// ```
pub fn check<CheckItem, Input>(
    closure: CheckItem,
) -> leaf::check::SingleCheckParser<CheckItem, Input>
where
    CheckItem: Fn(Input) -> bool,
{
    leaf::check::SingleCheckParser::new(closure)
}

/// This parser will match any character.
///
/// `Output`: `(Iterator::Item,)`
///
/// # Example
/// ```rust
/// use rusty_parser as rp;
/// use rp::IntoParser;
///
/// let parser = rp::any();
/// ```
pub fn any() -> leaf::any::AnyParser {
    leaf::any::AnyParser::new()
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

/// A Box\<dyn Parser\> wrapper for iterators of `std::str::Chars`.
///
/// This can take any parser with Output of `Output`.
///
/// Once you wrap the parser with this, you can only use input iterator of `std::str::Chars`.
pub use wrapper::boxed::DynBoxChars;

/// A Box\<dyn Parser\> wrapper for iterators of `std::iter::Cloned<std::slice::Iter>`.
///
/// This can take any parser with Output of `Output`.
///
/// Once you wrap the parser with this, you can only use input iterator of `std::iter::Cloned<std::slice::Iter>`.
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
