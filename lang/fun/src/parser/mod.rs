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
    use crate::syntax::{
        context::{ContextBinding, TypingContext},
        declarations::{CodataDeclaration, CtorSig, DataDeclaration, Definition, DtorSig, Module},
        terms::{Lit, Paren, Term, Var},
        types::Ty,
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
        let parser = fun::ContextParser::new();
        let expected = TypingContext {
            bindings: vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedCovar {
                    covar: "a".to_owned(),
                    ty: Ty::mk_int(),
                },
            ],
        };
        assert_eq!(parser.parse("x : Int, 'a:cntInt"), Ok(expected))
    }

    #[test]
    fn parse_prog() {
        let parser = fun::ProgParser::new();
        let expected = Module {
            declarations: vec![
                DataDeclaration {
                    span: Span::default(),
                    name: "ListInt".to_owned(),
                    ctors: vec![
                        CtorSig {
                            span: Span::default(),
                            name: "Nil".to_owned(),
                            args: TypingContext { bindings: vec![] },
                        },
                        CtorSig {
                            span: Span::default(),
                            name: "Cons".to_owned(),
                            args: TypingContext {
                                bindings: vec![
                                    ContextBinding::TypedVar {
                                        var: "x".to_owned(),
                                        ty: Ty::mk_int(),
                                    },
                                    ContextBinding::TypedVar {
                                        var: "xs".to_owned(),
                                        ty: Ty::mk_decl("ListInt"),
                                    },
                                ],
                            },
                        },
                    ],
                }
                .into(),
                CodataDeclaration {
                    span: Span::default(),
                    name: "StreamInt".to_owned(),
                    dtors: vec![
                        DtorSig {
                            span: Span::default(),
                            name: "Hd".to_owned(),
                            args: TypingContext { bindings: vec![] },
                            cont_ty: Ty::mk_int(),
                        },
                        DtorSig {
                            span: Span::default(),
                            name: "Tl".to_owned(),
                            args: TypingContext { bindings: vec![] },
                            cont_ty: Ty::mk_decl("StreamInt"),
                        },
                    ],
                }
                .into(),
                Definition {
                    span: Span::default(),
                    name: "main".to_owned(),
                    context: TypingContext { bindings: vec![] },
                    body: Lit::mk(1).into(),
                    ret_ty: Ty::mk_int(),
                }
                .into(),
            ],
        };
        let result = parser.parse(
            "data ListInt { Nil, Cons(x:Int,xs:ListInt) } codata StreamInt { Hd : Int , Tl : StreamInt } def main():Int:=1;",
        );
        assert_eq!(result, Ok(expected))
    }
}
