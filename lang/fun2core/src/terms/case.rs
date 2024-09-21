use std::rc::Rc;

use crate::definition::{Compile, CompileState, CompileWithCont};
use fun::syntax::context::context_vars;

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
        vars: context_vars(&clause.context).into_iter().collect(),
        covars: vec![],
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
        let term = parse_term!("case Tup(1,2) of { Tup(x: Int, y: Int) => y }");
        let result = term.compile_opt(&mut Default::default());
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
