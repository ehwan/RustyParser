use std::cell::RefCell;
use std::io::{stdin, stdout, Write};
use std::rc::Rc;

use rusty_parser as rp;

fn main() {
    use rp::*;

    // dynamic Parser type
    type DynParser = DynBoxChars<(i32,)>;

    // initialize with dummy parser that always panic
    let expr: Rc<RefCell<DynParser>> = Default::default();
    let expr0: Rc<RefCell<DynParser>> = Default::default();
    let expr1: Rc<RefCell<DynParser>> = Default::default();
    let expr2: Rc<RefCell<DynParser>> = Default::default();

    /*
    paren_expr: '(' expr ')'
    expr0: num | paren_expr
    expr1: expr0 ((*|/) expr0)*
    expr2: expr1 ((+|-) expr1)*
    line_parser: expr lineend
    lineend: '\0'
     */

    let whitespaces = or!(' ', '\n').repeat(0..).void();

    // one digit [0-9]
    let digit = ('0'..='9')
        .into_parser()
        .map(|c| -> i32 { c as i32 - '0' as i32 });

    // number [0-9]+
    // multiple digits -> build number
    let num = digit.repeat(1..).map(|digits| -> i32 {
        let mut res = 0;
        for digit in digits {
            res = res * 10 + digit;
        }
        res
    });

    // '(' expression ')'
    let paren_expr = seq!(
        '('.void(),
        whitespaces,
        Rc::clone(&expr),
        whitespaces,
        ')'.void()
    );

    // expr0: num | paren_expr
    expr0.borrow_mut().assign(or!(num, paren_expr));

    // expr1: expr0 ((*|/) expr0)*
    let mul_or_div_op = or!('*', '/');
    let mul_or_div = seq!(
        Rc::clone(&expr0),
        seq!(whitespaces, mul_or_div_op, whitespaces, Rc::clone(&expr0)).repeat(0..)
    )
    .map(|mut base, op_rhs_vec| -> i32 {
        for (op, rhs) in op_rhs_vec {
            if op == '*' {
                base *= rhs;
            } else {
                base /= rhs;
            }
        }
        base
    });

    expr1.borrow_mut().assign(mul_or_div);

    // expr2: expr1 ((+|-) expr1)*
    let add_or_sub_op = or!('+', '-');
    let add_or_sub = seq!(
        Rc::clone(&expr1),
        seq!(whitespaces, add_or_sub_op, whitespaces, Rc::clone(&expr1)).repeat(0..)
    )
    .map(|mut base, op_rhs_vec| -> i32 {
        for (op, rhs) in op_rhs_vec {
            if op == '+' {
                base += rhs;
            } else {
                base -= rhs;
            }
        }
        base
    });

    expr2.borrow_mut().assign(add_or_sub);

    expr.borrow_mut().assign(Rc::clone(&expr2));

    let line_parser = seq!(whitespaces, Rc::clone(&expr), whitespaces, end());

    loop {
        print!("Enter a expression: ");
        stdout().flush().expect("Failed to flush");
        let mut line: String = String::new();
        stdin().read_line(&mut line).expect("Failed to read line");

        let res = parse(&line_parser, line.chars());

        match res.output {
            Some((res,)) => println!("Result: {}", res),
            None => println!("Error"),
        }
    }
}
