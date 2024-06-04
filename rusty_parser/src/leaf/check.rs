use std::iter::Iterator;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

pub trait OptionOrBool {
    type Output;
    fn to_option(self) -> Option<Self::Output>;
}

impl OptionOrBool for bool {
    type Output = ();
    fn to_option(self) -> Option<Self::Output> {
        if self {
            Some(())
        } else {
            None
        }
    }
}
// wrap tuple Option<T> -> Option<(T,)>
impl<T> OptionOrBool for Option<T> {
    type Output = (T,);
    fn to_option(self) -> Option<Self::Output> {
        self.map(|x| (x,))
    }
}

#[derive(Debug)]
pub struct SingleCheckParser<ClosureType, Input, ClosureOutput>
where
    ClosureType: Fn(Input) -> ClosureOutput,
    ClosureOutput: OptionOrBool,
{
    closure: ClosureType,
    _phantom: std::marker::PhantomData<(Input, ClosureOutput)>,
}

impl<ClosureType, Input, ClosureOutput> Clone
    for SingleCheckParser<ClosureType, Input, ClosureOutput>
where
    ClosureType: Fn(Input) -> ClosureOutput + Clone,
    ClosureOutput: OptionOrBool,
{
    fn clone(&self) -> Self {
        Self {
            closure: self.closure.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<ClosureType, Input, ClosureOutput> Copy
    for SingleCheckParser<ClosureType, Input, ClosureOutput>
where
    ClosureType: Fn(Input) -> ClosureOutput + Copy,
    ClosureOutput: OptionOrBool,
    std::marker::PhantomData<(Input, ClosureOutput)>: Copy,
{
}

impl<ClosureType, Input, ClosureOutput> SingleCheckParser<ClosureType, Input, ClosureOutput>
where
    ClosureType: Fn(Input) -> ClosureOutput,
    ClosureOutput: OptionOrBool,
{
    pub fn new(closure: ClosureType) -> Self {
        Self {
            closure,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ClosureType, Input, ClosureOutput, It> Parser<It>
    for SingleCheckParser<ClosureType, Input, ClosureOutput>
where
    It: InputIteratorTrait + Iterator<Item = Input>,
    It::Item: Clone,
    ClosureType: Fn(Input) -> ClosureOutput,
    ClosureOutput: OptionOrBool,
    ClosureOutput::Output: Tuple,
{
    type Output = ClosureOutput::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = it.next() {
            if let Some(res) = (self.closure)(val).to_option() {
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
            if (self.closure)(val).to_option().is_some() {
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

impl<ClosureType, Input, ClosureOutput> IntoParser
    for SingleCheckParser<ClosureType, Input, ClosureOutput>
where
    ClosureType: Fn(Input) -> ClosureOutput,
    ClosureOutput: Tuple,
    ClosureOutput: OptionOrBool,
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
        let a_parser = SingleCheckParser::new(|c: char| if c == 'a' { Some(0) } else { None });
        let res = a_parser.parse("abcde".chars());
        assert_eq!(res.output, Some((0,)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn success_test2() {
        let a_parser = SingleCheckParser::new(|c: char| if c == 'a' { true } else { false });
        let res = a_parser.parse("abcde".chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn fail_test1() {
        let a_parser = SingleCheckParser::new(|c: char| if c == 'a' { Some(0) } else { None });
        let res = a_parser.parse("bbcde".chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bbcde");
    }
    #[test]
    fn fail_test2() {
        let b_parser = SingleCheckParser::new(|c: char| if c == 'b' { true } else { false });
        let res = b_parser.parse("abcde".chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }
}
