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

// 'void' in Rust is '()'; which is also tuple
// pub trait ResultVoid: Parser {}

// marker trait for parser that returns a tuple for Output
pub trait ResultTuple<It: Iterator + Clone>: Parser<It> {}

// marker trait for parser that returns a single value (not tuple) for Output
pub trait ResultRaw<It: Iterator + Clone>: Parser<It> {}
