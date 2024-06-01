use std::iter::Iterator;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct SingleCheckParser<ClosureType, Input>
where
    ClosureType: Fn(Input) -> bool,
{
    closure: ClosureType,
    _phantom: std::marker::PhantomData<Input>,
}

impl<ClosureType, Input> SingleCheckParser<ClosureType, Input>
where
    ClosureType: Fn(Input) -> bool,
{
    pub fn new(closure: ClosureType) -> Self {
        Self {
            closure,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ClosureType, Input, It> Parser<It> for SingleCheckParser<ClosureType, Input>
where
    It: InputIteratorTrait + Iterator<Item = Input>,
    It::Item: Clone,
    ClosureType: Fn(Input) -> bool,
{
    type Output = (It::Item,);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = it.next() {
            if (self.closure)(val.clone()) {
                ParseResult {
                    output: Some((val,)),
                    it,
                }
            } else {
                ParseResult {
                    output: None,
                    it: i0,
                }
            }
        } else {
            ParseResult {
                output: None,
                it: i0,
            }
        }
    }

    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = it.next() {
            if (self.closure)(val) {
                ParseResult {
                    output: Some(()),
                    it,
                }
            } else {
                ParseResult {
                    output: None,
                    it: i0,
                }
            }
        } else {
            ParseResult {
                output: None,
                it: i0,
            }
        }
    }
}

impl<ClosureType, Input> IntoParser for SingleCheckParser<ClosureType, Input>
where
    ClosureType: Fn(Input) -> bool,
{
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod tests {
    use std::string::String;

    use super::*;

    #[test]
    fn success_test1() {
        let a_parser = SingleCheckParser::new(|c: char| c == 'a');
        // success
        let start_with_a_string = String::from("abcde");
        let res = a_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, Some(('a',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn success_test2() {
        let b_parser = SingleCheckParser::new(|c: char| c == 'b');
        // success
        let start_with_a_string = String::from("bacde");
        let res = b_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, Some(('b',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "acde");
    }
    #[test]
    fn fail_test1() {
        let a_parser = SingleCheckParser::new(|c: char| c == 'a');
        // success
        let start_with_a_string = String::from("bbcde");
        let res = a_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bbcde");
    }
    #[test]
    fn fail_test2() {
        let a_parser = SingleCheckParser::new(|c: char| c == 'b');
        // success
        let start_with_a_string = String::from("abcde");
        let res = a_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }
    #[test]
    fn fail_test3() {
        let a_parser = SingleCheckParser::new(|c: char| c == 'b');
        // success
        let res = a_parser.parse("".chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "");
    }
}
