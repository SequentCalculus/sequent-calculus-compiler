use std::rc::Rc;

use crate::definition::{CompileState, CompileWithCont};

impl CompileWithCont for fun::syntax::terms::Label {
    /// ```text
    /// 〚label a {t} 〛_{c} = ⟨μa. 〚t 〛_{a} | c⟩
    /// 〚label a {t} 〛 = μa. 〚t 〛_{a}
    /// ```
    fn compile_opt(self, state: &mut CompileState) -> core::syntax::Producer {
        let cont = core::syntax::Covariable {
            covar: self.label.clone(),
        }
        .into();

        core::syntax::Mu {
            covariable: self.label,
            statement: Rc::new(self.term.compile_with_cont(cont, state)),
        }
        .into()
    }

    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        core::syntax::Cut {
            producer: Rc::new(self.compile_opt(state)),
            consumer: Rc::new(cont),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {

    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    fn example_label1() -> fun::syntax::terms::Label {
        fun::syntax::terms::Label {
            label: "a".to_owned(),
            term: Rc::new(fun::syntax::terms::Term::Lit(1)),
        }
    }

    fn example_label2() -> fun::syntax::terms::Label {
        fun::syntax::terms::Label {
            label: "a".to_owned(),
            term: Rc::new(
                fun::syntax::terms::Goto {
                    term: Rc::new(fun::syntax::terms::Term::Lit(1)),
                    target: "a".to_owned(),
                }
                .into(),
            ),
        }
    }

    #[test]
    fn compile_label1() {
        let result = example_label1().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(core::syntax::Literal { lit: 1 }.into()),
                    consumer: Rc::new(
                        core::syntax::Covariable {
                            covar: "a".to_owned(),
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
    fn compile_label2() {
        let result = example_label2().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a".to_owned(),
            statement: Rc::new(
                core::syntax::Cut {
                    producer: Rc::new(core::syntax::Literal { lit: 1 }.into()),
                    consumer: Rc::new(
                        core::syntax::Covariable {
                            covar: "a".to_owned(),
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
