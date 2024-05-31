use crate as rp;

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
    let num_parser = digit_parser.repeat(0..);

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

#[test]
fn dict_example() {
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
}

#[test]
fn seq_example() {
    // 'a', and then 'b'
    let ab_parser = rp::seq!('a', 'b', 'c'); // IntoParser for char

    let res = rp::parse(&ab_parser, "abcd".chars());
    assert_eq!(res.output.unwrap(), ('a', 'b', 'c')); // Output is concatenated
    assert_eq!(res.it.collect::<String>(), "d");
}

#[test]
fn or_example() {
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
}

#[test]
fn map_example() {
    // map the output
    // <Output of 'a'> -> (i32,)
    let int_parser = 'a'.map(|(ch,)| -> (i32,) { (ch as i32 - 'a' as i32,) }); // IntoParser for char

    let res = rp::parse(&int_parser, "abcd".chars());
    assert_eq!(res.output.unwrap(), (0,));
    assert_eq!(res.it.collect::<String>(), "bcd");
}

#[test]
fn repeat_example() {
    // repeat 'a' 3 to 5 times
    let multiple_a_parser = 'a'.repeat(3..=5); // IntoParser for char
    let res = rp::parse(&multiple_a_parser, "aaaabcd".chars());

    // four 'a' is parsed
    assert_eq!(res.output.unwrap(), (vec!['a', 'a', 'a', 'a',],));
    assert_eq!(res.it.collect::<String>(), "bcd");
}

#[test]
fn void_example() {
    let expensive_parser = 'a'.map(|(_,)| -> (i32,) {
        // some expensive operations for data extracting...
        panic!("This should not be called");
    });
    let expensive_parser = expensive_parser.void_();

    // ignore the output of parser
    // this internally calls 'match_pattern(...)' instead of 'parse(...)'
    let res = rp::parse(&expensive_parser, "abcd".chars());
    assert_eq!(res.output.unwrap(), ());
    assert_eq!(res.it.collect::<String>(), "bcd");
}

#[test]
fn custom_parser_example() {
    let custom_parser = rp::parser(|it: &mut std::str::Chars| {
        if it.take(5).eq("hello".chars()) {
            Some((0,))
        } else {
            // no need to move the iterator back
            None
        }
    });

    let res = rp::parse(&custom_parser, "hello0123".chars());
    assert_eq!(res.output.unwrap(), (0,));
    assert_eq!(res.it.collect::<String>(), "0123");
}
#[test]
fn box_example() {
    let hello_parser = "hello".into_parser();
    let digit_parser = ('0'..='9').void_();

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
}

#[test]
fn refcell_example() {
    let hello_parser = "hello".into_parser();
    let digit_parser = ('0'..='9').void_();

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
}

#[test]
fn rc_example() {
    let hello_parser = "hello".into_parser();
    let digit_parser = ('0'..='9').void_();

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
}

#[test]
fn optional_example() {
    let a_optional_parser = 'a'.optional(); // (Option<char>,)

    let res = rp::parse(&a_optional_parser, "abcd".chars()); // success
    assert_eq!(res.output.unwrap(), (Some('a'),));

    let res = rp::parse(&a_optional_parser, "bcd".chars()); // success, but 'a' is not matched
    assert_eq!(res.output.unwrap(), (None,));

    // if 'a' failed, return 'x'
    let a_optional_or = 'a'.optional_or(('x',)); // (char,)

    let res = rp::parse(&a_optional_or, "bcd".chars());
    assert_eq!(res.output.unwrap(), ('x',));
}

#[test]
fn not_example() {
    let digit_parser_except_4 = ('0'..='9').not('4');

    let res = rp::parse(&digit_parser_except_4, "3".chars());
    assert_eq!(res.output.unwrap(), ('3',));

    let res = rp::parse(&digit_parser_except_4, "4".chars());
    assert_eq!(res.output, None);
}

#[test]
fn output_example() {
    let digit_parser = ('0'..='9').output((1, 2, 3));

    let res = rp::parse(&digit_parser, "123456hello_world".chars());
    assert_eq!(res.output.unwrap(), (1, 2, 3));
    assert_eq!(res.it.collect::<String>(), "23456hello_world");
}

#[test]
fn string_example() {
    let digits_parser = ('0'..='9').repeat(0..).string();

    let res = rp::parse(&digits_parser, "123456hello_world".chars());
    assert_eq!(res.output.unwrap(), ("123456".to_string(),));
    assert_eq!(res.it.collect::<String>(), "hello_world");
}
