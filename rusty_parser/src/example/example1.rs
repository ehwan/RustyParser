use crate as rp;

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

#[test]
fn dict_example() {
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
}

#[test]
fn seq_example() {
    let a_parser = rp::one('a');
    let b_parser = rp::one('b');

    // 'a', and then 'b'
    let ab_parser = rp::seq!(a_parser, b_parser);

    let res = rp::parse(&ab_parser, "abcd".chars());
    assert_eq!(res.output, Some(('a', 'b')));
    assert_eq!(res.it.collect::<String>(), "cd");
}

#[test]
fn or_example() {
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
}

#[test]
fn map_example() {
    let a_parser = rp::one('a');

    // map the output
    // <Output of 'a'> (char,) -> (i32,)
    let int_parser = rp::map(a_parser, |(ch,)| -> (i32,) { (ch as i32 - 'a' as i32,) });

    let res = rp::parse(&int_parser, "abcd".chars());
    assert_eq!(res.output, Some((0,)));
    assert_eq!(res.it.collect::<String>(), "bcd");
}

#[test]
fn repeat_example() {
    let a_parser = rp::one('a');

    // repeat 'a' 3 to 5 times (inclusive)
    let multiple_a_parser = rp::repeat(a_parser.clone(), 3..=5);
    let res = rp::parse(&multiple_a_parser, "aaaabcd".chars());

    // four 'a' is parsed
    assert_eq!(res.output, Some((vec!['a', 'a', 'a', 'a',],)));
    assert_eq!(res.it.collect::<String>(), "bcd");
}

#[test]
fn void_example() {
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
}

#[test]
fn iter_example() {
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
}

/*
#[test]
fn custom_parser_example() {
    let parser = rp::parser(|it: &mut std::str::Chars| {
        if it.take(5).eq("hello".chars()) {
            Some((0,))
        } else {
            // no need to move the iterator back
            None
        }
    });

    let target_string = "hello0123";
    let res = parser.parse(target_string.chars());
    assert_eq!(res.output, Some((0,)));
    assert_eq!(res.it.collect::<String>(), "0123");
}
#[test]
fn box_example() {
    let hello_parser = rp::chars("hello");
    let digit_parser = rp::range('0'..='9').void_();

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
}

#[test]
fn refcell_example() {
    let hello_parser = rp::chars("hello");
    let digit_parser = rp::range('0'..='9').void_();

    // this will wrap the parser into Box< dyn Parser >
    let boxed_parser = hello_parser.boxed();
    let refcelled_parser = boxed_parser.refcelled();
    // Note. refcelled_parser is immutable
    // but you can change the parser(the Boxed Parser) inside it

    let target_string = "hello0123";

    let res_hello = refcelled_parser.parse(target_string.chars());
    // success
    assert_eq!(res_hello.output, Some(()));
    assert_eq!(res_hello.it.clone().collect::<String>(), "0123");

    // now change refcelled_parser to digit_parser
    refcelled_parser // RefCelledParser
        .refcelled_parser() // &RefCell<BoxedParser>
        .borrow_mut() // RefMut<BoxedParser> --> &mut BoxedParser
        .assign(digit_parser.clone()); // assign new parser

    // Thanks to Deref, you can call borrow_mut().assign() directly
    refcelled_parser.borrow_mut().assign(digit_parser);

    let res_digit = refcelled_parser.parse(res_hello.it);
    // success
    assert_eq!(res_digit.output, Some(()));
    assert_eq!(res_digit.it.collect::<String>(), "123");
}

#[test]
fn rc_example() {
    let hello_parser = rp::chars("hello");
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
    rced_parser1 // RCedParser
        .rced_parser() // &Rc<RefCelledParser>
        .refcelled_parser() // &RefCell<BoxedParser>
        .borrow_mut() // RefMut<BoxedParser> --> &mut BoxedParser
        .assign(digit_parser.clone()); // assign new parser

    // Thanks to Deref, you can call borrow_mut().assign() directly
    rced_parser1.borrow_mut().assign(digit_parser);

    // rced_parser2 should also be digit_parser
    let res_digit = rced_parser2.parse(res_hello.it);
    // success
    assert_eq!(res_digit.output, Some(()));
    assert_eq!(res_digit.it.collect::<String>(), "123");
}


*/
