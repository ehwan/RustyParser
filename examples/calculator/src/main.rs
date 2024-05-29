use std::io::{stdin, stdout, Write};

use rusty_parser as rp;

use rp::Parser;

fn main() {
    // Since there are no 'null parsers' in RustyParser, we need to create a dummy parser
    // this will tell compiler that the Output of the parser is (i32,)
    let dummy_parser = rp::constant((0,));
    // Note. dummy_parser impl Copy
    let expr = dummy_parser.boxed().refcelled().rced();
    let expr0 = dummy_parser.boxed().refcelled().rced();
    let expr1 = dummy_parser.boxed().refcelled().rced();
    let expr2 = dummy_parser.boxed().refcelled().rced();

    /*
    paren_expr: '(' expr ')'
    expr0: num | paren_expr
    expr1: expr0 ((*|/) expr0)*
    expr2: expr1 ((+|-) expr1)*
    line_parser: expr lineend
    lineend: '\0'
     */

    let whitespaces = rp::one(' ').or_(rp::one('\n')).repeat(0..).void_();

    // one digit [0-9]
    let digit = rp::range('0'..='9').map(|(c,)| -> (i32,) { (c as i32 - '0' as i32,) });

    // number [0-9]+
    // multiple digits -> build number
    let num = digit.repeat(1..).map(|(digits,)| -> (i32,) {
        let mut res = 0;
        for digit in digits {
            res = res * 10 + digit;
        }
        return (res,);
    });

    // '(' expression ')'
    let paren_expr = rp::seq!(
        rp::one('(').void_(),
        whitespaces.clone(),
        rp::Rc::clone(&expr),
        whitespaces.clone(),
        rp::one(')').void_()
    );

    // expr0: num | paren_expr
    expr0.borrow_mut().assign(num.or_(paren_expr));

    // expr1: expr0 ((*|/) expr0)*
    let mul_or_div_op = rp::one('*').or_(rp::one('/'));
    let mul_or_div = rp::seq!(
        rp::Rc::clone(&expr0),
        rp::seq!(
            whitespaces.clone(),
            mul_or_div_op,
            whitespaces.clone(),
            rp::Rc::clone(&expr0)
        )
        .repeat(0..)
    )
    .map(|(mut base, op_rhs_vec)| -> (i32,) {
        for (op, rhs) in op_rhs_vec {
            if op == '*' {
                base = base * rhs;
            } else {
                base = base / rhs;
            }
        }
        return (base,);
    });

    expr1.borrow_mut().assign(mul_or_div);

    // expr2: expr1 ((+|-) expr1)*
    let add_or_sub_op = rp::one('+').or_(rp::one('-'));
    let add_or_sub = rp::seq!(
        rp::Rc::clone(&expr1),
        rp::seq!(
            whitespaces.clone(),
            add_or_sub_op,
            whitespaces.clone(),
            rp::Rc::clone(&expr1)
        )
        .repeat(0..)
    )
    .map(|(mut base, op_rhs_vec)| -> (i32,) {
        for (op, rhs) in op_rhs_vec {
            if op == '+' {
                base = base + rhs;
            } else {
                base = base - rhs;
            }
        }
        return (base,);
    });

    expr2.borrow_mut().assign(add_or_sub);

    // @TODO: simplify this
    expr.borrow_mut().assign(rp::Rc::clone(&expr2));

    let line_parser = rp::seq!(
        whitespaces.clone(),
        rp::Rc::clone(&expr),
        whitespaces.clone(),
        rp::end()
    );

    loop {
        print!("Enter a expression: ");
        stdout().flush().expect("Failed to flush");
        let mut line: String = String::new();
        stdin().read_line(&mut line).expect("Failed to read line");

        let chars_vec: Vec<char> = line.chars().collect();
        let res = line_parser.parse(chars_vec.into_iter());
        match res.output {
            Some((res,)) => println!("Result: {}", res),
            None => println!("Error"),
        }
    }
}
