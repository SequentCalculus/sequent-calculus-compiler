use lalrpop_util::lalrpop_mod;

mod util;

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
        declarations::{CodataDeclaration, CtorSig, DataDeclaration, Definition, DtorSig, Module},
        terms::{Lit, Paren, Term, Var},
        types::Ty,
    };

    #[test]
    fn parse_parens() {
        let parser = fun::TermParser::new();
        let expected = Paren {
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

    #[test]
    fn parse_prog() {
        let parser = fun::ProgParser::new();
        let expected = Module {
            declarations: vec![
                DataDeclaration {
                    name: "ListInt".to_owned(),
                    ctors: vec![
                        CtorSig {
                            name: "Nil".to_owned(),
                            args: vec![],
                        },
                        CtorSig {
                            name: "Cons".to_owned(),
                            args: vec![
                                ContextBinding::TypedVar {
                                    var: "x".to_owned(),
                                    ty: Ty::Int(),
                                },
                                ContextBinding::TypedVar {
                                    var: "xs".to_owned(),
                                    ty: Ty::Decl("ListInt".to_owned()),
                                },
                            ],
                        },
                    ],
                }
                .into(),
                CodataDeclaration {
                    name: "StreamInt".to_owned(),
                    dtors: vec![
                        DtorSig {
                            name: "Hd".to_owned(),
                            args: vec![],
                            cont_ty: Ty::Int(),
                        },
                        DtorSig {
                            name: "Tl".to_owned(),
                            args: vec![],
                            cont_ty: Ty::Decl("StreamInt".to_owned()),
                        },
                    ],
                }
                .into(),
                Definition {
                    name: "main".to_owned(),
                    context: vec![],
                    body: Lit::mk(1).into(),
                    ret_ty: Ty::Int(),
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
