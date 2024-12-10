use lalrpop_util::lalrpop_mod;
use result::ParseError;

use crate::syntax::{declarations::Module, terms::Term};

pub mod result;
pub mod util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub fun, "/parser/fun.rs"
);

pub fn parse_term(s: &str) -> Result<Term, ParseError> {
    let parser = fun::TermParser::new();
    parser.parse(s).map_err(From::from)
}

pub fn parse_module(s: &str) -> Result<Module, ParseError> {
    let parser = fun::ProgParser::new();
    parser.parse(s).map_err(From::from)
}

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

    use codespan::Span;

    use super::*;
    use crate::{
        syntax::{
            context::TypingContext,
            declarations::Module,
            terms::{Lit, Paren, Term, Var},
            types::Ty,
        },
        test_common::{codata_stream, data_list, def_mult},
    };

    #[test]
    fn parse_parens() {
        let parser = fun::TermParser::new();
        let expected = Paren {
            span: Span::default(),
            inner: Rc::new(Term::Lit(Lit::mk(22))),
        }
        .into();
        assert_eq!(parser.parse("(22)"), Ok(expected));
    }

    #[test]
    fn parse_lit() {
        let parser = fun::TermParser::new();
        let expected = Term::Lit(Lit::mk(22));
        assert_eq!(parser.parse("22"), Ok(expected));
    }

    #[test]
    fn parse_var() {
        let parser = fun::TermParser::new();
        let expected = Var::mk("x").into();
        assert_eq!(parser.parse("x"), Ok(expected));
    }

    #[test]
    fn parse_int() {
        let parser = fun::TyParser::new();
        let expected = Ty::mk_int();
        assert_eq!(parser.parse("Int"), Ok(expected));
    }

    #[test]
    fn parse_ctx() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_int());
        ctx.add_covar("a", Ty::mk_int());
        let parser = fun::ContextParser::new();
        assert_eq!(parser.parse("x : Int, 'a:cntInt"), Ok(ctx))
    }

    #[test]
    fn parse_prog() {
        let parser = fun::ProgParser::new();
        let expected = Module {
            declarations: vec![
                data_list().into(),
                codata_stream().into(),
                def_mult().into(),
            ],
        };
        let result = parser.parse(
            "data ListInt { Nil, Cons(x:Int,xs:ListInt) } 
            codata StreamInt { Hd : Int , Tl : StreamInt } 
            def mult(l:ListInt):Int:=l.case{Nil => 1, Cons(x:Int, xs:ListInt) => x*mult(xs)};",
        );
        assert_eq!(result, Ok(expected))
    }
}
