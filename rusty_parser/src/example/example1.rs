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
fn dict_example() {}
