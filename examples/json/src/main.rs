use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
fn map_iter_pair_to_string<'a, 'b>(
    (beg, end): (std::str::Chars<'a>, std::str::Chars<'b>),
) -> (JsonValue,) {
    let size = beg.as_str().len() - end.as_str().len();
    let captured = beg.take(size);
    (JsonValue::String(captured.collect()),)
}

use rusty_parser::{self as rp, IntoParser};

type DynParser = rp::DynBoxChars<(JsonValue,)>;

fn main() {
    // Since there are no 'null parsers' in RustyParser, we need to create a dummy parser
    // this will tell compiler of dyn Parser's signature
    let dummy_parser = rp::constant((JsonValue::Null,));

    let value = dummy_parser.clone().box_chars().refcell().rc();
    let object = dummy_parser.clone().box_chars().refcell().rc();
    let array = dummy_parser.clone().box_chars().refcell().rc();
    let number = dummy_parser.clone().box_chars().refcell().rc();

    let true_ = "true".map(|_| (JsonValue::Bool(true),));
    let false_ = "false".map(|_| (JsonValue::Bool(false),));
    let bool_ = rp::or_(true_, false_);

    let null = "null".map(|_| (JsonValue::Null,));

    let digit = ('0'..='9')
        .into_parser()
        .map(|(c,): (char,)| (c as i32 - '0' as i32,));
    let hex_alpha_lower = ('a'..='f')
        .into_parser()
        .map(|(c,): (char,)| (c as i32 - 'a' as i32 + 10,));
    let hex_alpha_upper = ('A'..='F')
        .into_parser()
        .map(|(c,): (char,)| (c as i32 - 'A' as i32 + 10,));
    let hex = rp::or_!(digit, hex_alpha_lower, hex_alpha_upper);

    let unicode_char =
        rp::seq!('u'.void_(), hex.repeat(4..=4)).map(|(hexs,): (Vec<i32>,)| -> (char,) {
            let mut res: u32 = 0;
            for hex in hexs {
                res = res * 16 + hex as u32;
            }
            (char::from_u32(res).expect("invalid unicode character"),)
        });
    let escape = rp::or_!(
        '"',
        '\\',
        '/',
        'n'.output(('\n',)),
        '\r'.output(('\r',)),
        '\t'.output(('\t',)),
        unicode_char
    );
    let escape = rp::seq!('\\'.void_(), escape);
    let character = ('\u{0020}'..='\u{10FFFF}').not('"').not('\\');
    let character = rp::or_!(character, escape);

    let string = rp::seq!('"'.void_(), character.repeat(0..).iter(), '"'.void_())
        .map(map_iter_pair_to_string);

    value.borrow_mut().assign(rp::or_!(
        null,
        bool_,
        rp::Rc::clone(&number),
        string,
        rp::Rc::clone(&array),
        rp::Rc::clone(&object)
    ));

    // let ws = rp::or_!(' ', '\n', '\r', '\t').repeat(0..).void_();
    // let sign = rp::or_!('+', '-').optional();
    // let onenine = ('1'..='9')
    //     .into_parser()
    //     .map(|(c,): (char,)| (c as i32 - '0' as i32,));
    // let digits = digit.repeat(1..);
    // let exponent = rp::seq!(
    //     rp::or_('e', 'E'),
    //     sign,
    //     rp::one_of("0123456789").repeat(1..)
    // );
}
