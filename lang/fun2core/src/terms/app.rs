use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::App {
    /// ```text
    /// 〚f e〛_{c} = 〚f〛_{ap(〚f〛; c)}
    ///
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        // The new continuation `ap(〚f〛; c)`
        let new_cont = core::syntax::Destructor {
            id: core::syntax::Dtor::Ap,
            producers: vec![self.argument.compile_opt(st)],
            consumers: vec![cont],
        }
        .into();

        // 〚f〛_{new_cont}
        self.function.compile_with_cont(new_cont, st)
    }
}

#[cfg(test)]
mod compile_tests {
    use std::rc::Rc;

    use crate::definition::CompileWithCont;

    /// The function application `f 2`
    fn example() -> fun::syntax::App {
        fun::syntax::App {
            function: Rc::new(fun::syntax::Term::Var("f".to_string())),
            argument: Rc::new(fun::syntax::Term::Lit(2)),
        }
    }

    #[test]
    fn compile() {
        let result = example().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_string(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(
                        core::syntax::Variable {
                            var: "f".to_string(),
                        }
                        .into(),
                    ),
                    consumer: Rc::new(
                        core::syntax::Destructor {
                            id: core::syntax::Dtor::Ap,
                            producers: vec![core::syntax::Literal { lit: 2 }.into()],
                            consumers: vec![core::syntax::Covariable {
                                covar: "a0".to_string(),
                            }
                            .into()],
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
