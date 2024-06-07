# RustyParser
A Generic compile-time Parser generator and Pattern Matching Library written in Rust

RustyParser provides a set of basic parsers, combinators, and parser-generating functions.

This library is designed to work with general iterators, 
but some functionalities are limited to specific iterators.

## Example
 - **[Calculator Expresion Parser](examples/calculator)**

 - **[JSON Parser](examples/json)**

 - **[mini C language Tokenizer and AST Parser](https://github.com/ehwan/C-Parser-In-Rust)**


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
    // Vec<char>  -->  i32
    let num_parser = num_parser.map(|digits:Vec<char>| -> i32 {
        let mut num = 0;
        for ch in digits {
            num = num * 10 + (ch as i32 - '0' as i32);
        }
        num
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

## Parsers Overview

### Basic(Leaf) Parsers
| Parser | Description | Output |
| :------: | ----------- | :------: |
| `one`, `one_by` | Match one charactor | `(Iterator::Item,)` |
| `range` | Match one charactor in the range | `(Iterator::Item,)` |
| `str`, `str_by`, `slice`, `slice_by` | Match multiple charactors | `()` |
| `string`, `string_by`, `vec`, `vec_by` | Match multiple charactors | `()` |
| `check` | Check one charactor with closure | `(T,)` |
| `any` | Match any charactor | `(Iterator::Item,)` |
| `DictBTree`, `DictHashMap` | Trie Dictionary | `T` |
| `DynBoxChars`, `DynBoxSlice`, `DynBoxSliceCopied` | Dynamic Parser that can take any parser with same `Output` | `T` |

### Combinators
| Combinator | Description | Output |
| :------: | ----------- | :------: |
| `seq` | Sequence of parsers | `( *<Output of A>, *<Output of B> ... )`(Tuple Concatenated ) |
| `or` | Or combinator | `Output` of the all parsers |
| `map` | Map the output of the parser | `(T,)` |
| `repeat` | Repeat the parser multiple times | `(Vec<Output of Self>,)` |
| `optional` | Success whether the pattern is matched or not | `( Option<Output of Self>, )` |
| `optional_or` | Success whether the pattern is matched or not | `Output` of `Self` |
| `not` | Match for Pattern1 to success and Pattern2 to fail | `Output` of `Self` |
| `reduce_left`, `reduce_right` | Reduce the output of the parser | `Output` of `Self` |


### Others
| Parser | Description | Output |
| :------: | ----------- | :------: |
| `constant` | Always succeed, and return the constant value | `()` |
| `end` | Success if it reached to the end of input | `()` |
| `fail` | Always fail | `()` |
| `void` | Ignore the output of the parser | `()` |
| `output` | Change Parser's Output to `(output,)` | `(T,)` |
| `string`, `vec` | Captures the matched range into `String` or `Vec<T>` | `(String,)` or `(Vec<Iterator::Item>,)` |
| `not_consume` | Check if the pattern is matched or not, without consuming the input | `Output` of `Self` |


## Basic Parsers

### `one`, `one_by`: consumes one character if it is equal to `c`.
```rust
let parser = one( c: CharType )

let a_parser = one('a')
let a_parser = 'a'.into_parser()

let a_parser = one_by('a', |value:char, ch:&char| value.to_ascii_lowercase() == *ch );
```
`Output`: `(Iterator::Item,)`


### `range`: consumes one character if it is in the range `r`.
```rust
let parser = range( r: impl std::ops::RangeBounds )

let digit_parser = range( '0'..='9' )
let digit_parser = ('0'..='9').into_parser()
```
`Output`: `(Iterator::Item,)`


### `str`, `str_by`, `slice`, `slice_by`: consumes multiple characters if it is equal to `s`.

For borrowing-safety, the lifetime of str or slice must be 'static.

To use with other lifetime, you should use `string()` or `vec()` instead. Those functions will clone the items in `String`, `Vec`.

```rust
// must be 'static
let hello_parser = str("hello");
let hello_parser = "hello".into_parser();
let hello_parser = str_by("hello", |value:char, ch:char| value.to_ascii_lowercase() == ch );

let hello_parser = slice(&[104, 101, 108, 108, 111]);
let hello_parser = (&[104, 101, 108, 108, 111]).into_parser();
let hello_parser = slice_by(&[104, 101, 108, 108, 111], |value:i32, ch:&i32| value == *ch );
```
`Output`: `()`

### `string`, `string_by`, `vec`, `vec_by`: consumes multiple characters if it is equal to `s`.
This will copy all the characters into `String` or `Vec`, so lifetime belongs to the parser itself.

```rust
let hello_parser = string("hello".to_string());
let hello_parser = "hello".to_string().into_parser();
let hello_parser = string_by("hello".to_string(), |value:char, ch:char| value.to_ascii_lowercase() == ch );

let hello_parser = vec(vec![104, 101, 108, 108, 111]);
let hello_parser = (vec![104, 101, 108, 108, 111]).into_parser();
let hello_parser = vec_by(vec![104, 101, 108, 108, 111], |value:i32, ch:&i32| value == *ch );
```
`Output`: `()`

### `check`: check single character with a closure
The closure must be either of:
`Fn(Iterator::Item) -> Option<NewOutput>`
or
`Fn(Iterator::Item) -> bool`.

```rust
let parser = check( |ch:char| if ch.is_alphabetic() { Some(ch) }else{ None } ); // returns Option<char> -> `(char,)` as output
let parser = check( |ch:char| ch.is_alphabetic() ); // returns bool -> `()` as output
```

If the closure returns `Option<NewOutput>`, the output will be `(NewOutput,)`.
If the closure returns `bool`, the output will be `()`.


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

This will match as long as possible, regardless of the order of insertion.

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
Parser's Output(Tuple) will be unpacked and passed to the closure. The value returned from the closure will be new Output.
```rust
// map the output
// <Output of 'a'> -> i32
let int_parser = 'a'.map(|ch| -> i32 { ch as i32 - 'a' as i32 }); // IntoParser for char

let res = rp::parse(&int_parser, "abcd".chars());
assert_eq!(res.output.unwrap(), (0,));
assert_eq!(res.it.collect::<String>(), "bcd");
```
`Output`: `(T,)` where `T` is return type of the closure. The value `v` returned from the closure will be wrapped into `(v,)`.


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
 - otherwise, `Vec< Output of Self >`



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
 - otherwise, `( Option<Output of Self>, )`

 `Output` for `optional_or`:
  - <`Output` of `Self`>. 

#### Note
 - The passed value's type to `optional_or` must match with the `Output` of `Self`
 - For single-value-output ( which's output is `(T,)` ), passing either `T` or `(T,)` is permitted.



### `not`: match for Pattern1 to success and Pattern2 to fail
```rust
// all digit but not 4
let digit_parser_except_4 = ('0'..='9').not('4');

let res = rp::parse(&digit_parser_except_4, "3".chars());
assert_eq!(res.output.unwrap(), ('3',));

let res = rp::parse(&digit_parser_except_4, "4".chars());
assert_eq!(res.output, None);
```
`Output`: `Output` of `Self`

### `reduce_left`: reduce the output of the parser
With given input string `self rhs rhs rhs rhs ...` and the reducer `f`,
the output will be calculated as
`f( f( f(self,rhs), rhs ), rhs ), ...`

#### Note
- The signature of the reducer must be `Fn(A0, A1, A2, ..., B0, B1, B2, ...) -> ( A0, A1, A2 ... )`.
  Where `(A0, A1, A2, ...)` are the output of the first parser, and `(B0, B1, B2, ...)` are the output of the following parser.

- For single-value-output ( which's output is `(T,)` ),
  returning either `T` or `(T,)` is permitted.

```rust
let digit_parser = ('0'..='9').into_parser().map(|val: char| -> i32 { val as i32 - '0' as i32 });
let reduced_left = digit_parser.reduce_left(digit_parser, |lhs, rhs| lhs * 10 + rhs);
let res = rp::parse( &reduced_left, "123456abcd".chars() );
assert_eq!(res.output.unwrap(), (123456,));
assert_eq!(res.it.collect::<String>(), "abcd");
```

`Output`: `Output` of `Self`

### `reduce_right`: reduce the output of the parser
With given input string `lhs lhs lhs lhs ... self` and the reducer `f`,
the output will be calculated as
`f(lhs, f(lhs, f(lhs, f( ... f(lhs,self)))`

#### Note
- The signature of the reducer must be `Fn(A0, A1, A2, ..., B0, B1, B2, ...) -> ( B0, B1, B2 ... )`.
  Where `(A0, A1, A2, ...)` are the output of the first parser, and `(B0, B1, B2, ...)` are the output of the following parser.

- For single-value-output ( which's output is `(T,)` ),
  returning either `T` or `(T,)` is permitted.

```rust
let digit_parser =
    ('0'..='9').into_parser().map(|val: char| -> i32 { val as i32 - '0' as i32 });
let alphabet_parser =
    ('a'..='z').into_parser().map(|val: char| -> i32 { val as i32 - 'a' as i32 });
let reduced_right =
    alphabet_parser.reduce_right(digit_parser, |lhs: i32, rhs: i32| -> i32 { rhs * 10 + lhs });

let res = rp::parse(&reduced_right, "123456dcba".chars());
assert_eq!(res.output.unwrap(), (3654321,));
assert_eq!(res.it.collect::<String>(), "cba");
```

`Output`: `Output` of `Self`

## For complex, recursive pattern

By default, all the 'parser-generating' functions consumes input Parser and returns a new instance.
These processes create new generic Parser object entirely at compile-time.

However, in some cases, you may want to define a recursive parser.
Which involves 'reference-of-parser' or 'virtual-class-like' structure.

Luckily, Rust std provides wrapper for these cases.
`Rc`, `RefCell`, `Box` are the most common ones.

For `Rc` and `RefCell`, you can wrap any parser with them. They will be treated as a `Parser` object.
```rust
// making shared, interior-mutable parser
let hello_parser = "hello".into_parser();
let hello_parser = std::cell::RefCell::new(hello_parser);
let hello_parser = std::rc::Rc::new(hello_parser);
```

For `Box`, you can use `DynBox*` to wrap any parser.
With `DynBox*`, you can assign **any parser** with same `Output` type.
```rust
let hello_parser = "hello".into_parser();

let mut dynamic_parser: DynBoxChars<(char,)> = Default::new(); // Default implemented
dynamic_parser.parse( "hello".chars() ); // this will panic, since the parser is not assigned yet

// set dynamic_parser to hello_parser
dynamic_parser.assign( "hello" );
let res = dynamic_parser.parse( "hello".chars() ); // success

// set dynamic_parser to digit_parser
dynamic_parser.assign( '0'..='9' );
let res = dynamic_parser.parse( "01234".chars() ); // success
```

`Default` trait is implemented with always-panic-parser. You must assign it later.

For now, there are three types of `DynBox*`:
 - `DynBoxChars<Output>`: for `std::str::Chars`
 - `DynBoxSlice<Output,T>`: for `std::iter::Cloned<std::slice::Iter<T>>`
 - `DynBoxSliceCopied<Output,T>`: for `std::iter::Copied<std::slice::Iter<T>>`
Once you wrap the parser through `DynBox*`, you can only use corresponding iterator in `parse(...)`.

You can refer [HERE](rusty_parser/src/wrapper/boxed) for other iterator types.


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
let expensive_parser = 'a'.map(|_| -> i32 {
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




### `output`: Change Parser's Output to `(output,)`
```rust
let digit_parser = ('0'..='9').output(2024);

let res = rp::parse(&digit_parser, "123456hello_world".chars());
assert_eq!(res.output.unwrap(), (2024,));
assert_eq!(res.it.collect::<String>(), "23456hello_world");
```
`Output`: `(T,)` where `T` is the type of the value you provided.


### `string`, `vec`: captures the matched range into `String` or `Vec<T>`

#### Note
`string` can be only used for `std::str::Chars`, 
and `vec` can be only used for `ExactSizeIterator`.

```rust
let digits_parser = ('0'..='9').repeat(0..).string();

let res = rp::parse(&digits_parser, "123456hello_world".chars());
assert_eq!(res.output.unwrap(), ("123456".to_string(),));
assert_eq!(res.it.collect::<String>(), "hello_world");
```
`Output`: `(String,)` or `(Vec<Iterator::Item>,)`

### `not_consume`: check if the pattern is matched or not, without consuming the input
```rust
let digit_parser = ('0'..='9').not_consume();

let res = rp::parse(&digit_parser, "12345".chars());
assert_eq!(res.output.unwrap(), ('1',));
assert_eq!(res.it.collect::<String>(), "12345"); // iterator is not consumed
```
`Output`: `Output` of `Self`
