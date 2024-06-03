use std::iter::Iterator;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

#[derive(Debug)]
pub struct SingleCheckParser<ClosureType, Input, NewOutput>
where
    ClosureType: Fn(Input) -> Option<NewOutput>,
    NewOutput: Tuple,
{
    closure: ClosureType,
    _phantom: std::marker::PhantomData<(Input, NewOutput)>,
}

impl<ClosureType, Input, NewOutput> Clone for SingleCheckParser<ClosureType, Input, NewOutput>
where
    ClosureType: Fn(Input) -> Option<NewOutput> + Clone,
    NewOutput: Tuple,
{
    fn clone(&self) -> Self {
        Self {
            closure: self.closure.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<ClosureType, Input, NewOutput> Copy for SingleCheckParser<ClosureType, Input, NewOutput>
where
    ClosureType: Fn(Input) -> Option<NewOutput> + Copy,
    NewOutput: Tuple,
    std::marker::PhantomData<(Input, NewOutput)>: Copy,
{
}

impl<ClosureType, Input, NewOutput> SingleCheckParser<ClosureType, Input, NewOutput>
where
    ClosureType: Fn(Input) -> Option<NewOutput>,
    NewOutput: Tuple,
{
    pub fn new(closure: ClosureType) -> Self {
        Self {
            closure,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ClosureType, Input, NewOutput, It> Parser<It>
    for SingleCheckParser<ClosureType, Input, NewOutput>
where
    It: InputIteratorTrait + Iterator<Item = Input>,
    It::Item: Clone,
    ClosureType: Fn(Input) -> Option<NewOutput>,
    NewOutput: Tuple,
{
    type Output = NewOutput;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = it.next() {
            if let Some(res) = (self.closure)(val) {
                ParseResult {
                    output: Some(res),
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
            if (self.closure)(val).is_some() {
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

impl<ClosureType, Input, NewOutput> IntoParser for SingleCheckParser<ClosureType, Input, NewOutput>
where
    ClosureType: Fn(Input) -> Option<NewOutput>,
    NewOutput: Tuple,
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
        let a_parser = SingleCheckParser::new(|c: char| if c == 'a' { Some(()) } else { None });
        // success
        let start_with_a_string = String::from("abcde");
        let res = a_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn success_test2() {
        let b_parser = SingleCheckParser::new(|c: char| if c == 'b' { Some(()) } else { None });
        // success
        let start_with_a_string = String::from("bacde");
        let res = b_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "acde");
    }
    #[test]
    fn fail_test1() {
        let a_parser = SingleCheckParser::new(|c: char| if c == 'a' { Some(()) } else { None });
        let start_with_a_string = String::from("bbcde");
        let res = a_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bbcde");
    }
    #[test]
    fn fail_test2() {
        let b_parser = SingleCheckParser::new(|c: char| if c == 'b' { Some(()) } else { None });
        let start_with_a_string = String::from("abcde");
        let res = b_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }
    #[test]
    fn fail_test3() {
        let b_parser = SingleCheckParser::new(|c: char| if c == 'b' { Some(()) } else { None });
        let res = b_parser.parse("".chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "");
    }
}
