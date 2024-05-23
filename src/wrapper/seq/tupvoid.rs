use std::iter::Iterator;

use crate::core::result::ParseResult;
use crate::core::traits::Parser;
use crate::core::traits::ResultTuple;
use crate::core::traits::ResultVoid;

#[derive(Debug, Clone)]
pub struct SeqTupVoidParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
    ParserB: ResultVoid<It> + Parser<It>,
{
    pub parser_a: ParserA,
    pub parser_b: ParserB,
    _phantom: std::marker::PhantomData<It>,
}

impl<ParserA, ParserB, It> SeqTupVoidParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
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

impl<ParserA, ParserB, It> ResultTuple<It> for SeqTupVoidParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
    ParserB: ResultVoid<It> + Parser<It>,
{
}

impl<ParserA, ParserB, It> Parser<It> for SeqTupVoidParser<ParserA, ParserB, It>
where
    It: Iterator + Clone,
    ParserA: ResultTuple<It> + Parser<It>,
    ParserB: ResultVoid<It> + Parser<It>,
{
    type Output = <ParserA as Parser<It>>::Output;

    fn parse(&self, it: It) -> ParseResult<Self::Output, It> {
        let i0 = it.clone();
        let res_a = self.parser_a.parse(it);
        if let Some(val_a) = res_a.output {
            let res_b = self.parser_b.match_pattern(res_a.it);
            if let Some(_) = res_b.output {
                ParseResult {
                    output: Some(val_a),
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

    fn match_pattern(&self, it: It) -> ParseResult<(), It> {
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
    use crate::core::singleeq::SingleEqualParser;
    use crate::core::stringeq::StringEqualParser;
    use crate::core::traits::Parser;
    use crate::wrapper::seq::valval::SeqValValParser;

    #[test]
    fn success_test() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let a_parser = SingleEqualParser::new('a');
        let tup_parser = SeqValValParser::new(a_parser.clone(), a_parser);
        let seq_parser = SeqTupVoidParser::new(tup_parser, hello_parser);

        let str = "aahelloabcd";
        let res = seq_parser.parse(str.chars());
        assert_eq!(res.output, Some(('a', 'a')));
        let rest: String = res.it.collect();
        assert_eq!(rest, "abcd");
    }

    #[test]
    fn fail_test1() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let a_parser = SingleEqualParser::new('a');
        let tup_parser = SeqValValParser::new(a_parser.clone(), a_parser);
        let seq_parser = SeqTupVoidParser::new(tup_parser, hello_parser);

        let str = "bahelloabcd";
        let res = seq_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "bahelloabcd");
    }
    #[test]
    fn fail_test2() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let a_parser = SingleEqualParser::new('a');
        let tup_parser = SeqValValParser::new(a_parser.clone(), a_parser);
        let seq_parser = SeqTupVoidParser::new(tup_parser, hello_parser);

        let str = "abhelloabcd";
        let res = seq_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "abhelloabcd");
    }
    #[test]
    fn fail_test3() {
        let hello_parser = StringEqualParser::new("hello".chars());
        let a_parser = SingleEqualParser::new('a');
        let tup_parser = SeqValValParser::new(a_parser.clone(), a_parser);
        let seq_parser = SeqTupVoidParser::new(tup_parser, hello_parser);

        let str = "aahellaabcd";
        let res = seq_parser.parse(str.chars());
        assert_eq!(res.output, None);
        let rest: String = res.it.collect();
        assert_eq!(rest, "aahellaabcd");
    }
}
