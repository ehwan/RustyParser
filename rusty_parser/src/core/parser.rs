use super::iterator_bound::InputIteratorTrait;
use super::result::ParseResult;
use super::tuple::Tuple;

// defulat Parser trait
pub trait Parser<It>
where
    It: InputIteratorTrait,
{
    type Output: Tuple;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It>;

    // this is special implementation for parsing.
    // it does not parse data from string, just check if it matches the pattern
    // for some parser, there may be a cheaper way to check if it matches the pattern
    // than actually parsing the data
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let res = self.parse(it);
        if let Some(_) = res.output {
            ParseResult {
                output: Some(()),
                it: res.it,
            }
        } else {
            ParseResult {
                output: None,
                it: res.it,
            }
        }
    }
    fn void_(self) -> crate::wrapper::void::VoidParser<Self>
    where
        Self: Sized,
    {
        crate::wrapper::void::VoidParser::new(self)
    }
    fn seq<RhsParser>(self, rhs: RhsParser) -> crate::wrapper::seq::SeqParser<Self, RhsParser>
    where
        Self: Sized,
    {
        crate::wrapper::seq::SeqParser::new(self, rhs)
    }

    fn or_<ParserType>(self, parser: ParserType) -> crate::wrapper::or_::OrParser<Self, ParserType>
    where
        Self: Sized,
    {
        crate::wrapper::or_::OrParser::new(self, parser)
    }

    fn repeat<RangeType, Idx>(
        self,
        range: RangeType,
    ) -> crate::wrapper::repeat::RepeatParser<Self, RangeType, Idx>
    where
        Self: Sized,
        RangeType: std::ops::RangeBounds<Idx>,
        Idx: PartialOrd + PartialEq + PartialOrd<i32> + PartialEq<i32>,
        i32: PartialOrd + PartialEq + PartialOrd<Idx> + PartialEq<Idx>,
    {
        crate::wrapper::repeat::RepeatParser::new(self, range)
    }

    fn map<ClosureType, ClosureInput, ClosureOutput>(
        self,
        mapper: ClosureType,
    ) -> crate::wrapper::map::MapParser<Self, ClosureType, ClosureInput, ClosureOutput>
    where
        Self: Sized,
        ClosureInput: Tuple,
        ClosureType: Fn(ClosureInput) -> ClosureOutput,
        ClosureOutput: Tuple,
    {
        crate::wrapper::map::MapParser::new(self, mapper)
    }

    fn ref_<'a>(&'a self) -> crate::wrapper::reference::ReferenceParser<'a, Self>
    where
        Self: Sized,
    {
        crate::wrapper::reference::ReferenceParser::new(self)
    }

    fn iter(self) -> crate::wrapper::iter_range::IterParser<Self>
    where
        Self: Sized,
    {
        crate::wrapper::iter_range::IterParser::new(self)
    }

    fn refcell(self) -> crate::wrapper::refcelled::RefCelledParser<Self>
    where
        Self: Sized,
    {
        crate::wrapper::refcelled::RefCelledParser::new(self)
    }

    fn rc(self) -> crate::wrapper::rced::RcedParser<Self>
    where
        Self: Sized,
    {
        crate::wrapper::rced::RcedParser::new(self)
    }
}
