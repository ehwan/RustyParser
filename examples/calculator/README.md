# Simple Calculator Expression Parser

This is a simple calculator expression parser that can parse and evaluate arithmetic expressions.

## Rules
```
num: [0-9]+                   # integer
paren_expr: '(' expr ')'      # parenthesized expression
expr0: num | paren_expr       # ^
expr1: expr0 ((*|/) expr0)*   # | Operator precedence
expr2: expr1 ((+|-) expr1)*   # | goes stronger
expr: expr2                   # | as we go up
line_parser: expr lineend
lineend: '\0'

whitespaces: ' ' | '\n'
```

Source code in [src/main.rs](src/main.rs)
