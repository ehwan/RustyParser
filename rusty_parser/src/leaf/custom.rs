use std::marker::PhantomData;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct CustomParser<ParseClosure, ClosureOutput, It>
where
    It: InputIteratorTrait,
    ParseClosure: Fn(&mut It) -> Option<ClosureOutput>,
    ClosureOutput: Tuple,
{
    parse_closure: ParseClosure,
    _phantom: PhantomData<It>,
}

impl<ParseClosure, ClosureOutput, It> CustomParser<ParseClosure, ClosureOutput, It>
where
    It: InputIteratorTrait,
    ParseClosure: Fn(&mut It) -> Option<ClosureOutput>,
    ClosureOutput: Tuple,
{
    pub fn new(parse_closure: ParseClosure) -> CustomParser<ParseClosure, ClosureOutput, It> {
        CustomParser {
            parse_closure: parse_closure,
            _phantom: PhantomData,
        }
    }
}

impl<ParseClosure, ClosureOutput, It> Parser<It> for CustomParser<ParseClosure, ClosureOutput, It>
where
    It: InputIteratorTrait,
    ParseClosure: Fn(&mut It) -> Option<ClosureOutput>,
    ClosureOutput: Tuple,
{
    type Output = ClosureOutput;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = (self.parse_closure)(&mut it) {
            ParseResult {
                output: Some(val),
                it: it,
            }
        } else {
            ParseResult {
                output: None,
                it: i0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::string::String;

    use super::CustomParser;
    use crate::core::parser::Parser;

    #[test]
    fn success_test1() {
        let a_parser = CustomParser::new(|it: &mut std::str::Chars| {
            if let Some(val) = it.next() {
                if val == 'a' {
                    Some(('a',))
                } else {
                    None
                }
            } else {
                None
            }
        });
        // success
        let start_with_a_string = String::from("abcde");
        let res = a_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, Some(('a',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn success_test2() {
        let b_parser = CustomParser::new(|it: &mut std::str::Chars| {
            if let Some(val) = it.next() {
                if val == 'b' {
                    Some(('b',))
                } else {
                    None
                }
            } else {
                None
            }
        });
        // success
        let start_with_b_string = String::from("bacde");
        let res = b_parser.parse(start_with_b_string.chars());
        assert_eq!(res.output, Some(('b',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "acde");
    }
    #[test]
    fn fail_test1() {
        let a_parser = CustomParser::new(|it: &mut std::str::Chars| {
            if let Some(val) = it.next() {
                if val == 'a' {
                    Some(('a',))
                } else {
                    None
                }
            } else {
                None
            }
        });
        // this case is fail
        let start_with_b_string = String::from("bacde");
        let res = a_parser.parse(start_with_b_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bacde");
    }
    #[test]
    fn fail_test2() {
        let b_parser = CustomParser::new(|it: &mut std::str::Chars| {
            if let Some(val) = it.next() {
                if val == 'b' {
                    Some(('b',))
                } else {
                    None
                }
            } else {
                None
            }
        });
        // this case is fail
        let start_with_a_string = String::from("abcde");
        let res = b_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }
}
