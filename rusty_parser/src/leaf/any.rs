use std::iter::Iterator;

use crate::core::into_parser::IntoParser;
use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

#[derive(Debug, Clone, Copy)]
pub struct AnyParser {}

impl AnyParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl<It> Parser<It> for AnyParser
where
    It: InputIteratorTrait,
{
    type Output = (<It as Iterator>::Item,);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        if let Some(val) = it.next() {
            ParseResult {
                output: Some((val,)),
                it: it,
            }
        } else {
            ParseResult {
                output: None,
                it: it,
            }
        }
    }

    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let mut it = it;
        if let Some(_) = it.next() {
            ParseResult {
                output: Some(()),
                it: it,
            }
        } else {
            ParseResult {
                output: None,
                it: it,
            }
        }
    }
}

impl IntoParser for AnyParser {
    type Into = Self;
    fn into_parser(self) -> Self::Into {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let any_parser = AnyParser::new();

        let res = any_parser.parse("abcde".chars());
        assert_eq!(res.output, Some(('a',)));
        assert_eq!(res.it.collect::<String>(), "bcde");
    }
    #[test]
    fn fail() {
        let any_parser = AnyParser::new();

        let res = any_parser.parse("".chars());
        assert_eq!(res.output, None);
        assert_eq!(res.it.collect::<String>(), "");
    }
}
