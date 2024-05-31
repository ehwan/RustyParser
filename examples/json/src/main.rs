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

use rusty_parser::{self as rp, IntoParser};

type DynParser = rp::DynBoxChars<(JsonValue,)>;

fn string_parser() -> DynParser {
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

    let string = rp::seq!('"'.void_(), character.repeat(0..).string(), '"'.void_())
        .map(|(s,): (String,)| (JsonValue::String(s),));

    return DynParser::new(string);
}

fn number_parser() -> DynParser {
    let digit = ('0'..='9')
        .into_parser()
        .map(|(c,): (char,)| (c as i32 - '0' as i32,));
    let onenine = ('1'..='9')
        .into_parser()
        .map(|(c,): (char,)| (c as i32 - '0' as i32,));

    let digits = rp::seq!(onenine, digit.clone().repeat(0..));

    let fraction = rp::seq!('.'.void_(), digits.clone())
        .map(|(leaddigit, digits): (i32, Vec<i32>)| -> (f64,) {
            let mut base10: f64 = 0.01;
            let mut res: f64 = leaddigit as f64 * 0.1;
            for digit in digits {
                res += digit as f64 * base10;
                base10 *= 0.1;
            }
            (res,)
        })
        .optional()
        .map(|(fraction,): (Option<f64>,)| -> (f64,) {
            match fraction {
                Some(f) => (f,),
                None => (0.0,),
            }
        });

    let sign = rp::or_!('+', '-').optional();

    let exponent = rp::seq!(
        rp::or_('e', 'E').void_(),
        rp::seq!(
            sign,
            digits
                .clone()
                .map(|(leaddigit, digits): (i32, Vec<i32>)| -> (i32,) {
                    let mut res = leaddigit;
                    for digit in digits {
                        res = res * 10 + digit;
                    }
                    (res,)
                })
        )
        .map(|(sign, exponent): (Option<char>, i32)| -> (i32,) {
            if let Some('-') = sign {
                (-exponent,)
            } else {
                (exponent,)
            }
        })
    )
    .optional()
    .map(|(opt_exp,): (Option<i32>,)| -> (i32,) {
        match opt_exp {
            Some(exp) => (exp,),
            None => (0,),
        }
    });

    let integer = rp::seq!(
        '-'.optional(),
        rp::or_!(
            '0'.output((0,)),
            digits
                .clone()
                .map(|(leaddigit, digits): (i32, Vec<i32>,)| -> (i32,) {
                    let mut res = leaddigit;
                    for digit in digits {
                        res = res * 10 + digit;
                    }
                    (res,)
                })
        )
    )
    .map(|(sign, integer): (Option<char>, i32)| -> (i32,) {
        let mut res = integer;
        if let Some('-') = sign {
            res = -res;
        }
        (res,)
    });

    let number = rp::seq!(integer, fraction, exponent).map(
        |(integer, fraction, exponent): (i32, f64, i32)| -> (JsonValue,) {
            let mut res = integer as f64 + fraction;
            res *= 10f64.powi(exponent);
            (JsonValue::Number(res),)
        },
    );

    return DynParser::new(number);
}

fn main() {
    // Since there are no 'null parsers' in RustyParser, we need to create a dummy parser
    // this will tell compiler of dyn Parser's signature
    let dummy_parser = rp::constant((JsonValue::Null,));

    let value = dummy_parser.clone().box_chars().refcell().rc();
    let object = dummy_parser.clone().box_chars().refcell().rc();
    let array = dummy_parser.clone().box_chars().refcell().rc();

    let true_ = "true".map(|_| (JsonValue::Bool(true),));
    let false_ = "false".map(|_| (JsonValue::Bool(false),));
    let bool_ = rp::or_(true_, false_);

    let null = "null".map(|_| (JsonValue::Null,));

    value.borrow_mut().assign(rp::or_!(
        null,
        bool_,
        number_parser(),
        string_parser(),
        rp::Rc::clone(&array),
        rp::Rc::clone(&object)
    ));

    let ws = rp::or_!(' ', '\n', '\r', '\t').repeat(0..).void_();

    // let element = rp::seq!(ws.clone(), rp::Rc::clone(&value), ws.clone());

    // let elements =
    // rp::seq!( element.clone(), (',' .void_(), element.clone()).repeat(0..) )
}