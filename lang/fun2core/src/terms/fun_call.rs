use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_subst, compile_ty},
};
use core::syntax::{
    term::{Cns, Prd},
    types::Ty,
};
use fun::syntax::substitution::subst_covars;
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Fun {
    /// ```text
    /// 〚f(t_1, ...; a_1, ...) 〛_{c} = f(〚t_1〛, ...; a_1, ..., c)
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        let mut new_args = compile_subst(self.args, state);
        let ret_ty = compile_ty(self.ret_ty.clone().unwrap());
        new_args.push(
            core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                cns: cont,
                ty: ret_ty.clone(),
            },
        );
        core::syntax::statement::Fun {
            name: self.name,
            args: new_args,
            ret_ty,
        }
        .into()
    }

    fn compile_opt(self, state: &mut CompileState, ty: Ty) -> core::syntax::term::Term<Prd> {
        state.covars.extend(
            subst_covars(&self.args)
                .into_iter()
                .map(|(covar, ty)| (covar, compile_ty(ty))),
        );
        let new_covar = state.free_covar_from_state(ty);
        let ty = compile_ty(self.ret_ty.clone().unwrap());
        let new_statement = self.compile_with_cont(
            core::syntax::term::XVar {
                prdcns: Cns,
                var: new_covar.clone(),
                ty: ty.clone(),
            }
            .into(),
            state,
        );
        core::syntax::term::Mu {
            prdcns: Prd,
            variable: new_covar,
            var_ty: ty,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::{
        parse_term,
        typing::{
            check::terms::Check,
            symbol_table::{Polarity, SymbolTable},
        },
    };

    use crate::definition::{CompileState, CompileWithCont};
    use core::syntax::{
        term::{Cns, Prd},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn compile_fac() {
        let term = parse_term!("fac(3)");
        let mut symbol_table = SymbolTable::default();
        symbol_table.funs.insert(
            "fac".to_owned(),
            (
                vec![fun::syntax::context::ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                }],
                fun::syntax::types::Ty::mk_int(),
            ),
        );
        let term_typed = term
            .check(&symbol_table, &vec![], &fun::syntax::types::Ty::mk_int())
            .unwrap();
        let result = term_typed.compile_opt(&mut CompileState::default(), Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Fun {
                    name: "fac".to_owned(),
                    args: vec![
                        core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                            prd: core::syntax::term::Literal { lit: 3 }.into(),
                            ty: Ty::Int(),
                        },
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                            cns: core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a0".to_owned(),
                                ty: Ty::Int(),
                            }
                            .into(),
                            ty: Ty::Int(),
                        },
                    ],
                    ret_ty: Ty::Int(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_swap() {
        let term = parse_term!("swap(Tup(1,2))");
        let mut symbol_table = SymbolTable::default();
        symbol_table.funs.insert(
            "swap".to_owned(),
            (
                vec![fun::syntax::context::ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: fun::syntax::types::Ty::mk_decl("TupIntInt"),
                }],
                fun::syntax::types::Ty::mk_decl("TupIntInt"),
            ),
        );
        symbol_table.ty_ctors.insert(
            "TupIntInt".to_owned(),
            (Polarity::Data, vec!["Tup".to_owned()]),
        );
        symbol_table.ctors.insert(
            "Tup".to_owned(),
            vec![
                fun::syntax::context::ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                },
                fun::syntax::context::ContextBinding::TypedVar {
                    var: "y".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                },
            ],
        );
        let term_typed = term
            .check(
                &symbol_table,
                &vec![],
                &fun::syntax::types::Ty::mk_decl("TupIntInt"),
            )
            .unwrap();
        let mut state = CompileState::default();

        let result = term_typed.compile_opt(&mut state, Ty::Decl("TupIntInt".to_owned()));
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            var_ty: Ty::Decl("TupIntInt".to_owned()),
            statement: Rc::new(
                core::syntax::statement::Fun {
                    name: "swap".to_owned(),
                    args: vec![
                        core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                            prd: core::syntax::term::Xtor {
                                prdcns: Prd,
                                id: "Tup".to_owned(),
                                args: vec![
                            core::syntax::substitution::SubstitutionBinding::ProducerBinding{
                                prd:core::syntax::term::Literal { lit: 1 }.into(),
                                ty:Ty::Int()
                            },
                            core::syntax::substitution::SubstitutionBinding::ProducerBinding{
                                prd:core::syntax::term::Literal { lit: 2 }.into(),
                                ty:Ty::Int()
                            },
                        ],
                                ty: Ty::Decl("TupIntInt".to_owned()),
                            }
                            .into(),
                            ty: Ty::Decl("TupIntInt".to_owned()),
                        },
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                            cns: core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a0".to_owned(),
                                ty: Ty::Decl("TupIntInt".to_owned()),
                            }
                            .into(),
                            ty: Ty::Decl("TupIntInt".to_owned()),
                        },
                    ],
                    ret_ty: Ty::Decl("TupIntInt".to_owned()),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_multfast() {
        let term = parse_term!("multFast(Nil, 'a0)");
        let mut symbol_table = SymbolTable::default();
        symbol_table.funs.insert(
            "multFast".to_owned(),
            (
                vec![
                    fun::syntax::context::ContextBinding::TypedVar {
                        var: "x".to_owned(),
                        ty: fun::syntax::types::Ty::mk_decl("ListInt"),
                    },
                    fun::syntax::context::ContextBinding::TypedCovar {
                        covar: "a".to_owned(),
                        ty: fun::syntax::types::Ty::mk_int(),
                    },
                ],
                fun::syntax::types::Ty::mk_int(),
            ),
        );
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        symbol_table.ctors.insert("Nil".to_owned(), vec![]);
        symbol_table.ctors.insert(
            "Cons".to_owned(),
            vec![
                fun::syntax::context::ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                },
                fun::syntax::context::ContextBinding::TypedVar {
                    var: "xs".to_owned(),
                    ty: fun::syntax::types::Ty::mk_decl("ListInt"),
                },
            ],
        );
        let term_typed = term
            .check(
                &symbol_table,
                &vec![fun::syntax::context::ContextBinding::TypedCovar {
                    covar: "a0".to_owned(),
                    ty: fun::syntax::types::Ty::mk_int(),
                }],
                &fun::syntax::types::Ty::mk_int(),
            )
            .unwrap();
        let result = term_typed.compile_opt(&mut CompileState::default(), Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a1".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Fun {
                    name: "multFast".to_owned(),
                    args: vec![
                        core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                            prd: core::syntax::term::Xtor {
                                prdcns: Prd,
                                id: "Nil".to_owned(),
                                args: vec![],
                                ty: Ty::Decl("ListInt".to_owned()),
                            }
                            .into(),
                            ty: Ty::Decl("ListInt".to_owned()),
                        },
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                            cns: core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a0".to_owned(),
                                ty: Ty::Int(),
                            }
                            .into(),
                            ty: Ty::Int(),
                        },
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                            cns: core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a1".to_owned(),
                                ty: Ty::Int(),
                            }
                            .into(),
                            ty: Ty::Int(),
                        },
                    ],
                    ret_ty: Ty::Int(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
