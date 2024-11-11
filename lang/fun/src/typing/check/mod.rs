use miette::SourceSpan;
use printer::Print;

pub mod case;
pub mod cocase;
pub mod context;
pub mod ctor;
pub mod declarations;
pub mod dtor;
pub mod fun;
pub mod goto;
pub mod ifz;
pub mod label;
pub mod let_exp;
pub mod lit;
pub mod op;
pub mod paren;
pub mod terms;
pub mod var;

use context::lookup_covar;
use declarations::check_declaration;
use terms::Check;

use crate::{
    parser::util::ToMiette,
    syntax::{
        context::{ContextBinding, TypingContext},
        declarations::Module,
        substitution::{Substitution, SubstitutionBinding},
        types::Ty,
    },
    typing::symbol_table::build_symbol_table,
};

use super::{errors::Error, symbol_table::SymbolTable};

pub fn check_module(module: Module) -> Result<Module, Error> {
    let symbol_table = build_symbol_table(&module)?;
    check_module_with_table(module, &symbol_table)
}

fn check_module_with_table(module: Module, symbol_table: &SymbolTable) -> Result<Module, Error> {
    let mut new_decls = vec![];
    for decl in module.declarations.into_iter() {
        let decl_checked = check_declaration(decl, symbol_table)?;
        new_decls.push(decl_checked);
    }
    Ok(Module {
        declarations: new_decls,
    })
}

pub fn check_type(ty: &Ty, symbol_table: &SymbolTable) -> Result<(), Error> {
    match ty {
        Ty::Int { .. } => Ok(()),
        Ty::Decl { span, name } => match symbol_table.ty_ctors.get(name) {
            None => Err(Error::Undefined {
                span: span.to_miette(),
                name: name.clone(),
            }),
            Some(_) => Ok(()),
        },
    }
}

fn check_args(
    span: &SourceSpan,
    symbol_table: &SymbolTable,
    context: &TypingContext,
    args: Substitution,
    types: &TypingContext,
) -> Result<Substitution, Error> {
    if types.len() != args.len() {
        return Err(Error::WrongNumberOfArguments {
            span: *span,
            expected: types.len(),
            got: args.len(),
        });
    }
    let mut new_subst = vec![];
    for c in args.into_iter().zip(types.iter()) {
        match c {
            (SubstitutionBinding::TermBinding(term), ContextBinding::TypedVar { ty, .. }) => {
                let term_checked = term.check(symbol_table, context, ty)?;
                new_subst.push(SubstitutionBinding::TermBinding(term_checked));
            }
            (SubstitutionBinding::CovarBinding(cov), ContextBinding::TypedCovar { ty, .. }) => {
                let found_ty = lookup_covar(span, context, &cov)?;
                check_equality(span, ty, &found_ty)?;
                new_subst.push(SubstitutionBinding::CovarBinding(cov));
            }
            (SubstitutionBinding::CovarBinding(_), ContextBinding::TypedVar { .. }) => {
                return Err(Error::ExpectedTermGotCovariable { span: *span })
            }
            (SubstitutionBinding::TermBinding(..), ContextBinding::TypedCovar { .. }) => {
                return Err(Error::ExpectedCovariableGotTerm { span: *span })
            }
        }
    }
    Ok(new_subst)
}

fn check_equality(span: &SourceSpan, expected: &Ty, got: &Ty) -> Result<(), Error> {
    if expected != got {
        return Err(Error::Mismatch {
            span: *span,
            expected: expected.print_to_string(Default::default()),
            got: got.print_to_string(Default::default()),
        });
    }
    Ok(())
}

#[cfg(test)]
mod check_tests {
    use super::{check_args, check_equality, check_module, check_type};
    use crate::{
        parser::util::ToMiette,
        syntax::{
            context::ContextBinding,
            declarations::{
                CodataDeclaration, CtorSig, DataDeclaration, Definition, DtorSig, Module,
            },
            substitution::SubstitutionBinding,
            terms::{Constructor, Lit},
            types::Ty,
        },
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;

    #[test]
    fn module_check() {
        let result = check_module(Module {
            declarations: vec![
                DataDeclaration {
                    span: Span::default(),
                    name: "ListInt".to_owned(),
                    ctors: vec![
                        CtorSig {
                            span: Span::default(),
                            name: "Nil".to_owned(),
                            args: vec![],
                        },
                        CtorSig {
                            span: Span::default(),
                            name: "Cons".to_owned(),
                            args: vec![
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
                            args: vec![],
                            cont_ty: Ty::mk_int(),
                        },
                        DtorSig {
                            span: Span::default(),
                            name: "Tl".to_owned(),
                            args: vec![],
                            cont_ty: Ty::mk_decl("StreamInt"),
                        },
                    ],
                }
                .into(),
                Definition {
                    span: Span::default(),
                    name: "main".to_owned(),
                    context: vec![],
                    ret_ty: Ty::mk_decl("ListInt"),
                    body: Constructor {
                        span: Span::default(),
                        id: "Cons".to_owned(),
                        args: vec![
                            SubstitutionBinding::TermBinding(
                                Lit {
                                    span: Span::default(),
                                    val: 1,
                                }
                                .into(),
                            ),
                            SubstitutionBinding::TermBinding(
                                Constructor {
                                    span: Span::default(),
                                    id: "Nil".to_owned(),
                                    args: vec![],
                                    ty: None,
                                }
                                .into(),
                            ),
                        ],
                        ty: None,
                    }
                    .into(),
                }
                .into(),
            ],
        })
        .unwrap();
        let expected = Module {
            declarations: vec![
                DataDeclaration {
                    span: Span::default(),
                    name: "ListInt".to_owned(),
                    ctors: vec![
                        CtorSig {
                            span: Span::default(),
                            name: "Nil".to_owned(),
                            args: vec![],
                        },
                        CtorSig {
                            span: Span::default(),
                            name: "Cons".to_owned(),
                            args: vec![
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
                            args: vec![],
                            cont_ty: Ty::mk_int(),
                        },
                        DtorSig {
                            span: Span::default(),
                            name: "Tl".to_owned(),
                            args: vec![],
                            cont_ty: Ty::mk_decl("StreamInt"),
                        },
                    ],
                }
                .into(),
                Definition {
                    span: Span::default(),
                    name: "main".to_owned(),
                    context: vec![],
                    ret_ty: Ty::mk_decl("ListInt"),
                    body: Constructor {
                        span: Span::default(),
                        id: "Cons".to_owned(),
                        args: vec![
                            SubstitutionBinding::TermBinding(
                                Lit {
                                    span: Span::default(),
                                    val: 1,
                                }
                                .into(),
                            ),
                            SubstitutionBinding::TermBinding(
                                Constructor {
                                    span: Span::default(),
                                    id: "Nil".to_owned(),
                                    args: vec![],
                                    ty: Some(Ty::mk_decl("ListInt")),
                                }
                                .into(),
                            ),
                        ],
                        ty: Some(Ty::mk_decl("ListInt")),
                    }
                    .into(),
                }
                .into(),
            ],
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_check_int() {
        let result = check_type(&Ty::mk_int(), &SymbolTable::default());
        assert!(result.is_ok())
    }
    #[test]
    fn ty_check_decl() {
        let mut symbol_table = SymbolTable::default();
        symbol_table
            .ty_ctors
            .insert("ListInt".to_owned(), (Polarity::Data, vec![]));
        let result = check_type(&Ty::mk_decl("ListInt"), &symbol_table);
        assert!(result.is_ok())
    }
    #[test]
    fn ty_check_fail() {
        let result = check_type(&Ty::mk_decl("ListInt"), &SymbolTable::default());
        assert!(result.is_err())
    }
    #[test]
    fn equality_check() {
        let result = check_equality(&Span::default().to_miette(), &Ty::mk_int(), &Ty::mk_int());
        assert!(result.is_ok())
    }
    #[test]
    fn equality_check_fail() {
        let result = check_equality(
            &Span::default().to_miette(),
            &Ty::mk_int(),
            &Ty::mk_decl("ListInt"),
        );
        assert!(result.is_err())
    }

    #[test]
    fn check_arg_list() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        symbol_table.ctors.insert("Nil".to_owned(), vec![]);
        symbol_table.ctors.insert(
            "Cons".to_owned(),
            vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedVar {
                    var: "xs".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                },
            ],
        );
        let result = check_args(
            &Span::default().to_miette(),
            &symbol_table,
            &vec![],
            vec![
                SubstitutionBinding::TermBinding(
                    Lit {
                        span: Span::default(),
                        val: 1,
                    }
                    .into(),
                ),
                SubstitutionBinding::TermBinding(
                    Constructor {
                        span: Span::default(),
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: None,
                    }
                    .into(),
                ),
            ],
            &vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedVar {
                    var: "xs".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                },
            ],
        )
        .unwrap();
        let expected = vec![
            SubstitutionBinding::TermBinding(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            ),
            SubstitutionBinding::TermBinding(
                Constructor {
                    span: Span::default(),
                    id: "Nil".to_owned(),
                    args: vec![],
                    ty: Some(Ty::mk_decl("ListInt")),
                }
                .into(),
            ),
        ];
        assert_eq!(result, expected)
    }
    #[test]
    fn check_arg_covar() {
        let result = check_args(
            &Span::default().to_miette(),
            &SymbolTable::default(),
            &vec![
                ContextBinding::TypedCovar {
                    covar: "c".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedCovar {
                    covar: "d".to_owned(),
                    ty: Ty::mk_decl("FunIntInt"),
                },
            ],
            vec![
                SubstitutionBinding::CovarBinding("c".to_owned()),
                SubstitutionBinding::CovarBinding("d".to_owned()),
            ],
            &vec![
                ContextBinding::TypedCovar {
                    covar: "a".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedCovar {
                    covar: "b".to_owned(),
                    ty: Ty::mk_decl("FunIntInt"),
                },
            ],
        )
        .unwrap();
        let expected = vec![
            SubstitutionBinding::CovarBinding("c".to_owned()),
            SubstitutionBinding::CovarBinding("d".to_owned()),
        ];
        assert_eq!(result, expected)
    }
    #[test]
    fn check_fail() {
        let result = check_args(
            &Span::default().to_miette(),
            &SymbolTable::default(),
            &vec![],
            vec![SubstitutionBinding::TermBinding(
                Lit {
                    span: Span::default(),
                    val: 1,
                }
                .into(),
            )],
            &vec![],
        );
        assert!(result.is_err())
    }
}
