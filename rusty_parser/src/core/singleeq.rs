use std::iter::Iterator;

use super::iterator_bound::InputIteratorTrait;
use super::parser::Parser;
use super::result::ParseResult;

#[derive(Debug, Clone)]
pub struct SingleEqualParser<TargetCharacterType> {
    pub character: TargetCharacterType,
}

impl<TargetCharacterType> SingleEqualParser<TargetCharacterType> {
    pub fn new(character: TargetCharacterType) -> Self {
        SingleEqualParser {
            character: character,
        }
    }
}

impl<TargetCharacterType, It> Parser<It> for SingleEqualParser<TargetCharacterType>
where
    It: InputIteratorTrait,
    <It as Iterator>::Item: PartialEq<TargetCharacterType>,
{
    type Output = (<It as Iterator>::Item,);

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut it = it;
        let i0 = it.clone();
        if let Some(val) = it.next() {
            if val == self.character {
                ParseResult {
                    output: Some((val,)),
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

    use super::super::parser::Parser;
    use super::SingleEqualParser;

    #[test]
    fn success_test1() {
        let a_parser = SingleEqualParser::new('a');
        // success
        let start_with_a_string = String::from("abcde");
        let res = a_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, Some(('a',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn success_test2() {
        let b_parser = SingleEqualParser::new('b');
        // success
        let start_with_b_string = String::from("bacde");
        let res = b_parser.parse(start_with_b_string.chars());
        assert_eq!(res.output, Some(('b',)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "acde");
    }
    #[test]
    fn fail_test1() {
        let a_parser = SingleEqualParser::new('a');
        // this case is fail
        let start_with_b_string = String::from("bacde");
        let res = a_parser.parse(start_with_b_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bacde");
    }
    #[test]
    fn fail_test2() {
        let b_parser = SingleEqualParser::new('b');
        // this case is fail
        let start_with_a_string = String::from("abcde");
        let res = b_parser.parse(start_with_a_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }

    #[test]
    fn parse_null() {
        let x_parser = SingleEqualParser::new('x');
        let empty_string = String::from("");
        let res = x_parser.parse(empty_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "");
    }

    #[test]
    fn match_success1() {
        let a_parser = SingleEqualParser::new('a');
        // success
        let start_with_a_string = String::from("abcde");
        let res = a_parser.match_pattern(start_with_a_string.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bcde");
    }
    #[test]
    fn match_success2() {
        let b_parser = SingleEqualParser::new('b');
        // success
        let start_with_b_string = String::from("bacde");
        let res = b_parser.match_pattern(start_with_b_string.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "acde");
    }
    #[test]
    fn match_fail1() {
        let a_parser = SingleEqualParser::new('a');
        // this case is fail
        let start_with_b_string = String::from("bacde");
        let res = a_parser.match_pattern(start_with_b_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "bacde");
    }
    #[test]
    fn match_fail2() {
        let b_parser = SingleEqualParser::new('b');
        // this case is fail
        let start_with_a_string = String::from("abcde");
        let res = b_parser.match_pattern(start_with_a_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }

    #[test]
    fn match_null() {
        let x_parser = SingleEqualParser::new('x');
        let empty_string = String::from("");
        let res = x_parser.match_pattern(empty_string.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(&rest, "");
    }
}
