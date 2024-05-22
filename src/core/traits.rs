use super::result::ParseResult;

pub trait Parser<It>
where
    It: Iterator + Clone,
{
    type Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It>;
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

// marker trait for parser Output type
// either it is void, tuple or single value
pub trait ResultTuple<It: Iterator + Clone>: Parser<It> {}

// Note, 'void' is (), also a tuple in Rust, but we separate it for seq parser
pub trait ResultVoid<It: Iterator + Clone>: Parser<It> {}

// marker trait for parser that returns a single value (not tuple) for Output
pub trait ResultValue<It: Iterator + Clone>: Parser<It> {}
