use lalrpop_util::lalrpop_mod;
use result::ParseError;

use crate::{
    lexer::Lexer,
    syntax::{declarations::Module, terms::Term, types::Ty},
};

pub mod result;
pub mod util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub fun, "/parser/fun.rs"
);

pub fn parse_term(s: &str) -> Result<Term, ParseError> {
    let lexer = Lexer::new(s);
    let parser = fun::TermParser::new();
    parser.parse(lexer).map_err(From::from)
}

pub fn parse_type(s: &str) -> Result<Ty, ParseError> {
    let lexer = Lexer::new(s);
    let parser = fun::TyParser::new();
    parser.parse(lexer).map_err(From::from)
}

pub fn parse_module(s: &str) -> Result<Module, ParseError> {
    let lexer = Lexer::new(s);
    let parser = fun::ProgParser::new();
    parser.parse(lexer).map_err(From::from)
}

/// Parse a string and return the parsed term.
/// Panics if the string cannot be parsed.
#[macro_export]
macro_rules! parse_term {
    ($str:literal) => {
        fun::parser::parse_term(&str).unwrap_or_else(|_| panic!("Could not parse input: {}", $str))
    };
}

#[cfg(test)]
mod parser_tests {

    use std::rc::Rc;

    use codespan::Span;

    use super::*;
    use crate::{
        syntax::{
            declarations::Module,
            terms::{Lit, Paren, Term, XVar},
            types::Ty,
        },
        test_common::{codata_stream, data_list, def_mult},
    };

    #[test]
    fn parse_parens() {
        let expected = Paren {
            span: Span::default(),
            inner: Rc::new(Term::Lit(Lit::mk(22))),
        }
        .into();
        assert_eq!(parse_term("(22)"), Ok(expected));
    }

    #[test]
    fn parse_lit() {
        let expected = Term::Lit(Lit::mk(22));
        assert_eq!(parse_term("22"), Ok(expected));
    }

    #[test]
    fn parse_var() {
        let expected = XVar::mk("x").into();
        assert_eq!(parse_term("x"), Ok(expected));
    }

    #[test]
    fn parse_int() {
        let expected = Ty::mk_i64();
        assert_eq!(parse_type("i64"), Ok(expected));
    }

    #[test]
    fn parse_prog() {
        let expected = Module {
            declarations: vec![
                data_list().into(),
                codata_stream().into(),
                def_mult().into(),
            ],
        };
        let result = parse_module(
            "data List[A] { Nil, Cons(x:A,xs:List[A]) }
            codata Stream[A] { Hd : A , Tl : Stream[A] }
            def mult(l:List[i64]):i64 { l.case[i64] {Nil => 1, Cons(x, xs) => x*mult(xs)} }",
        );
        assert_eq!(result, Ok(expected))
    }
}
