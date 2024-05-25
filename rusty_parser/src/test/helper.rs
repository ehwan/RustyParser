#[cfg(test)]
mod test {
    use crate::core::parser::Parser;
    use crate::core::singleeq::SingleEqualParser;
    use crate::core::singlerange::SingleRangeParser;
    use crate::wrapper::rced::RcedParser;

    // these tests are for the parser helper functions
    #[test]
    fn success1() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_parser = digit_parser.map(|(x,)| (x as i32 - '0' as i32,));
        let digit_parser2 = digit_parser.clone().seq(digit_parser);

        let alpha_parser = SingleRangeParser::new('a'..='z');
        let alpha_parser = alpha_parser.map(|(x,)| (x as i32 - 'a' as i32,));
        let alpha_parser2 = alpha_parser.clone().seq(alpha_parser);

        let alphadigit_parser2 = alpha_parser2.or_(digit_parser2);

        let alphadigit_parser2_vec = alphadigit_parser2.repeat(2..);
        let alphadigit_parser2_vec_ref = alphadigit_parser2_vec.ref_();

        let str = "12abcdefg";

        let res = alphadigit_parser2_vec_ref.parse(str.chars());
        assert_eq!(res.output, Some((vec![(1, 2), (0, 1), (2, 3), (4, 5)],)));

        let rest: String = res.it.collect();
        assert_eq!(rest, "g");
    }
    #[test]
    fn success2() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let a_parser = SingleEqualParser::new('a');

        let mut boxed_parser = digit_parser.clone().boxed();

        let str = "1ahello";

        let res = boxed_parser.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "ahello");

        // set boxed_parser to alpha_parser
        boxed_parser = a_parser.clone().boxed();
        let res = boxed_parser.parse(res.it);
        assert_eq!(res.output, Some(('a',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "hello");

        // reset boxed_parser to digit_parser
        boxed_parser = digit_parser.boxed();
        // now, make immutable RefCell of boxed_parser
        let refcelled = boxed_parser.refcelled();

        let res = refcelled.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "ahello");

        // set immutable refcelled parser to alpha_parser
        *refcelled.refcelled_parser().borrow_mut() = a_parser.boxed();
        // refcelled is now 'a' parser
        let res = refcelled.parse(res.it);
        assert_eq!(res.output, Some(('a',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "hello");
    }

    #[test]
    fn success3() {
        let digit_parser = SingleRangeParser::new('0'..='9');
        let digit_boxed = digit_parser.boxed();
        let digit_refcelled = digit_boxed.refcelled();
        let a_parser = SingleEqualParser::new('a');
        let a_boxed = a_parser.boxed();

        let str = "123456";

        // let 2 parsers point to the same digit parser
        let rced1 = digit_refcelled.rced();
        let rced2 = RcedParser::clone(&rced1);

        let res = rced1.parse(str.chars());
        assert_eq!(res.output, Some(('1',)));
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "23456");

        // now change rcde1 to a_parser
        *(rced1.rced_parser().refcelled_parser().borrow_mut()) = a_boxed;
        //           ^               ^                ^
        //           |               |                |
        //           |               |          &mut Box<Parser>
        //           |        &RefCell<Box<Parser>>
        //      &Rc<RefCell<Box<Parser>>>

        // since rced1 and rced2 point to the same parser, rced2 should also be a_parser
        let res = rced2.parse(res.it);
        assert_eq!(res.output, None);
        let rest: String = res.it.clone().collect();
        assert_eq!(rest, "23456");
    }
}
