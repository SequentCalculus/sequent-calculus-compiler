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
    use crate::syntax::{
        terms::{Paren, Term},
        types::Ty,
    };

    #[test]
    fn parse_parens() {
        let parser = fun::TermParser::new();
        let expected = Paren {
            inner: Rc::new(Term::Lit(22)),
        }
        .into();
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
    fn parse_covar() {
        let parser = fun::CovarParser::new();
        let expected = "a".to_owned();
        assert_eq!(parser.parse("'a"), Ok(expected))
    }

    #[test]
    fn parse_int() {
        let parser = fun::TyParser::new();
        let expected = Ty::Int();
        assert_eq!(parser.parse("Int"), Ok(expected));
    }
}
