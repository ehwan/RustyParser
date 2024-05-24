#[cfg(test)]
mod test {
    use crate::core::parser::Parser;
    use crate::core::singlerange::SingleRangeParser;

    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_parser = digit_parser.map(|(x,)| (x as i32 - '0' as i32,));
        let digit_parser2 = digit_parser.clone().seq(digit_parser);

        let alpha_parser = SingleRangeParser::new('a'..='z');
        let alpha_parser = alpha_parser.map(|(x,)| (x as i32 - 'a' as i32,));
        let alpha_parser2 = alpha_parser.clone().seq(alpha_parser);

        let alphadigit_parser2 = alpha_parser2.or(digit_parser2);

        let alphadigit_parser2_vec = alphadigit_parser2.repeat(2..);

        let str = "12abcdefg";

        let res = alphadigit_parser2_vec.parse(str.chars());
        assert_eq!(res.output, Some((vec![(1, 2), (0, 1), (2, 3), (4, 5)],)));

        let rest: String = res.it.collect();
        assert_eq!(rest, "g");
    }
}
