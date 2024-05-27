pub(crate) mod core;
pub(crate) mod example;
pub(crate) mod leaf;
pub(crate) mod wrapper;

use std::ops::RangeBounds;

pub use core::parser::Parser;
pub use core::result::ParseResult;

use crate::core::iterator_bound::InputIteratorTrait;

pub fn one<CharType>(ch: CharType) -> leaf::singleeq::SingleEqualParser<CharType> {
    leaf::singleeq::SingleEqualParser::new(ch)
}

pub fn range<RangeType, Idx>(
    range_: RangeType,
) -> leaf::singlerange::SingleRangeParser<RangeType, Idx>
where
    Idx: PartialOrd + PartialEq,
    RangeType: RangeBounds<Idx>,
{
    leaf::singlerange::SingleRangeParser::new(range_)
}

// This Parser will compare the input string starts with the given string.
// 'string' may be a iterator returned by 'chars()', etc.
// string must be cheaply cloneable.
// since iterator from 'chars()' is borrowed reference to the original string,
// it is cheaply cloneable.
pub fn string<CharIntoIter>(str: CharIntoIter) -> leaf::stringeq::StringEqualParser<CharIntoIter>
where
    CharIntoIter: IntoIterator + Clone,
{
    leaf::stringeq::StringEqualParser::new(str)
}

// This Parser will use the closure to parse the input.
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

// This Parser will always success and return the clone of given output.
pub fn constant<Output>(output: Output) -> leaf::constant::ConstantParser<Output>
where
    Output: core::tuple::Tuple + Clone,
{
    leaf::constant::ConstantParser::new(output)
}

// parser that success if reached end of input
pub fn end() -> leaf::end::EndParser {
    leaf::end::EndParser::new()
}

// This Parser will always fail.
pub fn fail() -> leaf::fail::Fail {
    leaf::fail::Fail::new()
}

pub use leaf::dict_btree::DictBTreeParser as DictBTree;
pub use leaf::dict_hashmap::DictHashMapParser as DictHashMap;

pub use wrapper::boxed::BoxedParser as Boxed;
pub use wrapper::rced::RcedParser as RCed;
pub use wrapper::refcelled::RefCelledParser as RefCelled;

// ================== useful macros below ==================

#[macro_export]
macro_rules! seq {
    // Base case: just return
    ($single:expr) => {
        $single
    };

    // N arguments
    ($first:expr, $($rest:expr),+) => {
        $first.seq($crate::seq!($($rest),+))
    };
}

#[macro_export]
macro_rules! or_ {
    // Base case: just return
    ($single:expr) => {
        $single
    };

    // N arguments
    ($first:expr, $($rest:expr),+) => {
        $first.or_($crate::or_!($($rest),+))
    };
}
