use std::io::{stdin, stdout, Write};

use rusty_parser as rp;

fn main() {
    use rp::*;

    // Since there are no 'null parsers' in RustyParser, we need to create a dummy parser
    // this will tell compiler of dyn Parser's signature
    let dummy_parser = constant((0,));
    let expr = rc(refcell(box_chars(dummy_parser)));
    let expr0 = rc(refcell(box_chars(dummy_parser)));
    let expr1 = rc(refcell(box_chars(dummy_parser)));
    let expr2 = rc(refcell(box_chars(dummy_parser)));

    /*
    paren_expr: '(' expr ')'
    expr0: num | paren_expr
    expr1: expr0 ((*|/) expr0)*
    expr2: expr1 ((+|-) expr1)*
    line_parser: expr lineend
    lineend: '\0'
     */

    let whitespaces = void_(repeat(or_!(one(' '), one('\n')), 0..));

    // one digit [0-9]
    let digit = map(range('0'..='9'), |(c,)| -> (i32,) {
        (c as i32 - '0' as i32,)
    });

    // number [0-9]+
    // multiple digits -> build number
    let num = map(repeat(digit, 1..), |(digits,)| -> (i32,) {
        let mut res = 0;
        for digit in digits {
            res = res * 10 + digit;
        }
        return (res,);
    });

    // '(' expression ')'
    let paren_expr = seq!(
        void_(one('(')),
        whitespaces.clone(),
        Rc::clone(&expr),
        whitespaces.clone(),
        void_(one(')'))
    );

    // expr0: num | paren_expr
    expr0.borrow_mut().assign(or_!(num, paren_expr));

    // expr1: expr0 ((*|/) expr0)*
    let mul_or_div_op = or_!(one('*'), one('/'));
    let mul_or_div = map(
        seq!(
            Rc::clone(&expr0),
            repeat(
                seq!(
                    whitespaces.clone(),
                    mul_or_div_op,
                    whitespaces.clone(),
                    Rc::clone(&expr0)
                ),
                0..
            )
        ),
        |(mut base, op_rhs_vec)| -> (i32,) {
            for (op, rhs) in op_rhs_vec {
                if op == '*' {
                    base = base * rhs;
                } else {
                    base = base / rhs;
                }
            }
            return (base,);
        },
    );

    expr1.borrow_mut().assign(mul_or_div);

    // expr2: expr1 ((+|-) expr1)*
    let add_or_sub_op = or_!(one('+'), one('-'));
    let add_or_sub = map(
        seq!(
            Rc::clone(&expr1),
            repeat(
                seq!(
                    whitespaces.clone(),
                    add_or_sub_op,
                    whitespaces.clone(),
                    Rc::clone(&expr1)
                ),
                0..
            )
        ),
        |(mut base, op_rhs_vec)| -> (i32,) {
            for (op, rhs) in op_rhs_vec {
                if op == '+' {
                    base = base + rhs;
                } else {
                    base = base - rhs;
                }
            }
            return (base,);
        },
    );

    expr2.borrow_mut().assign(add_or_sub);

    // @TODO: simplify this
    expr.borrow_mut().assign(Rc::clone(&expr2));

    let line_parser = seq!(
        whitespaces.clone(),
        Rc::clone(&expr),
        whitespaces.clone(),
        end()
    );

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
