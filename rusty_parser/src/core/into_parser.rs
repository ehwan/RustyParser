pub trait IntoParser {
    type Into;

    fn into_parser(self) -> Self::Into;

    /// concatenate two parser
    fn seq<RhsParser: IntoParser>(
        self,
        rhs: RhsParser,
    ) -> crate::wrapper::seq::SeqParser<Self::Into, RhsParser::Into>
    where
        Self: Sized,
    {
        crate::wrapper::seq::SeqParser::new(self.into_parser(), rhs.into_parser())
    }

    /// repeat parser for given range ( this matches as long as possible )
    fn repeat<Idx, RangeType: std::ops::RangeBounds<Idx>>(
        self,
        range_: RangeType,
    ) -> crate::wrapper::repeat::RepeatParser<Self::Into, RangeType, Idx>
    where
        Self: Sized,
        RangeType: std::ops::RangeBounds<Idx>,
        Idx: PartialOrd + PartialEq + PartialOrd<i32> + PartialEq<i32>,
        i32: PartialOrd + PartialEq + PartialOrd<Idx> + PartialEq<Idx>,
    {
        crate::wrapper::repeat::RepeatParser::new(self.into_parser(), range_)
    }

    /// Or combinator of parsers
    fn or_<RhsParser: IntoParser>(
        self,
        rhs: RhsParser,
    ) -> crate::wrapper::or_::OrParser<Self::Into, RhsParser::Into>
    where
        Self: Sized,
    {
        crate::wrapper::or_::OrParser::new(self.into_parser(), rhs.into_parser())
    }

    /// Map parser's Output to new value
    fn map<ClosureType, ClosureInput, ClosureOutput>(
        self,
        callback: ClosureType,
    ) -> crate::wrapper::map::MapParser<Self::Into, ClosureType, ClosureInput, ClosureOutput>
    where
        ClosureInput: crate::core::tuple::Tuple,
        ClosureType: Fn(ClosureInput) -> ClosureOutput,
        ClosureOutput: crate::core::tuple::Tuple,
        Self: Sized,
    {
        crate::wrapper::map::MapParser::new(self.into_parser(), callback)
    }

    /// change Parser's Output to ().
    /// This internally call match_pattern() instead of parse()
    fn void_(self) -> crate::wrapper::void::VoidParser<Self::Into>
    where
        Self: Sized,
    {
        crate::wrapper::void::VoidParser::new(self.into_parser())
    }

    /// this parser always success whether the input is matched or not
    fn optional(self) -> crate::wrapper::option::OptionalParser<Self::Into>
    where
        Self: Sized,
    {
        crate::wrapper::option::OptionalParser::new(self.into_parser())
    }

    /// change Parser's Output to Iterator Pair [begin, end)
    fn iter(self) -> crate::wrapper::iter_range::IterParser<Self::Into>
    where
        Self: Sized,
    {
        crate::wrapper::iter_range::IterParser::new(self.into_parser())
    }

    /// create RefCell\<Parser\> wrapper
    fn refcell(self) -> crate::wrapper::refcelled::RefCelledParser<Self::Into>
    where
        Self: Sized,
    {
        crate::wrapper::refcelled::RefCelledParser::new(self.into_parser())
    }

    /// create Rc\<Parser\> wrapper
    fn rc(self) -> crate::wrapper::rced::RcedParser<Self::Into>
    where
        Self: Sized,
    {
        crate::wrapper::rced::RcedParser::new(self.into_parser())
    }

    /// create a Box\<dyn Parser\> wrapper for iterators of std::str::Chars
    /// This can take any parser with Output of `Output`
    fn box_chars<Output>(self) -> crate::wrapper::boxed::DynBoxChars<Output>
    where
        Output: crate::core::tuple::Tuple,
        Self: Sized,
        Self::Into:
            for<'a> crate::core::parser::Parser<std::str::Chars<'a>, Output = Output> + 'static,
    {
        crate::wrapper::boxed::DynBoxChars::new(self)
    }

    /// create a Box\<dyn Parser\> wrapper for iterators of std::slice::Iter
    /// This can take any parser with Output of `Output`
    fn box_slice<Output, T>(self) -> crate::wrapper::boxed::DynBoxSlice<Output, T>
    where
        Output: crate::core::tuple::Tuple,
        Self: Sized,
        Self::Into:
            for<'a> crate::core::parser::Parser<std::slice::Iter<'a, T>, Output = Output> + 'static,
    {
        crate::wrapper::boxed::DynBoxSlice::new(self)
    }

    /// match for parser1 parser2, parser1 must success and parser2 must fail
    /// This is equivalent to `not(parser1, parser2)`
    fn not<RhsParser: IntoParser>(
        self,
        rhs: RhsParser,
    ) -> crate::wrapper::not::NotParser<Self::Into, RhsParser::Into>
    where
        Self: Sized,
    {
        crate::wrapper::not::NotParser::new(self.into_parser(), rhs.into_parser())
    }

    /// change Parser's Output to output
    fn output<Output: crate::core::tuple::Tuple + Clone>(
        self,
        output: Output,
    ) -> crate::wrapper::output::OutputParser<Self::Into, Output>
    where
        Self: Sized,
    {
        crate::wrapper::output::OutputParser::new(self.into_parser(), output)
    }

    /// returns &str of parsed input
    fn str(self) -> crate::wrapper::slice::StrParser<Self::Into>
    where
        Self: Sized,
        Self::Into: for<'a> crate::core::parser::Parser<std::str::Chars<'a>>,
    {
        crate::wrapper::slice::StrParser::new(self.into_parser())
    }

    /// returns &[T] of parsed input
    fn slice<T>(self) -> crate::wrapper::slice::SliceParser<Self::Into>
    where
        Self: Sized,
        Self::Into: for<'a> crate::core::parser::Parser<std::slice::Iter<'a, T>>,
    {
        crate::wrapper::slice::SliceParser::new(self.into_parser())
    }
}
