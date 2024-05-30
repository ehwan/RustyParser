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

    // map
    fn map<ClosureType, ClosureOutput>(
        self,
        map: ClosureType,
    ) -> crate::wrapper::map::MapParser<Self, ClosureType, ClosureOutput, It>
    where
        Self: Sized,
        ClosureType: Fn(Self::Output) -> ClosureOutput,
        ClosureOutput: crate::core::tuple::Tuple,
    {
        crate::wrapper::map::MapParser::new(self, map)
    }

    // seq
    fn seq<RhsParser>(self, rhs: RhsParser) -> crate::wrapper::seq::SeqParser<Self, RhsParser>
    where
        Self: Sized,
        RhsParser: Parser<It>,
        Self::Output:
            crate::wrapper::tuplemerge::AppendTupleToTuple<<RhsParser as Parser<It>>::Output>,
        <Self::Output as crate::wrapper::tuplemerge::AppendTupleToTuple<
            <RhsParser as Parser<It>>::Output,
        >>::Output: crate::core::tuple::Tuple,
    {
        crate::wrapper::seq::SeqParser::new(self, rhs)
    }

    // or
    fn or_<RhsParser>(self, rhs: RhsParser) -> crate::wrapper::or_::OrParser<Self, RhsParser, It>
    where
        Self: Sized,
        RhsParser: Parser<It, Output = Self::Output>,
    {
        crate::wrapper::or_::OrParser::new(self, rhs)
    }

    // repeat
    fn repeat<RangeType, Idx>(
        self,
        range: RangeType,
    ) -> crate::wrapper::repeat::RepeatParser<Self, RangeType, Idx>
    where
        Self: Sized,
        RangeType: std::ops::RangeBounds<Idx>,
        Idx: PartialOrd + PartialEq + PartialOrd<i32> + PartialEq<i32>,
        i32: PartialOrd + PartialEq + PartialOrd<Idx> + PartialEq<Idx>,
        Self::Output: crate::wrapper::vecmerge::VectorOutputSpecialize,
        <Self::Output as crate::wrapper::vecmerge::VectorOutputSpecialize>::Output:
            crate::core::tuple::Tuple,
    {
        crate::wrapper::repeat::RepeatParser::new(self, range)
    }

    // void
    fn void_(self) -> crate::wrapper::void::VoidParser<Self>
    where
        Self: Sized,
    {
        crate::wrapper::void::VoidParser::new(self)
    }

    // ref
    fn ref_<'a>(&'a self) -> crate::wrapper::reference::ReferenceParser<'a, Self>
    where
        Self: Sized,
    {
        crate::wrapper::reference::ReferenceParser::new(self)
    }

    // boxed
    fn box_(self) -> std::boxed::Box<Self>
    where
        Self: Sized,
    {
        std::boxed::Box::new(self)
    }

    // refcelled
    fn refcell(self) -> std::cell::RefCell<Self>
    where
        Self: Sized,
    {
        std::cell::RefCell::new(self)
    }

    // RCed
    fn rc(self) -> std::rc::Rc<Self>
    where
        Self: Sized,
    {
        std::rc::Rc::new(self)
    }

    // iterator range
    fn iter(self) -> crate::wrapper::iter_range::IterParser<Self, It>
    where
        Self: Sized,
    {
        crate::wrapper::iter_range::IterParser::new(self)
    }
}
