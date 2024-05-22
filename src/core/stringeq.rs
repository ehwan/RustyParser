use std::iter::IntoIterator;
use std::iter::Iterator;

use super::result::ParseResult;
use super::traits::Parser;
use super::traits::ResultVoid;

#[derive(Debug, Clone)]
pub struct StringEqualParser<StringContainer, It>
where
    StringContainer: IntoIterator + Clone,
    It: Iterator + Clone,
{
    pub string: StringContainer,
    _phantom: std::marker::PhantomData<It>,
}

impl<StringContainer, It> StringEqualParser<StringContainer, It>
where
    StringContainer: IntoIterator + Clone,
    It: Iterator + Clone,
{
    pub fn new(string: StringContainer) -> StringEqualParser<StringContainer, It> {
        StringEqualParser {
            string: string,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<StringContainer, It> ResultVoid<It> for StringEqualParser<StringContainer, It>
where
    StringContainer: IntoIterator + Clone,
    It: Iterator + Clone,
    <It as Iterator>::Item:
        PartialEq<<<StringContainer as IntoIterator>::IntoIter as Iterator>::Item>,
{
}

impl<StringContainer, It> Parser<It> for StringEqualParser<StringContainer, It>
where
    StringContainer: IntoIterator + Clone,
    It: Iterator + Clone,
    <It as Iterator>::Item:
        PartialEq<<<StringContainer as IntoIterator>::IntoIter as Iterator>::Item>,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        let i0 = it.clone();
        for ch in self.string.clone() {
            match it.next() {
                Some(ch2) => {
                    if ch2 == ch {
                        continue;
                    } else {
                        return ParseResult {
                            output: None,
                            it: i0,
                        };
                    }
                }
                None => {
                    return ParseResult {
                        output: None,
                        it: i0,
                    }
                }
            }
        }
        return ParseResult {
            output: Some(()),
            it: it,
        };
    }
}

#[cfg(test)]
mod test {
    use super::super::traits::Parser;
    use super::*;

    #[test]
    fn test_string_match_success() {
        let pattern = "hello";
        let parser = StringEqualParser::new(pattern.chars());

        let str: String = "hello_world!!".to_string();
        let res = parser.parse(str.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "_world!!");
    }

    #[test]
    fn test_string_match_fail() {
        let pattern = "hello";
        let parser = StringEqualParser::new(pattern.chars());

        let str: String = "hell_world!!".to_string();
        let res = parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "hell_world!!");
    }

    #[test]
    fn test_pattern_longer_than_input() {
        let pattern = "hello";
        let parser = StringEqualParser::new(pattern.chars());

        let str: String = "hell".to_string();
        let res = parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "hell");
    }
}
