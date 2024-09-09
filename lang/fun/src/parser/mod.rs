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
        context::ContextBinding,
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
    fn parse_int() {
        let parser = fun::TyParser::new();
        let expected = Ty::Int();
        assert_eq!(parser.parse("Int"), Ok(expected));
    }

    #[test]
    fn parse_ctx() {
        let parser = fun::ContextParser::new();
        let expected = vec![
            ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::Int(),
            },
            ContextBinding::TypedVar {
                var: "a".to_owned(),
                ty: Ty::Int(),
            },
        ];
        assert_eq!(parser.parse("x : Int, a:Int"), Ok(expected))
    }
}
