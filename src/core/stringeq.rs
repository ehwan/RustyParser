use std::iter::IntoIterator;
use std::iter::Iterator;

use super::result::ParseResult;

#[derive(Debug, Clone)]
pub struct StringEqualParser<StringType> {
    pub string: StringType,
}

impl<StringType> StringEqualParser<StringType>
where
    StringType: IntoIterator + Clone,
{
    pub fn new(string: StringType) -> StringEqualParser<StringType> {
        StringEqualParser { string: string }
    }

    pub fn parse<It>(&self, mut it: It) -> ParseResult<(), It>
    where
        It: Iterator + Clone,
        <It as Iterator>::Item:
            PartialEq<<<StringType as IntoIterator>::IntoIter as Iterator>::Item>,
    {
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

    pub fn match_pattern<It>(&self, it: It) -> ParseResult<(), It>
    where
        It: Iterator + Clone,
        <It as Iterator>::Item:
            PartialEq<<<StringType as IntoIterator>::IntoIter as Iterator>::Item>,
    {
        self.parse(it)
    }
}

#[cfg(test)]
mod test {
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
