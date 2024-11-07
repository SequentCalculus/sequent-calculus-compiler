use crate::{
    definition::{CompileState, CompileWithCont},
    program::{compile_subst, compile_ty},
};
use core::syntax::term::Cns;
use fun::syntax::{substitution::subst_covars, types::OptTyped};

impl CompileWithCont for fun::syntax::terms::Destructor {
    /// ```text
    /// 〚t.D(t_1, ...) 〛_{c} = 〚t〛_{D(〚t_1〛, ...); c)}
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        state.covars.extend(
            subst_covars(&self.args)
                .into_iter()
                .map(|(covar, ty)| (covar, compile_ty(ty))),
        );
        let mut args = compile_subst(self.args, state);
        args.push(
            core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                cns: cont,
                ty: compile_ty(self.ty.unwrap()),
            },
        );
        // new continuation: D(〚t_1〛, ...); c)
        let new_cont = core::syntax::term::Xtor {
            prdcns: Cns,
            id: self.id,
            args,
            ty: compile_ty(self.destructee.get_type().unwrap()),
        }
        .into();

        // 〚t〛_{new_cont}
        self.destructee.compile_with_cont(new_cont, state)
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
        context::ContextBinding,
        term::{Cns, Prd},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn compile_fst() {
        let term = parse_term!("cocase { Fst => 1, Snd => 2}.Fst");
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "LPairIntInt".to_owned(),
            (Polarity::Codata, vec!["Fst".to_owned(), "Snd".to_owned()]),
        );
        symbol_table
            .dtors
            .insert("Fst".to_owned(), (vec![], fun::syntax::types::Ty::mk_int()));
        symbol_table
            .dtors
            .insert("Snd".to_owned(), (vec![], fun::syntax::types::Ty::mk_int()));
        let term_typed = term
            .check(&symbol_table, &vec![], &fun::syntax::types::Ty::mk_int())
            .unwrap();
        let mut st = CompileState::default();

        let result = term_typed.compile_opt(&mut st, Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(
                        core::syntax::term::XCase {
                            prdcns: Prd,
                            clauses: vec![
                                core::syntax::Clause {
                                    xtor: "Fst".to_owned(),
                                    context: vec![ContextBinding::CovarBinding {
                                        covar: "a1".to_owned(),
                                        ty: Ty::Int(),
                                    }],
                                    rhs: Rc::new(
                                        core::syntax::statement::Cut {
                                            producer: Rc::new(
                                                core::syntax::term::Literal { lit: 1 }.into(),
                                            ),
                                            ty: Ty::Int(),
                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a1".to_owned(),
                                                    ty: Ty::Int(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                                core::syntax::Clause {
                                    xtor: "Snd".to_owned(),
                                    context: vec![ContextBinding::CovarBinding {
                                        covar: "a2".to_owned(),
                                        ty: Ty::Int(),
                                    }],
                                    rhs: Rc::new(
                                        core::syntax::statement::Cut {
                                            producer: Rc::new(
                                                core::syntax::term::Literal { lit: 2 }.into(),
                                            ),
                                            ty: Ty::Int(),
                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a2".to_owned(),
                                                    ty: Ty::Int(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                            ],
                            ty: Ty::Decl("LPairIntInt".to_owned()),
                        }
                        .into(),
                    ),
                    ty: Ty::Decl("LPairIntInt".to_owned()),
                    consumer: Rc::new(
                        core::syntax::term::Xtor {
                            prdcns: Cns,
                            id: "Fst".to_owned(),
                            args: vec![
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
                            ty: Ty::Decl("LPairIntInt".to_owned()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_snd() {
        let term = parse_term!("cocase { Fst => 1, Snd => 2}.Snd");
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "LPairIntInt".to_owned(),
            (Polarity::Codata, vec!["Fst".to_owned(), "Snd".to_owned()]),
        );
        symbol_table
            .dtors
            .insert("Fst".to_owned(), (vec![], fun::syntax::types::Ty::mk_int()));
        symbol_table
            .dtors
            .insert("Snd".to_owned(), (vec![], fun::syntax::types::Ty::mk_int()));
        let term_typed = term
            .check(&symbol_table, &vec![], &fun::syntax::types::Ty::mk_int())
            .unwrap();
        let mut st = CompileState::default();

        let result = term_typed.compile_opt(&mut st, Ty::Int());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(
                        core::syntax::term::XCase {
                            prdcns: Prd,
                            clauses: vec![
                                core::syntax::Clause {
                                    xtor: "Fst".to_owned(),
                                    context: vec![ContextBinding::CovarBinding {
                                        covar: "a1".to_owned(),
                                        ty: Ty::Int(),
                                    }],
                                    rhs: Rc::new(
                                        core::syntax::statement::Cut {
                                            producer: Rc::new(
                                                core::syntax::term::Literal { lit: 1 }.into(),
                                            ),
                                            ty: Ty::Int(),
                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a1".to_owned(),
                                                    ty: Ty::Int(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                                core::syntax::Clause {
                                    xtor: "Snd".to_owned(),
                                    context: vec![ContextBinding::CovarBinding {
                                        covar: "a2".to_owned(),
                                        ty: Ty::Int(),
                                    }],
                                    rhs: Rc::new(
                                        core::syntax::statement::Cut {
                                            producer: Rc::new(
                                                core::syntax::term::Literal { lit: 2 }.into(),
                                            ),
                                            ty: Ty::Int(),
                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a2".to_owned(),
                                                    ty: Ty::Int(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                            ],
                            ty: Ty::Decl("LPairIntInt".to_owned()),
                        }
                        .into(),
                    ),
                    ty: Ty::Decl("LPairIntInt".to_owned()),
                    consumer: Rc::new(
                        core::syntax::term::Xtor {
                            prdcns: Cns,
                            id: "Snd".to_owned(),
                            args: vec![
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
                            ty: Ty::Decl("LPairIntInt".to_owned()),
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
