use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::Case {
    /// ```text
    /// 〚case t of { K_1(x_11, ...) => t_1, ...} 〛_{c} = 〚t〛_{case{ K_1(x_11, ...) => 〚t_1〛_{c}, ... }}
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        let clauses_compiled = self
            .cases
            .into_iter()
            .map(|clause| compile_clause(clause, cont.clone(), st))
            .collect();
        //the new continuation case{ K_1(x_11,...) => 〚t_1〛_{c}, ... }
        let new_cont = core::syntax::Case {
            cases: clauses_compiled,
        }
        .into();
        //〚t〛_{new_cont}
        Rc::unwrap_or_clone(self.destructee).compile_with_cont(new_cont, st)
    }
}

fn compile_clause(
    clause: fun::syntax::Clause<fun::syntax::Ctor>,
    cont: core::syntax::Consumer,
    st: &mut CompileState,
) -> core::syntax::Clause<core::syntax::Ctor> {
    core::syntax::Clause {
        xtor: clause.xtor.compile(st),
        vars: clause.vars,
        covars: vec![],
        rhs: Rc::new(clause.rhs.compile_with_cont(cont, st)),
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    fn list_example() -> fun::syntax::Case {
        let list = fun::syntax::Constructor {
            id: fun::syntax::Ctor::Cons,
            args: vec![
                fun::syntax::Term::Lit(1),
                fun::syntax::Constructor {
                    id: fun::syntax::Ctor::Nil,
                    args: vec![],
                }
                .into(),
            ],
        };
        let case_nil = fun::syntax::Clause {
            xtor: fun::syntax::Ctor::Nil,
            vars: vec![],
            rhs: fun::syntax::Term::Lit(0),
        };
        let case_cons = fun::syntax::Clause {
            xtor: fun::syntax::Ctor::Cons,
            vars: vec!["x".to_owned(), "xs".to_owned()],
            rhs: fun::syntax::Term::Var("x".to_owned()),
        };
        fun::syntax::Case {
            destructee: Rc::new(list.into()),
            cases: vec![case_nil, case_cons],
        }
    }

    fn tup_example() -> fun::syntax::Case {
        let tuple = fun::syntax::Constructor {
            id: fun::syntax::Ctor::Tup,
            args: vec![fun::syntax::Term::Lit(1), fun::syntax::Term::Lit(2)],
        };
        let clause = fun::syntax::Clause {
            xtor: fun::syntax::Ctor::Tup,
            vars: vec!["x".to_owned(), "y".to_owned()],
            rhs: fun::syntax::Term::Var("y".to_owned()),
        };
        fun::syntax::Case {
            destructee: Rc::new(tuple.into()),
            cases: vec![clause],
        }
    }

    #[test]
    fn compile_list() {
        let result = list_example().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Constructor {
                            id: core::syntax::Ctor::Cons,
                            producers: vec![
                                core::syntax::Literal { lit: 1 }.into(),
                                core::syntax::Constructor {
                                    id: core::syntax::Ctor::Nil,
                                    producers: vec![],
                                    consumers: vec![],
                                }
                                .into(),
                            ],
                            consumers: vec![],
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        core::syntax::Case {
                            cases: vec![
                                core::syntax::Clause {
                                    xtor: core::syntax::Ctor::Nil,
                                    vars: vec![],
                                    covars: vec![],
                                    rhs: Rc::new(
                                        core::syntax::Cut {
                                            producer: Rc::new(
                                                core::syntax::Literal { lit: 0 }.into(),
                                            ),
                                            consumer: Rc::new(
                                                core::syntax::Covariable {
                                                    covar: "a0".to_owned(),
                                                }
                                                .into(),
                                            ),
                                        }
                                        .into(),
                                    ),
                                },
                                core::syntax::Clause {
                                    xtor: core::syntax::Ctor::Cons,
                                    vars: vec!["x".to_owned(), "xs".to_owned()],
                                    covars: vec![],
                                    rhs: Rc::new(
                                        core::syntax::Cut {
                                            producer: Rc::new(
                                                core::syntax::Variable {
                                                    var: "x".to_owned(),
                                                }
                                                .into(),
                                            ),
                                            consumer: Rc::new(
                                                core::syntax::Covariable {
                                                    covar: "a0".to_owned(),
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
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected);
    }

    #[test]
    fn compile_tup() {
        let result = tup_example().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Constructor {
                            id: core::syntax::Ctor::Tup,
                            producers: vec![
                                core::syntax::Literal { lit: 1 }.into(),
                                core::syntax::Literal { lit: 2 }.into(),
                            ],
                            consumers: vec![],
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        core::syntax::Case {
                            cases: vec![core::syntax::Clause {
                                xtor: core::syntax::Ctor::Tup,
                                vars: vec!["x".to_owned(), "y".to_owned()],
                                covars: vec![],
                                rhs: Rc::new(
                                    core::syntax::Cut {
                                        producer: Rc::new(
                                            core::syntax::Variable {
                                                var: "y".to_owned(),
                                            }
                                            .into(),
                                        ),
                                        consumer: Rc::new(
                                            core::syntax::Covariable {
                                                covar: "a0".to_owned(),
                                            }
                                            .into(),
                                        ),
                                    }
                                    .into(),
                                ),
                            }],
                        }
                        .into(),
                    ),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected);
    }
}
