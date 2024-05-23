use std::iter::Iterator;

use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultVoid;

#[derive(Debug, Clone)]
pub struct SeqVoidVoidParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultVoid<It> + Parser<It>,
    ParserB: ResultVoid<It> + Parser<It>,
{
    pub parser_a: ParserA,
    pub parser_b: ParserB,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserA, ParserB, It> SeqVoidVoidParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultVoid<It> + Parser<It>,
    ParserB: ResultVoid<It> + Parser<It>,
{
    pub fn new(parser_a: ParserA, parser_b: ParserB) -> Self {
        Self {
            parser_a: parser_a,
            parser_b: parser_b,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<ParserA, ParserB, It> ResultVoid<It> for SeqVoidVoidParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultVoid<It> + Parser<It>,
    ParserB: ResultVoid<It> + Parser<It>,
{
}

impl<ParserA, ParserB, It> Parser<It> for SeqVoidVoidParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultVoid<It> + Parser<It>,
    ParserB: ResultVoid<It> + Parser<It>,
{
    type Output = ();

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let res_a = self.parser_a.match_pattern(it);
        if let Some(_) = res_a.output {
            let res_b = self.parser_b.match_pattern(res_a.it);
            if let Some(_) = res_b.output {
                ParseResult {
                    output: Some(()),
                    it: res_b.it,
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
mod test {
    use super::*;
    use crate::core::stringeq::StringEqualParser;
    use crate::core::traits::Parser;

    #[test]
    fn seq_parser_success_test() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let world_parser = StringEqualParser::new("world".chars());
        let seq_parser = SeqVoidVoidParser::new(hello_parser, world_parser);

        let str = "helloworldabcd";
        let res = seq_parser.parse(str.chars());
        assert_eq!(res.output, Some(()));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }

    #[test]
    fn seq_parser_fail_test() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let world_parser = StringEqualParser::new("world".chars());
        let seq_parser = SeqVoidVoidParser::new(hello_parser, world_parser);

        let str = "hello_worldabcd";
        let res = seq_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "hello_worldabcd");
    }
}
