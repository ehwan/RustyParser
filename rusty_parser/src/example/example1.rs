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
fn box_example() {}
