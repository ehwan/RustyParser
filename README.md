# RustyParser
A Generic Parser generator and Pattern Matching Library written in Rust

## Example
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

    // parse; put IntoIterator
    let res = digit_parser.parse(target_string.chars());

    // Output = (Charactor Type You Entered,)  -->  (char,)
    // All Parser's Output must be Tuple
    // res.output: Option< Output of the Parser >
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
    let res = a_parser.parse(target_string.chars());
    assert_eq!(res.output, None);

    // iterator will not move if parsing failed
    assert_eq!(res.it.collect::<String>(), target_string);

    // define pattern: [0-9][0-9]
    // perform 'digit_parser', and then 'digit_parser', sequentially
    // Output = ( Output of first Parser, Output of second Parser, )  -->  (char, char,)
    let two_digit_parser = digit_parser.clone().seq(digit_parser);
    //                          ^ move occured here, so clone() it.

    // parse; put IntoIterator
    let res = two_digit_parser.parse(target_string.chars());
    assert_eq!(
        type_name_of_val(&res.output),
        type_name::<Option<(char, char,)>>()
    );
    assert_eq!(res.output, Some(('1', '2')));

    // Output mapping
    // ( char, char, )  -->  (i32, )
    // Parser's Output must be Tuple
    let int_parser = two_digit_parser.map(|(x, y)| -> (i32,) {
        let x_i32 = x as i32 - '0' as i32;
        let y_i32 = y as i32 - '0' as i32;
        (x_i32 * 10 + y_i32,)
    });

    let res = int_parser.parse(target_string.chars());
    assert_eq!(res.output, Some((12,)));

    // pattern matching
    // .match_pattern only checks if the pattern is matched or not
    // it does not try to extract data from input string (e.g. push element in Vec above)
    // Output = always ()
    let res = int_parser.match_pattern(target_string.chars());
    assert_eq!(res.output, Some(()));
}
```

## Structures
Every Parser implements `trait Parser<It>`.
`It` is the type of the iterator that the Parser will work on.

`trait Parser` has associate type `Output` which is the type of the output, the extracted data from the input string.

`trait Parser` has following methods.

 ```rust 
 fn parse(&self, it: It) -> ParseResult<Self::Output, It>;
 fn match_pattern(&self, it: It) -> ParseResult<(), It>;
 ```
 
 which takes an iterator and returns `ParseResult<Self::Output, It>`.
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
(including null-tuple `()`). 
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

### `string`: consumes multiple charactors if it is equal to `s`.
```rust
  let parser = string( s: impl IntoIterator )
  let hello_parser = string("hello".chars()); // &str is not IntoIterator
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
    let res = parser.parse("hello_world_abcdefg".chars());
    assert_eq!(res.output, Some((2,)));
    // 'hello_world' is parsed, so the rest is "_abcdefg"
    assert_eq!(res.it.collect::<String>(), "_abcdefg");

    // match 'hello' only
    let res = parser.parse("hello_wo".chars());
    assert_eq!(res.output, Some((1,)));
  ```
  `Output`: generic type you support

There are two types of Dictionary: `DictBTree` and `DictHashMap`, for Trie implementation.
Both of them have their own Pros and Cons (the memory usage and time complexity of searching), so you can choose one of them.

### `End`: success if it reached to the end of input
```rust
let end_parser = rp::End::new();
let res = end_parser.parse("".chars());
assert_eq!( res.output, Some(()));
```

`Output`: `()`

### Combinators

### `seq`: sequence of parsers
```rust
    let a_parser = rp::one('a');
    let b_parser = rp::one('b');

    // parser sequence
    // 'a', and then 'b'
    let ab_parser = a_parser.seq(b_parser);

    let res = ab_parser.parse("abcd".chars());
    assert_eq!(res.output, Some(('a', 'b')));
    assert_eq!(res.it.collect::<String>(), "cd");
  ```
  `Output`: `( L0, L1, ..., R0, R1, ... )` 
  where `(L0, L1, ...)` are the outputs of the first parser, 
  and `(R0, R1, ...)` are the outputs of the second parser.

### `or_`: or combinator

```rust
    let a_parser = rp::one('a');
    let b_parser = rp::one('b');

    // parser sequence
    // if 'a' is not matched, then try 'b'
    // the order is preserved; if both parser shares condition
    let ab_parser = a_parser.or_(b_parser);

    // 'a' is matched
    let res = ab_parser.parse("abcd".chars());
    assert_eq!(res.output, Some(('a',)));
    assert_eq!(res.it.clone().collect::<String>(), "bcd");

    // continue parsing from the rest
    // 'a' is not matched, but 'b' is matched
    let res = ab_parser.parse(res.it);
    assert_eq!(res.output, Some(('b',)));
    assert_eq!(res.it.clone().collect::<String>(), "cd");

    // continue parsing from the rest
    // 'a' is not matched, 'b' is not matched; failed
    let res = ab_parser.parse(res.it);
    assert_eq!(res.output, None);
    assert_eq!(res.it.clone().collect::<String>(), "cd");
```
`Output`: `Output` of the first and second parser.
Note that the output of both parsers must be the same type.


### `map`: map the output of the parser
```rust
    let a_parser = rp::one('a');

    // map the output
    // (Charactor Type You Entered,)  -->  (i32, )
    let int_parser = a_parser.map(|(ch,)| -> (i32,) { (ch as i32 - 'a' as i32,) });

    let res = int_parser.parse("abcd".chars());
    assert_eq!(res.output, Some((0,)));
    assert_eq!(res.it.collect::<String>(), "bcd");
```
`Output`: return type of the closure ( must be Tuple )

### `repeat`: repeat the parser multiple times

```rust
    let a_parser = rp::one('a');

    // repeat 'a' 3 to 5 times (inclusive)
    let multiple_a_parser = a_parser.repeat(3..=5);

    let res = multiple_a_parser.parse("aaaabcd".chars());
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
    let a_parser = rp::one('a');
    let a_parser = a_parser.map(|(_,)| -> (i32,) {
        // some expensive operations....
        panic!("This should not be called");
    });
    let multiple_a_parser = a_parser.repeat(3..=5);
    let multiple_a_void_parser = multiple_a_parser.void_();

    // ignore the output of parser
    // this internally calls 'match_pattern(...)' instead of 'parse(...)'
    let res = multiple_a_void_parser.parse("aaaabcd".chars());
    assert_eq!(res.output, Some(()));
    assert_eq!(res.it.collect::<String>(), "bcd");
```
`Output`: `()`

## For complex, highly recursive pattern

By default, all the 'parser-generating' member functions consumes `self` and returns a new Parser. 
And `Parser::parse(&self)` takes immutable reference of Self.

However, in some cases, you may want to define a recursive parser.
Which involves 'reference-of-parser' or 'virtual-class-like' structure.

Luckily, Rust std provides wrapper for these cases.
`Rc`, `RefCell`, `Box` are the most common ones.

RustyParser provides `BoxedParser`, `RCedParser`, `RefCelledParser` which are Parser Wrapper for `Box`, `Rc`, `RefCell`.

### `boxed`: a `Box<dyn Parser>` wrapper

```rust
    let hello_parser = rp::string("hello".chars());
    let digit_parser = rp::range('0'..='9').void_(); // force the output to be ()

    // this will wrap the parser into Box< dyn Parser >
    let mut boxed_parser = hello_parser.boxed();
    // Note. boxed_parser is mutable

    let target_string = "hello0123";

    let res_hello = boxed_parser.parse(target_string.chars());
    // success
    assert_eq!(res_hello.output, Some(()));
    assert_eq!(res_hello.it.clone().collect::<String>(), "0123");

    // now change boxed_parser to digit_parser
    boxed_parser = digit_parser.boxed();
    // this is same as:
    // boxed_parser.assign(digit_parser);

    let res_digit = boxed_parser.parse(res_hello.it);
    // success
    assert_eq!(res_digit.output, Some(()));
    assert_eq!(res_digit.it.collect::<String>(), "123");
```
`Output`: the `Output` of child parser

### `refcelled`: a `RefCell<Parser>` wrapper
`RefCelledParser` is useful if it is combined with `BoxedParser` or `RCedParser`.
Since it provides internal mutability.

```rust
    let hello_parser = rp::string("hello".chars());
    let digit_parser = rp::range('0'..='9').void_();

    // this will wrap the parser into Box< dyn Parser >
    let boxed_parser = hello_parser.boxed();
    let refcelled_parser = boxed_parser.refcelled();
    // Note. refcelled_parser is immutable

    let target_string = "hello0123";

    let res_hello = refcelled_parser.parse(target_string.chars());
    // success
    assert_eq!(res_hello.output, Some(()));
    assert_eq!(res_hello.it.clone().collect::<String>(), "0123");

    // now change refcelled_parser to digit_parser
    refcelled_parser           // RefCelledParser
        .refcelled_parser()    // &RefCell<BoxedParser>
        .borrow_mut()          // RefMut<BoxedParser> --> &mut BoxedParser
        .assign(digit_parser); // assign new parser

    let res_digit = refcelled_parser.parse(res_hello.it);
    // success
    assert_eq!(res_digit.output, Some(()));
    assert_eq!(res_digit.it.collect::<String>(), "123");
```
`Output`: the `Output` of child parser

### `rced`: a `Rc<Parser>` wrapper
`RCedParser` is used to share the same parser.

```rust
    let hello_parser = rp::string("hello".chars());
    let digit_parser = rp::range('0'..='9').void_();

    // this will wrap the parser into Box< dyn Parser >
    let boxed_parser = hello_parser.boxed();
    let refcelled_parser = boxed_parser.refcelled();
    // Note. refcelled_parser is immutable

    let rced_parser1 = refcelled_parser.rced();
    let rced_parser2 = rp::RCed::clone(&rced_parser1);
    // rced_parser2 is now pointing to the same parser as rced_parser1

    let target_string = "hello0123";

    let res_hello = rced_parser1.parse(target_string.chars());
    // success
    assert_eq!(res_hello.output, Some(()));
    assert_eq!(res_hello.it.clone().collect::<String>(), "0123");

    // now change rced_parser1 to digit_parser
    rced_parser1               // RCedParser
        .rced_parser()         // &Rc<RefCelledParser>
        .refcelled_parser()    // &RefCell<BoxedParser>
        .borrow_mut()          // RefMut<BoxedParser> --> &mut BoxedParser
        .assign(digit_parser); // assign new parser

    // rced_parser2 should also be digit_parser
    let res_digit = rced_parser2.parse(res_hello.it);
    // success
    assert_eq!(res_digit.output, Some(()));
    assert_eq!(res_digit.it.collect::<String>(), "123");
```
`Output`: the `Output` of child parser