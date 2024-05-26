pub(crate) mod core;
pub(crate) mod example;
pub(crate) mod wrapper;

use std::ops::RangeBounds;

pub use core::parser::Parser;
pub use core::result::ParseResult;

use crate::core::iterator_bound::InputIteratorTrait;

pub fn one<CharType, It>(ch: CharType) -> core::singleeq::SingleEqualParser<CharType, It>
where
    It: InputIteratorTrait,
    <It as Iterator>::Item: PartialEq<CharType>,
{
    core::singleeq::SingleEqualParser::new(ch)
}

pub fn range<RangeType, Idx, It>(
    range_: RangeType,
) -> core::singlerange::SingleRangeParser<RangeType, Idx, It>
where
    It: InputIteratorTrait,
    Idx: PartialOrd
        + PartialEq
        + PartialOrd<<It as Iterator>::Item>
        + PartialEq<<It as Iterator>::Item>,
    <It as Iterator>::Item: PartialOrd<Idx> + PartialEq<Idx>,
    RangeType: RangeBounds<Idx>,
{
    core::singlerange::SingleRangeParser::new(range_)
}

// This Parser will compare the input string starts with the given string.
// 'string' may be a iterator returned by 'chars()', etc.
// string must be cheaply cloneable.
// since iterator from 'chars()' is borrowed reference to the original string,
// it is cheaply cloneable.
pub fn string<CharIntoIter, It>(
    str: CharIntoIter,
) -> core::stringeq::StringEqualParser<CharIntoIter, It>
where
    CharIntoIter: IntoIterator + Clone,
    It: InputIteratorTrait,
    <It as Iterator>::Item: PartialEq<<<CharIntoIter as IntoIterator>::IntoIter as Iterator>::Item>,
{
    core::stringeq::StringEqualParser::new(str)
}

// This Parser will use the closure to parse the input.
pub fn parser<ParseClosure, ClosureOutput, It>(
    parse_closure: ParseClosure,
) -> core::custom::CustomParser<ParseClosure, ClosureOutput, It>
where
    It: InputIteratorTrait,
    ParseClosure: Fn(&mut It) -> Option<ClosureOutput>,
    ClosureOutput: core::tuple::Tuple,
{
    core::custom::CustomParser::new(parse_closure)
}

pub use core::dict_btree::DictBTreeParser as DictBTree;
pub use core::dict_hashmap::DictHashMapParser as DictHashMap;
// parser that success if reached end of input
pub use core::end::EndParser as End;

pub use wrapper::boxed::BoxedParser as Boxed;
pub use wrapper::rced::RcedParser as RCed;
pub use wrapper::refcelled::RefCelledParser as RefCelled;
