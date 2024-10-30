use crate::{
    definition::{CompileState, CompileWithCont},
    program::compile_subst,
};
use core::syntax::term::{Cns, Prd};
use fun::syntax::substitution::subst_covars;
use std::rc::Rc;

impl CompileWithCont for fun::syntax::terms::Fun {
    /// ```text
    /// 〚f(t_1, ...; a_1, ...) 〛_{c} = f(〚t_1〛, ...; a_1, ..., c)
    /// ```
    fn compile_with_cont(
        self,
        cont: core::syntax::term::Term<Cns>,
        state: &mut CompileState,
    ) -> core::syntax::Statement {
        let mut new_args = compile_subst(self.args, state);
        let ret_ty = state.definitions.get(&self.name).unwrap();
        new_args.push(
            core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                cns: cont,
                ty: ret_ty.clone(),
            },
        );
        core::syntax::statement::Fun {
            name: self.name,
            args: new_args,
        }
        .into()
    }

    fn compile_opt(self, state: &mut CompileState) -> core::syntax::term::Term<Prd> {
        state.covars.extend(subst_covars(&self.args));
        // default implementation
        let new_covar = state.free_covar_from_state();
        let new_statement = self.compile_with_cont(
            core::syntax::term::XVar {
                prdcns: Cns,
                var: new_covar.clone(),
            }
            .into(),
            state,
        );
        let var_ty = state.vars.get(&new_covar).unwrap().clone();
        core::syntax::term::Mu {
            prdcns: Prd,
            variable: new_covar,
            var_ty,
            statement: Rc::new(new_statement),
        }
        .into()
    }
}

#[cfg(test)]
mod compile_tests {
    use fun::parse_term;

    use crate::definition::CompileWithCont;
    use core::syntax::{
        term::{Cns, Prd},
        types::Ty,
    };
    use std::rc::Rc;

    #[test]
    fn compile_fac() {
        let term = parse_term!("fac(3)");
        let result = term.compile_opt(&mut Default::default());
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Fun {
                    name: "fac".to_owned(),
                    args: vec![
                        core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                            prd: core::syntax::term::Literal { lit: 3 }.into(),
                            ty: Ty::Int(),
                        },
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                            cns: core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a0".to_owned(),
                            }
                            .into(),
                            ty: Ty::Int(),
                        },
                    ],
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
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a0".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Fun {
                    name: "swap".to_owned(),
                    args: vec![
                        core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                            prd: core::syntax::term::Xtor {
                                prdcns: Prd,
                                id: "Tup".to_owned(),
                                args: vec![
                            core::syntax::substitution::SubstitutionBinding::ProducerBinding{
                                prd:core::syntax::term::Literal { lit: 1 }.into(),
                                ty:Ty::Int()
                            },
                            core::syntax::substitution::SubstitutionBinding::ProducerBinding{
                                prd:core::syntax::term::Literal { lit: 2 }.into(),
                                ty:Ty::Int()
                            },
                        ],
                            }
                            .into(),
                            ty: Ty::Decl("TupIntInt".to_owned()),
                        },
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                            cns: core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a0".to_owned(),
                            }
                            .into(),
                            ty: Ty::Int(),
                        },
                    ],
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
        let expected = core::syntax::term::Mu {
            prdcns: Prd,
            variable: "a1".to_owned(),
            var_ty: Ty::Int(),
            statement: Rc::new(
                core::syntax::statement::Fun {
                    name: "multFast".to_owned(),
                    args: vec![
                        core::syntax::substitution::SubstitutionBinding::ProducerBinding {
                            prd: core::syntax::term::Xtor {
                                prdcns: Prd,
                                id: "Nil".to_owned(),
                                args: vec![],
                            }
                            .into(),
                            ty: Ty::Decl("ListInt".to_owned()),
                        },
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                            cns: core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a0".to_owned(),
                            }
                            .into(),
                            ty: Ty::Int(),
                        },
                        core::syntax::substitution::SubstitutionBinding::ConsumerBinding {
                            cns: core::syntax::term::XVar {
                                prdcns: Cns,
                                var: "a1".to_owned(),
                            }
                            .into(),
                            ty: Ty::Int(),
                        },
                    ],
                }
                .into(),
            ),
        }
        .into();
        assert_eq!(result, expected)
    }
}
