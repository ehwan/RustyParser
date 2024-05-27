use super::iterator_bound::InputIteratorTrait;
use super::parser::Parser;
use super::result::ParseResult;

// parser that success if reached end of input
#[derive(Debug, Clone)]
pub struct EndParser {}

impl EndParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl<It> Parser<It> for EndParser
where
    It: InputIteratorTrait,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let mut i0 = it.clone();
        match i0.next() {
            Some(_) => ParseResult {
                output: None,
                it: it,
            },
            None => ParseResult {
                output: Some(()),
                it: it,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::parser::Parser;
    use super::EndParser;

    #[test]
    fn success() {
        let end_parser = EndParser::new();
        let res = end_parser.parse("".chars());
        assert_eq!(res.output, Some(()));
    }
    #[test]
    fn fail() {
        let end_parser = EndParser::new();
        let res = end_parser.parse("a".chars());
        assert_eq!(res.output, None);
    }
}
