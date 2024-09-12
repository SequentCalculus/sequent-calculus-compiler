use std::rc::Rc;

use crate::{
    definition::{Compile, CompileState, CompileWithCont},
    program::compile_context,
};

impl CompileWithCont for fun::syntax::terms::Case {
    /// ```text
    /// 〚case t of { K_1(x_11, ...) => t_1, ...} 〛_{c} = 〚t〛_{case{ K_1(x_11, ...) => 〚t_1〛_{c}, ... }}
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        // new continuation: case{ K_1(x_11,...) => 〚t_1〛_{c}, ... }
        let new_cont = core::syntax::Case {
            cases: self
                .cases
                .into_iter()
                .map(|clause| compile_clause(clause, cont.clone(), state))
                .collect(),
        }
        .into();

        // 〚t〛_{new_cont}
        Rc::unwrap_or_clone(self.destructee).compile_with_cont(new_cont, state)
    }
}

fn compile_clause(
    clause: fun::syntax::terms::Clause<fun::syntax::Ctor>,
    cont: core::syntax::Consumer,
    state: &mut CompileState,
) -> core::syntax::Clause<core::syntax::Ctor> {
    core::syntax::Clause {
        xtor: clause.xtor.compile(state),
        context: compile_context(clause.context),
        rhs: Rc::new(clause.rhs.compile_with_cont(cont, state)),
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;
    use fun::parse_term;
    use std::rc::Rc;

    #[test]
    fn compile_list() {
        let term = parse_term!("case Cons(1,Nil) of { Nil => 0, Cons(x : Int,xs : ListInt) => x }");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Constructor {
                            id: core::syntax::Ctor::Cons,
                            args: vec![
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core::syntax::Literal { lit: 1 }.into(),
                                ),
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core::syntax::Constructor {
                                        id: core::syntax::Ctor::Nil,
                                        args: vec![],
                                    }
                                    .into(),
                                ),
                            ],
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        core::syntax::Case {
                            cases: vec![
                                core::syntax::Clause {
                                    xtor: core::syntax::Ctor::Nil,
                                    context: vec![],
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
                                    context: vec![
                                        core::syntax::context::ContextBinding::VarBinding {
                                            var: "x".to_owned(),
                                            ty: core::syntax::types::Ty::Int(),
                                        },
                                        core::syntax::context::ContextBinding::VarBinding {
                                            var: "xs".to_owned(),
                                            ty: core::syntax::types::Ty::Decl("ListInt".to_owned()),
                                        },
                                    ],
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
        let term = parse_term!("case Tup(1,2) of { Tup(x: Int, y: Int) => y }");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Constructor {
                            id: core::syntax::Ctor::Tup,
                            args: vec![
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core::syntax::Literal { lit: 1 }.into(),
                                ),
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core::syntax::Literal { lit: 2 }.into(),
                                ),
                            ],
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        core::syntax::Case {
                            cases: vec![core::syntax::Clause {
                                xtor: core::syntax::Ctor::Tup,
                                context: vec![
                                    core::syntax::context::ContextBinding::VarBinding {
                                        var: "x".to_owned(),
                                        ty: core::syntax::types::Ty::Int(),
                                    },
                                    core::syntax::context::ContextBinding::VarBinding {
                                        var: "y".to_owned(),
                                        ty: core::syntax::types::Ty::Int(),
                                    },
                                ],
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
