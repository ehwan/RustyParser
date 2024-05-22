use std::iter::Iterator;
use std::marker::PhantomData;

use super::result::ParseResult;
use super::traits::Parser;

#[derive(Debug)]
pub struct SingleEqualParser<TargetCharacterType, It: Iterator + Clone> {
    pub character: TargetCharacterType,
    _phantom: PhantomData<It>,
}

impl<TargetCharacterType, It: Iterator + Clone> SingleEqualParser<TargetCharacterType, It>
where
    <It as Iterator>::Item: PartialEq<TargetCharacterType>,
{
    pub fn new(character: TargetCharacterType) -> SingleEqualParser<TargetCharacterType, It> {
        SingleEqualParser {
            character: character,
            _phantom: PhantomData,
        }
    }
}

impl<TargetCharacterType, It: Iterator + Clone> Parser<It>
    for SingleEqualParser<TargetCharacterType, It>
where
    <It as Iterator>::Item: PartialEq<TargetCharacterType>,
{
    type Output = <It as Iterator>::Item;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = it.next() {
            if val == self.character {
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
            if val == self.character {
                ParseResult {
                    output: Some(()),
                    it: it,
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

#[cfg(test)]
mod tests {
    use std::string::String;

    use super::super::traits::Parser;
    use super::SingleEqualParser;

    #[test]
    fn parse_a_success_test() {
        let a_parser = SingleEqualParser::new('a');
        // success
        let start_with_a_string = String::from("abcde");
        let res = a_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, Some('a'));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn parse_a_fail_test() {
        let a_parser = SingleEqualParser::new('a');
        // this case is fail
        let start_with_b_string = String::from("bacde");
        let res = a_parser.parse(start_with_b_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bacde");
    }
    #[test]
    fn parse_b_success_test() {
        let b_parser = SingleEqualParser::new('b');
        // success
        let start_with_b_string = String::from("bacde");
        let res = b_parser.parse(start_with_b_string.chars());
        assert_eq!(res.output, Some('b'));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "acde");
    }
    #[test]
    fn parse_b_fail_test() {
        let b_parser = SingleEqualParser::new('b');
        // this case is fail
        let start_with_a_string = String::from("abcde");
        let res = b_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }

    #[test]
    fn parse_null_test() {
        let x_parser = SingleEqualParser::new('x');
        {
            let empty_string = String::from("");
            let res = x_parser.parse(empty_string.chars());
            assert_eq!(res.output, None);
            let rest: String = res.it.collect();
            assert_eq!(&rest, "");
        }
    }

    #[test]
    fn match_a_test() {
        let a_parser = SingleEqualParser::new('a');
        {
            // success
            let start_with_a_string = String::from("abcde");
            let res = a_parser.match_pattern(start_with_a_string.chars());
            assert_eq!(res.output, Some(()));
            let rest: String = res.it.collect();
            assert_eq!(&rest, "bcde");
        }
        {
            // this case is fail
            let start_with_b_string = String::from("bacde");
            let res = a_parser.match_pattern(start_with_b_string.chars());
            assert_eq!(res.output, None);
            let rest: String = res.it.collect();
            assert_eq!(&rest, "bacde");
        }
    }
    #[test]
    fn match_b_test() {
        let b_parser = SingleEqualParser::new('b');
        {
            // success
            let start_with_b_string = String::from("bacde");
            let res = b_parser.match_pattern(start_with_b_string.chars());
            assert_eq!(res.output, Some(()));
            let rest: String = res.it.collect();
            assert_eq!(&rest, "acde");
        }
        {
            // this case is fail
            let start_with_a_string = String::from("abcde");
            let res = b_parser.match_pattern(start_with_a_string.chars());
            assert_eq!(res.output, None);
            let rest: String = res.it.collect();
            assert_eq!(&rest, "abcde");
        }
    }

    #[test]
    fn match_null_test() {
        let x_parser = SingleEqualParser::new('x');
        {
            let empty_string = String::from("");
            let res = x_parser.match_pattern(empty_string.chars());
            assert_eq!(res.output, None);
            let rest: String = res.it.collect();
            assert_eq!(&rest, "");
        }
    }
}
