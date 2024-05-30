use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct Fail {}

impl Fail {
    pub fn new() -> Self {
        Self {}
    }
}

impl<It> Parser<It> for Fail
where
    It: InputIteratorTrait,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        ParseResult {
            output: None,
            it: it,
        }
    }

    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        ParseResult {
            output: None,
            it: it,
        }
    }
}

impl IntoParser for Fail {
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
    fn success() {
        let parser = Fail::new();
        // fail
        let str = String::from("abcde");
        let res = parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }
}
