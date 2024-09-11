use crate::definition::{CompileState, CompileWithCont};
use fun::syntax::substitution::split_subst;
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Fun {
    /// ```text
    /// 〚f(t_1, ...; a_1, ...) 〛_{c} = f(〚t_1〛, ...; a_1, ..., c)
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::Consumer,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        let (pargs, cargs) = split_subst(self.args);
        let mut new_coargs: Vec<core::syntax::Consumer> = cargs
            .into_iter()
            .map(|cv| core::syntax::Covariable { covar: cv }.into())
            .collect();
        new_coargs.push(cont);
        core::syntax::Fun {
            name: self.name,
            producers: pargs.into_iter().map(|p| p.compile_opt(state)).collect(),
            consumers: new_coargs,
        }
        .into()
    }

    fn compile_opt(self, state: &mut CompileState) -> core::syntax::Producer {
        let (_, cargs) = split_subst(self.args.clone());
        state.covars.extend(cargs.clone());
        // default implementation
        let new_covar = state.free_covar_from_state();
        let new_statement = self.compile_with_cont(
            core::syntax::Covariable {
                covar: new_covar.clone(),
            }
            .into(),
            state,
        );
        core::syntax::Mu {
            covariable: new_covar,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use std::rc::Rc;

    #[test]
    fn compile_fac() {
        let term = parse_term!("fac(3)");
        let result = term.compile_opt(&mut Default::default());
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
        let term = parse_term!("swap(Tup(1,2))");
        let result = term.compile_opt(&mut Default::default());
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
        let term = parse_term!("multFast(Nil, 'a0)");
        let result = term.compile_opt(&mut Default::default());
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
