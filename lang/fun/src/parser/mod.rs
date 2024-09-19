use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub fun, "/parser/fun.rs"
);

/// Parse a string and return the parsed term.
/// Panics if the string cannot be parsed.
#[macro_export]
macro_rules! parse_term {
    ($str:literal) => {
        fun::parser::fun::TermParser::new()
            .parse($str)
            .expect(&format!("Could not parse input: {}", $str))
    };
}

#[cfg(test)]
mod parser_tests {

    use std::rc::Rc;

    use super::*;
    use crate::syntax::{
        context::ContextBinding,
        terms::{Lit, Paren, Term, Var},
        types::Ty,
    };

    #[test]
    fn parse_parens() {
        let parser = fun::TermParser::new();
        let expected = Paren {
            inner: Rc::new(Term::Lit(Lit { val: 22 })),
        }
        .into();
        assert_eq!(parser.parse("(22)"), Ok(expected));
    }

    #[test]
    fn parse_lit() {
        let parser = fun::TermParser::new();
        let expected = Term::Lit(Lit { val: 22 });
        assert_eq!(parser.parse("22"), Ok(expected));
    }

    #[test]
    fn parse_var() {
        let parser = fun::TermParser::new();
        let expected = Var {
            var: "x".to_string(),
        }
        .into();
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
            ContextBinding::TypedCovar {
                covar: "a".to_owned(),
                ty: Ty::Int(),
            },
        ];
        assert_eq!(parser.parse("x : Int, 'a:cntInt"), Ok(expected))
    }
}
