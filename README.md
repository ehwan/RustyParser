# RustyParser
A Generic compile-time Parser generator and Pattern Matching Library written in Rust

RustyParser provides a set of basic parsers, combinators, and parser-generating functions.

This library is designed to work with general iterators, but some functionalities are limited to `std::str::Chars` or `std::iter::Cloned<std::slice::Iter>`.

## Example
 - **[Calculator Expresion Parser](examples/calculator)**

 - **[JSON Parser](examples/json)**


## Sample Code

```rust
// import rusty_parser
use rusty_parser as rp;

// useful trait member functions
use rp::IntoParser;

#[test]
fn example1() {
    // define pattern
    // digit: [0-9]
    // this will match one digit, and returns (char,), the character it parsed
    let digit_parser = rp::range('0'..='9');

    // define pattern
    // num: digit+
    // this will match one or more digits, and returns (Vec<char>,), the character it parsed
    let num_parser = digit_parser.repeat(1..);

    // map the output
    // ( Vec<char>, )  -->  (i32, )
    let num_parser = num_parser.map(|(digits,): (Vec<char>,)| -> (i32,) {
        let mut num = 0;
        for ch in digits {
            num = num * 10 + (ch as i32 - '0' as i32);
        }
        (num,)
    });

    // parse input iterator with given pattern, and return the result
    let res = rp::parse(&num_parser, "123456hello_world".chars());

    // res contains the result of parsing
    assert_eq!(res.output.unwrap(), (123456,));

    // res.it: iterator after parsing
    // here, '123456' is parsed, so the rest is "hello_world"
    assert_eq!(res.it.collect::<String>(), "hello_world");
}
```

## Structures
Define pattern, combine them, and parse the input.

RustyParser provides a set of basic parsers, combinators, and parser-generating functions.


Those generated parsers are used to parse the input string, and return the extracted data.
 ```rust 
 fn parse(pattern:&Pattern, it:It) -> ParseResult<(Parsed Output of Pattern), It>;
 fn match_pattern(pattern:&Pattern, it:It) -> ParseResult<(), It>;
 ```
`parse(...)` takes a Pattern Object and iterator of input string, then returns `ParseResult<Self::Output, It>`.

 `match_pattern(...)` is used 
 when you only want to check if the pattern is matched or not, without extracting data. 
 For some parsers, like `repeat`, it is expensive to call `parse(...)` to get the output since it invokes `Vec::push` inside.


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

 ### Note
  - Since the `parse(...)` internally clones the iterator, the iterator must be cheaply clonable.
  - `Output` must be `Tuple`, including `()`. If you want to return a single value, use `(Value,)`.


## Basic Parsers

 ### `one`: consumes one charactor if it is equal to `c`.
 ```rust
 let parser = one( c: CharactorType )

 let a_parser = one('a')
 let a_parser = 'a'.into_parser()
 ```
`Output`: `(Iterator::Item,)`


### `range`: consumes one charactor if it is in the range `r`.
```rust
let parser = range( r: impl std::ops::RangeBounds )

let digit_parser = range( '0'..='9' )
let digit_parser = ('0'..='9').into_parser()
```
`Output`: `(Iterator::Item,)`


### `str`, `slice`: consumes multiple charactors if it is equal to `s`.

For borrowing-safety, the lifetime of str or slice must be 'static.

To use with other lifetime, you should use `string()` or `vec()` instead. Those functions will clone the items in `String`, `Vec`.

```rust
// must be 'static
let hello_parser = str("hello");
let hello_parser = "hello".into_parser();

let hello_parser = slice(&[104, 101, 108, 108, 111]);
let hello_parser = (&[104, 101, 108, 108, 111]).into_parser();
```
`Output`: `()`

### `string`, `vec`: consumes multiple charactors if it is equal to `s`.
This will copy all the characters into `String` or `Vec`, so lifetime belongs to the parser itself.

```rust
let hello_parser = string("hello".to_string());
let hello_parser = "hello".to_string().into_parser();

let hello_parser = vec(vec![104, 101, 108, 108, 111]);
let hello_parser = (vec![104, 101, 108, 108, 111]).into_parser();
```
`Output`: `()`

### `check`: check single charactor with a closure
The closure must be: `Fn(Iterator::Item) -> bool`

```rust
let parser = check( |ch:char| ch.is_alphabetic() );
```

`Output`: `(Iterator::Item,)`


### `any`: Match any character.

```rust
let parser = any();
```

`Output`: `(Iterator::Item,)`


### Dictionary: build Trie from a list of strings
```rust
// let mut parser = rp::DictBTree::new();
let mut parser = rp::DictHashMap::new();

parser.insert("hello".chars(), (1,));
parser.insert("hello_world".chars(), (2,));
parser.insert("world".chars(), (3,));

// this will match as long as possible
let res = rp::parse(&parser, "hello_world_abcdefg".chars());
assert_eq!(res.output.unwrap(), (2,));
// 'hello_world' is parsed, so the rest is "_abcdefg"
assert_eq!(res.it.collect::<String>(), "_abcdefg");

// match 'hello' only
let res = rp::parse(&parser, "hello_wo".chars());
assert_eq!(res.output.unwrap(), (1,));
```
`Output`: generic type you inserted

There are two types of Dictionary: `DictBTree` and `DictHashMap` for Trie implementation.
Both of them have their own Pros and Cons (the memory usage and time complexity of searching), so you can choose one of them.




## Combinators

### `seq`: sequence of parsers
```rust
// 'a', and then 'b'
let ab_parser = rp::seq!('a', 'b', 'c'); // IntoParser for char

let res = rp::parse(&ab_parser, "abcd".chars());
assert_eq!(res.output.unwrap(), ('a', 'b', 'c')); // Output is concatenated
assert_eq!(res.it.collect::<String>(), "d");
```

`Output`: `( A0, A1, ..., B0, B1, ..., C0, C1, ... )`
where `(A0, A1, ...)` are the output of the first parser,
and `(B0, B1, ...)`, `(C0, C1, ...)` are the output of the following parsers.


### `or`: or combinator

```rust
// 'a' or 'b'
let ab_parser = rp::or!('a', 'b'); // IntoParser for char

// 'a' is matched
let res = rp::parse(&ab_parser, "abcd".chars());
assert_eq!(res.output.unwrap(), ('a',)); // Output of 'a'
assert_eq!(res.it.clone().collect::<String>(), "bcd");

// continue parsing from the rest
// 'a' is not matched, but 'b' is matched
let res = rp::parse(&ab_parser, res.it);
assert_eq!(res.output.unwrap(), ('b',));
assert_eq!(res.it.clone().collect::<String>(), "cd");

// continue parsing from the rest
// 'a' is not matched, 'b' is not matched; failed
let res = rp::parse(&ab_parser, res.it);
assert_eq!(res.output, None);
assert_eq!(res.it.clone().collect::<String>(), "cd");
```

`Output`: `Output` of the all parsers.
Note that the output of all parsers must be the same type.




### `map`: map the output of the parser
```rust
// map the output
// <Output of 'a'> -> (i32,)
let int_parser = 'a'.map(|(ch,)| -> (i32,) { (ch as i32 - 'a' as i32,) }); // IntoParser for char

let res = rp::parse(&int_parser, "abcd".chars());
assert_eq!(res.output.unwrap(), (0,));
assert_eq!(res.it.collect::<String>(), "bcd");
```
`Output`: return type of the closure ( must be Tuple )


### `repeat`: repeat the parser multiple times

```rust
// repeat 'a' 3 to 5 times
let multiple_a_parser = 'a'.repeat(3..=5); // IntoParser for char
let res = rp::parse(&multiple_a_parser, "aaaabcd".chars());

// four 'a' is parsed
assert_eq!(res.output.unwrap(), (vec!['a', 'a', 'a', 'a',],));
assert_eq!(res.it.collect::<String>(), "bcd");
```

`Output`: 
 - if `Output` of the repeated parser is `()`, then `Output` is `()`
 - if `Output` of the repeated parser is `(T,)`, then `Output` is `Vec<T>`
 - otherwise, `Vec< Output of the Repeated Parser >`



### `optional`, `optional_or`: success whether the pattern is matched or not
```rust
let a_optional_parser = 'a'.optional(); // (Option<char>,)

let res = rp::parse(&a_optional_parser, "abcd".chars()); // success
assert_eq!(res.output.unwrap(), (Some('a'),));

let res = rp::parse(&a_optional_parser, "bcd".chars()); // success, but 'a' is not matched
assert_eq!(res.output.unwrap(), (None,));

// if 'a' failed, return 'x'
let a_optional_or = 'a'.optional_or(('x',)); // (char,)

let res = rp::parse(&a_optional_or, "bcd".chars());
assert_eq!(res.output.unwrap(), ('x',));
```
`Output` for `optional`:
 - if `Output` of the origin parser is `(T0,)`, `(Option<T0>,)`
 - otherwise, `( Option<Output of the Origin Parser>, )`

 `Output` for `optional_or`:
  <`Output` of the origin parser>. 
  The value given to `optional_or` must match with the `Output` of the origin parser.



### `not`: match for Pattern1 to success and Pattern2 to fail
```rust
// all digit but not 4
let digit_parser_except_4 = ('0'..='9').not('4');

let res = rp::parse(&digit_parser_except_4, "3".chars());
assert_eq!(res.output.unwrap(), ('3',));

let res = rp::parse(&digit_parser_except_4, "4".chars());
assert_eq!(res.output, None);
```
`Output`: `Output` of the first parser



## For complex, recursive pattern

By default, all the 'parser-generating' functions consumes input Parser and returns a new instance.
These processes create new generic Parser object entirely at compile-time.

However, in some cases, you may want to define a recursive parser.
Which involves 'reference-of-parser' or 'virtual-class-like' structure.

Luckily, Rust std provides wrapper for these cases.
`Rc`, `RefCell`, `Box` are the most common ones.

RustyParser provides `box_*`, `rc`, `refcell` which are Parser wrapper for `Box`, `Rc`, `RefCell`.

### `box_chars`, `box_slice`: a `Box<dyn Parser>` wrapper

this function wraps the parser into `Box<dyn Parser>`.
You can dynamically assign ***any parsers*** with same `Output` type.

#### Note
Currently only implemented for `std::str::Chars` and `std::iter::Cloned<std::slice::Iter>`.
Once you wrap the parser through `box_chars` or `box_slice`, you can only use corresponding iterator in `parse(...)`.

```rust
let hello_parser = "hello".into_parser();
let digit_parser = ('0'..='9').void();

// this will wrap the parser into Box< dyn Parser >
let mut boxed_parser = hello_parser.box_chars();

let res_hello = rp::parse(&boxed_parser, "hello0123".chars());
// success
assert_eq!(res_hello.output.unwrap(), ());
assert_eq!(res_hello.it.clone().collect::<String>(), "0123");

// now change boxed_parser to digit_parser
boxed_parser.assign(digit_parser);

let res_digit = rp::parse(&boxed_parser, res_hello.it);
// success
assert_eq!(res_digit.output.unwrap(), ());
assert_eq!(res_digit.it.collect::<String>(), "123");
```
`Output`: the `Output` of child parser



### `refcell`: a `RefCell<Parser>` wrapper
`refcell` is useful when it is combined with `box_*` or `rc`,
since it provides internal mutability.

```rust
let hello_parser = "hello".into_parser();
let digit_parser = ('0'..='9').void();

let refcelled_parser = hello_parser.box_chars().refcell();

let res_hello = rp::parse(&refcelled_parser, "hello0123".chars());
// success
assert_eq!(res_hello.output.unwrap(), ());
assert_eq!(res_hello.it.clone().collect::<String>(), "0123");

// now change refcelled_parser to digit_parser
// Thanks to Deref, you can call borrow_mut().assign() directly
refcelled_parser.borrow_mut().assign(digit_parser);

let res_digit = rp::parse(&refcelled_parser, res_hello.it);
// success
assert_eq!(res_digit.output.unwrap(), ());
assert_eq!(res_digit.it.collect::<String>(), "123");
```
`Output`: the `Output` of child parser



### `rc`: a `Rc<Parser>` wrapper
`rc` is used to share the same parser.

```rust
let hello_parser = "hello".into_parser();
let digit_parser = ('0'..='9').void();

let rc_parser1 = hello_parser.box_chars().refcell().rc();
let rc_parser2 = rp::Rc::clone(&rc_parser1);
// rc_parser2 is now pointing to the same parser as rc_parser1

let res_hello = rp::parse(&rc_parser1, "hello0123".chars());
// success
assert_eq!(res_hello.output.unwrap(), ());
assert_eq!(res_hello.it.clone().collect::<String>(), "0123");

// now change rced_parser1 to digit_parser
// Thanks to Deref, you can call borrow_mut().assign() directly
rc_parser1.borrow_mut().assign(digit_parser);

// rced_parser2 should also be digit_parser
let res_digit = rp::parse(&rc_parser2, res_hello.it);
// success
assert_eq!(res_digit.output.unwrap(), ());
assert_eq!(res_digit.it.collect::<String>(), "123");
```
`Output`: the `Output` of child parser


## Others
 Trivial, but useful parsers

### `constant`: This parser will always succeed, and return the constant value
```rust
let parser = rp::constant( (1, 2, 3) );
```
`Output`: the Tuple value you provided



### `end`: success if it reached to the end of input
```rust
let end_parser = rp::end();
```
`Output`: `()`



### `fail`: This parser will always fail
```rust
let parser = rp::fail();
```
`Output`: `()`



### `void`: ignore the output of the parser
Force the output to be `()`. 
It internally calls `match_pattern(...)` instead of `parse(...)`. 
This is useful when you only want to check if the pattern is matched or not. 
For more information, see `match_pattern(...)` above.

```rust
let expensive_parser = 'a'.map(|(_,)| -> (i32,) {
    // some expensive operations for data extracting...
    panic!("This should not be called");
});
let expensive_parser = expensive_parser.void();

// ignore the output of parser
// this internally calls 'match_pattern(...)' instead of 'parse(...)'
let res = rp::parse(&expensive_parser, "abcd".chars());
assert_eq!(res.output.unwrap(), ());
assert_eq!(res.it.collect::<String>(), "bcd");
```
`Output`: `()`




### `output`: change the output of the parser
```rust
let digit_parser = ('0'..='9').output((1, 2, 3));

let res = rp::parse(&digit_parser, "123456hello_world".chars());
assert_eq!(res.output.unwrap(), (1, 2, 3));
assert_eq!(res.it.collect::<String>(), "23456hello_world");
```
`Output`: the Tuple value you provided



### `string`, `vec`: captures the matched range into String or Vec\<T\>

#### Note
`string` can be only used for `std::str::Chars`, and `vec` can be only used for `std::iter::Cloned<std::slice::Iter>`.

```rust
let digits_parser = ('0'..='9').repeat(0..).string();

let res = rp::parse(&digits_parser, "123456hello_world".chars());
assert_eq!(res.output.unwrap(), ("123456".to_string(),));
assert_eq!(res.it.collect::<String>(), "hello_world");
```
`Output`: `(String,)` or `(Vec<T of Slice>,)`

### `not_consume`: check if the pattern is matched or not, without consuming the input
```rust
let digit_parser = ('0'..='9').not_consume();

let res = rp::parse(&digit_parser, "12345".chars());
assert_eq!(res.output.unwrap(), ('1',));
assert_eq!(res.it.collect::<String>(), "12345"); // iterator is not consumed
```
`Output`: `Output` of the parser
