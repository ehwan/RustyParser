pub trait IntoParser {
    /// Target Parser type
    type Into;

    /// convert self to Parser
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let hello_parser = "hello".into_parser();
    /// let a_parser = 'a'.into_parser();
    /// let digit_parser = ('0'..='9').into_parser();
    /// ```
    fn into_parser(self) -> Self::Into;

    /// concatenate two parser
    ///
    /// `Output`: `( A0, A1, ..., B0, B1, ..., C0, C1, ... )`
    /// where `(A0, A1, ...)` are the output of the first parser,
    /// and `(B0, B1, ...)`, `(C0, C1, ...)` are the output of the following parsers.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// // 'a', and then 'b'
    /// let ab_parser = rp::seq!('a', 'b', 'c'); // IntoParser for char
    ///
    /// let res = rp::parse(&ab_parser, "abcd".chars());
    /// assert_eq!(res.output.unwrap(), ('a', 'b', 'c')); // Output is concatenated
    /// assert_eq!(res.it.collect::<String>(), "d");
    /// ```
    fn seq<RhsParser: IntoParser>(
        self,
        rhs: RhsParser,
    ) -> crate::wrapper::seq::SeqParser<Self::Into, RhsParser::Into>
    where
        Self: Sized,
    {
        crate::wrapper::seq::SeqParser::new(self.into_parser(), rhs.into_parser())
    }

    /// repeat parser multiple times. This tries to match as long as possible.
    ///
    /// `Output`:
    ///  - if `Output` of the repeated parser is `()`, then `Output` is `()`
    ///  - if `Output` of the repeated parser is `(T,)`, then `Output` is `(Vec<T>,)`
    ///  - otherwise, `(Vec<Output of Self>,)`
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// // repeat 'a' 3 to 5 times
    /// let multiple_a_parser = 'a'.repeat(3..=5);
    /// let res = rp::parse(&multiple_a_parser, "aaaabcd".chars());
    /// // four 'a' is parsed
    /// assert_eq!(res.output.unwrap(), (vec!['a', 'a', 'a', 'a',],));
    /// assert_eq!(res.it.collect::<String>(), "bcd");
    ///
    /// let multiple_a_parser = 'a'.repeat(3usize);
    /// let res = rp::parse(&multiple_a_parser, "aaaabcd".chars());
    /// // three 'a' is parsed
    /// assert_eq!(res.output.unwrap(), (vec!['a', 'a', 'a'],));
    /// ```
    fn repeat<RangeTypeIncludeInteger>(
        self,
        range: RangeTypeIncludeInteger,
    ) -> crate::wrapper::repeat::RepeatParser<Self::Into, RangeTypeIncludeInteger::Into>
    where
        Self: Sized,
        RangeTypeIncludeInteger: crate::core::range_copyable::ToCopyable,
        RangeTypeIncludeInteger::Into:
            crate::core::range_copyable::RangeBound<crate::wrapper::repeat::RepeatCountType>,
    {
        crate::wrapper::repeat::RepeatParser::from(self.into_parser(), range)
    }

    /// or combinator for two parsers
    ///
    /// `Output`: `Output` of the all parsers.
    /// Note that the output of all parsers must be the same type.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// // 'a' or 'b'
    /// let ab_parser = rp::or!('a', 'b'); // IntoParser for char
    ///
    /// // 'a' is matched
    /// let res = rp::parse(&ab_parser, "abcd".chars());
    /// assert_eq!(res.output.unwrap(), ('a',)); // Output of 'a'
    /// assert_eq!(res.it.clone().collect::<String>(), "bcd");
    ///
    /// // continue parsing from the rest
    /// // 'a' is not matched, but 'b' is matched
    /// let res = rp::parse(&ab_parser, res.it);
    /// assert_eq!(res.output.unwrap(), ('b',));
    /// assert_eq!(res.it.clone().collect::<String>(), "cd");
    ///
    /// // continue parsing from the rest
    /// // 'a' is not matched, 'b' is not matched; failed
    /// let res = rp::parse(&ab_parser, res.it);
    /// assert_eq!(res.output, None);
    /// assert_eq!(res.it.clone().collect::<String>(), "cd");
    /// ```
    fn or<RhsParser: IntoParser>(
        self,
        rhs: RhsParser,
    ) -> crate::wrapper::or::OrParser<Self::Into, RhsParser::Into>
    where
        Self: Sized,
    {
        crate::wrapper::or::OrParser::new(self.into_parser(), rhs.into_parser())
    }

    /// Map parser's Output to new value.
    ///
    /// Parser's Output will be unpacked and passed to the closure. The value returned from the closure will be new Output.
    ///
    /// `Output`: `(T,)` where `T` is return type of the closure.
    /// The value `v` returned from the closure will be wrapped into `(v,)`.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// // map the output
    /// // <Output of 'a'> -> i32
    /// let int_parser = 'a'.map(|ch| -> i32 { ch as i32 - 'a' as i32 });
    ///
    /// let res = rp::parse(&int_parser, "abcd".chars());
    /// assert_eq!(res.output.unwrap(), (0,));
    /// assert_eq!(res.it.collect::<String>(), "bcd");
    /// ```
    fn map<ClosureType>(
        self,
        callback: ClosureType,
    ) -> crate::wrapper::map::MapParser<Self::Into, ClosureType>
    where
        Self: Sized,
    {
        crate::wrapper::map::MapParser::new(self.into_parser(), callback)
    }

    /// Change Parser's Output to `()`.
    /// This internally call `crate::match_pattern()` instead of `crate::parse()`
    ///
    /// `Output`: `()`
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let expensive_parser = 'a'.map(|_| -> i32 {
    ///     // some expensive operations for data extracting...
    ///     panic!("This should not be called");
    /// });
    /// let expensive_parser = expensive_parser.void();
    ///
    /// // ignore the output of parser
    /// // this internally calls 'match_pattern(...)' instead of 'parse(...)'
    /// let res = rp::parse(&expensive_parser, "abcd".chars());
    /// assert_eq!(res.output.unwrap(), ());
    /// assert_eq!(res.it.collect::<String>(), "bcd");
    /// ```
    fn void(self) -> crate::wrapper::void::VoidParser<Self::Into>
    where
        Self: Sized,
    {
        crate::wrapper::void::VoidParser::new(self.into_parser())
    }

    /// This parser always success whether the input is matched or not.
    ///
    /// `Output`:
    ///  - if `Output` of the origin parser is `(T0,)`, `(Option<T0>,)`
    ///  - otherwise, `( Option<Output of Self>, )`
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let a_optional_parser = 'a'.optional(); // (Option<char>,)
    ///
    /// let res = rp::parse(&a_optional_parser, "abcd".chars()); // success
    /// assert_eq!(res.output.unwrap(), (Some('a'),));
    ///
    /// let res = rp::parse(&a_optional_parser, "bcd".chars()); // success, but 'a' is not matched
    /// assert_eq!(res.output.unwrap(), (None,));
    /// ```
    fn optional(self) -> crate::wrapper::option::OptionalParser<Self::Into>
    where
        Self: Sized,
    {
        crate::wrapper::option::OptionalParser::new(self.into_parser())
    }

    /// This parser always success whether the input is matched or not.
    /// If it failed, the given value will be returned.
    ///
    /// `Output`:
    /// <`Output` of Self>.
    ///
    /// The value given to `optional_or` must match with the `Output` of the origin parser.
    ///
    /// For single-value-output ( which's output is `(T,)` ),
    /// passing either `T` or `(T,)` is permitted.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// // if 'a' failed, return 'x'
    /// let a_optional_or = 'a'.optional_or('x'); // (char,)
    ///
    /// let res = rp::parse(&a_optional_or, "bcd".chars());
    /// assert_eq!(res.output.unwrap(), ('x',));
    /// ```
    fn optional_or<Output: Clone>(
        self,
        output: Output,
    ) -> crate::wrapper::option::OptionalOrParser<Self::Into, Output>
    where
        Self: Sized,
    {
        crate::wrapper::option::OptionalOrParser::new(self.into_parser(), output)
    }

    /// This parser always success whether the input is matched or not.
    /// If it failed, the given closure will be evaluated and returned.
    ///
    /// `Output`:
    /// <`Output` of Self>.
    ///
    /// The value given to `or_else` must match with the `Output` of the origin parser.
    ///
    /// For single-value-output ( which's output is `(T,)` ),
    /// returning either `T` or `(T,)` is permitted.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// // if 'a' failed, return 'x'
    /// let a_or_else = 'a'.or_else(|| 'x'); // (char,)
    ///
    /// let res = rp::parse(&a_or_else, "bcd".chars());
    /// assert_eq!(res.output.unwrap(), ('x',));
    /// ```
    fn or_else<Closure>(
        self,
        closure: Closure,
    ) -> crate::wrapper::or_else::OptionalOrElseParser<Self::Into, Closure>
    where
        Self: Sized,
    {
        crate::wrapper::or_else::OptionalOrElseParser::new(self.into_parser(), closure)
    }

    /// Match for parser1 but not parser2.
    ///
    /// `Output`: `Output` of `Self`
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let digit_parser_except_4 = ('0'..='9').not('4');
    ///
    /// let res = rp::parse(&digit_parser_except_4, "3".chars());
    /// assert_eq!(res.output.unwrap(), ('3',));
    ///
    /// let res = rp::parse(&digit_parser_except_4, "4".chars());
    /// assert_eq!(res.output, None);
    /// ```
    fn not<RhsParser: IntoParser>(
        self,
        rhs: RhsParser,
    ) -> crate::wrapper::not::NotParser<Self::Into, RhsParser::Into>
    where
        Self: Sized,
    {
        crate::wrapper::not::NotParser::new(self.into_parser(), rhs.into_parser())
    }

    /// Change Parser's Output to (output,).
    ///
    /// `Output`: `(T,)` where `T` is the type of the value you provided.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let digit_parser = ('0'..='9').output(2024);
    ///
    /// let res = rp::parse(&digit_parser, "123456hello_world".chars());
    /// assert_eq!(res.output.unwrap(), (2024,));
    /// assert_eq!(res.it.collect::<String>(), "23456hello_world");
    /// ```
    fn output<Output: Clone>(
        self,
        output: Output,
    ) -> crate::wrapper::output::OutputParser<Self::Into, Output>
    where
        Self: Sized,
    {
        crate::wrapper::output::OutputParser::new(self.into_parser(), output)
    }

    /// Returns String of parsed input.
    /// Only works for parsing with [`std::str::Chars`].
    ///
    /// `Output`: `(String,)`
    ///
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let digits_parser = ('0'..='9').repeat(0..).string();
    ///
    /// let res = rp::parse(&digits_parser, "123456hello_world".chars());
    /// assert_eq!(res.output.unwrap(), ("123456".to_string(),));
    /// assert_eq!(res.it.collect::<String>(), "hello_world");
    /// ```
    fn string(self) -> crate::wrapper::slice::StringParser<Self::Into>
    where
        Self: Sized,
        Self::Into: for<'a> crate::core::parser::Parser<std::str::Chars<'a>>,
    {
        crate::wrapper::slice::StringParser::new(self.into_parser())
    }

    /// Returns `Vec\<T\>` of parsed input.
    /// Only works for parsing with [`ExactSizeIterator`].
    ///
    /// `Output`: `(Vec<Iterator::Item>,)`
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let hello_bytes = &[104, 101, 108, 108, 111];
    /// let hello_parser = hello_bytes.into_parser().vec::<u8>();
    ///
    /// let res = rp::parse(&hello_parser, "hello_world1234".as_bytes().iter().copied());
    /// assert_eq!(res.output.unwrap(), (hello_bytes.iter().cloned().collect::<Vec<u8>>(),) );
    /// ```
    fn vec<T>(self) -> crate::wrapper::slice::VecParser<Self::Into>
    where
        Self: Sized,
        Self::Into: for<'a> crate::core::parser::Parser<std::iter::Cloned<std::slice::Iter<'a, T>>>,
    {
        crate::wrapper::slice::VecParser::new(self.into_parser())
    }

    /// Parser will not consume the input iterator.
    /// It still matches and return the output.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let digit_parser = ('0'..='9').not_consume();
    ///
    /// let res = rp::parse(&digit_parser, "12345".chars());
    /// assert_eq!(res.output.unwrap(), ('1',));
    /// assert_eq!(res.it.collect::<String>(), "12345"); // iterator is not consumed
    /// ```
    fn not_consume(self) -> crate::wrapper::notconsume::NotConsumeParser<Self::Into>
    where
        Self: Sized,
    {
        crate::wrapper::notconsume::NotConsumeParser::new(self.into_parser())
    }

    /// Reduce the output of the parser with the given reducer.
    ///
    /// With given input string `self rhs rhs rhs rhs ...` and the reducer `f`,
    /// the output will be calculated as
    /// `f( f( f(self,rhs), rhs ), rhs ), ...`
    ///
    /// ## Note
    ///
    ///  - The signature of the reducer must be `Fn(A0, A1, A2, ..., B0, B1, B2, ...) -> ( A0, A1, A2 ... )`.
    ///    Where `(A0, A1, A2, ...)` are the output of the first parser, and `(B0, B1, B2, ...)` are the output of the following parser.
    ///
    ///  - For single-value-output ( which's output is `(T,)` ),
    ///    returning either `T` or `(T,)` is permitted.
    ///
    /// `Output`: `Output` of `Self`
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let digit_parser = ('0'..='9').into_parser().map(|val: char| -> i32 { val as i32 - '0' as i32 });
    /// let reduced_left = digit_parser.reduce_left(digit_parser, |lhs, rhs| lhs * 10 + rhs);
    /// let res = rp::parse( &reduced_left, "123456abcd".chars() );
    /// assert_eq!(res.output.unwrap(), (123456,));
    /// ```
    fn reduce_left<RhsParser, Reducer>(
        self,
        rhs: RhsParser,
        reducer: Reducer,
    ) -> crate::wrapper::reduce::left::ReduceLeftParser<Self::Into, RhsParser::Into, Reducer>
    where
        Self: Sized,
        RhsParser: IntoParser,
    {
        crate::wrapper::reduce::left::ReduceLeftParser::new(
            self.into_parser(),
            rhs.into_parser(),
            reducer,
        )
    }

    /// Reduce the output of the parser with the given reducer.
    ///
    /// With given input string `lhs lhs lhs lhs ... self` and the reducer `f`,
    /// the output will be calculated as
    /// `f(lhs, f(lhs, f(lhs, f( ... f(lhs,self)))`
    ///
    /// ## Note
    ///
    ///  - The signature of the reducer must be `Fn(A0, A1, A2, ..., B0, B1, B2, ...) -> ( B0, B1, B2 ... )`.
    ///    Where `(A0, A1, A2, ...)` are the output of the first parser, and `(B0, B1, B2, ...)` are the output of the following parser.
    ///
    ///  - For single-value-output ( which's output is `(T,)` ),
    ///    returning either `T` or `(T,)` is permitted.
    ///
    /// `Output`: `Output` of `Self`
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let digit_parser =
    ///     ('0'..='9').into_parser().map(|val: char| -> i32 { val as i32 - '0' as i32 });
    /// let alphabet_parser =
    ///     ('a'..='z').into_parser().map(|val: char| -> i32 { val as i32 - 'a' as i32 });
    /// let reduced_right =
    ///     alphabet_parser.reduce_right(digit_parser, |lhs: i32, rhs: i32| -> i32 { rhs * 10 + lhs });
    ///
    /// let res = rp::parse(&reduced_right, "123456dcba".chars());
    /// assert_eq!(res.output.unwrap(), (3654321,));
    /// assert_eq!(res.it.collect::<String>(), "cba");
    /// ```
    fn reduce_right<LhsParser, Reducer>(
        self,
        lhs: LhsParser,
        reducer: Reducer,
    ) -> crate::wrapper::reduce::right::ReduceRightParser<LhsParser::Into, Self::Into, Reducer>
    where
        Self: Sized,
        LhsParser: IntoParser,
    {
        crate::wrapper::reduce::right::ReduceRightParser::new(
            lhs.into_parser(),
            self.into_parser(),
            reducer,
        )
    }

    /// Reduce the output of the parser with the given reducer.
    ///
    /// With given input string `self self self ...` and the reducer `f`,
    /// the output will be calculated as
    /// `f( f( f(init,self), self), self), ...`
    ///
    /// The signature of the reducer must be `Fn(Init, A0, A1, A2, ...) -> Init`.
    /// Where `(A0, A1, A2, ...)` are the output of `Self`.
    ///
    /// `Output`: `Init`
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let digit_parser =
    ///     ('0'..='9').into_parser().map(|val: char| -> i32 { val as i32 - '0' as i32 });
    /// let number_parser =
    ///     digit_parser.reduce_with(0, |acc, rhs| acc * 10 + rhs);
    ///
    /// let res = rp::parse(&number_parser, "123456abc".chars());
    /// assert_eq!(res.output.unwrap(), (123456,));
    /// assert_eq!(res.it.collect::<String>(), "abc");
    /// ```
    fn reduce_with<Init, Reducer>(
        self,
        init: Init,
        reducer: Reducer,
    ) -> crate::wrapper::reduce::init::ReduceInitParser<Self::Into, Init, Reducer>
    where
        Self: Sized,
        Init: Clone,
    {
        crate::wrapper::reduce::init::ReduceInitParser::new(self.into_parser(), init, reducer)
    }

    /// Reduce the output of the parser with the given reducer.
    ///
    /// With given input string `self self self ...` and the reducer `f`,
    /// the output will be calculated as
    /// `f(self, f(self, f(self, f( ... f(self,init)))`
    ///
    /// The signature of the reducer must be `Fn(A0, A1, A2, ..., Init) -> Init`.
    /// Where `(A0, A1, A2, ...)` are the output of `Self`.
    ///
    /// `Output`: `Init`
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let digit_parser =
    ///     ('0'..='9').into_parser().map(|val: char| -> i32 { val as i32 - '0' as i32 });
    /// let number_rev_parser =
    ///     digit_parser.reduce_right_with(0, |lhs, acc| acc * 10 + lhs);
    ///
    /// let res = rp::parse(&number_rev_parser, "123456abc".chars());
    /// assert_eq!(res.output.unwrap(), (654321,));
    /// assert_eq!(res.it.collect::<String>(), "abc");
    /// ```
    fn reduce_right_with<Init, Reducer>(
        self,
        init: Init,
        reducer: Reducer,
    ) -> crate::wrapper::reduce::init_right::ReduceRightInitParser<Self::Into, Init, Reducer>
    where
        Self: Sized,
        Init: Clone,
    {
        crate::wrapper::reduce::init_right::ReduceRightInitParser::new(
            self.into_parser(),
            init,
            reducer,
        )
    }

    /// This does what [`std::iter::Inspect`] do.
    /// The closure will be called before parsing, regardless of the success or failure of the parser.
    ///
    /// `Output`: `Output` of `Self`
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let digit_parser =
    ///     ('0'..='9').into_parser().map(|val: char| -> i32 { val as i32 - '0' as i32 });
    /// let digit_parser = digit_parser.inspect(|| {
    ///   println!( "digit parser entered!" );
    /// });
    ///
    /// let res = rp::parse(&digit_parser, "123456abcd".chars());
    /// assert_eq!(res.output.unwrap(), (1,));
    /// assert_eq!(res.it.collect::<String>(), "23456abcd");
    fn inspect<ClosureType>(
        self,
        closure: ClosureType,
    ) -> crate::wrapper::inspect::InspectParser<Self::Into, ClosureType>
    where
        Self: Sized,
        ClosureType: Fn(),
    {
        crate::wrapper::inspect::InspectParser::new(self.into_parser(), closure)
    }
}
