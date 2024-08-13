use crate::definition::{CompileState, CompileWithCont};
use std::rc::Rc;

impl CompileWithCont for fun::syntax::Fun {
    /// ```text
    /// 〚f(t_1,...;a_1,...) 〛_{c} = f(〚t_1〛,... ;a_1,...,c)
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        st: &mut CompileState,
    ) -> core::syntax::Statement {
        let mut new_coargs: Vec<core::syntax::Consumer> = self
            .coargs
            .into_iter()
            .map(|cv| core::syntax::Covariable { covar: cv }.into())
            .collect();
        new_coargs.push(cont);
        core::syntax::Fun {
            name: self.name,
            producers: self.args.into_iter().map(|p| p.compile_opt(st)).collect(),
            consumers: new_coargs,
        }
        .into()
    }
    fn compile_opt(self, st: &mut CompileState) -> core::syntax::Producer {
        st.covars.extend(self.coargs.clone());
        // default implementation
        let new_cv = st.free_covar_from_state();
        let new_st = self.compile_with_cont(
            core::syntax::Covariable {
                covar: new_cv.clone(),
            }
            .into(),
            st,
        );
        core::syntax::Mu {
            covariable: new_cv,
            statement: Rc::new(new_st),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    fn example_fac() -> fun::syntax::Fun {
        fun::syntax::Fun {
            name: "fac".to_owned(),
            args: vec![fun::syntax::Term::Lit(3)],
            coargs: vec![],
        }
    }
    fn example_swap() -> fun::syntax::Fun {
        fun::syntax::Fun {
            name: "swap".to_owned(),
            args: vec![fun::syntax::Constructor {
                id: fun::syntax::Ctor::Tup,
                args: vec![fun::syntax::Term::Lit(1), fun::syntax::Term::Lit(2)],
            }
            .into()],
            coargs: vec![],
        }
    }
    fn example_multfast() -> fun::syntax::Fun {
        fun::syntax::Fun {
            name: "multFast".to_owned(),
            args: vec![fun::syntax::Constructor {
                id: fun::syntax::Ctor::Nil,
                args: vec![],
            }
            .into()],
            coargs: vec!["a0".to_owned()],
        }
    }

    #[test]
    fn compile_fac() {
        let result = example_fac().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Fun {
                    name: "fac".to_owned(),
                    producers: vec![core::syntax::Literal { lit: 3 }.into()],
                    consumers: vec![core::syntax::Covariable {
                        covar: "a0".to_owned(),
                    }
                    .into()],
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn compile_swap() {
        let result = example_swap().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a0".to_owned(),
            statement: Rc::new(
                core::syntax::Fun {
                    name: "swap".to_owned(),
                    producers: vec![core::syntax::Constructor {
                        id: core::syntax::Ctor::Tup,
                        producers: vec![
                            core::syntax::Literal { lit: 1 }.into(),
                            core::syntax::Literal { lit: 2 }.into(),
                        ],
                        consumers: vec![],
                    }
                    .into()],
                    consumers: vec![core::syntax::Covariable {
                        covar: "a0".to_owned(),
                    }
                    .into()],
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
    #[test]
    fn compile_multfast() {
        let result = example_multfast().compile_opt(&mut Default::default());
        let expected = core::syntax::Mu {
            covariable: "a1".to_owned(),
            statement: Rc::new(
                core::syntax::Fun {
                    name: "multFast".to_owned(),
                    producers: vec![core::syntax::Constructor {
                        id: core::syntax::Ctor::Nil,
                        producers: vec![],
                        consumers: vec![],
                    }
                    .into()],
                    consumers: vec![
                        core::syntax::Covariable {
                            covar: "a0".to_owned(),
                        }
                        .into(),
                        core::syntax::Covariable {
                            covar: "a1".to_owned(),
                        }
                        .into(),
                    ],
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
