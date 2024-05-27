use crate::core::iterator_bound::InputIteratorTrait;
use crate::core::parser::Parser;
use crate::core::result::ParseResult;
use crate::core::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct ConstantParser<Output>
where
    Output: Clone + Tuple,
{
    pub output: Output,
}

impl<Output> ConstantParser<Output>
where
    Output: Clone + Tuple,
{
    pub fn new(output: Output) -> Self {
        Self { output: output }
    }
}

impl<Output, It> Parser<It> for ConstantParser<Output>
where
    Output: Clone + Tuple,
    It: InputIteratorTrait,
{
    type Output = Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        ParseResult {
            output: Some(self.output.clone()),
            it: it,
        }
    }

    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
        ParseResult {
            output: Some(()),
            it: it,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::string::String;

    use super::ConstantParser;
    use crate::core::parser::Parser;

    #[test]
    fn success() {
        let parser = ConstantParser::new((1,));
        // success
        let str = String::from("abcde");
        let res = parser.parse(str.chars());
        assert_eq!(res.output, Some((1,)));
        let rest: String = res.it.collect();
        assert_eq!(&rest, "abcde");
    }
}
