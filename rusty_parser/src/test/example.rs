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

    // define pattern: [0-9]{3,5}
    // repeat 'digit_parser' 3 to 5 times (inclusive)
    // this will parse as long as possible
    // Output = ( Vec< OutputType of the Repeated Parser >, ) --> ( Vec<(char,)>, )
    let multiple_digit_parser = digit_parser.repeat(3..=5);

    // parse; put IntoIterator
    let res = multiple_digit_parser.parse(target_string.chars());
    assert_eq!(
        type_name_of_val(&res.output),
        type_name::<Option<(Vec<(char,)>,)>>()
    );
    assert_eq!(
        res.output,
        Some((vec![('1',), ('2',), ('3',), ('4',), ('5',)],))
    );

    // Output mapping
    // ( Vec<(char,)>, )  -->  (i32, )
    // Parser's Output must be Tuple
    let int_parser = multiple_digit_parser.map(|(vec,)| -> (i32,) {
        let mut res = 0;
        for (ch,) in vec {
            res = res * 10 + (ch as i32 - '0' as i32);
        }
        (res,)
    });

    let res = int_parser.parse(target_string.chars());
    assert_eq!(res.output, Some((12345,)));

    // pattern matching
    // .match_pattern only checks if the pattern is matched or not
    // it does not try to extract data from input string (e.g. push element in Vec above)
    // Output = always ()
    let res = int_parser.match_pattern(target_string.chars());
    assert_eq!(res.output, Some(()));
}
