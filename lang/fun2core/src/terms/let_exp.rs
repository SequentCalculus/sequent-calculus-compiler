use crate::definition::{CompileState, CompileWithCont};
use core::syntax::term::Cns;
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Let {
    /// ```text
    /// 〚let x := t_1 in t_2 〛_{c} = 〚t_1 〛_{μ~x.〚t_2 〛_{c}}
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        // new continuation: μ~x.〚t_2 〛_{c}
        let new_cont = core::syntax::term::Mu {
            prdcns: Cns,
            variable: self.variable,
            statement: Rc::new(self.in_term.compile_with_cont(cont, state)),
        };

        // 〚t_1 〛_{new_cont}
        self.bound_term.compile_with_cont(new_cont.into(), state)
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use core::syntax::term::{Cns, Prd};
    use std::rc::Rc;

    #[test]
    fn compile_let1() {
        let term = parse_term!("let x : Int = 1 in x * x");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(core::syntax::term::Literal { lit: 1 }.into()),
                    consumer: Rc::new(
                        core::syntax::term::Mu {
                            prdcns: Cns,
                            variable: "x".to_owned(),
                            statement: Rc::new(
                                core::syntax::statement::Op {
                                    fst: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                        }
                                        .into(),
                                    ),
                                    op: core::syntax::BinOp::Prod,
                                    snd: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                        }
                                        .into(),
                                    ),
                                    continuation: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Cns,
                                            var: "a0".to_owned(),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
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
    fn compile_let2() {
        let term = parse_term!("let x : ListInt = Cons(x,Nil) in x");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::statement::Cut {
                    producer: Rc::new(
                        core::syntax::term::Xtor {
                            prdcns: Prd,
                            id: "Cons".to_owned(),
                            args: vec![
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core::syntax::term::XVar {
                                        prdcns: Prd,
                                        var: "x".to_owned(),
                                    }
                                    .into(),
                                ),
                                core::syntax::substitution::SubstitutionBinding::ProducerBinding(
                                    core::syntax::term::Xtor {
                                        prdcns: Prd,
                                        id: "Nil".to_owned(),
                                        args: vec![],
                                    }
                                    .into(),
                                ),
                            ],
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        core::syntax::term::Mu {
                            prdcns: Cns,
                            variable: "x".to_owned(),
                            statement: Rc::new(
                                core::syntax::statement::Cut {
                                    producer: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Prd,
                                            var: "x".to_owned(),
                                        }
                                        .into(),
                                    ),
                                    consumer: Rc::new(
                                        core::syntax::term::XVar {
                                            prdcns: Cns,
                                            var: "a0".to_owned(),
                                        }
                                        .into(),
                                    ),
                                }
                                .into(),
                            ),
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
