use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub fun, "/grammar/fun.rs"
);

#[cfg(test)]
mod parser_tests {
    use std::rc::Rc;

    use super::*;
    use crate::fun::syntax::Term;

    #[test]
    fn parse_lit() {
        let parser = fun::TermParser::new();
        let expected = Term::Lit(22);
        assert_eq!(parser.parse("22"), Ok(expected));
    }

    #[test]
    fn parse_var() {
        let parser = fun::TermParser::new();
        let expected = Term::Var("x".to_string());
        assert_eq!(parser.parse("x"), Ok(expected));
    }

    #[test]
    fn parse_let() {
        let parser = fun::TermParser::new();
        let expected = Term::Let(
            "x".to_string(),
            Rc::new(Term::Lit(2)),
            Rc::new(Term::Lit(4)),
        );
        assert_eq!(parser.parse("let x = 2 in 4"), Ok(expected));
    }

    #[test]
    fn parse_label() {
        let parser = fun::TermParser::new();
        let expected = Term::Label("x".to_string(), Rc::new(Term::Lit(2)));
        assert_eq!(parser.parse("label x { 2 }"), Ok(expected));
    }

    #[test]
    fn parse_lam() {
        let parser = fun::TermParser::new();
        let expected = Term::Lam("x".to_string(), Rc::new(Term::Lit(2)));
        assert_eq!(parser.parse("\\x => 2"), Ok(expected));
    }

    #[test]
    fn parse_goto() {
        let parser = fun::TermParser::new();
        let expected = Term::Goto(Rc::new(Term::Lit(2)), "x".to_string());
        assert_eq!(parser.parse("goto(2;x)"), Ok(expected));
    }
}
