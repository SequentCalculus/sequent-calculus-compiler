use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub fun, "/parser/fun.rs"
);

#[cfg(test)]
mod parser_tests {
    use std::rc::Rc;

    use super::*;
    use crate::syntax::{Goto, Label, Lam, Let, Term};

    #[test]
    fn parse_parens() {
        let parser = fun::TermParser::new();
        let expected = Term::Lit(22);
        assert_eq!(parser.parse("(22)"), Ok(expected));
    }

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
        let expected = Let {
            variable: "x".to_string(),
            bound_term: Rc::new(Term::Lit(2)),
            in_term: Rc::new(Term::Lit(4)),
        }
        .into();
        assert_eq!(parser.parse("let x = 2 in 4"), Ok(expected));
    }

    #[test]
    fn parse_label() {
        let parser = fun::TermParser::new();
        let expected = Label {
            label: "x".to_string(),
            term: Rc::new(Term::Lit(2)),
        };
        assert_eq!(parser.parse("label x { 2 }"), Ok(expected.into()));
    }

    #[test]
    fn parse_lam() {
        let parser = fun::TermParser::new();
        let expected = Lam {
            variable: "x".to_string(),
            body: Rc::new(Term::Lit(2)),
        };
        assert_eq!(parser.parse("\\x => 2"), Ok(expected.into()));
    }

    #[test]
    fn parse_goto() {
        let parser = fun::TermParser::new();
        let expected = Goto {
            term: Rc::new(Term::Lit(2)),
            target: "x".to_string(),
        };
        assert_eq!(parser.parse("goto(2;x)"), Ok(expected.into()));
    }
}
