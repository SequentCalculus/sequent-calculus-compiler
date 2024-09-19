use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::terms::Paren {
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::Producer {
        self.inner.compile_opt(state)
    }

    fn compile_with_cont(
        self,
        c: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        self.inner.compile_with_cont(c, state)
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;
    use fun::syntax::terms::{Lit, Paren, Term};
    use std::rc::Rc;

    fn example_paren1() -> Paren {
        Paren {
            inner: Rc::new(Lit { val: 1 }.into()),
        }
    }

    fn example_paren2() -> Paren {
        Paren {
            inner: Rc::new(Term::Var("x".to_owned())),
        }
    }

    #[test]
    fn compile_paren1() {
        let result = example_paren1().compile_opt(&mut Default::default());
        let expected = core::syntax::Literal { lit: 1 }.into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_inner_paren1() {
        let result = example_paren1().compile_with_cont(
            core::syntax::Covariable {
                covar: "a".to_owned(),
            }
            .into(),
            &mut Default::default(),
        );
        let expected = core::syntax::Cut {
            producer: Rc::new(core::syntax::Literal { lit: 1 }.into()),
            consumer: Rc::new(
                core::syntax::Covariable {
                    covar: "a".to_owned(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_paren2() {
        let result = example_paren2().compile_opt(&mut Default::default());
        let expected = core::syntax::Variable {
            var: "x".to_owned(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_inner_paren2() {
        let result = example_paren2().compile_with_cont(
            core::syntax::Covariable {
                covar: "a".to_owned(),
            }
            .into(),
            &mut Default::default(),
        );
        let expected = core::syntax::Cut {
            producer: Rc::new(
                core::syntax::Variable {
                    var: "x".to_owned(),
                }
                .into(),
            ),
            consumer: Rc::new(
                core::syntax::Covariable {
                    covar: "a".to_owned(),
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
