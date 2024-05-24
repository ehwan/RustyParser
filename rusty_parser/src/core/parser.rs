use super::result::ParseResult;
use super::tuple::Tuple;

// defulat Parser trait
pub trait Parser<It>
where
    It: Iterator + Clone,
{
    type Output: Tuple;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It>;

    // this is special implementation for parsing.
    // it does not parse data from string, just check if it matches the pattern
    // for some parser, there may be a cheaper way to check if it matches the pattern
    // than actually parsing the data
    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        let res = self.parse(it);
        if let Some(_) = res.output {
            ParseResult {
                output: Some(()),
                it: res.it,
            }
        } else {
            ParseResult {
                output: None,
                it: res.it,
            }
        }
    }
}
