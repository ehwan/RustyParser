use std::iter::IntoIterator;
use std::iter::Iterator;

use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;

// This Parser will compare the input string starts with the given string.
// 'string' may be a iterator returned by 'chars()', 'bytes()', etc.
// string must be cheaply cloneable.
#[derive(Debug, Clone, Copy)]
pub struct StringEqualParser<CharIterType>
where
    CharIterType: IntoIterator + Clone,
{
    string: CharIterType,
}

impl<StringContainer> StringEqualParser<StringContainer>
where
    StringContainer: IntoIterator + Clone,
{
    pub fn new(string: StringContainer) -> Self {
        StringEqualParser { string: string }
    }
}

impl<StringContainer, It> Parser<It> for StringEqualParser<StringContainer>
where
    StringContainer: IntoIterator + Clone,
    It: InputIteratorTrait,
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
    use super::*;

    #[test]
    fn success1() {
        let pattern = "hello";
        let parser = StringEqualParser::new(pattern.chars());

        let str: String = "hello_world!!".to_string();
        let res = parser.parse(str.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "_world!!");
    }

    #[test]
    fn fail1() {
        let pattern = "hello";
        let parser = StringEqualParser::new(pattern.chars());

        let str: String = "hell_world!!".to_string();
        let res = parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "hell_world!!");
    }

    #[test]
    fn fail2() {
        let pattern = "hello";
        let parser = StringEqualParser::new(pattern.chars());

        let str: String = "hell".to_string();
        let res = parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "hell");
    }
}
