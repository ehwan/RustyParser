pub(crate) mod core;
pub(crate) mod example;
pub(crate) mod leaf;
pub(crate) mod wrapper;

use std::ops::RangeBounds;

/// Trait for converting possible types to Parser
pub use crate::core::into_parser::IntoParser;

/// convert the given type to Parser ( if it impl IntoParser )
pub fn into_parser<ParserType: IntoParser>(parser: ParserType) -> ParserType::Into {
    parser.into_parser()
}

/// Parser trait
/// all parsers must implement this trait
pub use core::parser::Parser;

/// struct that holds the result of parsing
/// output: parsed data
/// it: iterator after parsing
pub use core::result::ParseResult;

/// a trait alias that Input Iterator must hold
pub use crate::core::iterator_bound::InputIteratorTrait;

/// parse the input with the given parser
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

/// match the input with the given parser
/// This does not construct the output, just check the input is matched or not.
pub fn match_pattern<ParserType, It>(parser: &ParserType, it: It) -> ParseResult<(), It>
where
    It: InputIteratorTrait,
    ParserType: Parser<It>,
{
    parser.match_pattern(it)
}

/// Check one character is equal to the given character.
pub fn one<CharType>(ch: CharType) -> leaf::singleeq::SingleEqualParser<CharType> {
    leaf::singleeq::SingleEqualParser::new(ch)
}

/// Check one character is in the given range.
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
pub fn chars<'a>(str: &'a str) -> leaf::sliceeq::SliceEqualParser<std::str::Chars<'a>> {
    leaf::sliceeq::SliceEqualParser::new(str.chars())
}

/// This Parser will compare the input string starts with the given slice
pub fn slice<'a, T>(
    str: &'a [T],
) -> leaf::sliceeq::SliceEqualParser<std::iter::Copied<std::slice::Iter<'a, T>>>
where
    T: Clone + Copy,
{
    leaf::sliceeq::SliceEqualParser::new(str.iter().copied())
}

/// This Parser will use the closure to parse the input.
pub fn parser<ParseClosure, ClosureOutput, It>(
    parse_closure: ParseClosure,
) -> leaf::custom::CustomParser<ParseClosure, ClosureOutput, It>
where
    It: InputIteratorTrait,
    ParseClosure: Fn(&mut It) -> Option<ClosureOutput>,
    ClosureOutput: core::tuple::Tuple,
{
    leaf::custom::CustomParser::new(parse_closure)
}

/// This Parser will always success and return the clone of given output.
pub fn constant<Output>(output: Output) -> leaf::constant::ConstantParser<Output>
where
    Output: core::tuple::Tuple + Clone,
{
    leaf::constant::ConstantParser::new(output)
}

/// parser that success if reached end of input
pub fn end() -> leaf::end::EndParser {
    leaf::end::EndParser::new()
}

/// This Parser will always fail.
pub fn fail() -> leaf::fail::Fail {
    leaf::fail::Fail::new()
}

/// Dictionary using trie
/// implementation uses BTreeMap; O(log(N)) search
pub use leaf::dict_btree::DictBTreeParser as DictBTree;

/// Dictionary using trie
/// implementation uses HashMap; O(1) search
pub use leaf::dict_hashmap::DictHashMapParser as DictHashMap;

/// Rc\<Parser\> wrapper;
pub use wrapper::rced::RcedParser as Rc;

/// RefCell\<Parser\> wrapper;
pub use wrapper::refcelled::RefCelledParser as RefCell;

/// a Box\<dyn Parser\> wrapper for iterators of std::str::Chars
/// This can take any parser with Output of `Output`
pub use wrapper::boxed::DynBoxChars;

/// a Box\<dyn Parser\> wrapper for iterators of std::slice::Iter
/// This can take any parser with Output of `Output`
pub use wrapper::boxed::DynBoxSlice;

// ================== useful macros below ==================

/// A binary or combinator
pub use wrapper::or::or;

/// A binary seq combinator
pub use wrapper::seq::seq;

/// Sequence of parsers
/// Example:
/// seq!( parser_a, and_then_b, and_then_c )
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

/// Or combinator of parsers
/// Example:
/// or!( parser_a, or_b, or_c )
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
