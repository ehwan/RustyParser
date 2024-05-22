use std::iter::Iterator;

// define Default Parser Trait
// and ParseResult, MatchResult Struct

#[derive(Debug)]
pub struct ParseResult<Output, It>
where
    It: Iterator + Clone,
{
    pub output: Option<Output>,
    pub it: It,
}

/*
pub trait Parser {
    type Derived;

    fn parse<It>(&self, it: It) -> ParseResult<, It>
    where
        It: Iterator;
    fn pattern_match<It>(&self, it: It) -> MatchResult<It>
    where
        It: Iterator,
    {
        let result = self.parse(it);
        MatchResult {
            output: result.output.is_some(),
            it: result.it,
        }
    }
}
*/
