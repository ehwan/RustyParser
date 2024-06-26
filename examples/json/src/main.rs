use std::cell::RefCell;
use std::rc::Rc;
use std::string::String;
use std::vec::Vec;
use std::{collections::HashMap, io::Write};

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

fn string_parser() -> Rc<DynParser> {
    let digit = ('0'..='9')
        .into_parser()
        .map(|c: char| c as i32 - '0' as i32);
    let hex_alpha_lower = ('a'..='f')
        .into_parser()
        .map(|c: char| c as i32 - 'a' as i32 + 10);
    let hex_alpha_upper = ('A'..='F')
        .into_parser()
        .map(|c: char| c as i32 - 'A' as i32 + 10);
    let hex = rp::or!(digit, hex_alpha_lower, hex_alpha_upper);

    let unicode_char = rp::seq!('u'.void(), hex.repeat(4usize)).map(|hexs: Vec<i32>| -> char {
        let mut res: u32 = 0;
        for hex in hexs {
            res = res * 16 + hex as u32;
        }
        char::from_u32(res).expect("invalid unicode character")
    });
    let escape = rp::or!(
        '"',
        '\\',
        '/',
        'n'.output('\n'),
        'r'.output('\r'),
        't'.output('\t'),
        unicode_char
    );
    let escape = rp::seq!('\\'.void(), escape);
    let character = ('\u{0020}'..='\u{10FFFF}').not('"').not('\\');
    let character = rp::or!(character, escape);

    let string = rp::seq!(
        '"'.void(),
        character
            .repeat(0..)
            .map(|chars: Vec<char>| -> String { chars.into_iter().collect::<String>() },),
        '"'.void()
    )
    .map(|s: String| JsonValue::String(s));

    Rc::new(DynParser::new(string))
}

fn number_parser() -> DynParser {
    let digit = ('0'..='9')
        .into_parser()
        .map(|c: char| c as i32 - '0' as i32);
    let onenine = ('1'..='9')
        .into_parser()
        .map(|c: char| c as i32 - '0' as i32);

    let digits = rp::seq!(digit, digit.repeat(0..));
    let onedigits = rp::seq!(onenine, digit.repeat(0..));

    let fraction = rp::seq!('.'.void(), digits)
        .map(|leaddigit: i32, digits: Vec<i32>| -> f64 {
            let mut base10: f64 = 0.01;
            let mut res: f64 = leaddigit as f64 * 0.1;
            for digit in digits {
                res += digit as f64 * base10;
                base10 *= 0.1;
            }
            res
        })
        .optional_or(0.0_f64);

    let sign = rp::or!('+', '-').optional_or('+');

    let exponent = rp::seq!(
        rp::or!('e', 'E').void(),
        rp::seq!(
            sign,
            digits.map(|leaddigit: i32, digits: Vec<i32>| -> i32 {
                let mut res = leaddigit;
                for digit in digits {
                    res = res * 10 + digit;
                }
                res
            })
        )
        .map(|sign: char, exponent: i32| -> i32 {
            if sign == '-' {
                -exponent
            } else {
                exponent
            }
        })
    )
    .optional_or(0);

    let integer = rp::seq!(
        '-'.optional_or('+'),
        rp::or!(
            '0'.output(0),
            onedigits.map(|leaddigit: i32, digits: Vec<i32>| -> i32 {
                let mut res = leaddigit;
                for digit in digits {
                    res = res * 10 + digit;
                }
                res
            })
        )
    )
    .map(|sign: char, integer: i32| -> i32 {
        let mut res = integer;
        if '-' == sign {
            res = -res;
        }
        res
    });

    let number = rp::seq!(integer, fraction, exponent).map(
        |integer: i32, fraction: f64, exponent: i32| -> JsonValue {
            let mut res = integer as f64 + fraction;
            res *= 10f64.powi(exponent);
            JsonValue::Number(res)
        },
    );

    DynParser::new(number)
}

fn main() {
    // init with dummy parser
    let value: Rc<RefCell<rp::DynBoxChars<(JsonValue,)>>> = Default::default();
    let object: Rc<RefCell<rp::DynBoxChars<(JsonValue,)>>> = Default::default();
    let array: Rc<RefCell<rp::DynBoxChars<(JsonValue,)>>> = Default::default();

    let true_ = "true".map(|| JsonValue::Bool(true));
    let false_ = "false".map(|| JsonValue::Bool(false));
    let bool_ = rp::or(true_, false_);

    let null = "null".map(|| JsonValue::Null);

    value.borrow_mut().assign(rp::or!(
        null,
        bool_,
        number_parser(),
        string_parser(),
        std::rc::Rc::clone(&array),
        std::rc::Rc::clone(&object)
    ));

    let ws = rp::or!(' ', '\n', '\r', '\t').repeat(0..).void();

    let element = rp::seq!(ws, std::rc::Rc::clone(&value), ws);

    let elements = rp::seq!(
        element.clone(),
        rp::seq!(','.void(), element.clone()).repeat(0..)
    )
    .map(|first: JsonValue, rest: Vec<JsonValue>| -> JsonValue {
        let mut res = Vec::with_capacity(rest.len() + 1);
        res.push(first);
        for r in rest {
            res.push(r);
        }
        JsonValue::Array(res)
    });

    array.borrow_mut().assign(rp::seq!(
        '['.void(),
        rp::or!(elements, ws.output(JsonValue::Array(Vec::new()))),
        ']'.void()
    ));

    let member = rp::seq!(ws, string_parser(), ws, ':'.void(), element.clone());

    let members = rp::seq!(
        member.clone(),
        rp::seq!(','.void(), member.clone()).repeat(0..)
    )
    .map(
        |first_key: JsonValue,
         first_value: JsonValue,
         rest: Vec<(JsonValue, JsonValue)>|
         -> JsonValue {
            let mut res: HashMap<String, JsonValue> = HashMap::new();
            match first_key {
                JsonValue::String(first_key) => {
                    res.insert(first_key, first_value);
                }
                _ => panic!("Key must be String type"),
            }
            for (key, value) in rest {
                match key {
                    JsonValue::String(key) => {
                        res.insert(key, value);
                    }
                    _ => panic!("Key must be String type"),
                }
            }
            JsonValue::Object(res)
        },
    );

    object.borrow_mut().assign(rp::seq!(
        '{'.void(),
        rp::or!(members, ws.output(JsonValue::Object(HashMap::new()))),
        '}'.void()
    ));

    let json = rp::seq!(element, rp::end());

    loop {
        let mut line = String::new();
        print!("Enter JSON: ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let res = rp::parse(&json, line.chars());
        match res.output {
            Some((json_value,)) => {
                println!("{:?}", json_value);
            }
            None => {
                println!("Failed to parse JSON");
            }
        }
    }
}
