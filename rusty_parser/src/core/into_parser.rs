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
    ///  - otherwise, `(Vec< Output of the Repeated Parser >,)`
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

    /// Map parser's Output to new value
    ///
    /// `Output`: return type of the closure ( must be Tuple )
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// // map the output
    /// // <Output of 'a'> -> (i32,)
    /// let int_parser = 'a'.map(|(ch,)| -> (i32,) { (ch as i32 - 'a' as i32,) }); // IntoParser for char
    ///
    /// let res = rp::parse(&int_parser, "abcd".chars());
    /// assert_eq!(res.output.unwrap(), (0,));
    /// assert_eq!(res.it.collect::<String>(), "bcd");
    /// ```
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

    /// Change Parser's Output to ().
    /// This internally call match_pattern() instead of parse()
    ///
    /// `Output`: `()`
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let expensive_parser = 'a'.map(|(_,)| -> (i32,) {
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
    ///  - otherwise, `( Option<Output of the Origin Parser>, )`
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
    ///
    /// // if 'a' failed, return 'x'
    /// let a_optional_or = 'a'.optional_or(('x',)); // (char,)
    ///
    /// let res = rp::parse(&a_optional_or, "bcd".chars());
    /// assert_eq!(res.output.unwrap(), ('x',));
    /// ```
    fn optional(self) -> crate::wrapper::option::OptionalParser<Self::Into>
    where
        Self: Sized,
    {
        crate::wrapper::option::OptionalParser::new(self.into_parser())
    }

    /// This parser always success whether the input is matched or not.
    ///
    /// `Output`:
    /// <`Output` of the origin parser>.
    /// The value given to `optional_or` must match with the `Output` of the origin parser.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// // if 'a' failed, return 'x'
    /// let a_optional_or = 'a'.optional_or(('x',)); // (char,)
    ///
    /// let res = rp::parse(&a_optional_or, "bcd".chars());
    /// assert_eq!(res.output.unwrap(), ('x',));
    /// ```
    fn optional_or<Output: Clone + crate::core::tuple::Tuple>(
        self,
        output: Output,
    ) -> crate::wrapper::option::OptionalOrParser<Self::Into, Output>
    where
        Self: Sized,
    {
        crate::wrapper::option::OptionalOrParser::new(self.into_parser(), output)
    }

    /// create RefCell\<Parser\> wrapper.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let hello_parser = "hello".into_parser();
    /// let digit_parser = ('0'..='9').void();
    ///
    /// let refcelled_parser = hello_parser.box_chars().refcell();
    ///
    /// let res_hello = rp::parse(&refcelled_parser, "hello0123".chars());
    /// // success
    /// assert_eq!(res_hello.output.unwrap(), ());
    /// assert_eq!(res_hello.it.clone().collect::<String>(), "0123");
    ///
    /// // now change refcelled_parser to digit_parser
    /// // Thanks to Deref, you can call borrow_mut().assign() directly
    /// refcelled_parser.borrow_mut().assign(digit_parser);
    ///
    /// let res_digit = rp::parse(&refcelled_parser, res_hello.it);
    /// // success
    /// assert_eq!(res_digit.output.unwrap(), ());
    /// assert_eq!(res_digit.it.collect::<String>(), "123");
    /// ```
    fn refcell(self) -> std::cell::RefCell<Self::Into>
    where
        Self: Sized,
    {
        std::cell::RefCell::new(self.into_parser())
    }

    /// Create Rc\<Parser\> wrapper.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let hello_parser = "hello".into_parser();
    /// let digit_parser = ('0'..='9').void();
    ///
    /// let rc_parser1 = hello_parser.box_chars().refcell().rc();
    /// let rc_parser2 = std::rc::Rc::clone(&rc_parser1);
    /// // rc_parser2 is now pointing to the same parser as rc_parser1
    ///
    /// let res_hello = rp::parse(&rc_parser1, "hello0123".chars());
    /// // success
    /// assert_eq!(res_hello.output.unwrap(), ());
    /// assert_eq!(res_hello.it.clone().collect::<String>(), "0123");
    ///
    /// // now change rced_parser1 to digit_parser
    /// // Thanks to Deref, you can call borrow_mut().assign() directly
    /// rc_parser1.borrow_mut().assign(digit_parser);
    ///
    /// // rced_parser2 should also be digit_parser
    /// let res_digit = rp::parse(&rc_parser2, res_hello.it);
    /// // success
    /// assert_eq!(res_digit.output.unwrap(), ());
    /// assert_eq!(res_digit.it.collect::<String>(), "123");
    /// ```
    fn rc(self) -> std::rc::Rc<Self::Into>
    where
        Self: Sized,
    {
        std::rc::Rc::new(self.into_parser())
    }

    /// create a Box\<dyn Parser\> wrapper for iterators of `std::str::Chars`.
    ///
    /// This can take any parser with Output of `Output`.
    ///
    /// Once you wrap the parser with this, you can only use input iterator of `std::str::Chars`.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let hello_parser = "hello".into_parser();
    /// let digit_parser = ('0'..='9').void();
    ///
    /// // this will wrap the parser into Box< dyn Parser >
    /// let mut boxed_parser = hello_parser.box_chars();
    ///
    /// let res_hello = rp::parse(&boxed_parser, "hello0123".chars());
    /// // success
    /// assert_eq!(res_hello.output.unwrap(), ());
    /// assert_eq!(res_hello.it.clone().collect::<String>(), "0123");
    ///
    /// // now change boxed_parser to digit_parser
    /// boxed_parser.assign(digit_parser);
    ///
    /// let res_digit = rp::parse(&boxed_parser, res_hello.it);
    /// // success
    /// assert_eq!(res_digit.output.unwrap(), ());
    /// assert_eq!(res_digit.it.collect::<String>(), "123");
    /// ```
    fn box_chars<Output>(self) -> crate::wrapper::boxed::DynBoxChars<Output>
    where
        Output: crate::core::tuple::Tuple,
        Self: Sized,
        Self::Into:
            for<'a> crate::core::parser::Parser<std::str::Chars<'a>, Output = Output> + 'static,
    {
        crate::wrapper::boxed::DynBoxChars::new(self)
    }

    /// create a Box\<dyn Parser\> wrapper for iterators of `std::iter::Cloned<std::slice::Iter>`.
    ///
    /// This can take any parser with Output of `Output`.
    ///
    /// Once you wrap the parser with this, you can only use input iterator of `std::iter::Cloned<std::slice::Iter>`.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let hello_parser = (&[104, 101, 108, 108, 111]).into_parser();
    /// let world_parser = (&[119, 111, 114, 108, 100]).into_parser();
    ///
    /// // this will wrap the parser into Box< dyn Parser >
    /// let mut boxed_parser = hello_parser.box_slice();
    ///
    /// let res_hello = rp::parse(&boxed_parser, "helloworld".as_bytes().iter().cloned());
    /// // success
    /// assert_eq!(res_hello.output.unwrap(), ());
    ///
    /// // now change boxed_parser to world_parser
    /// boxed_parser.assign(world_parser);
    ///
    /// let res_digit = rp::parse(&boxed_parser, res_hello.it);
    /// // success
    /// assert_eq!(res_digit.output.unwrap(), ());
    /// ```
    fn box_slice<Output, T>(self) -> crate::wrapper::boxed::DynBoxSlice<Output, T>
    where
        Output: crate::core::tuple::Tuple,
        T: Clone + Copy,
        Self: Sized,
        Self::Into: for<'a> crate::core::parser::Parser<
                std::iter::Cloned<std::slice::Iter<'a, T>>,
                Output = Output,
            > + 'static,
    {
        crate::wrapper::boxed::DynBoxSlice::new(self)
    }

    /// Match for parser1 parser2, parser1 must success and parser2 must fail.
    ///
    /// `Output`: `Output` of the first parser.
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

    /// Change Parser's Output to output.
    ///
    /// `Output`: value you provided.
    ///
    /// # Example
    /// ```rust
    /// use rusty_parser as rp;
    /// use rp::IntoParser;
    ///
    /// let digit_parser = ('0'..='9').output((1, 2, 3));
    ///
    /// let res = rp::parse(&digit_parser, "123456hello_world".chars());
    /// assert_eq!(res.output.unwrap(), (1, 2, 3));
    /// assert_eq!(res.it.collect::<String>(), "23456hello_world");
    /// ```
    fn output<Output: crate::core::tuple::Tuple + Clone>(
        self,
        output: Output,
    ) -> crate::wrapper::output::OutputParser<Self::Into, Output>
    where
        Self: Sized,
    {
        crate::wrapper::output::OutputParser::new(self.into_parser(), output)
    }

    /// Returns String of parsed input.
    /// Only works for parsing with `std::str::Chars`.
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
    /// Only works for parsing with `ExactSizeIterator`.
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
    /// It still match and return the output.
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
}
