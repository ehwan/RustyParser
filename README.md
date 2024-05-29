# RustyParser
A Generic Parser generator and Pattern Matching Library written in Rust

## Example
 - **[Calculator](examples/calculator)**


## Sample Code
`rusty_parser/src/example/example1.rs`

```rust
// import rusty_parser
use rusty_parser as rp;

// for assert_eq!()
use std::any::type_name;
use std::any::type_name_of_val;

// trait Parser; must be imported for .parse( ... ) method
use rp::Parser;

#[test]
fn example1() {
    // target string to parse
    let target_string = "123456789";

    // define pattern: [0-9]
    let digit_parser = rp::range('0'..='9');

    // parse
    let res = rp::parse(&digit_parser, target_string.chars());

    // res contains the result of parsing
    assert_eq!(
        type_name_of_val(&res.output),
        type_name::<Option<(char,)>>()
    );
    assert_eq!(res.output, Some(('1',)));

    // res.it: iterator after parsing
    // here, '1' is parsed, so the rest is "23456789"
    assert_eq!(res.it.collect::<String>(), target_string[1..]);

    // define pattern: 'a'
    let a_parser = rp::one('a');
    // this will fail
    let res = rp::parse(&a_parser, target_string.chars());
    assert_eq!(res.output, None);

    // iterator will not move if parsing failed
    assert_eq!(res.it.collect::<String>(), target_string);

    // define pattern: [0-9][0-9][0-9]
    // perform 'digit_parser', and then 'digit_parser', sequentially
    // Output = ( Output of first Parser, Output of second Parser, ... )  -->  (char, char, char,)
    let three_digit_parser = rp::seq!(
        digit_parser.clone(), // clone() is required
        digit_parser.clone(), // clone() is required
        digit_parser
    );

    let res = rp::parse(&three_digit_parser, target_string.chars());
    assert_eq!(
        type_name_of_val(&res.output),
        type_name::<Option<(char, char, char,)>>()
    );
    assert_eq!(res.output, Some(('1', '2', '3',)));

    // Output mapping
    // ( char, char, char )  -->  (i32, )
    // Parser's Output must be Tuple
    let int_parser = rp::map(three_digit_parser, |(x, y, z)| -> (i32,) {
        let x_i32 = x as i32 - '0' as i32;
        let y_i32 = y as i32 - '0' as i32;
        let z_i32 = z as i32 - '0' as i32;
        (x_i32 * 100 + y_i32 * 10 + z_i32,)
    });

    let res = rp::parse(&int_parser, target_string.chars());
    assert_eq!(res.output, Some((123,)));
}
```

## Structures
Define pattern, combine them, and parse the input string.

RustyParser provides a set of basic parsers, combinators, and parser-generating functions.

 ```rust 
 // mod rusty_parser {
 fn parse<Pattern,It:Iterator+Clone>(pattern:&Pattern, it:It) -> ParseResult<(Parsed Output of Pattern), It>;
 fn match_pattern<Pattern,It:Iterator+Clone>(pattern:&Pattern, it:It) -> ParseResult<(), It>;
 // }
 ```
`parse(...)` takes an Pattern Object and iterator of input string, then returns `ParseResult<Self::Output, It>`.

 `match_pattern(...)` is used 
 when you only want to check if the pattern is matched or not, without extracting data. 
 For some parsers, like `repeat`, it is expensive to call `parse(...)` to get the output since it invokes `Vec::insert` inside.


`ParseResult` is a struct representing the result of parsing.

``` rust
pub struct ParseResult<Output, It>
where
    Output: Tuple,
    It: Iterator + Clone,
{
    // the output; extracted data
    // 'None' means parsing failed
    pub output: Option<Output>,

    // iterator after parsing
    // if parsing failed, this will be the same as the input iterator
    pub it: It,
}
```

Note that `Output` must be a Tuple 
(include null-tuple `()`). 
Even if the Parser extracts only one element, the output must be a Tuple.

Since the `parse(...)` internally clones the iterator, 
the iterator must be cheaply clonable.


## Basic Parsers

 ### `one`: consumes one charactor if it is equal to `c`.
 ```rust
 let parser = one( c: CharactorType )
 let a_parser = one('a');
 ```
`Output`: `(Iterator::Item,)`

### `range`: consumes one charactor if it is in the range `r`.
```rust
let parser = range( r: impl std::ops::RangeBounds )
let digit_parser = range( '0'..='9' )
```
`Output`: `(Iterator::Item,)`

### `chars`, `slice`: consumes multiple charactors if it is equal to `s`.
```rust
fn chars( s: &'a str );
fn slice( s: &'a [T] );
let hello_parser = chars("hello");
let hello_parser = slice("hello".bytes());
```
`Output`: `()`

### Dictionary: build Trie from a list of strings
```rust
// let mut parser = rp::DictBTree::new();
let mut parser = rp::DictHashMap::new();

parser.insert("hello".chars(), (1,));
parser.insert("hello_world".chars(), (2,));
parser.insert("world".chars(), (3,));

// this will match as long as possible
let res = rp::parse(&parser, "hello_world_abcdefg".chars());
assert_eq!(res.output, Some((2,)));
// 'hello_world' is parsed, so the rest is "_abcdefg"
assert_eq!(res.it.collect::<String>(), "_abcdefg");

// match 'hello' only
let res = rp::parse(&parser, "hello_wo".chars());
assert_eq!(res.output, Some((1,)));
```
`Output`: generic type you support

There are two types of Dictionary: `DictBTree` and `DictHashMap`, for Trie implementation.
Both of them have their own Pros and Cons (the memory usage and time complexity of searching), so you can choose one of them.


### Combinators

### `seq`: sequence of parsers
```rust
let a_parser = rp::one('a');
let b_parser = rp::one('b');

// 'a', and then 'b'
let ab_parser = rp::seq!(a_parser, b_parser);

let res = rp::parse(&ab_parser, "abcd".chars());
assert_eq!(res.output, Some(('a', 'b')));
assert_eq!(res.it.collect::<String>(), "cd");
```
or you can use macro `seq!`
```rust
let parser = rp::seq!( parser_a, and_then_b, and_then_c, ... );
```
`Output`: `( L0, L1, ..., R0, R1, ... )` 
where `(L0, L1, ...)` are the outputs of the first parser, 
and `(R0, R1, ...)` are the outputs of the second parser.

### `or_`: or combinator

```rust
let a_parser = rp::one('a');
let b_parser = rp::one('b');

// 'a' or 'b'
let ab_parser = rp::or_!(a_parser, b_parser);

// 'a' is matched
let res = rp::parse(&ab_parser, "abcd".chars());
assert_eq!(res.output, Some(('a',)));
assert_eq!(res.it.clone().collect::<String>(), "bcd");

// continue parsing from the rest
// 'a' is not matched, but 'b' is matched
let res = rp::parse(&ab_parser, res.it);
assert_eq!(res.output, Some(('b',)));
assert_eq!(res.it.clone().collect::<String>(), "cd");

// continue parsing from the rest
// 'a' is not matched, 'b' is not matched; failed
let res = rp::parse(&ab_parser, res.it);
assert_eq!(res.output, None);
assert_eq!(res.it.clone().collect::<String>(), "cd");
```
or you can use macro `or_!` macro
```rust
let parser = rp::or!( parser_a, else_b, else_c, ... );
```
`Output`: `Output` of the first and second parser.
Note that the output of both parsers must be the same type.


### `map`: map the output of the parser
```rust
let a_parser = rp::one('a');

// map the output
// <Output of 'a'> (char,) -> (i32,)
let int_parser = rp::map(a_parser, |(ch,)| -> (i32,) { (ch as i32 - 'a' as i32,) });

let res = rp::parse(&int_parser, "abcd".chars());
assert_eq!(res.output, Some((0,)));
assert_eq!(res.it.collect::<String>(), "bcd");
```
`Output`: return type of the closure ( must be Tuple )

### `repeat`: repeat the parser multiple times

```rust
let a_parser = rp::one('a');

// repeat 'a' 3 to 5 times (inclusive)
let multiple_a_parser = rp::repeat(a_parser.clone(), 3..=5);
let res = rp::parse(&multiple_a_parser, "aaaabcd".chars());

// four 'a' is parsed
assert_eq!(res.output, Some((vec!['a', 'a', 'a', 'a',],)));
assert_eq!(res.it.collect::<String>(), "bcd");
```

`Output`: 
 - if `Output` of the repeated parser is `()`, then `Output` is `()`
 - if `Output` of the repeated parser is `(T,)`, then `Output` is `Vec<T>`
 - otherwise, `Vec< Output of the Repeated Parser >`

### `void_`: ignore the output of the parser
Force the output to be `()`. 
It internally calls `match_pattern(...)` instead of `parse(...)`. 
This is useful when you only want to check if the pattern is matched or not. 
For more information, see `match_pattern(...)` above.

```rust
let expensive_parser = rp::one('a');
let expensive_parser = rp::map(expensive_parser, |(_,)| -> (i32,) {
    // some expensive operations.... for data parsing
    panic!("This should not be called");
});

// ignore the output of parser
// this internally calls 'match_pattern(...)' instead of 'parse(...)'
let res = rp::match_pattern(&expensive_parser, "abcd".chars());
assert_eq!(res.output, Some(()));
assert_eq!(res.it.collect::<String>(), "bcd");
```
`Output`: `()`


### `iter`: capture a [begin, end) iterator range on input string
```rust
let hello_parser = rp::chars("hello");
let digit_parser = rp::void_(rp::range('0'..='9'));

let parser = rp::iter(rp::seq!(hello_parser, rp::repeat(digit_parser, 3..=3)));

//                   <------> parsed range
let target_string = "hello0123";
//                   |       ^ end
//                   ^ begin
let res = rp::parse(&parser, target_string.chars());
assert_eq!(res.output.is_some(), true);

let (begin, end) = res.output.unwrap();
assert_eq!(begin.collect::<String>(), "hello0123");
assert_eq!(end.collect::<String>(), "3");
```
`Output`: `(It, It)`


## For complex, highly recursive pattern

By default, all the 'parser-generating' member functions consumes `self` and returns a new Parser. 
And `Parser::parse(&self)` takes immutable reference of Self.

However, in some cases, you may want to define a recursive parser.
Which involves 'reference-of-parser' or 'virtual-class-like' structure.

Luckily, Rust std provides wrapper for these cases.
`Rc`, `RefCell`, `Box` are the most common ones.

RustyParser provides `BoxedParser`, `RCedParser`, `RefCelledParser` which are Parser Wrapper for `Box`, `Rc`, `RefCell`.

### `box_`: a `Box<dyn Parser>` wrapper

```rust
let hello_parser = rp::chars("hello");
let digit_parser = rp::void_(rp::range('0'..='9'));

// this will wrap the parser into Box< dyn Parser >
let mut boxed_parser = rp::box_(hello_parser);
// Note. boxed_parser is mutable

let target_string = "hello0123";

let res_hello = rp::parse(&boxed_parser, target_string.chars());
// success
assert_eq!(res_hello.output, Some(()));
assert_eq!(res_hello.it.clone().collect::<String>(), "0123");

// now change boxed_parser to digit_parser
boxed_parser.assign(digit_parser);

let res_digit = rp::parse(&boxed_parser, res_hello.it);
// success
assert_eq!(res_digit.output, Some(()));
assert_eq!(res_digit.it.collect::<String>(), "123");
```
`Output`: the `Output` of child parser

### `refcell`: a `RefCell<Parser>` wrapper
`RefCelledParser` is useful if it is combined with `BoxedParser` or `RCedParser`.
Since it provides internal mutability.

```rust
let hello_parser = rp::chars("hello");
let digit_parser = rp::void_(rp::range('0'..='9'));

// this will wrap the parser into Box< dyn Parser >
let boxed_parser = rp::box_(hello_parser);
let refcelled_parser = rp::refcell(boxed_parser);
// Note. refcelled_parser is immutable
// but you can change the parser(the Boxed Parser) inside it

let target_string = "hello0123";

let res_hello = rp::parse(&refcelled_parser, target_string.chars());
// success
assert_eq!(res_hello.output, Some(()));
assert_eq!(res_hello.it.clone().collect::<String>(), "0123");

// now change refcelled_parser to digit_parser
refcelled_parser                    // RefCelledParser
    .refcelled_parser()             // &RefCell<BoxedParser>
    .borrow_mut()                   // RefMut<BoxedParser> --> &mut BoxedParser
    .assign(digit_parser.clone());  // assign new parser

// Thanks to Deref, you can call borrow_mut().assign() directly
refcelled_parser.borrow_mut().assign(digit_parser);

let res_digit = rp::parse(&refcelled_parser, res_hello.it);
// success
assert_eq!(res_digit.output, Some(()));
assert_eq!(res_digit.it.collect::<String>(), "123");
```
`Output`: the `Output` of child parser

### `rc`: a `Rc<Parser>` wrapper
`RCedParser` is used to share the same parser.

```rust
let hello_parser = rp::chars("hello");
let digit_parser = rp::void_(rp::range('0'..='9'));

// this will wrap the parser into Box< dyn Parser >
let boxed_parser = rp::box_(hello_parser);
let refcelled_parser = rp::refcell(boxed_parser);
// Note. refcelled_parser is immutable

let rced_parser1 = rp::rc(refcelled_parser);
let rced_parser2 = rp::Rc::clone(&rced_parser1);
// rced_parser2 is now pointing to the same parser as rced_parser1

let target_string = "hello0123";

let res_hello = rp::parse(&rced_parser1, target_string.chars());
// success
assert_eq!(res_hello.output, Some(()));
assert_eq!(res_hello.it.clone().collect::<String>(), "0123");

// now change rced_parser1 to digit_parser
rced_parser1                       // RCedParser
    .rced_parser()                 // &Rc<RefCelledParser>
    .refcelled_parser()            // &RefCell<BoxedParser>
    .borrow_mut()                  // RefMut<BoxedParser> --> &mut BoxedParser
    .assign(digit_parser.clone()); // assign new parser

// Thanks to Deref, you can call borrow_mut().assign() directly
rced_parser1.borrow_mut().assign(digit_parser);

// rced_parser2 should also be digit_parser
let res_digit = rp::parse(&rced_parser2, res_hello.it);
// success
assert_eq!(res_digit.output, Some(()));
assert_eq!(res_digit.it.collect::<String>(), "123");
```
`Output`: the `Output` of child parser


## Making your own Parser
You can design your own Parser by
```rust
parser( closure: impl Fn(&mut It) -> Option<NewOutput> ) -> impl Parser<It>
```

the closure takes mutable reference of the iterator and returns `Option<NewOutput>`.

```rust
let custom_parser = rp::parser(|it: &mut std::str::Chars| {
    if it.take(5).eq("hello".chars()) {
        Some((0,))
    } else {
        // no need to move the iterator back
        None
    }
});

let target_string = "hello0123";
let res = rp::parse(&custom_parser, target_string.chars());
assert_eq!(res.output, Some((0,)));
assert_eq!(res.it.collect::<String>(), "0123");
```

## Others
 Trivial, but useful parsers

### `constant`: This parser will always succeed, and return the constant value
```rust
let parser = rp::constant( (1, 2, 3) );
// Output = (i32, i32, i32)
```

### `end`: success if it reached to the end of input
```rust
let end_parser = rp::end();
```


### `fail`: This parser will always fail
```rust
let parser = rp::fail();
```