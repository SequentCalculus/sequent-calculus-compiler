use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_subst,
};
use core::syntax::term::Cns;
use fun::syntax::substitution::subst_covars;

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
                .keys()
                .cloned()
                .collect::<Vec<String>>(),
        );
        let mut args = compile_subst(self.args, state);
        args.push(core::syntax::substitution::SubstitutionBinding::ConsumerBinding(cont));
        // new continuation: D(〚t_1〛, ...); c)
        let new_cont = core::syntax::term::Xtor {
            prdcns: Cns,
            id: self.id,
            args,
        }
        .into();

        // 〚t〛_{new_cont}
        self.destructee.compile_with_cont(new_cont, state)
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use core::syntax::{
        context::ContextBinding,
        term::{Cns, Prd},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn compile_fst() {
        let term = parse_term!("cocase { Fst => 1, Snd => 2}.Fst");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
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
                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a1".to_owned(),
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
                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a2".to_owned(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                            ],
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        core::syntax::term::Xtor {
                            prdcns: Cns,
                            id: "Fst".to_owned(),
                            args: vec![
                                core::syntax::substitution::SubstitutionBinding::ConsumerBinding(
                                    core::syntax::term::XVar {
                                        prdcns: Cns,
                                        var: "a0".to_owned(),
                                    }
                                    .into(),
                                ),
                            ],
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
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
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
                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a1".to_owned(),
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
                                            consumer: Rc::new(
                                                core::syntax::term::XVar {
                                                    prdcns: Cns,
                                                    var: "a2".to_owned(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                            ],
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        core::syntax::term::Xtor {
                            prdcns: Cns,
                            id: "Snd".to_owned(),
                            args: vec![
                                core::syntax::substitution::SubstitutionBinding::ConsumerBinding(
                                    core::syntax::term::XVar {
                                        prdcns: Cns,
                                        var: "a0".to_owned(),
                                    }
                                    .into(),
                                ),
                            ],
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
