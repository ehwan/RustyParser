pub(crate) mod core;
pub(crate) mod test;
pub(crate) mod wrapper;

use std::ops::RangeBounds;

pub use core::parser::Parser;
pub use core::result::ParseResult;

pub fn one<CharType, It>(ch: CharType) -> core::singleeq::SingleEqualParser<CharType, It>
where
    It: Iterator + Clone,
    <It as Iterator>::Item: PartialEq<CharType>,
{
    core::singleeq::SingleEqualParser::new(ch)
}

pub fn range<RangeType, Idx, It>(
    range_: RangeType,
) -> core::singlerange::SingleRangeParser<RangeType, Idx, It>
where
    It: Iterator + Clone,
    Idx: PartialOrd
        + PartialEq
        + PartialOrd<<It as Iterator>::Item>
        + PartialEq<<It as Iterator>::Item>,
    <It as Iterator>::Item: PartialOrd<Idx> + PartialEq<Idx>,
    RangeType: RangeBounds<Idx>,
{
    core::singlerange::SingleRangeParser::new(range_)
}

pub fn string<CharIntoIter, It>(
    str: CharIntoIter,
) -> core::stringeq::StringEqualParser<CharIntoIter, It>
where
    CharIntoIter: IntoIterator + Clone,
    It: Iterator + Clone,
    <It as Iterator>::Item: PartialEq<<<CharIntoIter as IntoIterator>::IntoIter as Iterator>::Item>,
{
    core::stringeq::StringEqualParser::new(str)
}

pub use core::dict_btree::DictBTreeParser as DictBTree;
pub use core::dict_hashmap::DictHashMapParser as DictHashMap;

pub use wrapper::boxed::BoxedParser as Boxed;
pub use wrapper::rced::RcedParser as RCed;
pub use wrapper::refcelled::RefCelledParser as RefCelled;
